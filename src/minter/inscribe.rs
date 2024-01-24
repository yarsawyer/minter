use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Context};
use bitcoin::{blockdata::{opcodes, script::{self, Instruction}}, secp256k1::Secp256k1};
use itertools::Itertools;

use super::{utxo::{InscriptionId, UtxoData}, Minter};

pub struct Inscription {
    body: Option<Vec<u8>>,
    content_type: Option<Vec<u8>>,
}

struct CreateInscriptionTx {
    inscription: Inscription,
    utxo_in: Vec<(String, UtxoData)>,
    dest: bitcoin::Address,
    inscriptions: HashMap<bitcoin::OutPoint, InscriptionId>,
    change_privk: bitcoin::PrivateKey,
    change_address: bitcoin::Address,
}

fn instruction_size(instr: &script::Instruction) -> usize {
    match instr {
        script::Instruction::PushBytes(x) => x.len(),
        script::Instruction::Op(_) => 1,
    }
}

impl Minter {
    // fn create_inscription_tx(info: CreateInscriptionTx) -> anyhow::Result<Vec<bitcoin::Transaction>> {
        // // get outpoint
        // let outpoint = info.utxo_in
        //     .iter()
        //     .map(|(_,x)| bitcoin::OutPoint { txid: x.txid, vout: x.vout })
        //     .find(|x| !info.inscriptions.contains_key(x))
        //     .context("No cardinal utxo's found in wallet")?;

        // // check if any of inscriptions is already inscribed
        // for (i_outpoint, i_id) in &info.inscriptions {
        //     if i_outpoint == &outpoint {
        //         bail!("Inscription {i_id:?} is already inscribed on {i_outpoint}");
        //     }
        // }

        // const PROTOCOL_ID: &[u8] = b"ord";
        // const CHUNK_SIZE: usize = 240;
        // const DUST_SAT: u64 = 10_000;
        
        // // build inscription script
        // let shit = vec![];
        // let mut parts = info.inscription.body
        //     .as_ref()
        //     .unwrap_or(&shit) //todo: what if we have None?
        //     .chunks(CHUNK_SIZE)
        //     .enumerate();

        // let parts_count = parts.by_ref().count();

        // let mut builder = script::Builder::new()
        //     .push_slice(PROTOCOL_ID)
        //     .push_int(parts_count as i64)
        //     .push_slice(info.inscription.content_type.as_ref().unwrap());

        // for (n,part) in parts {
        //     builder = builder
        //         .push_int(parts_count as i64 - n as i64 - 1)
        //         .push_slice(part);
        // }
        // let script = builder.into_script();

        // // make transactions from instructions
        // let mut transactions = vec![];
        // let mut instructions = VecDeque::from_iter(script.instructions().flatten());
        // let mut p2sh_input: Option<bitcoin::TxIn> = None;
        // let mut last_partial: Option<bitcoin::Script> = None;
        // let mut last_lock: Option<bitcoin::Script> = None;

        // let mut the_thing = vec![]; // please, do not ask me what it is
        // while !instructions.is_empty() {
        //     the_thing.clear();
        //     let mut chunk_bytes_len = 0;

        //     if transactions.is_empty() {
        //         let i = instructions.pop_front().unwrap();
        //         chunk_bytes_len += instruction_size(&i);
        //         the_thing.push(i);
        //     }

        //     while chunk_bytes_len <= 1500 && !instructions.is_empty() {
        //         let i = instructions.pop_front().unwrap();
        //         chunk_bytes_len += instruction_size(&i);
        //         the_thing.push(i);

        //         let i = instructions.pop_front().unwrap();
        //         chunk_bytes_len += instruction_size(&i);
        //         the_thing.push(i);
        //     }

        //     if chunk_bytes_len > 1500 {
        //         instructions.push_front(the_thing.pop().unwrap());
        //         instructions.push_front(the_thing.pop().unwrap());
        //     }

        //     // build lockscript
        //     let secp = Secp256k1::new(); //todo: use lazy_static

        //     let mut lock = script::Builder::new()
        //         .push_slice(&info.change_privk.public_key(&secp).to_bytes())
        //         .push_opcode(opcodes::all::OP_CHECKSIGVERIFY);
        //     for _ in &the_thing {
        //         lock = lock.push_opcode(opcodes::all::OP_DROP)
        //     }
        //     lock = lock.push_opcode(opcodes::OP_TRUE);

        //     let p2sh_out = bitcoin::TxOut {
        //         value: DUST_SAT,
        //         script_pubkey: lock.into_script().to_p2sh(),
        //     };
        //     let change = bitcoin::TxOut {
        //         value: 0,
        //         script_pubkey: info.change_address.script_pubkey(),
        //     };
        //     let mut tx = bitcoin::Transaction {
        //         version: 1,
        //         lock_time: bitcoin::PackedLockTime::ZERO,
        //         input: vec![],
        //         output: vec![p2sh_out, change],
        //     };

        //     if let Some(ref p2sh_input) = p2sh_input {
        //         tx.input.push(p2sh_input.clone());
        //     }

        //     // estimate fee
        //     let unlock_size = if let (Some(last_partial), Some(last_lock)) = (&last_partial, &last_lock) {
        //         let mut unlock = script::Builder::new();
        //         for instruction in last_partial.instructions() {
        //             match instruction {
        //                 Ok(Instruction::Op(x)) => unlock = unlock.push_opcode(x),
        //                 Ok(Instruction::PushBytes(x)) => unlock = unlock.push_slice(x),
        //                 Err(e) => bail!("Invalid instruction: {e}"),
        //             }
        //         }
        //         unlock
        //             .push_slice(&[0;74])
        //             .push_slice(last_lock.as_bytes())
        //             .into_script()
        //             .as_bytes()
        //             .len()
        //     } else { 0 };

        //     let mut total_val = bitcoin::Amount::ZERO;


        // }
    
        // Ok(transactions)
        


        // //todo: get satpoint
        // //todo: check if one of inscriptions is inscribed
        // //todo: build inscription script
        // //todo: make transactions from instructions
        // //todo: make final transaction
        // //todo: calc fees
    // }
}