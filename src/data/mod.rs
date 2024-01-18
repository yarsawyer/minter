use std::sync::Arc;

use self::db::{Database, OwnedDbTable};

pub mod db;

pub struct MinterDbTables {
    pub wallets: OwnedDbTable,
    pub addresses: OwnedDbTable,
    pub utxo: OwnedDbTable,
}

impl MinterDbTables {
    pub fn load(db: &Arc<Database>) -> anyhow::Result<MinterDbTables> {
        Ok(Self {
            wallets: db.owned_column_family("wallets")?,
            addresses: db.owned_column_family("addresses")?,
            utxo: db.owned_column_family("utxo")?,
        })
    }
}
