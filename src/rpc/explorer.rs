use anyhow::Result;
use bitcoincore_rpc::{RpcApi, bitcoin::Txid};
use serde_json::Value;
use std::str::FromStr;

impl super::BitcoinRPC {
    pub fn get_explorer_transaction(&self, txid: &str) -> Result<Value> {
        let tx_id = Txid::from_str(txid)?;
        
        let tx_info = self.client.get_raw_transaction_info(&tx_id, None)?;
        Ok(serde_json::to_value(tx_info)?)
    }
} 