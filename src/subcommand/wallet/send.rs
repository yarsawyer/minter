use std::sync::Arc;

use crate::minter::Minter;


// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct Output {
//     utxo: Vec<UtxoData>,
// }


#[derive(Debug, clap::Parser)]
pub struct Send {
    #[arg(help = "target public wallet address")]
    pub address_dest: String,

    #[arg(help = "sum or inscription id")]
    pub value: String,
}


impl Send {
    pub async fn run(self, options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
        // get utxo and vout
        // make input
        // make output
        // serialize
        // broadcast

        // bitcoin::TxIn {
        //     previous_output: bitcoin::OutPoint { txid: (), vout: () },
        //     script_sig: todo!(),
        //     sequence: todo!(),
        //     witness: todo!(),
        // }

        Ok(())
    }
}
