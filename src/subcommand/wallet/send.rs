use std::sync::Arc;
use super::*;
use crate::{minter::Minter, wallet::AddressType};


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
    //todo: ord
    pub async fn run(self, options: crate::subcommand::Options, state: Arc<Minter>) -> anyhow::Result<()> {
        let amount = match self.outgoing {
            Outgoing::Amount(x) => x,
            Outgoing::InscriptionId(_) => bail!("shit"),
        };
        state.send_utxo(&options.wallet, self.address, amount).await.context("Failed to send")?;
        // debug!("Sending tx");

        // trace!("Collecting utxo's for transaction");

        // let utxo = state.get_all_utxo(&options.wallet, |_,v| v.ty == AddressType::Utxo).context("Failed to retrieve available utxo's for transaction")?;
        // let utxo = state.g
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
