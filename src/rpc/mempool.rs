use anyhow::Result;
use bitcoincore_rpc::RpcApi;
use serde_json::Value;
use std::time::{Instant, Duration};
use rayon::prelude::*;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct FeeCategory {
    pub count: u64,
    pub rate: f64,    // sat/vB
    pub usd_price: f64,
}

#[derive(Debug, Clone)]
pub struct MempoolStats {
    pub tx_count: u64,
    pub size: u64,
    pub memory_usage: u64,
    pub max_memory: u64,
    pub no_priority: FeeCategory,
    pub low_priority: FeeCategory,
    pub medium_priority: FeeCategory,
    pub high_priority: FeeCategory,
}

pub struct MempoolCache {
    stats: MempoolStats,
    last_update: Instant,
}

// Thread-sicherer Cache mit once_cell
static MEMPOOL_CACHE: Lazy<Mutex<Option<MempoolCache>>> = Lazy::new(|| Mutex::new(None));

impl super::BitcoinRPC {
    const CACHE_DURATION: Duration = Duration::from_secs(10);

    pub fn get_mempool_stats(&self) -> Result<MempoolStats> {
        // Prüfe Cache
        if let Ok(cache_lock) = MEMPOOL_CACHE.lock() {
            if let Some(cache) = &*cache_lock {
                if cache.last_update.elapsed() < Self::CACHE_DURATION {
                    return Ok(cache.stats.clone());
                }
            }
        }

        // Cache ist abgelaufen oder nicht vorhanden - neue Daten holen
        let stats = self.fetch_mempool_stats()?;

        // Cache aktualisieren
        if let Ok(mut cache_lock) = MEMPOOL_CACHE.lock() {
            *cache_lock = Some(MempoolCache {
                stats: stats.clone(),
                last_update: Instant::now(),
            });
        }

        Ok(stats)
    }

    fn fetch_mempool_stats(&self) -> Result<MempoolStats> {
        // Parallele Abfragen mit join statt join4
        let (info, raw_mempool) = rayon::join(
            || self.client.get_mempool_info(),
            || self.client.call::<Value>("getrawmempool", &[true.into()])
        );

        let (estimate_low, estimate_high) = rayon::join(
            || self.client.call::<Value>("estimatesmartfee", &[144.into()]),
            || self.client.call::<Value>("estimatesmartfee", &[2.into()])
        );

        let info = info?;
        let raw_mempool = raw_mempool?;
        let estimate_low = estimate_low?;
        let estimate_high = estimate_high?;

        // Gebühren in sat/vB umrechnen
        let get_fee_rate = |v: &Value| -> f64 {
            v.get("feerate")
                .and_then(|f| f.as_f64())
                .map(|btc| btc * 100_000.0) // BTC/kB zu sat/vB
                .unwrap_or(0.0)
        };

        let low_rate = get_fee_rate(&estimate_low);
        let high_rate = get_fee_rate(&estimate_high);
        let medium_rate = (low_rate + high_rate) / 2.0;

        let category_rates = [
            1.0,           // No Priority (Minimum)
            low_rate,      // Low Priority (~24h)
            medium_rate,   // Medium Priority (~12h)
            high_rate,     // High Priority (~20min)
        ];

        // Rest des Codes wie gehabt...
        let mut categories = vec![
            FeeCategory { count: 0, rate: 0.0, usd_price: 0.0 },
            FeeCategory { count: 0, rate: 0.0, usd_price: 0.0 },
            FeeCategory { count: 0, rate: 0.0, usd_price: 0.0 },
            FeeCategory { count: 0, rate: 0.0, usd_price: 0.0 },
        ];

        // Zähle Transaktionen in den entsprechenden Kategorien
        if let Some(txs) = raw_mempool.as_object() {
            for tx in txs.values() {
                if let Some(fee_rate) = tx.get("fees")
                    .and_then(|f| f.get("base"))
                    .and_then(|b| b.as_f64())
                    .map(|f| f * 100_000_000.0)
                {
                    let idx = match fee_rate {
                        r if r <= category_rates[0] => 0,
                        r if r <= category_rates[1] => 1,
                        r if r <= category_rates[2] => 2,
                        _ => 3,
                    };
                    categories[idx].count += 1;
                }
            }
        }

        // Setze die geschätzten Gebühren
        let btc_price = 43000.0; // TODO: Echten Preis via API abrufen
        for (idx, category) in categories.iter_mut().enumerate() {
            category.rate = category_rates[idx];
            category.usd_price = (category.rate * 250.0 * btc_price) / 100_000_000.0;
        }

        Ok(MempoolStats {
            tx_count: info.size as u64,
            size: info.bytes as u64,
            memory_usage: info.usage as u64,
            max_memory: info.max_mempool as u64,
            no_priority: categories[0].clone(),
            low_priority: categories[1].clone(),
            medium_priority: categories[2].clone(),
            high_priority: categories[3].clone(),
        })
    }
} 