use std::str::FromStr;
use std::sync::Arc;

use crate::minter::Minter;
use crate::subcommand::print_json;
use anyhow::Context;

#[derive(serde::Serialize)]
struct Output {
	mnemonic: bip39::Mnemonic,
	passphrase: Option<String>,
}

#[derive(Debug, clap::Parser)]
pub(crate) struct Create {
	#[clap(long, default_value = "bells", help = "Use <PASSPHRASE> to derive wallet seed.")] 
	pub(crate) passphrase: String,
}

impl Create {
	pub(crate) fn run(self, options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
		let wallet = state.create_wallet(self.passphrase.clone(), options.wallet).context("Failed to create wallet")?;

		print_json(Output {
			mnemonic: bip39::Mnemonic::from_str(&wallet.mnemonic).unwrap(),
			passphrase: Some(self.passphrase),
		})?;

		Ok(())
	}
}