use std::str::FromStr;
use std::sync::Arc;

use anyhow::Context;
use bitcoin::secp256k1::rand::RngCore;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey, rand};
use bitcoin::PublicKey as BitcoinPublicKey;
use bitcoin::PrivateKey as BitcoinPrivateKey;
use tracing::info;

use crate::minter::Minter;
use crate::wallet::Wallet;

// #[derive(serde::Deserialize, serde::Serialize)]
// pub struct Output {
//     pub address: Address,
// }

#[derive(Debug, clap::Parser)]
pub struct ReceiveArgs {
    // #[clap(long, help = "Utxo or Inscription")]
    // pub ty: AddressType,
}

pub(crate) fn run(options: crate::subcommand::Options, state: Arc<Minter>, args: ReceiveArgs) -> anyhow::Result<()> {
    let mut entropy = [0; 16];
    rand::thread_rng().fill_bytes(&mut entropy);

    let wallet = state.db.get::<Wallet>(&"wallet/")?.context("Wallet not found")?;
    let mnemonic = bip39::Mnemonic::from_str(&wallet.mnemonic).context("Invalid mnemonic is saved in DB")?;
    let seed = mnemonic.to_seed(wallet.passphrase.as_deref().unwrap_or("bells"));
    
    let secp = Secp256k1::new();
    let master_key = bitcoin::util::bip32::ExtendedPrivKey::new_master(bitcoin::Network::Bitcoin, &seed).context("Failed to create master key")?;

    // let ty = match args.ty {
    //     AddressType::Utxo => "utxo/",
    //     AddressType::Inscription => "ord/",
    // };
    let address_count = state.db.iterate(&"addr/").context("Failed to list addresses")?.count();
    info!("Found {address_count} transactions");

    let derivation_path = vec![
        bitcoin::util::bip32::ChildNumber::Hardened { index: 44 },
        bitcoin::util::bip32::ChildNumber::Hardened { index: 0 },
        bitcoin::util::bip32::ChildNumber::Hardened { index: 0 },
        bitcoin::util::bip32::ChildNumber::Normal { index: address_count as u32 },
        // bitcoin::util::bip32::ChildNumber::Normal { index: 0 },
    ];

    let derived_key = master_key.derive_priv(&secp, &derivation_path).context("Failed to derive a key")?;
    let public_key = PublicKey::from_secret_key(&secp, &derived_key.private_key);

    let bitcoin_public_key = BitcoinPublicKey {
        compressed: true,
        inner: public_key,
    };

    let address = bitcoin::Address::p2pkh(&bitcoin_public_key, bitcoin::Network::Bitcoin);

    state.db.set(&("addr/", &address), &()).context("Failed to save address")?;

    println!("Address: {}", address);

    Ok(())
}