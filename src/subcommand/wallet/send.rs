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

//todo: multiple wallets
//todo: save utxo to db. retrieve from api if there are not enough
//todo: cmd to get private from public
//todo: rename 'ty' to 'type' in json


impl Send {
    pub async fn run(self, _options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
        debug!("Sending tx");

        trace!("Collecting utxo's for transaction");
        // let utxo = state.get_all_utxo(crate::wallet::AddressType::Utxo).await.context("Failed to retrieve available utxo's for transaction")?;



        // println!("address dest: {}", self.address);
        // println!("value: {:?}", self.outgoing);




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
