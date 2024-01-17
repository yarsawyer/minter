use std::{sync::Arc, str::FromStr};

use anyhow::Context;
use bitcoin::secp256k1::SecretKey;

use crate::{minter::Minter, wallet::{AddressType, WalletAddressData}, subcommand::print_json};


// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct Output {
// }
#[derive(Debug, clap::Parser)]
pub struct AddAddress {
    #[arg(help = "utxo or ord")]
    pub ty: AddressType,

    #[arg(help = "public wallet address")]
    pub address: String,

    #[arg(help = "private wallet address", required=false)]
    pub private: Option<String>,
}

impl AddAddress {
    pub async fn run(self, _: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
        let private = self.private.as_deref().map(SecretKey::from_str).transpose().context("Invalid private address")?;
        state.push_address(&self.address.to_string(), &WalletAddressData {
            private,
            ty: self.ty,
        })?;

        Ok(())
    }
}

#[derive(Debug, clap::Parser)]
pub struct RemoveAddress {
    #[arg(help = "public wallet address")]
    pub address: String,
}

impl RemoveAddress {
    pub async fn run(self, _: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
        state.remove_address(&self.address)?;
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
    pub ty: AddressType,
}

#[derive(Debug, clap::Parser)]
pub struct ListAddresses {
}

impl ListAddresses {
    pub async fn run(self, _: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
        let items = state.addresses()?
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

