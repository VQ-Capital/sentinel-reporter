use serde::{Deserialize, Serialize};

/// Success criteria definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Criterion {
    pub name: String,
    pub metric: String,
    pub green_threshold: f64,
    pub yellow_threshold: f64,
    pub red_threshold: f64,
    pub higher_is_better: bool,
    pub unit: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Success,
    Warning,
    Danger,
}

impl Status {
    pub fn emoji(&self) -> &'static str {
        match self {
            Status::Success => "✅",
            Status::Warning => "⚠️",
            Status::Danger => "🔴",
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            Status::Success => "green",
            Status::Warning => "orange",
            Status::Danger => "red",
        }
    }
}

impl Criterion {
    pub fn evaluate(&self, value: f64) -> Status {
        if self.higher_is_better {
            if value >= self.green_threshold {
                Status::Success
            } else if value >= self.yellow_threshold {
                Status::Warning
            } else {
                Status::Danger
            }
        } else {
            if value <= self.green_threshold {
                Status::Success
            } else if value <= self.yellow_threshold {
                Status::Warning
            } else {
                Status::Danger
            }
        }
    }
}

/// Default criteria set for HFT optimization
pub fn default_criteria() -> Vec<Criterion> {
    vec![
        Criterion {
            name: "Net PnL".to_string(),
            metric: "pnl".to_string(),
            green_threshold: 0.0,
            yellow_threshold: -5.0,
            red_threshold: -999.0,
            higher_is_better: true,
            unit: "$".to_string(),
        },
        Criterion {
            name: "Win Rate".to_string(),
            metric: "win_rate".to_string(),
            green_threshold: 55.0,
            yellow_threshold: 45.0,
            red_threshold: 0.0,
            higher_is_better: true,
            unit: "%".to_string(),
        },
        Criterion {
            name: "Profit Factor".to_string(),
            metric: "profit_factor".to_string(),
            green_threshold: 1.5,
            yellow_threshold: 1.0,
            red_threshold: 0.0,
            higher_is_better: true,
            unit: "x".to_string(),
        },
        Criterion {
            name: "Sharpe Ratio".to_string(),
            metric: "sharpe".to_string(),
            green_threshold: 1.0,
            yellow_threshold: 0.0,
            red_threshold: -999.0,
            higher_is_better: true,
            unit: "".to_string(),
        },
        Criterion {
            name: "Max Drawdown".to_string(),
            metric: "max_drawdown".to_string(),
            green_threshold: 0.10,
            yellow_threshold: 0.25,
            red_threshold: 1.0,
            higher_is_better: false,
            unit: "%".to_string(),
        },
        Criterion {
            name: "Daily Trades".to_string(),
            metric: "daily_trades".to_string(),
            green_threshold: 100.0,
            yellow_threshold: 20.0,
            red_threshold: 0.0,
            higher_is_better: true,
            unit: "".to_string(),
        },
        Criterion {
            name: "Fitness Trend".to_string(),
            metric: "fitness_trend".to_string(),
            green_threshold: 1.0,
            yellow_threshold: 0.0,
            red_threshold: -1.0,
            higher_is_better: true,
            unit: "".to_string(),
        },
    ]
}

/// Risk score calculator (0-100)
pub struct RiskScore;

impl RiskScore {
    pub fn calculate(latest: &super::metrics::GenerationRecord, _days: f64) -> u8 {
        let mut score = 0u8;

        // PnL contribution (0-25)
        if latest.pnl < -10.0 {
            score += 25;
        } else if latest.pnl < 0.0 {
            score += (25.0 * latest.pnl.abs() / 10.0) as u8;
        }

        // WinRate contribution (0-20)
        if latest.win_rate < 30.0 {
            score += 20;
        } else if latest.win_rate < 50.0 {
            score += (20.0 * (50.0 - latest.win_rate) / 20.0) as u8;
        }

        // PF contribution (0-20)
        if latest.profit_factor < 0.5 {
            score += 20;
        } else if latest.profit_factor < 1.0 {
            score += (20.0 * (1.0 - latest.profit_factor) / 0.5) as u8;
        }

        // Sharpe contribution (0-20)
        if latest.sharpe < -2.0 {
            score += 20;
        } else if latest.sharpe < 0.0 {
            score += (20.0 * latest.sharpe.abs() / 2.0) as u8;
        }

        // Drawdown contribution (0-15)
        if latest.max_drawdown > 0.50 {
            score += 15;
        } else if latest.max_drawdown > 0.10 {
            score += (15.0 * (latest.max_drawdown - 0.10) / 0.40) as u8;
        }

        score.min(100)
    }

    pub fn classify(score: u8) -> (Status, &'static str) {
        match score {
            0..=29 => (Status::Success, "DÜŞÜK RİSK 🟢"),
            30..=59 => (Status::Warning, "ORTA RİSK 🟡"),
            _ => (Status::Danger, "YÜKSEK RİSK 🔴"),
        }
    }
}
