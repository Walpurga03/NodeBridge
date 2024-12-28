use std::env;
use std::time::{Duration, Instant};
use std::str::FromStr;
use bitcoincore_rpc::{Client, Auth, RpcApi};
use bitcoincore_rpc::bitcoin::BlockHash;
use dotenv::dotenv;
use anyhow::Result;
use hex;
use serde_json;

pub struct BitcoinRPC {
    client: Client,
}

#[derive(Debug)]
pub struct MempoolInfo {
    pub size: u64,
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
    pub difficulty: f64,
    pub chain_work: String,  // Neues Feld
    pub initial_block_download: bool,
    pub size_on_disk: u64,
    pub pruned: bool,
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

#[derive(Debug)]
pub struct BlockDetails {
    pub height: u64,
    pub hash: String,
    pub timestamp: i64,
    pub tx_count: u64,
    pub size: u64,
    pub weight: u64,
    pub version: u32,
    pub merkle_root: String,
    pub bits: String,
    pub nonce: u32,
}

impl BitcoinRPC {
    pub fn new() -> anyhow::Result<Self> {
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

    pub fn test_connection(&self) -> anyhow::Result<NodeStatus> {
        let timeout = Duration::from_secs(15);
        let start = Instant::now();
        
        let check_timeout = |start: Instant, timeout: Duration| -> Result<()> {
            if start.elapsed() > timeout {
                return Err(anyhow::anyhow!(
                    "RPC Timeout nach {} Sekunden - Bitte prüfen Sie:\n• Bitcoin Core läuft\n• RPC ist aktiviert\n• Credentials sind korrekt", 
                    timeout.as_secs()
                ));
            }
            Ok(())
        };

        check_timeout(start, timeout)?;
        match self.client.version() {
            Ok(version) => {
                let height = self.client.get_block_count()? as u64;
                let block_hash = self.client.get_best_block_hash()?;
                let block_info = self.client.get_block_header_info(&block_hash)?;
                
                let connections = self.client.get_connection_count().unwrap_or(0) as u64;

                let difficulty = match self.client.get_difficulty() {
                    Ok(diff) => diff,
                    Err(_) => 0.0
                };

                let chain_work = match self.client.get_block_header_info(&block_hash) {
                    Ok(info) => {
                        let hex_string = hex::encode(&info.chainwork);
                        format!("0x{:0>64}", hex_string)
                    },
                    Err(_) => {
                        "0x0000000000000000000000000000000000000000000000000000000000000000".to_string()
                    }
                };

                let (verification_progress, network, size_on_disk, pruned) = match self.client.call::<serde_json::Value>("getblockchaininfo", &[]) {
                    Ok(info) => {
                        let size = info.get("size_on_disk")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0);
                            
                        let network = info.get("chain")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                            
                        let progress = info.get("verificationprogress")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(1.0);
                            
                        let is_pruned = info.get("pruned")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                            
                        (progress, network, size, is_pruned)
                    },
                    Err(_) => (
                        1.0,
                        "unknown".to_string(),
                        0,
                        false,
                    )
                };

                let mempool_size = self.client.get_mempool_info()
                    .map(|info| info.size as u64)
                    .unwrap_or(0);

                let mempool_info = self.get_mempool_info()?;

                let peers = self.get_peer_info()?;

                Ok(NodeStatus {
                    version: version as u64,
                    height,
                    block_hash: block_hash.to_string(),
                    timestamp: block_info.time as i64,
                    connections,
                    verification_progress,
                    mempool_size,
                    network,
                    mempool_info,
                    peers,
                    difficulty,
                    chain_work,
                    initial_block_download: false,
                    size_on_disk,
                    pruned,
                })
            },
            Err(e) => {
                Err(anyhow::anyhow!(
                    "Verbindung fehlgeschlagen:\n• {}\n• Prüfen Sie die Einstellungen in .env", 
                    e
                ))
            }
        }
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

    pub fn get_mempool_info(&self) -> anyhow::Result<MempoolInfo> {
        let info = self.client.get_mempool_info()?;
        Ok(MempoolInfo {
            size: info.size as u64,
        })
    }

    pub fn get_block_details(&self, input: &str) -> Result<BlockDetails> {
        // Prüfen ob Eingabe eine Zahl ist
        if let Ok(height) = input.parse::<u64>() {
            // Block nach Höhe suchen
            let block_hash = self.client.get_block_hash(height)?;
            let block = self.client.get_block_info(&block_hash)?;
            
            Ok(BlockDetails {
                height,
                hash: block_hash.to_string(),
                timestamp: block.time as i64,
                tx_count: block.tx.len() as u64,
                size: block.size as u64,
                weight: block.weight as u64,
                version: block.version as u32,
                merkle_root: block.merkleroot.to_string(),
                bits: block.bits,
                nonce: block.nonce,
            })
        } else {
            // Block nach Hash suchen
            let block_hash = BlockHash::from_str(input)?;
            let block = self.client.get_block_info(&block_hash)?;
            
            // Block-Höhe über Block-Header ermitteln
            let block_header = self.client.get_block_header_info(&block_hash)?;
            let height = block_header.height as u64;
            
            Ok(BlockDetails {
                height,
                hash: block_hash.to_string(),
                timestamp: block.time as i64,
                tx_count: block.tx.len() as u64,
                size: block.size as u64,
                weight: block.weight as u64,
                version: block.version as u32,
                merkle_root: block.merkleroot.to_string(),
                bits: block.bits,
                nonce: block.nonce,
            })
        }
    }
} 