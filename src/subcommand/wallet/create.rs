use std::sync::Arc;

use crate::minter::Minter;
use crate::subcommand::print_json;
use crate::wallet::Wallet;
use anyhow::{Context, bail};
use bitcoin::secp256k1::rand::RngCore;
use bitcoin::secp256k1::rand;

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
		if state.get_wallet(&options.wallet)?.is_some() {
			bail!("Wallet {} already exists. Create new one with --wallet <name> flag", &options.wallet);
		}

		let mut entropy = [0; 16];
		rand::thread_rng().fill_bytes(&mut entropy);

		let mnemonic = bip39::Mnemonic::from_entropy(&entropy)?;

		let wallet = Wallet::new(mnemonic.to_string(), Some(self.passphrase.clone()), options.wallet.clone());

		state.db.set(state.tables.wallets.table(), &options.wallet, &wallet).context("Failed to save wallet to database")?;

		state.push_important("There is no wallet saved in DB");
		state.push_important(format!("Created new wallet with mnemonic: {mnemonic}"));

		print_json(Output {
			mnemonic: mnemonic.clone(),
			passphrase: Some(self.passphrase),
		})?;

		Ok(())
	}
}