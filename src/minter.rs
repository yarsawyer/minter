use std::sync::Arc;

use crate::data::db::Database;

pub struct Minter {
    pub db: Arc<Database>,
}

impl Minter {
    pub fn new(db_path: &str) -> anyhow::Result<Arc<Self>> {
        let db = Database::open(db_path)?;

        let minter = Arc::new(Self {
            db,
        });

        Ok(minter)
    }
}