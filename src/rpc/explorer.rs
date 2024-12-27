use super::BitcoinRPC;
use serde_json::Value;
use anyhow::Result;

impl BitcoinRPC {
    pub fn get_block_by_height(&self, height: u64) -> Result<Value> {
        // Erst Block-Hash holen
        let block_hash: String = self.call("getblockhash", vec![height.into()])?;
        // Dann Block-Details
        self.get_block_by_hash(&block_hash)
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Result<Value> {
        self.call("getblock", vec![hash.into(), 2.into()])
    }

    pub fn get_transaction(&self, txid: &str) -> Result<Value> {
        self.call("getrawtransaction", vec![txid.into(), true.into()])
    }

    pub fn get_address_info(&self, address: &str) -> Result<Value> {
        // Mehrere Calls kombinieren für Adress-Informationen
        let script_hash: String = self.call("getaddressinfo", vec![address.into()])?;
        let utxos: Vec<Value> = self.call("scantxoutset", vec!["start".into(), format!("addr({})", address).into()])?;
        
        // Ergebnisse kombinieren...
        Ok(json!({
            "address": address,
            "script_hash": script_hash,
            "utxos": utxos,
        }))
    }
} 