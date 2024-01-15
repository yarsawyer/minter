use super::*;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::util::address::Address;
use bitcoin::PublicKey as BitcoinPublicKey;
use bitcoin::PrivateKey as BitcoinPrivateKey;

#[derive(Serialize)]
struct Output {
  mnemonic: Mnemonic,
  passphrase: Option<String>,
}

#[derive(Debug, Parser)]
pub(crate) struct Create {
  #[clap(
    long,
    default_value = "bells",
    help = "Use <PASSPHRASE> to derive wallet seed."
  )]
  pub(crate) passphrase: String,
}

impl Create {
  pub(crate) fn run(self, options: Options) -> Result<()> {
    let mut entropy = [0; 16];
    rand::thread_rng().fill_bytes(&mut entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy)?;

    let wallet_config = WalletConfig::new(mnemonic.to_string(), Some(self.passphrase.clone()));

    WalletConfig::save(wallet_config, &options)?;

    print_json(Output {
      mnemonic,
      passphrase: Some(self.passphrase),
    })?;



    let m = Mnemonic::from_str("").unwrap();
    let s = m.to_seed("bells");

    let secp = Secp256k1::new();
    let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &s).expect("Failed to create master key");

    let derivation_path = vec![
    ChildNumber::Hardened { index: 44 },
    ChildNumber::Hardened { index: 0 },
    ChildNumber::Hardened { index: 0 },
    ChildNumber::Normal { index: 0 },
    //ChildNumber::Normal { index: 0 },
    ];
    
    let derived_key = master_key.derive_priv(&secp, &derivation_path).expect("Failed to derive a key");
    let public_key = PublicKey::from_secret_key(&secp, &derived_key.private_key);
    
    let bitcoin_public_key = BitcoinPublicKey {
        compressed: true,
        inner: public_key,
    };

    let address = Address::p2pkh(&bitcoin_public_key, Network::Bitcoin);
    println!("Address: {}", address);


    Ok(())
  }
}