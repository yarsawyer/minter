use std::{path::Path, fmt::Debug, sync::Arc};

use anyhow::Context;
use rocksdb::DBAccess;
use tracing::{info, trace};

pub struct Database {
    pub db: rocksdb::DB,
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

    pub fn remove(&self, k: &(impl serde::Serialize + Debug)) -> anyhow::Result<()> {
        trace!("db remove at {k:?}");
        let k = bincode::serialize(k).context("Failed to serialize key")?;
        self.db.delete(k).context("Failed to delete value from DB")
    }

    pub fn contains(&self, k: &impl serde::Serialize) -> anyhow::Result<bool> {
        let k = bincode::serialize(k).context("Failed to serialize key")?;
        Ok(self.db.get_pinned(k).context("Failed to get value from DB")?.is_some())
    }

    pub fn flush(&self) -> anyhow::Result<()> {
        trace!("db flush");
        self.db.flush().context("Failed to flush DB")
    }

    pub fn iterate(&self, k: &(impl serde::Serialize + Debug)) -> anyhow::Result<ScanIterator> {
        let k = bincode::serialize(k).context("Failed to serialize key")?;
        Ok(ScanIterator {
            iter: self.db.prefix_iterator(&k),
            prefix: k,
            done: false,
        })
    }
}

pub struct ScanIterator<'a> {
    prefix: Vec<u8>,
    iter: rocksdb::DBIterator<'a>,
    done: bool,
}

impl<'a> Iterator for ScanIterator<'a> {
    type Item = (Box<[u8]>, Box<[u8]>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None; }
        let (key, value) = self.iter.next().map(Result::ok)??;
        if !key.starts_with(&self.prefix) {
            self.done = true;
            return None;
        }
        Some((key,value))
    }
}

