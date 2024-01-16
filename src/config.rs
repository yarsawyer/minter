// use super::*;

// use serde::{Serialize, Deserialize};
// use serde_yaml;
// use std::fs::File;
// use std::io::Write;

// // Define your data structure
// #[derive(Serialize, Deserialize)]
// pub(crate) struct WalletConfig {
//     mnemonic: String,
//     passphrase: Option<String>,
//     addresses: Vec<String>,
//     utxos: Vec<u128>,    
//     inscriptions: Vec<u128>,    
// }

// impl WalletConfig {    
    
//     pub(crate) fn new(mnemonic: String, passphrase: Option<String>) -> Self {
//         Self {
//             mnemonic,
//             passphrase,
//             addresses: Vec::new(),
//             utxos: Vec::new(),
//             inscriptions: Vec::new(),
//         }
//     }

//     pub(crate) fn save(wallet: WalletConfig, options: &Options) -> Result<Self> {
//         let mut path = PathBuf::from(&options.wallet);
//         path.set_extension("yaml");    
//         if path.exists() {
//            bail!("Wallet file already exists.");
//         } else {
//             let yaml_data = serde_yaml::to_string(&wallet)?;
        
//             let mut file = File::create(&path).expect("Unable to create file");
//             file.write_all(yaml_data.as_bytes()).expect("Unable to write data");

//             Ok(wallet)
//         }
//     }

// }
