use std::sync::Arc;
use super::*;
use crate::minter::Minter;


// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct Output {
//     utxo: Vec<UtxoData>,
// }


#[derive(Debug, clap::Parser)]
pub struct Send {
    address: Address,
    outgoing: Outgoing,
    #[clap(long, help = "Use fee rate of <FEE_RATE> nook/vB")]
    fee_rate: FeeRate,
}


impl Send {
    pub async fn run(self, options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {

        println!("address dest: {}", self.address);
        println!("value: {:?}", self.outgoing);




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
