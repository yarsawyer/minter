use std::{sync::Arc, fmt::Display, io::{Write, Read}, str::from_utf8};

use anyhow::Context;
use itertools::Itertools;
use tracing::{warn, error};

use crate::{data::{db::Database, MinterDbTables}, wallet::{Wallet, WalletAddressData}};

pub mod utxo;

pub struct Minter {
    pub db: Arc<Database>,
    pub reqwest_client: reqwest::Client,
    pub api_url: String,
    pub tables: MinterDbTables,
}

impl Minter {
    pub fn new(db_path: &str, api_url: String) -> anyhow::Result<Arc<Self>> {
        let db = Database::open(db_path)?;
        let tables = MinterDbTables::load(&db).context("Failed to load column families from DB")?;
        let reqwest_client = reqwest::Client::builder().user_agent("rust").build().context("Failed to build reqwest client")?;

        let minter = Arc::new(Self {
            db,
            reqwest_client,
            api_url,
            tables,
        });

        Ok(minter)
    }

    pub fn push_important(&self, txt: impl Display) {
        let file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("important.log");
        let Ok(mut file) = file else {
            warn!("Failed to save important info: {txt}");
            return;
        };
        if file.write_all(format!("[{}] {txt}\n", chrono::Utc::now().to_rfc2822()).as_bytes()).is_err() {
            warn!("Failed to save important info: {txt}");
        }
    }

    pub fn push_address(&self, pub_key: &str, data: &WalletAddressData, wallet: &str) -> anyhow::Result<()> {
        let mut key_start = wallet.to_owned();
        key_start.push('/');
        key_start.push_str(pub_key);

        self.db.set(self.tables.addresses.table(), key_start.as_bytes(), data).context("Failed to save address")?;
        self.push_important(format!("Created new address for #{wallet} {:?} with private: '{:?}' and public: '{}'", &data.ty, &data.private, &pub_key));
        Ok(())
    }

    pub fn remove_address(&self, pub_key: &str, wallet: &str) -> anyhow::Result<()> {
        let mut key_start = wallet.to_owned();
        key_start.push('/');
        key_start.push_str(pub_key);

        self.db.remove(self.tables.addresses.table(), key_start.as_bytes()).context("Failed to remove address")?;
        self.push_important(format!("Removed address '{pub_key}' in #{wallet}"));
        Ok(())
    }

    pub fn addresses<'a: 'b, 'b>(&'a self, wallet: &str) -> anyhow::Result<impl Iterator<Item = (String,WalletAddressData)> + 'b> {
        let mut key_start = wallet.to_owned();
        key_start.push('/');

        let iter = self.db
            .iterate(self.tables.addresses.table(), key_start.clone().into_bytes())
            .context("Failed to query wallet addresses")?
            .filter_map(move |(key,val)| {
                let Ok(addr) = from_utf8(&key) else {
                    let key = format!("{:?}", &key.take(256).bytes().flatten().collect_vec());
                    error!("Found invalid wallet address: {key} (non utf-8). Skipping");
                    return None;
                };
                let addr = addr.split('/').last().unwrap();
                let Ok(addr_data) = bincode::deserialize::<WalletAddressData>(&val) else {
                    error!("Invalid wallet address format {addr}. Skipping");
                    return None;
                };

                Some((addr.to_owned(),addr_data))
            });
        Ok(iter)
    }

    pub fn push_wallet(&self, id: &str, wallet: &Wallet) -> anyhow::Result<()> {
        self.db.set(self.tables.wallets.table(), id.as_bytes(), wallet)?;
		self.push_important(format!("Created a new wallet with mnemonic: {}", &wallet.mnemonic));
        Ok(())
    }

    pub fn get_wallet(&self, id: &str) -> anyhow::Result<Option<Wallet>> {
        self.db.get(self.tables.wallets.table(), id.as_bytes())
    }

    pub fn wallets<'a: 'b, 'b>(&'a self) -> anyhow::Result<impl Iterator<Item = Wallet> + 'b> {
        let iter = self.db
            .iterate(self.tables.wallets.table(), vec![])
            .context("Failed to query wallets")?
            .filter_map(|(_,val)| {
                let Ok(data) = bincode::deserialize::<Wallet>(&val) else {
                    error!("Invalid wallet address format. Skipping");
                    return None;
                };
                Some(data)
            });

        Ok(iter)
    }
}

