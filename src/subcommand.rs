use self::minter::Minter;

use super::*;
pub mod wallet;


fn print_json(output: impl Serialize) -> Result {
	serde_json::to_writer_pretty(io::stdout(), &output)?;
	println!();
	Ok(())
}

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {  
	#[clap(subcommand, about = "Wallet commands")]
	Wallet(wallet::Wallet),
}

impl Subcommand {
	pub(crate) async fn run(self, options: Options, state: Arc<Minter>) -> Result {
		match self {      
			Self::Wallet(wallet) => wallet.run(options, state).await,
		}
	}
}