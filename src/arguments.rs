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
    pub(crate) fn run(self) -> Result {
        self.subcommand.run(self.options)
    }
}