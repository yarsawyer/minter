use std::sync::Arc;

use crate::minter::Minter;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    transactions: Vec<String>,
}
#[derive(Debug, clap::Parser)]
pub struct ListUtxo {
    #[arg(help = "public wallet address to use (generated by receive)")]
    pub address: String,
}

impl ListUtxo {
    pub async fn run(self, options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
        Ok(())
    }
}
