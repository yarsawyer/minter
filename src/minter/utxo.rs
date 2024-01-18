use std::{collections::HashMap, str::from_utf8};

use anyhow::{bail, Context};
use bitcoin::BlockHash;

use crate::wallet::AddressType;


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UtxoData {
    pub txid: String,
    pub vout: u32,
    pub status: Status,
    pub value: u64,
    #[serde(default, flatten)] pub inscription_meta: Option<InscriptionMeta>,
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
pub struct InscriptionMeta {
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
    //todo: deny wallets with '/' symbol
    pub fn get_utxo(&self, address: &str, wallet: &str) -> anyhow::Result<Vec<UtxoData>> {
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
        Ok(utxo)
    }

    //todo: replace hashmap with something less hardcoded
    //todo: better error log
    pub fn get_all_utxo_for<V>(&self, addresses: &HashMap<String,V>, wallet: &str) -> anyhow::Result<Vec<UtxoData>> {
        let utxo = self.db.iterate(self.tables.utxo.table(), wallet.to_owned().into_bytes())
            .context("Failed to get utxo's")?
            .filter_map(|(k,v)| {
                let Some(addr) = k.split(|&x|x==b'/').nth(1) else {
                    error!("Invalid UTXO path");
                    return None;
                };
                let Ok(addr) = from_utf8(addr) else {
                    error!("Invalid UTXO path (address is not utf-8)");
                    return None;
                };
                if !addresses.contains_key(addr) { return None; }

                let Ok(data) = bincode::deserialize::<UtxoData>(&v) else {
                    error!("Invalid UTXO data");
                    return None;
                };
                Some(data)
            })
            .collect();
        Ok(utxo)
    }
    
    //todo: use our structures for utxo lists
    //todo: do something with this shitcode
    //todo: cache?
    pub fn get_all_utxo(&self, ty: AddressType, wallet: &str) -> anyhow::Result<Vec<UtxoData>> {
        let addresses = self.addresses(wallet)?
            .filter_map(|x|(x.1.ty == ty).then_some((x.0.clone(), UtxoList::new(x.0, vec![]))))
            .collect::<HashMap<_,_>>();

        self.get_all_utxo_for(&addresses, wallet)
    }

    /// Get utxo's from api without any DB interaction
    async fn get_utxo_from_api(&self, address: &str) -> anyhow::Result<Vec<UtxoData>> {
        debug!("Retrieving utxo of address {}", address);
    
        let url = format!("{}/address/{}/utxo", &self.api_url.trim_end_matches('/'), &address);
        let resp = self.reqwest_client.get(url).send().await.context("Failed to send api get utxo request")?;
        
        match resp.status() {
            reqwest::StatusCode::OK => Ok(resp.json::<Vec<UtxoData>>().await.context("Api get utxo invalid json")?),
            err => bail!("Api get utxo error: {err}")
        }
    }

    /// Get utxo's from api without any DB interaction
    async fn get_all_utxo_from_api(&self, ty: AddressType, wallet: &str) -> anyhow::Result<UtxoMultiList> {
        let mut utxo = UtxoMultiList::new();
        for (addr, addr_data) in self.addresses(wallet)? {
            if addr_data.ty != ty { continue; }
            let new_utxo = self.get_utxo_from_api(&addr).await.context("Failed to get utxo")?;
            utxo.push(UtxoList {
                addr,
                utxo: new_utxo,
            });
        }
        Ok(utxo)
    }

    /// Remove all saved utxo's from DB
    pub fn clear_saved_utxo(&self, wallet: &str) -> anyhow::Result<usize> {
        let mut prefix = wallet.to_owned().into_bytes();
        prefix.push(b'/');
        self.db.remove_where(self.tables.utxo.table(), prefix, |_|true).context("Failed to delete saved utxo")
    }
    
    //todo: implement more clever way to check updates
    /// Get all utxo's from api and save them to DB
    pub async fn fetch_utxo(&self, wallet: &str) -> anyhow::Result<UtxoMultiList> {
        debug!("Fetching utxo's");
        let utxo = self.get_all_utxo_from_api(AddressType::Utxo, wallet).await?;

        //todo: drop utxo on error
        let removed = self.clear_saved_utxo(wallet)?;
        debug!("Removed {removed} utxo for {wallet}");

        //todo: optimize
        self.db.set_many(self.tables.utxo.table(), utxo.iter().map(|(addr,x)| {
                let mut key = wallet.to_owned();
                key.push('/');
                key.push_str(addr);
                key.push('/');
                key.push_str(&x.txid);
                key.push(':');
                key.push_str(&x.vout.to_string());
                (key.into_bytes(), x)
            }
        )).context("Failed to push new utxo's")?;

        let added = utxo.len();
        debug!("Added {added} utxo for {wallet}");

        Ok(utxo)
    }

    // pub async fn send_utxo(&self, dest: bitcoin::Address, amount: bitcoin::Amount) -> anyhow::Result<()> {
    //     debug!("Sending tx");

    //     trace!("Collecting utxo's for transaction");
    //     let utxo = self.get_all_utxo(crate::wallet::AddressType::Utxo).await.context("Failed to retrieve available utxo's for transaction")?;

        
    // }
}
