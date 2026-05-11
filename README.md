
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                    🛡️ SENTINEL-REPORTER v0.1.0                                ║
║                                                                              ║
║           Tamamen Rust ile Yazılmış HFT Eğitim Raporlama Sistemi              ║
║                                                                              ║
╠══════════════════════════════════════════════════════════════════════════════╣

📦 PROJE HAKKINDA
═══════════════════
sentinel-reporter, VQ-CAPITAL HFT Optimizer'ın CSV log dosyasını izleyerek 
otomatik olarak markdown rapor ve SVG grafik üreten, tamamen Rust ile 
yazılmış bir CLI aracıdır.

🎯 TEMEL ÖZELLİKLER
═══════════════════
✅ 100% Rust — Python/matplotlib GEREKTİRMEZ
✅ Gerçek Zamanlı İzleme — CSV değişikliğinde otomatik rapor güncellemesi
✅ Inline SVG Grafikler — Tarayıcıda/VS Code'da doğrudan görünür
✅ Başarı Kriterleri — 🟢BAŞARILI / 🟡UYARI / 🔴BAŞARISIZ değerlendirme
✅ Risk Skoru — 0-100 arası otomatik risk hesaplama
✅ Aşırı Uyum Tespiti — Overfitting sinyalleri otomatik algılama
✅ Web Dashboard — Opsiyonel canlı web arayüzü (SSE)
✅ Tek Seferlik Rapor — CLI ile anlık rapor üretimi

📂 PROJE YAPISI
════════════════
sentinel-reporter/
├── Cargo.toml              # Bağımlılıklar ve build config
├── README.md               # Dokümantasyon
├── build.sh                # Build script
├── .gitignore
├── SAMPLE_REPORT.md        # Örnek çıktı
├── PROJECT_SUMMARY.txt     # Proje özeti
├── src/
│   ├── main.rs             # CLI entry point (clap)
│   ├── models/
│   │   ├── metrics.rs      # GenerationRecord, TrainingSession, Trend
│   │   └── criteria.rs     # SuccessCriteria, RiskScore (0-100)
│   ├── analyzer/
│   │   ├── parser.rs       # CSV okuyucu
│   │   └── calculator.rs   # İstatistik hesaplayıcı
│   ├── reporter/
│   │   ├── markdown.rs     # Markdown rapor üretici
│   │   └── svg_charts.rs   # Pure Rust SVG grafik motoru
│   ├── watcher/
│   │   └── fs_watcher.rs   # notify crate ile dosya izleme
│   └── dashboard/
│       └── server.rs       # Axum SSE web sunucusu (opsiyonel)
└── templates/
    └── dashboard.html      # Canlı web arayüzü

🚀 KULLANIM
═══════════
# 1. İzleme Modu (Varsayılan)
./sentinel-reporter   --csv ../sentinel-optimizer/optimization_audit_log.csv   --output SENTINEL_REPORT.md

# 2. Tek Seferlik Rapor
./sentinel-reporter --csv train-30d.log --once

# 3. Web Dashboard ile
./sentinel-reporter --csv train-30d.log --web --port 8080

# 4. Detaylı Log
RUST_LOG=debug ./sentinel-reporter -c train-30d.log

📊 BAŞARI KRİTERLERİ
════════════════════
| Kriter        | 🟢 BAŞARILI | 🟡 UYARI   | 🔴 BAŞARISIZ |
|---------------|------------|-----------|-------------|
| PnL           | > 0$       | -5$ ~ 0$  | < -5$       |
| WinRate       | > 55%      | 45-55%    | < 45%       |
| Profit Factor | > 1.5      | 1.0-1.5   | < 1.0       |
| Sharpe        | > 1.0      | 0-1.0     | < 0         |
| Max Drawdown  | < 10%      | 10-25%    | > 25%       |
| Daily Trades  | > 100      | 20-100    | < 20        |

🎯 RISK SKORU (0-100)
══════════════════════
  0-29:  🟢 DÜŞÜK RİSK   → Devam edin
  30-59: 🟡 ORTA RİSK    → Dikkatli olun
  60-100:🔴 YÜKSEK RİSK  → Eğitimi DURDURUN!

🔧 BAĞIMLILIKLAR
════════════════
• tokio         — Async runtime
• notify        — Cross-platform file watcher
• csv           — CSV parsing
• serde         — Serialization
• chrono        — Time handling
• clap          — CLI parser
• tracing       — Logging
• anyhow        — Error handling
• axum          — Web framework (opsiyonel)
• tower-http    — HTTP middleware (opsiyonel)

🛠️ GELİŞTİRME
═══════════════
cargo test          # Testleri çalıştır
cargo fmt           # Kod formatla
cargo clippy        # Lint kontrolü
cargo doc --open    # Dokümantasyon

📜 LİSANS
═════════
MIT License — VQ-CAPITAL Team

╚══════════════════════════════════════════════════════════════════════════════╝
