use self::minter::Minter;

use super::*;

#[derive(Debug, Parser)]
#[clap(version)]
#[clap(about = "Bells Mint by Yar Sawyer <https://x.com/yarsawyer> ")]
pub(crate) struct Arguments {
    #[clap(flatten)]
    pub(crate) options: Options,
    #[clap(subcommand)]
    pub(crate) subcommand: Subcommand,
}

impl Arguments {
    pub(crate) async fn run(self, state: Arc<Minter>) -> Result {
        //todo: add more forbidden chars?
        if self.options.wallet.contains('/') {
            bail!("Wallet names can't contain '/' symbols");
        }
        self.subcommand.run(self.options, state).await
    }
}