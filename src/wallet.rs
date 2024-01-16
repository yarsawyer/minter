// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
// pub enum AddressType {
// 	#[clap(name = "utxo")]
// 	Utxo,
// 	#[clap(name = "ord")]
// 	Inscription,
// }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WalletAddress {
	pub(crate) private: String,
	// pub ty: AddressType,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Wallet {
    pub mnemonic: String,
    pub passphrase: Option<String>,
    // pub addresses: Vec<WalletAddress>,
    // utxos: Vec<u128>,
    // inscriptions: Vec<u128>,
}

impl Wallet {
	pub fn new(mnemonic: String, passphrase: Option<String>) -> Self {
		Self {
			mnemonic,
			passphrase,
			// addresses: vec![],
			// utxo_address_count: 0,
			// inscription_address_count: 0,
		}
	}

	// pub fn add_address(&mut self, addr: WalletAddress) {
	// 	// self.addresses.push(addr)
	// }
}