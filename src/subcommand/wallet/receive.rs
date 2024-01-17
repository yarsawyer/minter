use std::str::FromStr;
use std::sync::Arc;

use anyhow::Context;
use bitcoin::secp256k1::rand::RngCore;
use bitcoin::secp256k1::{Secp256k1, PublicKey, rand};
use bitcoin::PublicKey as BitcoinPublicKey;
use tracing::info;

use crate::minter::Minter;
use crate::subcommand::print_json;
use crate::wallet::{Wallet, AddressType, WalletAddressData};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Output {
    pub address: bitcoin::Address,
}

#[derive(Debug, clap::Parser)]
pub struct ReceiveArgs {
    #[arg(help = "utxo or ord")]
    pub ty: AddressType,
}

pub(crate) fn run(_options: crate::subcommand::Options, state: Arc<Minter>, args: ReceiveArgs) -> anyhow::Result<()> {
    let mut entropy = [0; 16];
    rand::thread_rng().fill_bytes(&mut entropy);

    let wallet = state.db.get::<Wallet>(&b"wallet")?.context("Wallet not found")?;
    let mnemonic = bip39::Mnemonic::from_str(&wallet.mnemonic).context("Invalid mnemonic is saved in DB")?;
    let seed = mnemonic.to_seed(wallet.passphrase.as_deref().unwrap_or("bells"));
    
    let secp = Secp256k1::new();
    let master_key = bitcoin::util::bip32::ExtendedPrivKey::new_master(bitcoin::Network::Bitcoin, &seed).context("Failed to create master key")?;

    let ty_int = match args.ty {
        AddressType::Utxo => 0,
        AddressType::Ord => 1,
    };
    let address_count = state.db.iterate(b"A/".to_vec()).context("Failed to list addresses")?.count();
    info!("Found {address_count} transactions");

    let derivation_path = vec![
        bitcoin::util::bip32::ChildNumber::Hardened { index: 44 },
        bitcoin::util::bip32::ChildNumber::Hardened { index: 0 },
        bitcoin::util::bip32::ChildNumber::Hardened { index: 0 },
        bitcoin::util::bip32::ChildNumber::Normal { index: ty_int },
        bitcoin::util::bip32::ChildNumber::Normal { index: address_count as u32 },
    ];

    let derived_key = master_key.derive_priv(&secp, &derivation_path).context("Failed to derive a key")?;
    let public_key = PublicKey::from_secret_key(&secp, &derived_key.private_key);

    let bitcoin_public_key = BitcoinPublicKey {
        compressed: true,
        inner: public_key,
    };

    let address = bitcoin::Address::p2pkh(&bitcoin_public_key, bitcoin::Network::Bitcoin);

    state.push_address(&address.to_string(), &WalletAddressData {
        private: derived_key.private_key,
        ty: args.ty,
    })?;

    print_json(Output {
        address,
    }).unwrap();

    Ok(())
}