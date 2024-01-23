use std::str::FromStr;

use anyhow::{bail, Context};

use crate::wallet::Wallet;

use super::Minter;


impl Minter {
    pub fn push_wallet(&self, id: &str, wallet: &Wallet) -> anyhow::Result<()> {
        self.db.set(self.tables.wallets.table(), id.as_bytes(), wallet)?;
		self.push_important(format!("Created a new wallet with mnemonic: {}", &wallet.mnemonic));
        Ok(())
    }

    pub fn create_wallet(&self, passphrase: String, name: String) -> anyhow::Result<Wallet> {
        use bitcoin::secp256k1::rand::RngCore;

		let mut entropy = [0; 16];
		bitcoin::secp256k1::rand::thread_rng().fill_bytes(&mut entropy);

		let mnemonic = bip39::Mnemonic::from_entropy(&entropy)?;
        self.create_wallet_with_mnemonic(passphrase, name, mnemonic.to_string())
    }
    pub fn create_wallet_with_mnemonic(&self, passphrase: String, name: String, mnemonic: String) -> anyhow::Result<Wallet> {
        if self.get_wallet(&name)?.is_some() {
			bail!("Wallet {} already exists. Create new one with --wallet <name> flag", &name);
		}
		let wallet = Wallet::new(mnemonic, Some(passphrase.clone()), name.clone());
		let mnemonic = bip39::Mnemonic::from_str(&wallet.mnemonic).context("Invalid mnemonic")?;

		self.db.set(self.tables.wallets.table(), &name, &wallet).context("Failed to save wallet to database")?;

		self.push_important("There is no wallet saved in DB");
		self.push_important(format!("Created new wallet with mnemonic: {mnemonic}"));

        Ok(wallet)
    }

    pub fn get_wallet(&self, id: &str) -> anyhow::Result<Option<Wallet>> {
        self.db.get(self.tables.wallets.table(), id.as_bytes())
    }

    pub fn wallets<'a: 'b, 'b>(&'a self) -> anyhow::Result<impl Iterator<Item = Wallet> + 'b> {
        let iter = self.db
            .iterate(self.tables.wallets.table(), vec![])
            .context("Failed to query wallets")?
            .filter_map(|(_,val)| {
                let Ok(data) = bincode::deserialize::<Wallet>(&val) else {
                    error!("Invalid wallet address format. Skipping");
                    return None;
                };
                Some(data)
            });

        Ok(iter)
    }
}

