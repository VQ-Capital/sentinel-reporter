use clap::Parser;
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod analyzer;
mod dashboard;
mod models;
mod reporter;
mod watcher;

use watcher::fs_watcher::FileWatcher;

#[derive(Parser, Debug)]
#[command(name = "sentinel-reporter")]
#[command(about = "Real-time training audit reporter for VQ-CAPITAL HFT Optimizer")]
#[command(version = "0.1.0")]
struct Cli {
    /// Path to CSV audit log file
    #[arg(short, long, default_value = "optimization_audit_log.csv")]
    csv: PathBuf,

    /// Output markdown report path
    #[arg(short, long, default_value = "SENTINEL_REPORT.md")]
    output: PathBuf,

    /// Watch mode - auto-regenerate on CSV changes
    #[arg(short, long, default_value_t = true)]
    watch: bool,

    /// Generate one-shot report and exit
    #[arg(long)]
    once: bool,

    /// Enable web dashboard (requires dashboard feature)
    #[cfg(feature = "dashboard")]
    #[arg(long)]
    web: bool,

    /// Dashboard port
    #[cfg(feature = "dashboard")]
    #[arg(long, default_value_t = 8080)]
    port: u16,

    /// Log level
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // Setup tracing
    let level = match args.log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .compact()
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("🚀 Sentinel-Reporter v0.1.0 başlatılıyor...");
    info!("📂 CSV Kaynağı: {}", args.csv.display());
    info!("📝 Rapor Çıktısı: {}", args.output.display());

    let csv_path = args.csv;
    let output_path = args.output;

    if args.once {
        info!("📊 Tek seferlik rapor üretiliyor...");
        FileWatcher::generate_report(&csv_path, &output_path).await?;
        info!("✅ Rapor hazır: {}", output_path.display());
        return Ok(());
    }

    // Watch mode
    let watcher = FileWatcher::new(csv_path.clone(), output_path.clone());

    #[cfg(feature = "dashboard")]
    if args.web {
        info!("🌐 Web dashboard başlatılıyor: http://localhost:{}", args.port);
        let csv_path_clone = csv_path.clone();
        tokio::spawn(async move {
            if let Err(e) = dashboard::server::start_server(args.port, csv_path_clone.to_string_lossy().to_string()).await {
                tracing::error!("Dashboard error: {}", e);
            }
        });
    }

    info!("👁️  İzleme modu aktif. Ctrl+C ile durdurun.");

    let watcher_handle = tokio::spawn(async move {
        if let Err(e) = watcher.run().await {
            tracing::error!("Watcher error: {}", e);
        }
    });

    // Wait for shutdown
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("
🛑 Sentinel-Reporter kapatılıyor...");
        }
        _ = watcher_handle => {}
    }

    info!("👋 Görüşmek üzere!");
    Ok(())
}
