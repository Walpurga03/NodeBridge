use anyhow::Result;
use bitcoincore_rpc::RpcApi;
use super::BitcoinRPC;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct MempoolInfo {
    pub size: u64,
    pub bytes: u64,
    pub usage: u64,
    pub max_mem: u64,
    pub min_fee: f64,
    pub max_fee: f64,
    pub fee_histogram: Vec<(f64, u64)>,
    // Neue Felder für detaillierte Statistiken
    pub total_fee: f64,           // Gesamtgebühren im Mempool
    pub avg_fee_rate: f64,        // Durchschnittliche Gebührenrate
    pub size_distribution: Vec<(u64, u64)>,  // (Größenbereich, Anzahl)
    pub age_distribution: Vec<(u64, u64)>,   // (Alter in Minuten, Anzahl)
}

impl BitcoinRPC {
    pub fn get_mempool_info(&self) -> Result<MempoolInfo> {
        let raw_mempool = self.client.get_raw_mempool_verbose()?;
        let mempool_info = self.client.get_mempool_info()?;
        
        // Aktuelle Unix-Zeit in Sekunden (als u64)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Gebühren-Histogramm berechnen
        let mut fees: Vec<(f64, u64)> = raw_mempool
            .iter()
            .map(|(_, tx)| {
                let fee_rate = (tx.fees.base.to_btc() * 100_000_000.0) / tx.vsize as f64;
                (fee_rate, 1)
            })
            .collect();
        
        fees.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

        // Neue Gebührenbereiche
        let fee_ranges = vec![1.0, 4.0, 7.0, 11.0, 16.0, 21.0];
        let mut histogram = Vec::new();
        
        for window in fee_ranges.windows(2) {
            let min_fee = window[0];
            let max_fee = window[1];
            
            let count = fees.iter()
                .filter(|(fee, _)| *fee >= min_fee && *fee < max_fee)
                .map(|(_, count)| *count)
                .sum();
                
            histogram.push((min_fee, count));
        }
        
        // Über 21 sat/vB
        let high_fee_count = fees.iter()
            .filter(|(fee, _)| *fee >= 21.0)
            .map(|(_, count)| *count)
            .sum();
        histogram.push((21.0, high_fee_count));

        let min_fee = fees.first().map(|(fee, _)| *fee).unwrap_or(0.0);
        let max_fee = fees.last().map(|(fee, _)| *fee).unwrap_or(0.0);

        // Erweiterte Statistiken berechnen
        let mut total_fee = 0.0;
        let mut size_dist = vec![(0, 0); 5];  // 5 Größenbereiche
        let mut age_dist = vec![(0, 0); 6];   // 6 Zeitbereiche

        for (_, tx) in &raw_mempool {
            total_fee += tx.fees.base.to_btc();
            
            // Größenverteilung
            let size_idx = match tx.vsize {
                s if s < 500 => 0,
                s if s < 1000 => 1,
                s if s < 2000 => 2,
                s if s < 5000 => 3,
                _ => 4,
            };
            size_dist[size_idx].1 += 1;

            // Altersverteilung mit korrekter Zeit (beide als u64)
            let age = if now > tx.time {
                (now - tx.time) / 60
            } else {
                0
            };
            
            let age_idx = match age {
                a if a < 10 => 0,
                a if a < 30 => 1,
                a if a < 60 => 2,
                a if a < 180 => 3,
                a if a < 360 => 4,
                _ => 5,
            };
            age_dist[age_idx].1 += 1;
        }

        Ok(MempoolInfo {
            size: mempool_info.size as u64,
            bytes: mempool_info.bytes as u64,
            usage: mempool_info.usage as u64,
            max_mem: mempool_info.max_mempool as u64,
            min_fee,
            max_fee,
            fee_histogram: histogram,
            total_fee,
            avg_fee_rate: total_fee / mempool_info.size as f64,
            size_distribution: size_dist,
            age_distribution: age_dist,
        })
    }
} 