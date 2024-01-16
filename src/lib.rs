use data::db::Database;

use {
    self::{
      arguments::Arguments,
      subcommand::Subcommand,
      options::Options,
      config::WalletConfig,
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

mod arguments;
mod wallet;
mod config;

pub mod options;
pub mod subcommand;

pub mod data;


type Result<T = (), E = Error> = std::result::Result<T, E>;

static INTERRUPTS: AtomicU64 = AtomicU64::new(0);


fn timestamp(seconds: u32) -> DateTime<Utc> {
    Utc.timestamp_opt(seconds.into(), 0).unwrap()
}
  
  const INTERRUPT_LIMIT: u64 = 5;

fn quick_test() {

  let db_path = "./db";
  let db = Database::open(db_path).unwrap();
  db.set(&"test", &42).unwrap();
  dbg!(db.get::<i32>(&"test"));
}
  
pub fn main() {
  {
    // use crate::util::log::{AndFilter, NotFilter};
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::*;
    //better_panic::install();
    let indicatif_layer = tracing_indicatif::IndicatifLayer::new();
    // let verbosity = match m.occurrences_of("verbosity") {
    //     0 => "error",
    //     1 => "warn",
    //     2 => "info",
    //     3 => "debug",
    //     _ => "trace",
    // };
    let verbosity = "trace";
    let fmt_layer_a = fmt::layer()
        .with_writer(indicatif_layer.get_stderr_writer())
        .with_filter(EnvFilter::new(verbosity));
    let logger = registry()
        // .with(fmt_layer_b)
        .with(fmt_layer_a)
        .with(indicatif_layer);

    logger.init();
}
  quick_test();
  return;
  //env_logger::init();
  
  ctrlc::set_handler(move || {
  
    println!("Detected Ctrl-C, attempting to shut down minter gracefully. Press Ctrl-C {INTERRUPT_LIMIT} times to force shutdown.");
  
    let interrupts = INTERRUPTS.fetch_add(1, atomic::Ordering::Relaxed);
  
    if interrupts > INTERRUPT_LIMIT {
      process::exit(1);
    }
  })
  .expect("Error setting ctrl-c handler");
  
  if let Err(err) = Arguments::parse().run() {
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
