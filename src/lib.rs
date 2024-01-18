use std::collections::HashMap;

use data::db::Database;
use minter::Minter;

use {
		self::{
			deserialize_from_str::DeserializeFromStr,
			inscription_id::InscriptionId,
			outgoing::Outgoing,
			arguments::Arguments,
			subcommand::Subcommand,
			options::Options,
			// config::WalletConfig,
		},    
		bip39::Mnemonic,
		anyhow::{anyhow, bail, Context, Error},
		chrono::{DateTime, TimeZone, Utc},
		clap::{ArgGroup, Parser},
		serde::{Deserialize, Deserializer, Serialize, Serializer},
		bitcoin::{
			blockdata::constants::COIN_VALUE,
			consensus::{self, Decodable, Encodable},
			hash_types::BlockHash,
			hashes::Hash,
			Address, Amount, Block, Network, OutPoint, Script, Sequence, Transaction, TxIn, TxOut, Txid,
		},
		std::{
				cmp,
				collections::{BTreeMap, HashSet, VecDeque},
				env,
				ffi::OsString,
				fmt::{self, Display, Formatter},
				fs::{self, File},
				io,
				net::{TcpListener, ToSocketAddrs},
				ops::{Add, AddAssign, Sub},
				path::{Path, PathBuf},
				process::{self, Command},
				str::FromStr,
				sync::{
					atomic::{self, AtomicU64},
					Arc, Mutex,
				},
			thread,
			time::{Duration, Instant, SystemTime},
		},
		reqwest,
		tokio
};

#[macro_use] extern crate tracing;

pub use crate::{
	fee_rate::FeeRate,
};
  

mod arguments;
mod wallet;
mod config;
mod deserialize_from_str;
mod outgoing;
mod inscription_id;
mod fee_rate;


pub mod options;
pub mod subcommand;

pub mod data;
pub mod minter;


type Result<T = (), E = Error> = std::result::Result<T, E>;

static INTERRUPTS: AtomicU64 = AtomicU64::new(0);


fn timestamp(seconds: u32) -> DateTime<Utc> {
	Utc.timestamp_opt(seconds.into(), 0).unwrap()
}
	
const INTERRUPT_LIMIT: u64 = 5;

fn quick_test() {
	// let db_path = "./db";
	// let db = Database::open(db_path).unwrap();

	// let start = Instant::now();

	// for i in 0..1_000_000 {
	// 	db.set(&("test/shit/", i), &()).unwrap();
	// 	db.set(&("test/crap/", i), &()).unwrap();
	// 	db.set(&("test/crab/", i), &()).unwrap();
	// }

	// println!("A: {} ms", start.elapsed().as_millis());
	// let start = Instant::now();

	// // for i in 0..1_000_000 {
	// // 	db.get::<()>(&("test/shit/", i)).unwrap();
	// // }

	// println!("B: {} ms", start.elapsed().as_millis());
	// let start = Instant::now();

	// let mut i = 0;
	// for _ in db.iterate(&("test/shit/").unwrap() {
	// 	i += 1;
	// }
	// dbg!(i);

	// println!("C: {} ms", start.elapsed().as_millis());
	// let start = Instant::now();


	// let mut h0 = HashMap::new();
	// let mut h1 = HashMap::new();
	// let mut h2 = HashMap::new();
	// for i in 0..5 {
	// 	h0.insert(i, 44);
	// 	h1.insert(i, 44);
	// 	h2.insert(i, 44);
	// }
	
	// let v0: Vec<_> = h0.keys().collect();
	// let v1: Vec<_> = h1.keys().collect();
	// let v2: Vec<_> = h2.keys().collect();
	// dbg!(v0);
	// dbg!(v1);
	// dbg!(v2);


	// for i in 5..9 {
	// 	h0.insert(i, 44);
	// 	h1.insert(i, 44);
	// 	h2.insert(i, 44);
	// }


	// let v0: Vec<_> = h0.keys().collect();
	// let v1: Vec<_> = h1.keys().collect();
	// let v2: Vec<_> = h2.keys().collect();
	// dbg!(v0);
	// dbg!(v1);
	// dbg!(v2);
	// let db_path = "./db";
	// let db = Database::open(db_path).unwrap();
	// for x in db.iterate(&"addr").unwrap() {
	// 	dbg!(x);
	// }
	// // db.set(&"test", &42).unwrap();
	// // dbg!(db.get::<i32>(&"test"));
}
	
#[tokio::main]
pub async fn main() {
	{
		use tracing_subscriber::layer::SubscriberExt;
		use tracing_subscriber::util::SubscriberInitExt;
		use tracing_subscriber::*;
		let indicatif_layer = tracing_indicatif::IndicatifLayer::new();
		let verbosity = "debug";
		let fmt_layer_a = fmt::layer()
				.with_writer(indicatif_layer.get_stderr_writer())
				.with_filter(EnvFilter::new(verbosity));
		let logger = registry()
				.with(fmt_layer_a)
				.with(indicatif_layer);

		logger.init();
}
	// quick_test();
	// return;
	
	ctrlc::set_handler(move || {
	
		println!("Detected Ctrl-C, attempting to shut down minter gracefully. Press Ctrl-C {INTERRUPT_LIMIT} times to force shutdown.");
	
		let interrupts = INTERRUPTS.fetch_add(1, atomic::Ordering::Relaxed);
	
		if interrupts > INTERRUPT_LIMIT {
			process::exit(1);
		}
	})
	.expect("Error setting ctrl-c handler");

	let db_path = "./db";
	let args = Arguments::parse();
	let minter = Minter::new(db_path, args.options.api_url.clone()).unwrap();
	
	if let Err(err) = args.run(minter).await {
		eprintln!("error: {err}");
		err
			.chain()
			.skip(1)
			.for_each(|cause| eprintln!("because: {cause}"));
		if env::var_os("RUST_BACKTRACE")
			.map(|val| val == "1")
			.unwrap_or_default()
		{
			eprintln!("{}", err.backtrace());
		}
		process::exit(1);
	}
}
