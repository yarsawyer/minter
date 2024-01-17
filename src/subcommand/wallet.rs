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
pub mod transactions;
pub mod list_utxo;
pub mod util_commands;




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
	
	#[clap(about = "Get utxo's of address")]
	ListUtxo(list_utxo::ListUtxo),

	#[clap(about = "Add address")]
	AddAddress(util_commands::AddAddress),
	#[clap(about = "Remove address")]
	RemoveAddress(util_commands::RemoveAddress),
	#[clap(about = "List addresses")]
	ListAddresses(util_commands::ListAddresses),
//   #[clap(about = "Restore wallet")]
//   Restore(restore::Restore),
	//#[clap(about = "Send sat or inscription")]
	//Send(send::Send),
	// #[clap(about = "See wallet transactions")]
	// Transactions(transactions::Transactions),
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
			Self::ListUtxo(args) => args.run(options, state).await,
			Self::AddAddress(args) => args.run(options, state).await,
			Self::RemoveAddress(args) => args.run(options, state).await,
			Self::ListAddresses(args) => args.run(options, state).await,
			//Self::Outputs => outputs::run(options),
		}
	}
}



