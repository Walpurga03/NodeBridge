use std::env;
use std::time::{Duration, Instant};
use std::str::FromStr;
use bitcoincore_rpc::{Client, Auth, RpcApi};
use bitcoincore_rpc::bitcoin::BlockHash;
use dotenv::dotenv;
use anyhow::Result;
use hex;
use serde_json::{Value, json};
use bitcoincore_rpc::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::collections::HashMap;
use parking_lot::Mutex;
use once_cell::sync::Lazy;
use reqwest::blocking::Client as HttpClient;

// Re-export wichtiger Typen
pub use self::mempool::MempoolStats;
pub mod explorer;  // Neues Modul hinzufügen

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

#[derive(Debug, Clone)]
pub struct AddressDetails {
    pub tx_count: usize,
    pub received: f64,
    pub sent: f64,
    pub balance: f64,
    pub first_seen: i64,
    pub last_seen: i64,
    pub funded_txo_count: usize,   // Anzahl empfangener Outputs
    pub spent_txo_count: usize,    // Anzahl ausgegebener Outputs
    pub unspent_txo_count: usize,  // Anzahl unausgegebener Outputs
    pub has_mempool_tx: bool,      // Hat unbestätigte Transaktionen
    pub address_type: String,      // z.B. "p2wpkh" (native segwit)
}

pub trait BitcoinRPCInterface: Clone {
    fn get_address_balance(&self, address: &str) -> Result<f64>;
    fn get_address_tx_count(&self, address: &str) -> Result<usize>;
    fn get_address_details(&self, address: &str) -> Result<AddressDetails>;
}

impl BitcoinRPCInterface for BitcoinRPC {
    fn get_address_balance(&self, address: &str) -> Result<f64> {
        self.get_address_balance(address)
    }

    fn get_address_tx_count(&self, address: &str) -> Result<usize> {
        self.get_address_tx_count(address)
    }

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

    pub fn get_explorer_address(&self, address: &str) -> Result<Value, Error> {
        let mut address_info = json!({
            "address": address
        });

        // Adressinformationen von der Node abfragen
        if let Ok(addr_info) = self.client.call::<Value>("getaddressinfo", &[json!(address)]) {
            // Grundlegende Informationen von der Node
            address_info["type"] = addr_info.get("type").unwrap_or(&json!("unknown")).clone();
            address_info["scriptPubKey"] = addr_info.get("scriptPubKey").unwrap_or(&json!({})).clone();
            address_info["ismine"] = addr_info.get("ismine").unwrap_or(&json!(false)).clone();
            address_info["iswatchonly"] = addr_info.get("iswatchonly").unwrap_or(&json!(false)).clone();
            address_info["solvable"] = addr_info.get("solvable").unwrap_or(&json!(false)).clone();
        }

        // UTXO Informationen von der Node abfragen
        if let Ok(utxos) = self.client.call::<Value>("listunspent", &[json!(0), json!(9999999), json!([address])]) {
            address_info["utxos"] = utxos;
        }

        // Transaktionshistorie von der Node abfragen
        if let Ok(history) = self.client.call::<Value>("listtransactions", &[json!("*"), json!(10), json!(0), json!(true)]) {
            let addr_txs: Vec<_> = history.as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .filter(|tx| tx.get("address").and_then(|a| a.as_str()) == Some(address))
                .cloned()
                .collect();
            
            address_info["transactions"] = json!(addr_txs);
        }

        Ok(address_info)
    }

    pub fn get_address_balance(&self, address: &str) -> Result<f64> {
        let descriptor = format!("addr({})", address);
        let scan_result = self.client.call::<Value>(
            "scantxoutset",
            &[
                json!("start"),
                json!([descriptor])
            ]
        )?;

        let total_amount = scan_result
            .get("total_amount")
            .and_then(|t| t.as_f64())
            .unwrap_or(0.0);

        Ok(total_amount)
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

    pub fn get_address_tx_count(&self, address: &str) -> Result<usize> {
        self.log_debug(&format!("Abfrage mempool.space API für Adresse {}", address));
        
        // HTTP Client für mempool.space API
        let client = HttpClient::new();
        let url = format!("https://mempool.space/api/address/{}", address);
        
        match client.get(&url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    let data: Value = response.json()?;
                    let tx_count = data.get("chain_stats")
                        .and_then(|stats| stats.get("tx_count"))
                        .and_then(|count| count.as_u64())
                        .unwrap_or(0) as usize;
                    
                    self.log_debug(&format!("Gefunden: {} Transaktionen", tx_count));
                    Ok(tx_count)
                } else {
                    self.log_debug(&format!("API Fehler: {}", response.status()));
                    Ok(0)
                }
            },
            Err(e) => {
                self.log_debug(&format!("Netzwerk Fehler: {}", e));
                Ok(0)
            }
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
                    let first_seen = chain_stats.get("first_seen")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);
                    
                    let last_seen = chain_stats.get("last_seen")
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
                        first_seen,
                        last_seen,
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
} 