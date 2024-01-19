use bitcoin::secp256k1::SecretKey;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, clap::ValueEnum, PartialEq, Eq)]
pub enum AddressType {
	#[clap(name = "utxo")]
	Utxo,
	#[clap(name = "ord")]
	Ord,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WalletAddressData {
	pub(crate) private: Option<SecretKey>,
	pub ty: AddressType,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Wallet {
	pub name: String,
    pub mnemonic: String,
    pub passphrase: Option<String>,
    // pub addresses: Vec<WalletAddress>,
    // utxos: Vec<u128>,
    // inscriptions: Vec<u128>,
}

impl Wallet {
	pub fn new(mnemonic: String, passphrase: Option<String>, name: String) -> Self {
		Self {
			name,
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