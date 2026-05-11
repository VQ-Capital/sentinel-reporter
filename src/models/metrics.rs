use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

/// Single generation record from CSV
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRecord {
    pub timestamp: DateTime<Utc>,
    pub status: RecordStatus,
    pub gen: u32,
    pub fitness: f64,
    pub pnl: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub sharpe: f64,
    pub max_drawdown: f64,
    pub trades: u32,
    pub mutation_rate: f64,
    pub tp: f64,
    pub sl: f64,
    pub risk: f64,
    pub cooldown: u32,
    pub confidence: f64,
    pub time_sec: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum RecordStatus {
    Record,
    Update,
}

/// Parsed CSV row
#[derive(Debug, Clone)]
pub struct CsvRow {
    pub raw: Vec<String>,
}

impl GenerationRecord {
    pub fn from_csv_row(row: &csv::StringRecord) -> anyhow::Result<Self> {
        let parse_f64 = |idx: usize| -> anyhow::Result<f64> {
            row.get(idx)
                .ok_or_else(|| anyhow::anyhow!("Missing column {}", idx))?
                .parse::<f64>()
                .map_err(|e| anyhow::anyhow!("Parse error at column {}: {}", idx, e))
        };

        let parse_u32 = |idx: usize| -> anyhow::Result<u32> {
            row.get(idx)
                .ok_or_else(|| anyhow::anyhow!("Missing column {}", idx))?
                .parse::<u32>()
                .map_err(|e| anyhow::anyhow!("Parse error at column {}: {}", idx, e))
        };

        let status_str = row.get(1).unwrap_or("RECORD");
        let status = match status_str.to_uppercase().as_str() {
            "UPDATE" => RecordStatus::Update,
            _ => RecordStatus::Record,
        };

        let timestamp_str = row.get(0).unwrap_or("");
        let timestamp = DateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S%.f%z")
            .or_else(|_| DateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S%z"))
            .or_else(|_| DateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S"))
            .unwrap_or_else(|_| Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()))
            .with_timezone(&Utc);

        Ok(GenerationRecord {
            timestamp,
            status,
            gen: parse_u32(2)?,
            fitness: parse_f64(3)?,
            pnl: parse_f64(4)?,
            win_rate: parse_f64(5)?,
            profit_factor: parse_f64(6)?,
            sharpe: parse_f64(7)?,
            max_drawdown: parse_f64(8)?,
            trades: parse_u32(9)?,
            mutation_rate: parse_f64(10)?,
            tp: parse_f64(11)?,
            sl: parse_f64(12)?,
            risk: parse_f64(13)?,
            cooldown: parse_u32(14)?,
            confidence: parse_f64(15)?,
            time_sec: parse_f64(16)?,
        })
    }

    pub fn is_profitable(&self) -> bool {
        self.pnl > 0.0
    }

    pub fn is_acceptable_drawdown(&self) -> bool {
        self.max_drawdown < 0.25
    }

    pub fn daily_trades(&self, days: f64) -> f64 {
        self.trades as f64 / days
    }
}

/// Collection of all records
#[derive(Debug, Clone, Default)]
pub struct TrainingSession {
    pub records: Vec<GenerationRecord>,
    pub symbol: String,
    pub dataset_path: String,
    pub total_generations: u32,
    pub population_size: u32,
}

impl TrainingSession {
    pub fn latest(&self) -> Option<&GenerationRecord> {
        self.records.last()
    }

    pub fn record_entries(&self) -> Vec<&GenerationRecord> {
        self.records.iter()
            .filter(|r| r.status == RecordStatus::Record)
            .collect()
    }

    pub fn fitness_trend(&self) -> Trend {
        if self.records.len() < 2 {
            return Trend::Flat;
        }
        let first = self.records.first().unwrap().fitness;
        let last = self.records.last().unwrap().fitness;
        let delta = last - first;

        if delta > 10.0 {
            Trend::Improving
        } else if delta < -10.0 {
            Trend::Declining
        } else {
            Trend::Flat
        }
    }

    pub fn pnl_trend(&self) -> Trend {
        if self.records.len() < 2 {
            return Trend::Flat;
        }
        let first = self.records.first().unwrap().pnl;
        let last = self.records.last().unwrap().pnl;

        if last > first {
            Trend::Improving
        } else if last < first {
            Trend::Declining
        } else {
            Trend::Flat
        }
    }

    pub fn convergence_rate(&self) -> f64 {
        let records = self.record_entries();
        if records.len() < 2 {
            return 0.0;
        }
        let first = records.first().unwrap().fitness;
        let last = records.last().unwrap().fitness;
        let gens = records.last().unwrap().gen.saturating_sub(records.first().unwrap().gen);
        if gens == 0 {
            return 0.0;
        }
        (last - first) / gens as f64
    }

    pub fn plateau_detected(&self, threshold: f64) -> bool {
        let records = self.record_entries();
        if records.len() < 10 {
            return false;
        }
        let recent = &records[records.len()-10..];
        let first = recent.first().unwrap().fitness;
        let last = recent.last().unwrap().fitness;
        (last - first).abs() < threshold
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Trend {
    Improving,
    Declining,
    Flat,
}
