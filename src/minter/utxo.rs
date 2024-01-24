use std::{collections::HashMap, str::{from_utf8, FromStr}};

use anyhow::{bail, Context};
use bitcoin::BlockHash;
use itertools::Itertools;

use crate::wallet::{AddressType, WalletAddressData};

// bincode does not support 'flatten' but we need it to access api
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UtxoData {
    pub txid: bitcoin::Txid,
    pub vout: u32,
    pub status: Status,
    pub value: u64,
    pub ty: AddressType,
    #[serde(default)] pub inscription_meta: Option<InscriptionMeta>,
    #[serde(default)] pub owner: Option<String>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InscriptionMeta {
    pub content_type: String,
    pub content_length: usize,
    pub outpoint: bitcoin::OutPoint,
    pub genesis: bitcoin::OutPoint,
    pub inscription_id: InscriptionId,
    pub number: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UtxoApiData {
    pub txid: String,
    pub vout: u32,
    pub status: Status,
    pub value: u64,
    #[serde(default, flatten)] pub inscription_meta: Option<InscriptionApiMeta>,
    #[serde(default)] pub owner: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Status {
    pub confirmed: bool,
    #[serde(default)] pub block_height: Option<usize>,
    #[serde(default)] pub block_hash: Option<BlockHash>,
    #[serde(default)] pub block_time: Option<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InscriptionId {
    pub txid: bitcoin::Txid,
    pub index: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InscriptionApiMeta {
    pub content_type: String,
    pub content_length: usize,
    pub outpoint: bitcoin::Txid,
    pub genesis: bitcoin::Txid,
    pub inscription_id: InscriptionId,
    pub number: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UtxoList {
    pub addr: String,
    pub utxo: Vec<UtxoData>,
}
impl UtxoList {
    pub fn new(addr: String, utxo: Vec<UtxoData>) -> Self {
        Self { addr, utxo }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UtxoMultiList {
    pub per_address: Vec<UtxoList>,
}
impl From<UtxoList> for UtxoMultiList {
    fn from(value: UtxoList) -> Self { Self { per_address: vec![value] } }
}
impl From<Vec<UtxoList>> for UtxoMultiList {
    fn from(value: Vec<UtxoList>) -> Self { Self { per_address: value } }
}
impl From<UtxoMultiList> for Vec<UtxoList> {
    fn from(value: UtxoMultiList) -> Self { value.per_address }
}
impl From<HashMap<String, UtxoList>> for UtxoMultiList {
    fn from(value: HashMap<String, UtxoList>) -> Self {
        Self { per_address: value.into_values().collect_vec() }
    }
}
impl IntoIterator for UtxoList {
    type Item = UtxoData;
    type IntoIter = <Vec<UtxoData> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { self.utxo.into_iter() }
}
pub struct UtxoMultiListIterator<'a> {
    utxo: &'a UtxoMultiList,
    offset_a: usize,
    offset_b: usize,
}
impl<'a> Iterator for UtxoMultiListIterator<'a> {
    type Item = (&'a str, &'a UtxoData);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let list = self.utxo.per_address.get(self.offset_a)?;
            if let Some(utxo) = list.utxo.get(self.offset_b) {
                self.offset_b += 1;
                return Some((&list.addr, &utxo))
            }
            self.offset_b = 0;
            self.offset_a += 1;
        }
    }
}
impl<'a> UtxoMultiListIterator<'a> {
    pub fn next_address(&mut self) {
        self.offset_a += 1;
    }
    pub fn reset_position(&mut self) {
        self.offset_a = 0;
        self.offset_b = 0;
    }
    pub fn reset_inner_list_position(&mut self) {
        self.offset_b = 0;
    }
}
impl UtxoMultiList {
    pub fn new() -> Self { UtxoMultiList { per_address: vec![] }}
    pub fn with_capacity(cap: usize) -> Self { UtxoMultiList { per_address: Vec::with_capacity(cap) }}
    pub fn iter(&self) -> UtxoMultiListIterator { UtxoMultiListIterator { utxo: self, offset_a: 0, offset_b: 0 } }
    pub fn push(&mut self, l: UtxoList) { self.per_address.push(l); }

    pub fn len(&self) -> usize { self.per_address.iter().map(|x|x.utxo.len()).sum() }
    pub fn is_empty(&self) -> bool { self.per_address.is_empty() || self.len() == 0 }
}
impl Default for UtxoMultiList { fn default() -> Self { Self::new() } }



impl super::Minter {
    /// Get cached (saved to DB) utxo's using specific address
    pub fn get_utxo(&self, address: &str, wallet: &str) -> anyhow::Result<UtxoList> {
        let mut prefix = wallet.to_owned();
        prefix.push('/');
        prefix.push_str(address);

        let utxo = self.db.iterate(self.tables.utxo.table(), prefix.into_bytes())
            .context("Failed to get utxo's")?
            .filter_map(|(_,v)| {
                let Ok(data) = bincode::deserialize::<UtxoData>(&v) else {
                    error!("Invalid UTXO data");
                    return None;
                };
                Some(data)
            })
            .collect();
        Ok(UtxoList { addr: address.to_owned(), utxo })
    }

    //todo: better error log
    fn get_all_utxo_for(&self, addresses: &mut HashMap<String,UtxoList>, wallet: &str) -> anyhow::Result<()> {
        let iter = self.db.iterate(self.tables.utxo.table(), wallet.to_owned().into_bytes())
            .context("Failed to get utxo's")?;
        for (k,v) in iter {
            let Some(addr) = k.split(|&x|x==b'/').nth(1) else {
                error!("Invalid UTXO path");
                continue;
            };
            let Ok(addr) = from_utf8(addr) else {
                error!("Invalid UTXO path (address is not utf-8)");
                continue;
            };
            if !addresses.contains_key(addr) { continue; }
            let Ok(data) = bincode::deserialize::<UtxoData>(&v) else {
                error!("Invalid UTXO data");
                continue;
            };
            addresses.get_mut(addr).unwrap().utxo.push(data);
        }
        Ok(())
    }
    
    /// Get all cached (saved to DB) utxo's using specific filter
    pub fn get_all_utxo(&self, wallet: &str, selector: impl Fn(&str, &WalletAddressData) -> bool) -> anyhow::Result<UtxoMultiList> {
        let mut addresses = self.addresses(wallet)?
            .filter_map(|x| selector(&x.0,&x.1).then_some((x.0.clone(), UtxoList::new(x.0, vec![]))))
            .collect::<HashMap<_,_>>();

        self.get_all_utxo_for(&mut addresses, wallet).map(|_|addresses.into())
    }

    //todo: add timeouts
    /// Get utxo's from api without any DB interaction
    async fn get_utxo_from_api(&self, address: &str, ty: AddressType) -> anyhow::Result<Vec<UtxoData>> {
        debug!("Retrieving utxo of address {}", address);
    
        let url = format!("{}/address/{}/utxo", &self.api_url.trim_end_matches('/'), &address);
        let resp = self.reqwest_client.get(url).send().await.context("Failed to send api get utxo request")?;
        
        match resp.status() {
            reqwest::StatusCode::OK => Ok(
                resp.json::<Vec<UtxoApiData>>()
                    .await
                    .context("Api get utxo invalid json")?
                    .into_iter()
                    .map(|x| UtxoData {
                        txid: bitcoin::Txid::from_str(&x.txid).unwrap(), //todo: remove unwrap
                        vout: x.vout,
                        status: x.status,
                        value: x.value,
                        inscription_meta: x.inscription_meta.map(|x| InscriptionMeta { 
                            content_type: x.content_type, 
                            content_length: x.content_length, 
                            outpoint: bitcoin::OutPoint { txid: x.outpoint, vout: 0 }, 
                            genesis: bitcoin::OutPoint { txid: x.genesis, vout: 0 }, 
                            inscription_id: x.inscription_id, 
                            number: x.number,
                        }),
                        owner: x.owner,
                        ty,
                    })
                    .collect_vec()
            ),
            err => bail!("Api get utxo error: {err}")
        }
    }

    /// Get utxo's from api without any DB interaction
    async fn get_all_utxo_from_api(&self, wallet: &str, selector: impl Fn(&str, &WalletAddressData) -> bool) -> anyhow::Result<UtxoMultiList> {
        let mut utxo = UtxoMultiList::new();
        for (addr, addr_data) in self.addresses(wallet)? {
            if !selector(&addr, &addr_data) { continue; }
            let new_utxo = self.get_utxo_from_api(&addr, addr_data.ty).await.context("Failed to get utxo")?;
            utxo.push(UtxoList {
                addr,
                utxo: new_utxo,
            });
        }
        Ok(utxo)
    }

    /// Remove all saved utxo's from DB
    pub fn clear_saved_utxo(&self, wallet: &str, selector: impl Fn(&str, &UtxoData) -> bool) -> anyhow::Result<usize> {
        let mut prefix = wallet.to_owned().into_bytes();
        prefix.push(b'/');
        self.db.remove_where(self.tables.utxo.table(), prefix, |k,v| {
            let Some(k) = k else { return true };
            let Some(v) = v else { return true };
            selector(k,&v)
        }).context("Failed to delete saved utxo")
    }

    //todo: remove and set in one transaction
    //todo: better path for DB keys
    //todo: better selectors?
    //todo: implement more clever way to check updates
    //todo: implement more clever way to overwrite values (so only changed items will be updated)
    pub async fn fetch_utxo(&self, wallet: &str, wallet_selector: impl Fn(&str, &WalletAddressData) -> bool, utxo_selector: impl Fn(&str, &UtxoData) -> bool) -> anyhow::Result<UtxoMultiList> {
        debug!("Fetching utxo's");
        let utxo = self.get_all_utxo_from_api(wallet, &wallet_selector).await?;

        //todo: drop utxo on error
        let removed = self.clear_saved_utxo(wallet, &utxo_selector)?;
        debug!("Removed {removed} utxo for {wallet}");

        //todo: optimize
        self.db.set_many(self.tables.utxo.table(), utxo.iter().map(|(addr,x)| {
                let mut key = wallet.to_owned();
                key.push('/');
                key.push_str(addr);
                key.push('/');
                key.push_str(&x.txid.to_string());
                key.push(':');
                key.push_str(&x.vout.to_string());
                (key.into_bytes(), x)
            }
        )).context("Failed to push new utxo's")?;

        let added = utxo.len();
        debug!("Added {added} utxo for {wallet}");

        Ok(utxo)
    }

    //todo: update in DB after transactions
    pub async fn gather_utxo(&self, wallet: &str, ty: AddressType, value: u64) -> anyhow::Result<Vec<(String, UtxoData)>> {
        let mut cur_value = 0;
        let mut gathered_utxo = vec![];

        for (addr,utxo) in self.get_all_utxo(wallet, |_,v| v.ty == ty).context("Failed to get cached utxo")?.iter() {
            cur_value += utxo.value;
            gathered_utxo.push((addr.to_owned(), utxo.clone()));
            if cur_value >= value { return Ok(gathered_utxo); }
        }

        info!("Not enough cached utxo. Getting new from api");
        cur_value = 0;
        gathered_utxo.clear();
        for (addr,utxo) in self.fetch_utxo(wallet, |_,v| v.ty == ty, |_,v| v.ty == ty).await.context("Failed to get cached utxo")?.iter() {
            cur_value += utxo.value;
            gathered_utxo.push((addr.to_owned(), utxo.clone()));
            if cur_value >= value { return Ok(gathered_utxo); }
        }

        warn!("Not enough utxo");
        
        Ok(vec![])
    }

    pub async fn send_utxo(&self, wallet: &str, dest: bitcoin::Address, amount: bitcoin::Amount) -> anyhow::Result<()> {
        debug!("Sending tx");

        trace!("Collecting utxo's for transaction");
        let utxo = self.gather_utxo(wallet, AddressType::Utxo, amount.to_sat()).await.context("Failed to retrieve available utxo's for transaction")?;
        
        

        dbg!(utxo);
        Ok(())
    }
}
