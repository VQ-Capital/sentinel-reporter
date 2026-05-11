
═══════════════════════════════════════════════════════════════════════════════
                    SENTINEL-REPORTER v0.1.1 - FIX LOG
═══════════════════════════════════════════════════════════════════════════════

Tüm derleme hataları düzeltildi. İşte yapılan değişiklikler:

1. Cargo.toml
   ───────────
   • owo-colors bağımlılığı kaldırıldı (kullanılmıyordu)
   • tokio-stream "sync" feature eklendi (BroadcastStream için)
   • rust-version = "1.75" eklendi

2. src/models/metrics.rs
   ──────────────────────
   • DateTime<FixedOffset> → DateTime<Utc> dönüşüm hatası düzeltildi
   • unwrap_or_else(|_| Utc::now()) yerine with_timezone kullanıldı

3. src/models/criteria.rs
   ─────────────────────
   • use owo_colors::OwoColorize; kaldırıldı
   • RiskScore::calculate() _days parametresi _days olarak yeniden adlandırıldı

4. src/analyzer/calculator.rs
   ─────────────────────────
   • Python format syntax (.2f, .1f) → Rust format syntax (.2, .1) düzeltildi
   • std_dev fonksiyonu Self::std_dev olarak çağrıldı (scope hatası)

5. src/reporter/svg_charts.rs
   ─────────────────────────
   • format!() makrosu içinde ## hex renk kodları sorunu ÇÖZÜLDÜ
   • Tamamen string concatenation (push_str) ile yeniden yazıldı
   • Artık format! raw string içinde # karakteri sorunu yok

6. src/reporter/markdown.rs
   ───────────────────────
   • Tüm .2f, .4f, .6f format specifier'ları .2, .4, .6 olarak düzeltildi
   • Kullanılmayan import'lar kaldırıldı (RecordStatus, Status)

7. src/watcher/fs_watcher.rs
   ─────────────────────────
   • Kullanılmayan import'lar kaldırıldı (Arc, warn, TrainingSession)
   • format! string'lerindeki .2f → .2 düzeltildi
   • process_events fonksiyonu kaldırıldı (artık gerek yok)

8. src/main.rs
   ───────────
   • String → PathBuf dönüşüm hatası düzeltildi (.into())
   • FileWatcher::new() tuple dönüşümü kaldırıldı
   • process_events çağrısı kaldırıldı
   • dashboard::server::start_server path düzeltildi

9. src/dashboard/server.rs
   ──────────────────────
   • tokio_stream::wrappers::BroadcastStream "sync" feature gereksinimi düzeltildi
   • let mut rx = tx.subscribe(); (mut eklendi)
   • Kullanılmayan import'lar temizlendi

10. src/dashboard/mod.rs
    ───────────────────
    • #[cfg(feature = "dashboard")] guard'ları eklendi
    • Non-dashboard build için stub modül eklendi

11. src/analyzer/parser.rs
    ─────────────────────
    • Kullanılmayan import'lar kaldırıldı (RecordStatus, Trend)

═══════════════════════════════════════════════════════════════════════════════
                          DERLEME KOMUTLARI
═══════════════════════════════════════════════════════════════════════════════

# Minimal build (dashboard olmadan):
cargo build --no-default-features

# Tam build (dashboard ile):
cargo build --release

# Test et:
cargo test --no-default-features

# Örnek veri ile çalıştır:
./target/release/sentinel-reporter --csv test_data.csv --output REPORT.md --once

═══════════════════════════════════════════════════════════════════════════════
