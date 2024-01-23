use std::{sync::Arc, str::FromStr};

use anyhow::Context;
use bitcoin::secp256k1::SecretKey;

use crate::{minter::Minter, wallet::{AddressType, WalletAddressData}, subcommand::print_json};


// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct Output {
// }
#[derive(Debug, clap::Parser)]
pub struct AddAddress {
    #[arg(help = "utxo or ord", name="type")]
    pub ty: AddressType,

    #[arg(help = "public wallet address")]
    pub address: String,

    #[arg(help = "private wallet address", required=false)]
    pub private: Option<String>,
}

impl AddAddress {
    pub async fn run(self, options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
        let private = self.private.as_deref().map(SecretKey::from_str).transpose().context("Invalid private address")?;
        state.push_address(&self.address.to_string(), &WalletAddressData {
            private,
            ty: self.ty,
        }, &options.wallet)?;

        Ok(())
    }
}

#[derive(Debug, clap::Parser)]
pub struct RemoveAddress {
    #[arg(help = "public wallet address")]
    pub address: String,
}

impl RemoveAddress {
    pub async fn run(self, options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
        state.remove_address(&self.address, &options.wallet)?;
        Ok(())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ListAddressesOutput {
    pub addresses: Vec<ListAddressesOutputItem>,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ListAddressesOutputItem {
    pub address: String,
    pub private: Option<String>,
    #[serde(rename="type")] pub ty: AddressType,
}

#[derive(Debug, clap::Parser)]
pub struct ListAddresses {
}

impl ListAddresses {
    pub async fn run(self, options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
        let items = state.addresses(&options.wallet)?
            .map(|x| ListAddressesOutputItem {
                address: x.0,
                private: x.1.private.map(|x| format!("{}", x.display_secret())),
                ty: x.1.ty,
            })
            .collect();
        print_json(ListAddressesOutput {
            addresses: items,
        }).unwrap();
        Ok(())
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetPrivateOutput {
    pub private: Option<String>,
}

#[derive(Debug, clap::Parser)]
pub struct GetPrivate {
    pub address: String,
}

impl GetPrivate {
    pub async fn run(self, options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
        let addr = state.get_address(&options.wallet, &self.address)?.context("Address not found")?;
        print_json(GetPrivateOutput {
            private: addr.private.map(|x|format!("{}", x.display_secret())),
        }).unwrap();
        Ok(())
    }
}

#[derive(Debug, serde::Serialize)]
pub struct ImportYamlOutput {
	mnemonic: bip39::Mnemonic,
	passphrase: Option<String>,
}

#[derive(Debug, clap::Parser)]
pub struct ImportYaml {
    #[arg(help = "path to legacy yaml file")]
    pub path: String,
}
#[derive(Debug, serde::Deserialize)]
struct LegacyYaml {
    mnemonic: String,
    passphrase: Option<String>,
}

impl ImportYaml {
    pub async fn run(self, options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
        let f = std::fs::read_to_string(self.path).context("Can't read file")?;
        let wallet = serde_yaml::from_str::<LegacyYaml>(&f).context("Invalid legacy yaml")?;
        
        let wallet = state.create_wallet_with_mnemonic(wallet.passphrase.unwrap_or_else(||"bells".to_owned()), options.wallet, wallet.mnemonic).context("Failed to import wallet")?;

		print_json(ImportYamlOutput {
			mnemonic: bip39::Mnemonic::from_str(&wallet.mnemonic).unwrap(),
			passphrase: Some(wallet.passphrase.unwrap_or_else(||"bells".to_owned())),
		})?;

        info!("Wallet imported");
        Ok(())
    }
}





