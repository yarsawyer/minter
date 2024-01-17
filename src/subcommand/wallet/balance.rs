use std::sync::Arc;

use reqwest::StatusCode;
use anyhow::{Result, Context, bail};
use tracing::{debug, info};

use crate::{ minter::Minter, subcommand::print_json};


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub cardinal: f64,
    pub ordinal: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ApiChainStats {
    pub funded_txo_count: u64,
    pub funded_txo_sum: u64,
    pub spent_txo_count: u64,
    pub spent_txo_sum: u64,
    pub tx_count: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ApiMempoolStats {
    pub funded_txo_count: u64,
    pub funded_txo_sum: u64,
    pub spent_txo_count: u64,
    pub spent_txo_sum: u64,
    pub tx_count: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ApiAddress {
    pub address: String,
    pub chain_stats: ApiChainStats,
    pub mempool_stats: ApiMempoolStats,
}



pub(crate) async fn run(options: crate::subcommand::Options, state: Arc<Minter>) -> Result<()> {
    let mut balance_utxo_sat = 0;
    let mut balance_ord_sat = 0;
    for (pub_key, addr) in state.addresses()? {
        debug!("Checking balance of address {pub_key}");
        let url = format!("{}/address/{}", &options.api_url.trim_end_matches('/'), &pub_key);
        let resp = state.reqwest_client.get(url).send().await.context("Failed to send api get balance request")?;
        match resp.status() {
            StatusCode::OK => {
                let addr_data = resp.json::<ApiAddress>().await.context("Api get balance invalid json")?;
                if addr_data.chain_stats.funded_txo_sum < addr_data.chain_stats.spent_txo_sum {
                    bail!("Api is insane! Funded is less than spent!");
                }
                //todo: add from mempool?
                let addr_balance = addr_data.chain_stats.funded_txo_sum - addr_data.chain_stats.spent_txo_sum;
                info!("Address {pub_key} balance: {addr_balance}");

                match addr.ty {
                    crate::wallet::AddressType::Utxo => balance_utxo_sat += addr_balance,
                    crate::wallet::AddressType::Ord => balance_ord_sat += addr_balance,
                }
            }
            err => {
                bail!("Api get balance error: {err}");
            }
        }
    }

    print_json(Output {
        cardinal: bitcoin::Amount::from_sat(balance_utxo_sat).to_btc(),
        ordinal: bitcoin::Amount::from_sat(balance_ord_sat).to_btc(),
    }).unwrap();
    
    Ok(())
}
