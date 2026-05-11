use chrono::Local;
use crate::models::metrics::{TrainingSession, Trend};
use crate::models::criteria::{default_criteria, RiskScore};
use crate::analyzer::calculator::Statistics;
use crate::reporter::svg_charts::SvgChartGenerator;

pub struct MarkdownReporter;

impl MarkdownReporter {
    pub fn generate(session: &TrainingSession) -> String {
        let latest = match session.latest() {
            Some(r) => r,
            None => return Self::empty_report(),
        };

        let criteria = default_criteria();
        let risk_score = RiskScore::calculate(latest, 30.0);
        let (risk_status, risk_text) = RiskScore::classify(risk_score);
        let records = session.record_entries();
        let trade_change = Statistics::trade_count_change(session);
        let overfitting = Statistics::overfitting_signals(session);
        let ev = Statistics::ev_per_trade(latest);
        let stability = Statistics::parameter_stability(session, 10);

        // Build criteria table rows
        let mut criteria_rows = String::new();
        for crit in &criteria {
            let value = match crit.metric.as_str() {
                "pnl" => latest.pnl,
                "win_rate" => latest.win_rate,
                "profit_factor" => latest.profit_factor,
                "sharpe" => latest.sharpe,
                "max_drawdown" => latest.max_drawdown * 100.0,
                "daily_trades" => latest.daily_trades(30.0),
                "fitness_trend" => match session.fitness_trend() {
                    Trend::Improving => 1.0,
                    Trend::Flat => 0.0,
                    Trend::Declining => -1.0,
                },
                _ => 0.0,
            };
            let status = crit.evaluate(value);
            criteria_rows.push_str(&format!(
                "| {} | {:.4}{} | {} {} |
",
                crit.name, value, crit.unit, status.emoji(), status.color_class()
            ));
        }

        // Build overfitting alerts
        let overfitting_section = if overfitting.is_empty() {
            "✅ **Aşırı uyum sinyali tespit edilmedi**
".to_string()
        } else {
            let mut s = "🔴 **Aşırı Uyum (Overfitting) Uyarıları:**

".to_string();
            for signal in &overfitting {
                s.push_str(&format!("- ⚠️ {}
", signal));
            }
            s
        };

        // Parameter stability
        let mut stability_rows = String::new();
        for (name, std) in &stability {
            let status = if *std < 0.001 {
                "✅ Stabil"
            } else if *std < 0.01 {
                "🟡 Dalgalı"
            } else {
                "🔴 Volatil (Overfitting?)"
            };
            stability_rows.push_str(&format!("| {} | {:.6} | {} |
", name, std, status));
        }

        // Generate charts
        let fitness_chart = SvgChartGenerator::line_chart(session, "fitness", "Fitness Evrimi", "##2563eb", 800, 300);
        let pnl_chart = SvgChartGenerator::line_chart(session, "pnl", "PnL ($)", "##dc2626", 800, 300);
        let dashboard = SvgChartGenerator::dashboard_grid(session);

        // Trend indicators
        let fitness_trend_emoji = match session.fitness_trend() {
            Trend::Improving => "📈",
            Trend::Declining => "📉",
            Trend::Flat => "➡️",
        };
        let pnl_trend_emoji = match session.pnl_trend() {
            Trend::Improving => "📈",
            Trend::Declining => "📉",
            Trend::Flat => "➡️",
        };

        format!(
            r#"# 📊 Sentinel Eğitim Raporu
> **Otomatik Üretim:** {timestamp}
> **Sembol:** {symbol}
> **Toplam Jenerasyon:** {total_gen}
> **Popülasyon:** {pop}
> **RECORD Girişleri:** {record_count}

---

## 🎯 Genel Durum Özeti

| Metrik | Değer | Trend |
|--------|-------|-------|
| **Risk Skoru** | <span style="color:{risk_color};font-size:20px;">**{risk_score}/100**</span> — {risk_text} | — |
| **Son Fitness** | {fitness:.2} | {fitness_emoji} {fitness_trend:?} |
| **Son PnL** | {pnl_sign}{pnl:.4}$ | {pnl_emoji} {pnl_trend:?} |
| **Son WinRate** | {win_rate:.2}% | — |
| **Son Profit Factor** | {pf:.2} | — |
| **Son Sharpe** | {sharpe:.2} | — |
| **Son Drawdown** | {dd:.2}% | — |
| **Toplam İşlem** | {trades} | — |
| **İşlem Sayısı Değişimi** | {trade_delta:+} ({trade_pct:+.1}%) | — |
| **İşlem Başına EV** | {ev:.6}$ | — |

---

## ✅ Başarı Kriterleri Değerlendirmesi

| Kriter | Değer | Durum |
|--------|-------|-------|
{criteria_rows}

---

## 📈 Fitness & PnL Evrimi

{fitness_chart}

**Yorum:** Fitness {fitness_comment}. PnL {pnl_comment}.

{pnl_chart}

---

## 📊 Risk Dashboard

{dashboard}

---

## 🚨 Aşırı Uyum Analizi

{overfitting_section}

---

## 🔧 Parametre Stabilitesi

| Parametre | Std. Sapma | Durum |
|-----------|------------|-------|
{stability_rows}

---

## 🧠 Konverjans Analizi

- **Yakınsama Hızı:** {conv_rate:.2} fitness/jenerasyon
- **Fitness İyileşmesi:** {fitness_pct:+.1}%
- **Plato Tespiti:** {plateau}
- **Son 10 Gen İyileşme:** {last_10_delta:+.2}

---

## 💡 AI Tavsiyesi

{advice}

---

*Bu rapor Sentinel-Reporter v0.1.0 tarafından otomatik üretilmiştir.*
*VQ-CAPITAL HFT Optimizer | Rust-Powered Analysis*
"#,
            timestamp = Local::now().format("%Y-%m-%d %H:%M:%S"),
            symbol = session.symbol,
            total_gen = session.total_generations,
            pop = session.population_size,
            record_count = records.len(),
            risk_color = risk_status.color_class(),
            risk_score = risk_score,
            risk_text = risk_text,
            fitness = latest.fitness,
            fitness_emoji = fitness_trend_emoji,
            fitness_trend = session.fitness_trend(),
            pnl_sign = if latest.pnl > 0.0 { "+" } else { "" },
            pnl = latest.pnl,
            pnl_emoji = pnl_trend_emoji,
            pnl_trend = session.pnl_trend(),
            win_rate = latest.win_rate,
            pf = latest.profit_factor,
            sharpe = latest.sharpe,
            dd = latest.max_drawdown * 100.0,
            trades = latest.trades,
            trade_delta = trade_change.0,
            trade_pct = trade_change.1,
            ev = ev,
            criteria_rows = criteria_rows,
            fitness_chart = fitness_chart,
            pnl_chart = pnl_chart,
            fitness_comment = if session.fitness_trend() == Trend::Improving { "iyileşiyor ✅" } else { "sorunlu ❌" },
            pnl_comment = if latest.pnl > 0.0 { "pozitif 🟢" } else { "hâlâ negatif 🔴" },
            dashboard = dashboard,
            overfitting_section = overfitting_section,
            stability_rows = stability_rows,
            conv_rate = Statistics::fitness_improvement_rate(session),
            fitness_pct = Statistics::fitness_pct_improvement(session),
            plateau = if session.plateau_detected(10.0) { "EVET ⚠️ (Son 10 jenerasyonda değişim < 10)" } else { "HAYIR ✅" },
            last_10_delta = if records.len() >= 10 { 
                records.last().unwrap().fitness - records[records.len()-10].fitness 
            } else { 0.0 },
            advice = Self::generate_advice(risk_score, latest, &overfitting),
        )
    }

    fn empty_report() -> String {
        r#"# 📊 Sentinel Eğitim Raporu

> Henüz veri yok. CSV dosyası bekleniyor...

*sentinel-reporter çalışıyor...*
"#.to_string()
    }

    fn generate_advice(risk_score: u8, latest: &crate::models::metrics::GenerationRecord, signals: &[String]) -> String {
        if risk_score > 70 {
            format!(
                r#"🔴 **KRİTİK DURUM** — Eğitimi DURDURUN!

- Risk skoru {} çok yüksek
- Sistem zarar ediyor (PnL: {:.4}$)
- Fitness fonksiyonunu gözden geçirin
- Genom boyutunu azaltın (overfitting!)
- Confidence threshold'u düşürün (şu an: {:.3})

Tespit edilen sorunlar:
{}
"#,
                risk_score,
                latest.pnl,
                latest.confidence,
                signals.iter().map(|s| format!("  - {}", s)).collect::<Vec<_>>().join("
")
            )
        } else if risk_score > 40 {
            format!(
                r#"🟡 **DİKKAT** — İyileştirme Gerekli

- PnL hâlâ negatif, parametreleri ayarlayın
- İşlem sayısı yeterli mi kontrol edin
- Mutasyon oranını artırın (plato olabilir)
- Validation set ile doğrulayın
"#
            )
        } else {
            r#"🟢 **İYİ DURUM** — Devam Edin

- Sistem sağlıklı ilerliyor
- Mevcut parametreleri koruyun
- Canlı test için hazırlanın
"#.to_string()
        }
    }
}
