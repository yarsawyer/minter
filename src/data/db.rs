use std::{fmt::Debug, path::Path, sync::Arc};

use anyhow::Context;
use tracing::{info, trace};

pub struct Database {
    pub db: rocksdb::DB,
}

pub type DbTable<'a> = rocksdb::ColumnFamilyRef<'a>;

#[derive(Clone)]
pub struct OwnedDbTable {
    name: String,
    table: DbTable<'static>, // 🤡
    _owner: Arc<Database>,
}
impl OwnedDbTable {
    pub fn from_bounded(db: Arc<Database>, table: DbTable, name: String) -> Self {
        Self {
            name,
            table: unsafe { std::mem::transmute::<_,DbTable<'static>>(table) },
            _owner: db,
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn table(&self) -> &DbTable { &self.table }
}
unsafe impl Send for OwnedDbTable {}
unsafe impl Sync for OwnedDbTable {}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Arc<Database>> {
        let db = rocksdb::DB::open_default(path).context("Failed to open DB")?;
        info!("RocksDB connected");
        Ok(Arc::new(Self { db }))
    }

    pub fn column_family<'this: 'db, 'db>(&'this self, name: &'this str) -> anyhow::Result<DbTable<'db>> {
        if let Some(cf) = self.db.cf_handle(name) { return Ok(cf); }
        self.db.create_cf(name, &rocksdb::Options::default()).context("Failed to create column family for DB")?;
        Ok(self.db.cf_handle(name).unwrap())
    }
    pub fn owned_column_family(self: &Arc<Self>, name: &str) -> anyhow::Result<OwnedDbTable> {
        Ok(OwnedDbTable::from_bounded(self.clone(), self.column_family(name)?, name.to_owned()))
    }
    pub fn recreate_column_family(&self, name: &str) -> anyhow::Result<()> {
        self.db.drop_cf(name).context("Failed to drop column family")?;
        self.db.create_cf(name, &rocksdb::Options::default()).context("Failed to create column family for DB")
    }

    pub fn set_raw(&self, f: &DbTable, k: impl AsRef<[u8]>, v: impl AsRef<[u8]>) -> anyhow::Result<()> {
        self.db.put_cf(f, k, v).context("Failed to put value to DB")
    }

    pub fn set(&self, f: &DbTable, k: impl AsRef<[u8]> + Debug, v: &impl serde::Serialize) -> anyhow::Result<()> {
        trace!("db set at {k:?}");
        let v = bincode::serialize(v).context("Failed to serialize val")?;
        self.set_raw(f, k, v)
    }

    pub fn set_many(&self, f: &DbTable, kv: impl IntoIterator<Item = (impl AsRef<[u8]> + Debug, impl serde::Serialize)>) -> anyhow::Result<()> {
        let mut batch = rocksdb::WriteBatch::default();
        for (k,v) in kv.into_iter() {
            let v = bincode::serialize(&v).context("Failed to serialize val")?;
            batch.put_cf(f, k, v);
        }
        self.db.write(batch).context("Failed to delete rows")
    }

    pub fn get_raw(&self, f: &DbTable, k: impl AsRef<[u8]>) -> anyhow::Result<Option<Vec<u8>>> {
        self.db.get_cf(f, k).context("Failed to get value from DB")
    }
    pub fn get_ref_raw(&self, f: &DbTable, k: impl AsRef<[u8]>) -> anyhow::Result<Option<rocksdb::DBPinnableSlice>> {
        self.db.get_pinned_cf(f, k).context("Failed to get value from DB")
    }

    pub fn get<T: for<'a> serde::Deserialize<'a>>(&self, f: &DbTable, k: impl AsRef<[u8]> + Debug) -> anyhow::Result<Option<T>> {
        trace!("db get at {k:?}");
        let v = self.get_ref_raw(f, k)?;
        v.map(|x| bincode::deserialize::<T>(&x).context("Failed to deserialize val")).transpose()
    }

    pub fn remove(&self, f: &DbTable, k: impl AsRef<[u8]> + Debug) -> anyhow::Result<()> {
        trace!("db remove at {k:?}");
        self.db.delete_cf(f, k).context("Failed to delete value from DB")
    }

    //todo: optimize
    pub fn remove_where(&self, f: &DbTable, prefix: Vec<u8>, predicate: impl Fn(&[u8]) -> bool) -> anyhow::Result<usize> {
        let mut batch = rocksdb::WriteBatch::default();
        let mut c = 0;
        for (k,_) in self.iterate(f, prefix).context("Failed to iterate rows for delete")? {
            if !predicate(&k) { continue; } 
            batch.delete_cf(f, k);
            c += 1;
        }
        self.db.write(batch).context("Failed to delete rows")?;
        Ok(c)
    }

    pub fn contains(&self, f: &DbTable, k: impl AsRef<[u8]> + Debug) -> anyhow::Result<bool> {
        Ok(self.db.get_pinned_cf(f, k).context("Failed to get value from DB")?.is_some())
    }

    pub fn flush(&self) -> anyhow::Result<()> {
        trace!("db flush");
        self.db.flush().context("Failed to flush DB")
    }

    pub fn iterate(&self, f: &DbTable, k: Vec<u8>) -> anyhow::Result<ScanIterator> {
        Ok(ScanIterator {
            iter: self.db.prefix_iterator_cf(f, &k),
            prefix: k.to_owned(),
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

