mod mempool;  // Neues Modul registrieren
pub use mempool::MempoolInfo;  // Re-export

use anyhow::Result;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use dotenv::dotenv;
use std::env;
use std::time::{Duration, Instant};

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
    pub mempool_info: MempoolInfo,
    pub peers: Vec<PeerInfo>,  // Neue Peer-Informationen
}

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub addr: String,
    pub version: u64,
    pub subver: String,
    pub latency: f64,
    pub ping: f64,
    pub connected_time: u64,
    pub last_send: u64,
    pub last_recv: u64,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    pub inbound: bool,
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
        
        Ok(Self { 
            client,
        })
    }

    pub fn test_connection(&self) -> Result<NodeStatus> {
        let timeout = Duration::from_secs(5);
        let start = Instant::now();
        
        let check_timeout = |start: Instant, timeout: Duration| -> Result<()> {
            if start.elapsed() > timeout {
                return Err(anyhow::anyhow!("RPC Timeout nach {} Sekunden", timeout.as_secs()));
            }
            Ok(())
        };

        // Alle RPC-Aufrufe mit Timeout prüfen
        check_timeout(start, timeout)?;
        let version = self.client.version()? as u64;

        check_timeout(start, timeout)?;
        let height = self.client.get_block_count()? as u64;

        check_timeout(start, timeout)?;
        let block_hash = self.client.get_best_block_hash()?;

        check_timeout(start, timeout)?;
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

        // Mempool-Informationen abrufen
        let mempool_info = self.get_mempool_info()?;

        // Peer-Informationen abrufen
        let peers = self.get_peer_info()?;

        Ok(NodeStatus {
            version,
            height,
            block_hash: block_hash.to_string(),
            timestamp: block_info.time as i64,
            connections,
            verification_progress,
            mempool_size,
            network,
            mempool_info,
            peers,  // Neue Peer-Informationen hinzufügen
        })
    }

    pub fn get_peer_info(&self) -> Result<Vec<PeerInfo>> {
        let peers = self.client.get_peer_info()?;
        
        Ok(peers.into_iter().map(|p| PeerInfo {
            addr: p.addr,
            version: p.version as u64,
            subver: p.subver,
            latency: p.pingtime.unwrap_or(0.0),
            ping: p.minping.unwrap_or(0.0),
            connected_time: p.conntime as u64,
            last_send: p.lastsend as u64,
            last_recv: p.lastrecv as u64,
            bytes_sent: p.bytessent as u64,
            bytes_recv: p.bytesrecv as u64,
            inbound: p.inbound,
        }).collect())
    }
} 