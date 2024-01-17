use {super::*, crate::wallet::Wallet, std::collections::BTreeSet};
use std::{str::from_utf8, io::Read};

use bitcoin::Amount;
use itertools::Itertools;
use reqwest::StatusCode;
use anyhow::{Result, Error};
use tracing::{error, debug, trace, info};

use crate::wallet::WalletAddressData;


#[derive(Serialize, Deserialize)]
pub struct Output {
    pub cardinal: u64,
    pub ordinal: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiChainStats {
    pub funded_txo_count: u64,
    pub funded_txo_sum: u64,
    pub spent_txo_count: u64,
    pub spent_txo_sum: u64,
    pub tx_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiMempoolStats {
    pub funded_txo_count: u64,
    pub funded_txo_sum: u64,
    pub spent_txo_count: u64,
    pub spent_txo_sum: u64,
    pub tx_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiAddress {
    pub address: String,
    pub chain_stats: ApiChainStats,
    pub mempool_stats: ApiMempoolStats,
}



pub(crate) async fn run(options: Options, state: Arc<Minter>) -> Result<()> {
    let mut balance_utxo_sat = 0;
    let mut balance_ord_sat = 0;
    for (key, val) in state.db.iterate(b"A/".to_vec()).context("Failed to query wallet addresses")? {
        let Ok(addr) = from_utf8(&key) else {
            let key = format!("{:?}", &key.take(256).bytes().flatten().collect_vec());
            error!("Found invalid wallet address: {key} (non utf-8). Skipping");
            continue;
        };
        let addr = addr.strip_prefix("A/").unwrap();
        let Ok(addr_data) = bincode::deserialize::<WalletAddressData>(&val) else {
            error!("Invalid wallet address format. Skipping");
            continue;
        };
        let addr_ty = addr_data.ty;

        debug!("Checking balance of address {addr}");
        let url = format!("{}/address/{}", &options.api_url.trim_end_matches('/'), addr);
        let resp = state.reqwest_client.get(url).send().await.context("Failed to send api get balance request")?;
        match resp.status() {
            StatusCode::OK => {
                let addr_data = resp.json::<ApiAddress>().await.context("Api get balance invalid json")?;
                let addr_balance = addr_data.chain_stats.funded_txo_sum - addr_data.chain_stats.spent_txo_sum;
                info!("Address {addr} balance: {addr_balance}");

                match addr_ty {
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
        cardinal: balance_utxo_sat,
        ordinal: balance_ord_sat,
    }).unwrap();
    
    Ok(())
}
