use anyhow::{Result, anyhow};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use dotenv::dotenv;
use std::env;
use std::fmt;

pub struct BitcoinRPC {
    client: Client,
}

#[derive(Debug)]
pub struct NodeStatus {
    pub version: u64,
    pub height: u64,
    pub block_hash: String,
    pub timestamp: i64,
    pub connections: u64,        
    pub verification_progress: f64,
    pub mempool_size: u64,      
    pub network: String,        
}

impl BitcoinRPC {
    pub fn new() -> Result<Self> {
        dotenv().ok();
        
        let host = env::var("BTC_RPC_HOST").expect("BTC_RPC_HOST must be set");
        let port = env::var("BTC_RPC_PORT").expect("BTC_RPC_PORT must be set");
        let rpc_url = format!("http://{}:{}", host, port);
        
        let rpc_user = env::var("BTC_RPC_USER").expect("BTC_RPC_USER must be set");
        let rpc_pass = env::var("BTC_RPC_PASSWORD").expect("BTC_RPC_PASSWORD must be set");
        
        let auth = Auth::UserPass(rpc_user, rpc_pass);
        let client = Client::new(&rpc_url, auth)?;
        
        Ok(Self { client })
    }

    pub fn test_connection(&self) -> Result<NodeStatus> {
        // Basis-Informationen abrufen
        let version = self.client.version()? as u64;
        let height = self.client.get_block_count()? as u64;
        let block_hash = self.client.get_best_block_hash()?;
        let block_info = self.client.get_block_header_info(&block_hash)?;

        // Versuche zusätzliche Informationen zu bekommen
        let connections = match self.client.get_connection_count() {
            Ok(count) => count as u64,
            Err(_) => 0
        };

        let (verification_progress, network) = match self.client.get_blockchain_info() {
            Ok(info) => (info.verification_progress, info.chain),
            Err(_) => (1.0, "unknown".to_string())
        };

        let mempool_size = match self.client.get_mempool_info() {
            Ok(info) => info.size as u64,
            Err(_) => 0
        };

        Ok(NodeStatus {
            version,
            height,
            block_hash: block_hash.to_string(),
            timestamp: block_info.time as i64,
            connections,
            verification_progress,
            mempool_size,
            network,
        })
    }
} 