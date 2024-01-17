use std::{sync::Arc, fmt::Display, io::Write};

use anyhow::Context;
use tracing::warn;

use crate::data::db::Database;

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
}