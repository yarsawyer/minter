use std::{sync::Arc, fmt::Display, io::{Write, Read}, str::from_utf8};

use anyhow::Context;
use itertools::Itertools;
use tracing::{warn, error};

use crate::{data::db::Database, wallet::WalletAddressData};

pub struct Minter {
    pub db: Arc<Database>,
    pub reqwest_client: reqwest::Client,
}

impl Minter {
    pub fn new(db_path: &str) -> anyhow::Result<Arc<Self>> {
        let db = Database::open(db_path)?;
        let reqwest_client = reqwest::Client::builder().user_agent("rust").build().context("Failed to build reqwest client")?;

        let minter = Arc::new(Self {
            db,
            reqwest_client,
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

    pub fn push_address(&self, pub_key: &str, data: &WalletAddressData) -> anyhow::Result<()> {
        self.db.set(format!("A/{pub_key}").as_bytes(), data).context("Failed to save address")?;
        self.push_important(format!("Created new address for {:?} with private: '{:?}' and public: '{}'", &data.ty, &data.private, &pub_key));
        Ok(())
    }

    pub fn addresses<'a: 'b, 'b>(&'a self) -> anyhow::Result<impl Iterator<Item = (String,WalletAddressData)> + 'b> {
        let iter = self.db
            .iterate(b"A/".to_vec())
            .context("Failed to query wallet addresses")?
            .filter_map(|(key,val)| {
                let Ok(addr) = from_utf8(&key) else {
                    let key = format!("{:?}", &key.take(256).bytes().flatten().collect_vec());
                    error!("Found invalid wallet address: {key} (non utf-8). Skipping");
                    return None;
                };
                let addr = addr.strip_prefix("A/").unwrap();
                let Ok(addr_data) = bincode::deserialize::<WalletAddressData>(&val) else {
                    error!("Invalid wallet address format. Skipping");
                    return None;
                };

                Some((addr.to_owned(),addr_data))
            });
        Ok(iter)
    }
}