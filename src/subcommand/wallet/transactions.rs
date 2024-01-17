use std::sync::Arc;

use anyhow::{Context, bail};
use reqwest::StatusCode;
use tracing::debug;

use crate::minter::Minter;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    transactions: Vec<String>,
}
#[derive(Debug, clap::Parser)]
pub struct Transactions {

}

// mod get_transactions_api_types {
//     pub struct 
// }

// impl Transactions {
//     pub async fn run(self, options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
//         for (pub_key, addr) in state.addresses()? {
//             debug!("Retrieving transactions of address {pub_key}");
//             let url = format!("{}/address/{}/txs", &options.api_url.trim_end_matches('/'), &pub_key);
//             let resp = state.reqwest_client.get(url).send().await.context("Failed to send api get transactions request")?;
            
//             match resp.status() {
//                 StatusCode::OK => {
                    
//                 }
//                 err => {
//                     bail!("Api get transactions error: {err}");
//                 }
//             }
//         }

//         Ok(())
//     }
// }