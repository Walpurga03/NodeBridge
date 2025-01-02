use std::env;
use std::time::{Duration, Instant};
use std::str::FromStr;
use bitcoincore_rpc::{Client, Auth, RpcApi};
use bitcoincore_rpc::bitcoin::{self, BlockHash};
use dotenv::dotenv;
use anyhow::Result;
use hex;
use serde_json::{Value, json};
use std::fs::OpenOptions;
use std::io::Write;
use std::collections::HashMap;
use parking_lot::Mutex;
use once_cell::sync::Lazy;
use reqwest::blocking::Client as HttpClient;
use log::{info};

// Re-export wichtiger Typen
pub use self::mempool::MempoolStats;

// Module
mod mempool;

pub struct BitcoinRPC {
    client: Client,
}

impl Clone for BitcoinRPC {
    fn clone(&self) -> Self {
        // Neue RPC-Verbindung aufbauen
        Self::new().expect("Failed to clone BitcoinRPC")
    }
}

#[derive(Debug)]
pub struct MempoolInfo {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub version: u64,
    pub subver: String,
    pub latency: f64,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    pub inbound: bool,
    pub connected_time: u64,
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

#[derive(Debug, Clone)]
pub struct AddressDetails {
    pub tx_count: usize,
    pub received: f64,
    pub sent: f64,
    pub balance: f64,
    pub funded_txo_count: usize,
    pub spent_txo_count: usize,
    pub unspent_txo_count: usize,
    pub has_mempool_tx: bool,
    pub address_type: String,
}

pub trait BitcoinRPCInterface: Clone {
    fn get_address_details(&self, address: &str) -> Result<AddressDetails>;
}

impl BitcoinRPCInterface for BitcoinRPC {
    fn get_address_details(&self, address: &str) -> Result<AddressDetails> {
        self.get_address_details(address)
    }
}

// Cache-Struktur für Adressdetails
struct AddressDetailsCache {
    details: AddressDetails,
    last_update: Instant
}

// Globaler Cache mit Thread-Safety
static ADDRESS_DETAILS_CACHE: Lazy<Mutex<HashMap<String, AddressDetailsCache>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

#[derive(Debug)]
pub struct Transaction {
    #[allow(dead_code)]
    pub txid: String,
    pub size: u32,
    pub weight: u32,
    pub blocktime: Option<u64>,
    pub blockhash: Option<String>,
    pub vin: Vec<Value>,
    pub vout: Vec<Value>,
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
            bytes_sent: p.bytessent as u64,
            bytes_recv: p.bytesrecv as u64,
            inbound: p.inbound,
            connected_time: p.conntime as u64,
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

    fn log_debug(&self, msg: &str) {
        static FIRST_CALL: std::sync::Once = std::sync::Once::new();
        
        // Beim ersten Aufruf die Datei leeren
        FIRST_CALL.call_once(|| {
            if let Ok(mut file) = std::fs::File::create("debug.log") {
                let _ = writeln!(file, "=== Debug Log Start ===");
            }
        });

        // Dann normal weiterschreiben
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("debug.log") 
        {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            let _ = writeln!(file, "[{}] {}", timestamp, msg);
        }
    }

    pub fn get_address_details(&self, address: &str) -> Result<AddressDetails> {
        const CACHE_DURATION: Duration = Duration::from_secs(30);
        
        // Prüfe Cache
        let mut cache = ADDRESS_DETAILS_CACHE.lock();
        if let Some(cached) = cache.get(address) {
            if cached.last_update.elapsed() < CACHE_DURATION {
                return Ok(cached.details.clone());
            }
        }

        self.log_debug(&format!("Abfrage mempool.space API für Adresse {}", address));
        
        let client = HttpClient::new();
        let url = format!("https://mempool.space/api/address/{}", address);
        
        match client.get(&url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    let data: Value = response.json()?;
                    
                    // Debug: Komplette API-Antwort loggen
                    self.log_debug(&format!("API Antwort: {}", serde_json::to_string_pretty(&data)?));
                    
                    let chain_stats = data.get("chain_stats")
                        .ok_or_else(|| anyhow::anyhow!("Keine Chain-Stats gefunden"))?;
                    
                    // Debug: Chain-Stats separat loggen
                    self.log_debug(&format!("Chain Stats: {}", serde_json::to_string_pretty(&chain_stats)?));

                    let mempool_stats = data.get("mempool_stats")
                        .ok_or_else(|| anyhow::anyhow!("Keine Mempool-Stats gefunden"))?;
                    
                    // Debug: Mempool-Stats separat loggen
                    self.log_debug(&format!("Mempool Stats: {}", serde_json::to_string_pretty(&mempool_stats)?));

                    // Werte extrahieren und direkt loggen
                    let received_chain = chain_stats.get("received").and_then(|v| v.as_u64()).unwrap_or(0);
                    let received_mempool = mempool_stats.get("received").and_then(|v| v.as_u64()).unwrap_or(0);
                    self.log_debug(&format!("Received: chain={}, mempool={}", received_chain, received_mempool));

                    let spent_chain = chain_stats.get("spent").and_then(|v| v.as_u64()).unwrap_or(0);
                    let spent_mempool = mempool_stats.get("spent").and_then(|v| v.as_u64()).unwrap_or(0);
                    self.log_debug(&format!("Spent: chain={}, mempool={}", spent_chain, spent_mempool));

                    // Werte kombinieren (bestätigt + unbestätigt)
                    let tx_count = (chain_stats.get("tx_count").and_then(|v| v.as_u64()).unwrap_or(0) +
                                  mempool_stats.get("tx_count").and_then(|v| v.as_u64()).unwrap_or(0)) as usize;

                    let received = (chain_stats.get("funded_txo_sum").and_then(|v| v.as_u64()).unwrap_or(0) +
                                  mempool_stats.get("funded_txo_sum").and_then(|v| v.as_u64()).unwrap_or(0)) as f64 / 100_000_000.0;

                    let sent = (chain_stats.get("spent_txo_sum").and_then(|v| v.as_u64()).unwrap_or(0) +
                              mempool_stats.get("spent_txo_sum").and_then(|v| v.as_u64()).unwrap_or(0)) as f64 / 100_000_000.0;

                    let balance = received - sent;

                    // Erste/Letzte Aktivität
                    let _first_seen = chain_stats.get("first_seen")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);
                    
                    let _last_seen = chain_stats.get("last_seen")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);

