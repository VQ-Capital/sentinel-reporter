use crate::models::metrics::TrainingSession;

pub struct Statistics;

impl Statistics {
    /// Calculate fitness improvement rate
    pub fn fitness_improvement_rate(session: &TrainingSession) -> f64 {
        let records = session.record_entries();
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

    /// Calculate percentage improvement
    pub fn fitness_pct_improvement(session: &TrainingSession) -> f64 {
        let records = session.record_entries();
        if records.len() < 2 {
            return 0.0;
        }
        let first = records.first().unwrap().fitness;
        let last = records.last().unwrap().fitness;
        if first == 0.0 {
            return 0.0;
        }
        ((last - first) / first.abs()) * 100.0
    }

    /// Calculate trade count change
    pub fn trade_count_change(session: &TrainingSession) -> (i64, f64) {
        if session.records.len() < 2 {
            return (0, 0.0);
        }
        let first = session.records.first().unwrap().trades as i64;
        let last = session.records.last().unwrap().trades as i64;
        let delta = last - first;
        let pct = if first == 0 {
            0.0
        } else {
            (delta as f64 / first as f64) * 100.0
        };
        (delta, pct)
    }

    /// Detect if system is overfitting (too few trades, weird winrate)
    pub fn overfitting_signals(session: &TrainingSession) -> Vec<String> {
        let mut signals = Vec::new();
        let latest = match session.latest() {
            Some(r) => r,
            None => return signals,
        };
        let first = match session.records.first() {
            Some(r) => r,
            None => return signals,
        };

        // Signal 1: Trade count dropped too much
        let trade_pct_change = if first.trades > 0 {
            ((latest.trades as f64 - first.trades as f64) / first.trades as f64) * 100.0
        } else {
            0.0
        };
        if trade_pct_change < -50.0 {
            signals.push(format!(
                "İşlem sayısı aşırı düştü: {} -> {} ({:.1}%)",
                first.trades, latest.trades, trade_pct_change
            ));
        }

        // Signal 2: WinRate worse than random
        if latest.win_rate < 50.0 && latest.win_rate > 0.0 {
            signals.push(format!(
                "WinRate rastgele tahminden KÖTÜ: {:.2}% (rastgele=50%)",
                latest.win_rate
            ));
        }

        // Signal 3: PF extremely low
        if latest.profit_factor < 0.5 {
            signals.push(format!(
                "Profit Factor çok düşük: {:.2} (her 1$ kazanç için {:.0}$ zarar)",
                latest.profit_factor,
                1.0 / latest.profit_factor.max(0.001)
            ));
        }

        // Signal 4: Sharpe very negative
        if latest.sharpe < -10.0 {
            signals.push(format!(
                "Sharpe aşırı negatif: {:.2} (risk alarak kaybediyor)",
                latest.sharpe
            ));
        }

        // Signal 5: Drawdown too high
        if latest.max_drawdown > 0.50 {
            signals.push(format!(
                "Max Drawdown çok yüksek: {:.2}% (>50% sermaye erimesi)",
                latest.max_drawdown * 100.0
            ));
        }

        signals
    }

    /// Calculate expected value per trade
    pub fn ev_per_trade(latest: &crate::models::metrics::GenerationRecord) -> f64 {
        if latest.trades == 0 {
            return 0.0;
        }
        latest.pnl / latest.trades as f64
    }

    /// Calculate parameter stability (std dev of recent params)
    pub fn parameter_stability(session: &TrainingSession, window: usize) -> Vec<(String, f64)> {
        let records = session.record_entries();
        if records.len() < window {
            return Vec::new();
        }

        let recent = &records[records.len() - window..];

        let tp_values: Vec<f64> = recent.iter().map(|r| r.tp).collect();
        let sl_values: Vec<f64> = recent.iter().map(|r| r.sl).collect();
        let conf_values: Vec<f64> = recent.iter().map(|r| r.confidence).collect();

        vec![
            ("TP".to_string(), Self::std_dev(&tp_values)),
            ("SL".to_string(), Self::std_dev(&sl_values)),
            ("Confidence".to_string(), Self::std_dev(&conf_values)),
        ]
    }

    fn std_dev(values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        variance.sqrt()
    }
}
