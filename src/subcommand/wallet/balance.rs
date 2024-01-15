use {super::*, crate::wallet::Wallet, std::collections::BTreeSet};
use bitcoin::Amount;
use reqwest::StatusCode;
use anyhow::{Result, Error};


#[derive(Serialize, Deserialize)]
pub struct Output {
  pub cardinal: u64,
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



pub(crate) fn run(options: Options) -> Result<()> {

        let url = format!("{}/address/{}", &options.api_url, "BSS2AhRHdZKXM469EG2nPhtq6LUSawendK");

        //let response = reqwest::get(&url).await?;
        let response = reqwest::blocking::Client::builder()
            .user_agent("rust")
            .build()?
            .get(&url)
            .send()?;

        match response.status() {
            StatusCode::OK => {
                let address_data: ApiAddress = response.json().expect("Address balance API JSON parsing failed");
                let address_balance = Amount::from_sat(address_data.chain_stats.funded_txo_sum - address_data.chain_stats.spent_txo_sum);
                println!("Balance: {}", address_balance);
                Ok(())
            }
            s => {
                return Err(Error::msg(format!("Received response status: {:?}", s)));            
            }
        }
}