                    self.log_debug(&format!("Details gefunden: {} Transaktionen, {:.8} BTC Balance", 
                        tx_count, balance));

                    let funded_txo_count = chain_stats.get("funded_txo_count")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as usize;
                        
                    let spent_txo_count = chain_stats.get("spent_txo_count")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as usize;

                    let unspent_txo_count = funded_txo_count - spent_txo_count;
                    
                    let has_mempool_tx = mempool_stats.get("tx_count")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) > 0;

                    // Adresstyp aus dem Format erkennen
                    let address_type = if address.starts_with("bc1p") {
                        "Taproot (P2TR)".to_string()
                    } else if address.starts_with("bc1") {
                        "Native SegWit (P2WPKH)".to_string()
                    } else if address.starts_with("3") {
                        "Nested SegWit (P2SH)".to_string()
                    } else if address.starts_with("1") {
                        "Legacy (P2PKH)".to_string()
                    } else {
                        "Unbekannt".to_string()
                    };

                    let details = AddressDetails {
                        tx_count,
                        received,
                        sent,
                        balance,
                        funded_txo_count,
                        spent_txo_count,
                        unspent_txo_count,
                        has_mempool_tx,
                        address_type,
                    };

                    // Nach erfolgreicher API-Abfrage:
                    cache.insert(address.to_string(), AddressDetailsCache {
                        details: details.clone(),
                        last_update: Instant::now()
                    });

                    Ok(details)
                } else {
                    self.log_debug(&format!("API Fehler: {}", response.status()));
                    Err(anyhow::anyhow!("API Fehler: {}", response.status()))
                }
            },
            Err(e) => {
                self.log_debug(&format!("Netzwerk Fehler: {}", e));
                Err(anyhow::anyhow!("Netzwerk Fehler: {}", e))
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_explorer_transaction(&self, txid: &str) -> Result<Value> {
        // Implementierung der Transaktion-Details hier
        // Ähnlich wie get_address_details, aber für Transaktionen
        let client = HttpClient::new();
        let url = format!("https://mempool.space/api/tx/{}", txid);
        
        match client.get(&url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    let data: Value = response.json()?;
                    Ok(data)
                } else {
                    Err(anyhow::anyhow!("API Fehler: {}", response.status()))
                }
            },
            Err(e) => Err(anyhow::anyhow!("Netzwerk Fehler: {}", e))
        }
    }

    pub fn get_raw_transaction(&self, txid: &str) -> Result<Transaction> {
        let tx_id = bitcoin::Txid::from_str(txid)?;
        let raw_tx = self.client.get_raw_transaction(&tx_id, None)?;
        let tx_info = self.client.get_raw_transaction_info(&tx_id, None)?;
        
        Ok(Transaction {
            txid: tx_info.txid.to_string(),
            size: tx_info.size as u32,
            weight: tx_info.vsize as u32,
            blocktime: tx_info.blocktime.map(|t| t as u64),
            blockhash: tx_info.blockhash.map(|h| h.to_string()),
            vin: raw_tx.input.iter().map(|input| {
                let prev_tx = self.client.get_raw_transaction_info(&input.previous_output.txid, None).ok();
                let prev_output = prev_tx.as_ref()
                    .and_then(|tx| tx.vout.get(input.previous_output.vout as usize));
                
                json!({
                    "txid": input.previous_output.txid.to_string(),
                    "vout": input.previous_output.vout,
                    "value": prev_output.map(|out| out.value.to_btc()).unwrap_or(0.0),
                    "address": prev_output
                        .and_then(|out| {
                            let hex_bytes = hex::decode(&out.script_pub_key.hex).unwrap_or_default();
                            let script = bitcoin::Script::from_bytes(&hex_bytes);
                            bitcoin::Address::from_script(&script, bitcoin::Network::Bitcoin)
                                .ok()
                                .map(|addr| addr.to_string())
                        })
                        .unwrap_or_else(|| "Unbekannte Adresse".to_string())
                })
            }).collect(),
            vout: raw_tx.output.iter().map(|output| {
                json!({
                    "value": bitcoin::Amount::from_sat(output.value).to_btc(),
                    "scriptPubKey": {
                        "address": match bitcoin::Address::from_script(&output.script_pubkey, bitcoin::Network::Bitcoin) {
                            Ok(addr) => addr.to_string(),
                            Err(_) => "Unbekannte Adresse".to_string()
                        }
                    }
                })
            }).collect(),
        })
    }

    #[allow(dead_code)]
    pub fn connect_rpc() -> Result<Client> {
        info!("Verbindung zum Bitcoin RPC-Client wird hergestellt.");
        // Verbindungslogik
        // Beispielhafte Initialisierung, passen Sie dies entsprechend Ihrer Implementierung an
        let rpc_url = "http://localhost:8332";
        let rpc_user = "your_rpc_username";
        let rpc_password = "your_rpc_password";
        let auth = Auth::UserPass(rpc_user.to_string(), rpc_password.to_string());
        let client = Client::new(rpc_url, auth)?;
        info!("Verbindung erfolgreich hergestellt.");
        Ok(client)
    }
} 