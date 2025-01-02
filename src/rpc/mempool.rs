use anyhow::Result;
use serde_json::Value;
use reqwest::blocking::Client as HttpClient;
use std::time::{Instant, Duration};
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct MempoolStats {
    pub tx_count: u64,
    pub size: u64,  // Größe in Bytes
    pub no_priority: FeeCategory,
    pub low_priority: FeeCategory,
    pub medium_priority: FeeCategory,
    pub high_priority: FeeCategory,
}

#[derive(Debug, Clone)]
pub struct FeeCategory {
    pub count: u64,
    pub rate: f64,
    pub usd_price: f64,
}

struct MempoolCache {
    stats: MempoolStats,
    last_update: Instant,
}

static MEMPOOL_CACHE: Lazy<Mutex<Option<MempoolCache>>> = Lazy::new(|| Mutex::new(None));

impl super::BitcoinRPC {
    pub fn get_mempool_stats(&self) -> Result<MempoolStats> {
        const CACHE_DURATION: Duration = Duration::from_secs(30);

        // Prüfe Cache
        if let Ok(cache) = MEMPOOL_CACHE.lock() {
            if let Some(cached) = &*cache {
                if cached.last_update.elapsed() < CACHE_DURATION {
                    return Ok(cached.stats.clone());
                }
            }
        }

        // Nur eine API-Abfrage für die wichtigsten Daten
        let client = HttpClient::new();
        let mempool_data = client.get("https://mempool.space/api/mempool")
            .send()?
            .json::<Value>()?;

        self.log_debug(&format!("Mempool API Antwort:\n{}", serde_json::to_string_pretty(&mempool_data)?));

        // Basis-Statistiken
        let tx_count = mempool_data.get("count").and_then(|v| v.as_u64()).unwrap_or(0);
        let size = mempool_data.get("vsize").and_then(|v| v.as_u64()).unwrap_or(0);

        self.log_debug(&format!("Mempool Details:
- Transaktionen: {}
- Größe: {:.2} MB", 
            tx_count,
            size as f64 / 1_000_000.0));

        // Hole BTC Preis für USD Berechnung
        let btc_price = client.get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
            .send()?
            .json::<Value>()?
            .get("bitcoin")
            .and_then(|btc| btc.get("usd"))
            .and_then(|usd| usd.as_f64())
            .unwrap_or(43000.0);

        // Gebührenkategorien aus dem Histogramm berechnen
        let mut categories = vec![
            FeeCategory { count: 0, rate: 1.0, usd_price: 0.0 },
            FeeCategory { count: 0, rate: 2.0, usd_price: 0.0 },
            FeeCategory { count: 0, rate: 3.0, usd_price: 0.0 },
            FeeCategory { count: 0, rate: 5.0, usd_price: 0.0 },
        ];

        // Verarbeite Gebühren-Histogramm
        if let Some(histogram) = mempool_data.get("fee_histogram").and_then(|v| v.as_array()) {
            for entry in histogram {
                if let Some(entry_array) = entry.as_array() {
                    if entry_array.len() >= 2 {
                        let fee_rate = entry_array[0].as_f64().unwrap_or(0.0);
                        let count = entry_array[1].as_u64().unwrap_or(0);
                        
                        let idx = match fee_rate {
                            r if r <= 1.0 => 0,  // No Priority: ≤ 1 sat/vB
                            r if r <= 2.0 => 1,  // Low Priority: ≤ 2 sat/vB
                            r if r <= 3.0 => 2,  // Medium Priority: ≤ 3 sat/vB
                            _ => 3,              // High Priority: > 3 sat/vB
                        };
                        categories[idx].count += count;
                        // Aktualisiere die höchste Rate in jeder Kategorie
                        categories[idx].rate = categories[idx].rate.max(fee_rate);
                    }
                }
            }
        }

        // Berechne USD-Preise für eine typische 250-Byte-Transaktion
        for category in &mut categories {
            category.usd_price = (category.rate * 250.0 * btc_price) / 100_000_000.0;
        }

        self.log_debug(&format!("Gebührenkategorien:
- No Priority:   {} TXs @ {:.1} sat/vB (${:.2})
- Low Priority:  {} TXs @ {:.1} sat/vB (${:.2})
- Medium Prior.: {} TXs @ {:.1} sat/vB (${:.2})
- High Priority: {} TXs @ {:.1} sat/vB (${:.2})",
            categories[0].count, categories[0].rate, categories[0].usd_price,
            categories[1].count, categories[1].rate, categories[1].usd_price,
            categories[2].count, categories[2].rate, categories[2].usd_price,
            categories[3].count, categories[3].rate, categories[3].usd_price));

        // Verwende die berechneten Kategorien
        let stats = MempoolStats {
            tx_count,
            size,
            no_priority: categories[0].clone(),
            low_priority: categories[1].clone(),
            medium_priority: categories[2].clone(),
            high_priority: categories[3].clone(),
        };

        // Cache aktualisieren
        if let Ok(mut cache) = MEMPOOL_CACHE.lock() {
            *cache = Some(MempoolCache {
                stats: stats.clone(),
                last_update: Instant::now(),
            });
        }

        Ok(stats)
    }
} 