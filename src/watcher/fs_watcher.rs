use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use tracing::{info, error};

use crate::analyzer::parser::CsvParser;
use crate::reporter::markdown::MarkdownReporter;

pub struct FileWatcher {
    csv_path: PathBuf,
    output_path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum WatchEvent {
    FileChanged,
    Shutdown,
}

impl FileWatcher {
    pub fn new(csv_path: PathBuf, output_path: PathBuf) -> Self {
        Self {
            csv_path,
            output_path,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let csv_path = self.csv_path.clone();
        let output_path = self.output_path.clone();

        info!("🔍 İzleniyor: {}", csv_path.display());
        info!("📝 Rapor Çıktısı: {}", output_path.display());

        // Initial parse
        Self::generate_report(&csv_path, &output_path).await?;

        // Setup watcher using notify v8 API
        let (tx, mut rx) = mpsc::channel::<Result<Event, notify::Error>>(100);

        let csv_filename = csv_path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let watch_path = csv_path.parent()
            .unwrap_or(Path::new("."))
            .to_path_buf();

        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                let _ = tx.try_send(res);
            },
            Config::default(),
        )?;

        watcher.watch(&watch_path, RecursiveMode::NonRecursive)?;

        info!("👁️  Watcher aktif. Değişiklik bekleniyor... (Ctrl+C ile durdurun)");

        let mut last_modified = std::time::SystemTime::UNIX_EPOCH;
        let mut pending_update = false;
        let mut debounce_timer = tokio::time::interval(tokio::time::Duration::from_secs(2));

        loop {
            tokio::select! {
                Some(result) = rx.recv() => {
                    match result {
                        Ok(event) => {
                            // Check if our CSV file is in the event paths
                            let is_our_file = event.paths.iter().any(|p| {
                                p.file_name()
                                    .map(|n| n.to_string_lossy() == csv_filename)
                                    .unwrap_or(false)
                            });

                            if is_our_file && (event.kind.is_modify() || event.kind.is_create()) {
                                // Check actual modification time to avoid duplicate triggers
                                if let Ok(metadata) = std::fs::metadata(&csv_path) {
                                    if let Ok(modified) = metadata.modified() {
                                        if modified > last_modified {
                                            last_modified = modified;
                                            pending_update = true;
                                            info!("📁 CSV değişikliği tespit edildi (debounce bekleniyor)...");
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("Watch error: {}", e);
                        }
                    }
                }
                _ = debounce_timer.tick() => {
                    if pending_update {
                        pending_update = false;
                        info!("🔄 Rapor güncelleniyor...");
                        if let Err(e) = Self::generate_report(&csv_path, &output_path).await {
                            error!("Rapor üretme hatası: {}", e);
                        }
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    info!("
🛑 Ctrl+C alındı, kapatılıyor...");
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn generate_report(csv_path: &Path, output_path: &Path) -> anyhow::Result<()> {
        let session = CsvParser::parse_file(csv_path)?;
        let report = MarkdownReporter::generate(&session);

        tokio::fs::write(output_path, report).await?;

        if let Some(latest) = session.latest() {
            let risk_score = crate::models::criteria::RiskScore::calculate(latest, 30.0);
            let (_status, text) = crate::models::criteria::RiskScore::classify(risk_score);

            info!(
                "✅ Rapor güncellendi | Gen: {:3} | Fitness: {:8.2} | PnL: {:8.4}$ | Risk: {:2}/100 {}",
                latest.gen,
                latest.fitness,
                latest.pnl,
                risk_score,
                text,
            );
        } else {
            info!("✅ Rapor güncellendi (henüz veri yok)");
        }

        Ok(())
    }
}
