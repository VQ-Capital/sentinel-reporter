use std::fs::File;
use std::path::Path;
use csv::ReaderBuilder;
use tracing::{info, warn};

use crate::models::metrics::GenerationRecord;

pub struct CsvParser;

impl CsvParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> anyhow::Result<crate::models::metrics::TrainingSession> {
        let file = File::open(&path)?;
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut records = Vec::new();
        let mut row_count = 0;

        for result in rdr.records() {
            let row = result?;
            match GenerationRecord::from_csv_row(&row) {
                Ok(record) => {
                    records.push(record);
                    row_count += 1;
                }
                Err(e) => {
                    warn!("Failed to parse row: {}", e);
                }
            }
        }

        info!("Parsed {} records from {}", row_count, path.as_ref().display());

        // Extract metadata from filename or first record
        let symbol = "BTCUSDT".to_string(); // Could be parsed from filename
        let dataset_path = "unknown".to_string();
        let total_generations = records.last().map(|r| r.gen).unwrap_or(0);

        Ok(crate::models::metrics::TrainingSession {
            records,
            symbol,
            dataset_path,
            total_generations,
            population_size: 250, // Default or parse from config
        })
    }

    pub fn parse_incremental<P: AsRef<Path>>(
        path: P,
        last_known_gen: u32,
    ) -> anyhow::Result<Vec<GenerationRecord>> {
        let file = File::open(&path)?;
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut new_records = Vec::new();

        for result in rdr.records() {
            let row = result?;
            if let Ok(record) = GenerationRecord::from_csv_row(&row) {
                if record.gen > last_known_gen {
                    new_records.push(record);
                }
            }
        }

        Ok(new_records)
    }
}
