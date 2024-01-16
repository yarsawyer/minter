use std::{path::Path, fmt::Debug, sync::Arc};

use anyhow::Context;
use tracing::{info, trace};

pub struct Database {
    db: rocksdb::DB,
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Arc<Database>> {
        let db = rocksdb::DB::open_default(path).context("Failed to open DB")?;
        info!("RocksDB connected");
        Ok(Arc::new(Self { db }))
    }

    pub fn set_raw(&self, k: impl AsRef<[u8]>, v: impl AsRef<[u8]>) -> anyhow::Result<()> {
        self.db.put(k, v).context("Failed to put value to DB")
    }

    pub fn set(&self, k: &(impl serde::Serialize + Debug), v: &impl serde::Serialize) -> anyhow::Result<()> {
        trace!("db set at {k:?}");
        let k = bincode::serialize(k).context("Failed to serialize key")?;
        let v = bincode::serialize(v).context("Failed to serialize val")?;
        self.set_raw(k, v)
    }

    pub fn get_raw(&self, k: impl AsRef<[u8]>) -> anyhow::Result<Option<Vec<u8>>> {
        self.db.get(k).context("Failed to get value from DB")
    }
    pub fn get_ref_raw(&self, k: impl AsRef<[u8]>) -> anyhow::Result<Option<rocksdb::DBPinnableSlice>> {
        self.db.get_pinned(k).context("Failed to get value from DB")
    }

    pub fn get<T: for<'a> serde::Deserialize<'a>>(&self, k: &(impl serde::Serialize + Debug)) -> anyhow::Result<Option<T>> {
        trace!("db get at {k:?}");
        let k = bincode::serialize(k).context("Failed to serialize key")?;
        let v = self.get_ref_raw(k)?;
        v.map(|x| bincode::deserialize::<T>(&x).context("Failed to deserialize val")).transpose()
    }
}

