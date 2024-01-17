use bitcoin::{secp256k1::SecretKey, PrivateKey};

use self::{minter::Minter, receive::ReceiveArgs};

use {
	super::*,
	bitcoin::secp256k1::{
		rand::{self, RngCore},
		Secp256k1,
	},
	bitcoin::{
		util::bip32::{ChildNumber, DerivationPath, ExtendedPrivKey, Fingerprint},
		Network,
	},
 tokio::runtime,
};

pub mod balance;
pub mod create;
pub mod restore;
pub mod receive;




#[derive(Debug, Parser)]
pub(crate) enum Wallet {
	#[clap(about = "Get wallet balance")]
	Balance,
	#[clap(about = "Create new wallet")]
	Create(create::Create),
	//#[clap(about = "Create inscription")]
	//Inscribe(inscribe::Inscribe),
	//#[clap(about = "List wallet inscriptions")]
	//Inscriptions,
	#[clap(about = "Generate receive address")]
	Receive(ReceiveArgs),
//   #[clap(about = "Restore wallet")]
//   Restore(restore::Restore),
	//#[clap(about = "Send sat or inscription")]
	//Send(send::Send),
	//#[clap(about = "See wallet transactions")]
	//Transactions(transactions::Transactions),
	//#[clap(about = "List wallet outputs")]
	//Outputs,
}

impl Wallet {
	pub(crate) async fn run(self, options: Options, state: Arc<Minter>) -> Result<()> {
		//let rt = runtime::Runtime::new().map_err(|e| Error::msg(e.to_string()))?;

		match self {
			Self::Balance => balance::run(options, state).await,
			Self::Create(create) => create.run(options, state),
			//Self::Inscribe(inscribe) => inscribe.run(options),
			//Self::Inscriptions => inscriptions::run(options),
			Self::Receive(args) => receive::run(options, state, args),
			//Self::Restore(restore) => restore.run(options),
			//Self::Send(send) => send.run(options),
			//Self::Transactions(transactions) => transactions.run(options),
			//Self::Outputs => outputs::run(options),
		}
	}
}



