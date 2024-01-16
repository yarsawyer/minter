use std::sync::Arc;

use crate::minter::Minter;
use crate::subcommand::print_json;
use crate::wallet::Wallet;
use anyhow::{Context, bail};
use bitcoin::secp256k1::rand::RngCore;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey, rand};
use bitcoin::util::address::Address;
use bitcoin::PublicKey as BitcoinPublicKey;
use bitcoin::PrivateKey as BitcoinPrivateKey;

#[derive(serde::Serialize)]
struct Output {
	mnemonic: bip39::Mnemonic,
	passphrase: Option<String>,
}

#[derive(Debug, clap::Parser)]
pub(crate) struct Create {
	#[clap(
		long,
		default_value = "bells",
		help = "Use <PASSPHRASE> to derive wallet seed."
	)]
	pub(crate) passphrase: String,
}

impl Create {
	pub(crate) fn run(self, options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
		let mut entropy = [0; 16];
		rand::thread_rng().fill_bytes(&mut entropy);

		let mnemonic = bip39::Mnemonic::from_entropy(&entropy)?;

		let wallet = Wallet::new(mnemonic.to_string(), Some(self.passphrase.clone()));

		let db_key = "wallet/";
		if state.db.contains(&db_key).context("Failed to query wallet to database")? {
			bail!("Wallet already exists");
		}
		state.db.set(&db_key, &wallet).context("Failed to save wallet to database")?;

		print_json(Output {
			mnemonic: mnemonic.clone(),
			passphrase: Some(self.passphrase),
		})?;

		// let m = Mnemonic::from_str("").unwrap();
		// let m = mnemonic;
		// let s = m.to_seed("bells");

		// let secp = Secp256k1::new();
		// let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &s).expect("Failed to create master key");

		// let derivation_path = vec![
		// 	ChildNumber::Hardened { index: 44 },
		// 	ChildNumber::Hardened { index: 0 },
		// 	ChildNumber::Hardened { index: 0 },
		// 	ChildNumber::Normal { index: 0 },
		// 	ChildNumber::Normal { index: 0 },
		// ];
		
		// let derived_key = master_key.derive_priv(&secp, &derivation_path).expect("Failed to derive a key");
		// let public_key = PublicKey::from_secret_key(&secp, &derived_key.private_key);
		
		// let bitcoin_public_key = BitcoinPublicKey {
		// 	compressed: true,
		// 	inner: public_key,
		// };

		// let address = Address::p2pkh(&bitcoin_public_key, Network::Bitcoin);
		// println!("Address: {}", address);

		Ok(())
	}
}