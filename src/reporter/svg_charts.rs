use crate::models::metrics::TrainingSession;

pub struct SvgChartGenerator;

impl SvgChartGenerator {
    pub fn line_chart(
        session: &TrainingSession,
        data_key: &str,
        title: &str,
        color: &str,
        width: u32,
        height: u32,
    ) -> String {
        let records = session.record_entries();
        if records.is_empty() {
            return Self::empty_chart(title, width, height);
        }

        let values: Vec<f64> = match data_key {
            "fitness" => records.iter().map(|r| r.fitness).collect(),
            "pnl" => records.iter().map(|r| r.pnl).collect(),
            "win_rate" => records.iter().map(|r| r.win_rate).collect(),
            "profit_factor" => records.iter().map(|r| r.profit_factor).collect(),
            "sharpe" => records.iter().map(|r| r.sharpe).collect(),
            "max_drawdown" => records.iter().map(|r| r.max_drawdown * 100.0).collect(),
            "trades" => records.iter().map(|r| r.trades as f64).collect(),
            _ => records.iter().map(|r| r.fitness).collect(),
        };

        let gens: Vec<u32> = records.iter().map(|r| r.gen).collect();
        Self::render_svg_line(&gens, &values, title, color, width, height)
    }

    pub fn dashboard_grid(session: &TrainingSession) -> String {
        let records = session.record_entries();
        if records.is_empty() {
            return Self::empty_dashboard();
        }

        let w = 400u32;
        let h = 200u32;

        let chart1 = Self::line_chart(session, "fitness", "Fitness", "#2563eb", w, h);
        let chart2 = Self::line_chart(session, "pnl", "PnL ($)", "#dc2626", w, h);
        let chart3 = Self::line_chart(session, "win_rate", "WinRate (%)", "#16a34a", w, h);
        let chart4 = Self::line_chart(session, "profit_factor", "Profit Factor", "#9333ea", w, h);

        let mut result = String::from("<div style=\"display: grid; grid-template-columns: 1fr 1fr; gap: 16px; max-width: 900px;\">");
        result.push_str("\n  <div>");
        result.push_str(&chart1);
        result.push_str("</div>");
        result.push_str("\n  <div>");
        result.push_str(&chart2);
        result.push_str("</div>");
        result.push_str("\n  <div>");
        result.push_str(&chart3);
        result.push_str("</div>");
        result.push_str("\n  <div>");
        result.push_str(&chart4);
        result.push_str("</div>");
        result.push_str("\n</div>");
        result
    }

    fn render_svg_line(
        gens: &[u32],
        values: &[f64],
        title: &str,
        color: &str,
        width: u32,
        height: u32,
    ) -> String {
        let padding = 40u32;
        let chart_w = width - padding * 2;
        let chart_h = height - padding * 2;

        let min_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let range = if max_val == min_val { 1.0 } else { max_val - min_val };

        let min_gen = *gens.first().unwrap_or(&0) as f64;
        let max_gen = *gens.last().unwrap_or(&0) as f64;
        let gen_range = if max_gen == min_gen { 1.0 } else { max_gen - min_gen };

        let mut path_d = String::new();
        for (i, (gen, val)) in gens.iter().zip(values.iter()).enumerate() {
            let x = padding as f64 + ((*gen as f64 - min_gen) / gen_range) * chart_w as f64;
            let y = padding as f64 + chart_h as f64 - ((val - min_val) / range) * chart_h as f64;

            if i == 0 {
                path_d.push_str(&format!("M {:.1},{:.1} ", x, y));
            } else {
                path_d.push_str(&format!("L {:.1},{:.1} ", x, y));
            }
        }

        let mut area_d = path_d.clone();
        let last_x = padding as f64 + chart_w as f64;
        let base_y = padding as f64 + chart_h as f64;
        let first_x = padding as f64 + ((gens[0] as f64 - min_gen) / gen_range) * chart_w as f64;
        area_d.push_str(&format!("L {:.1},{:.1} L {:.1},{:.1} Z", last_x, base_y, first_x, base_y));

        let mut grid_lines = String::new();
        for i in 0..=5 {
            let y = padding as f64 + (chart_h as f64 * i as f64 / 5.0);
            let line = format!(
                "<line x1=\"{}\" y1=\"{:.1}\" x2=\"{}\" y2=\"{:.1}\" stroke=\"rgb(229,231,235)\" stroke-width=\"1\"/>",
                padding, y, width - padding, y
            );
            grid_lines.push_str(&line);
        }

        let mut points = String::new();
        for (gen, val) in gens.iter().zip(values.iter()) {
            let x = padding as f64 + ((*gen as f64 - min_gen) / gen_range) * chart_w as f64;
            let y = padding as f64 + chart_h as f64 - ((val - min_val) / range) * chart_h as f64;
            points.push_str(&format!(
                "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"{}\"/>",
                x, y, color
            ));
        }

        let y_label = format!("{:.2}", max_val);
        let y_label_min = format!("{:.2}", min_val);
        let title_safe = title.replace(" ", "-");
        let x_min = gens.first().unwrap_or(&0);
        let x_max = gens.last().unwrap_or(&0);
        let x_max_pos = width - padding;
        let pad_minus = padding - 5;
        let y_bottom = padding + chart_h;
        let y_bottom_plus = padding + chart_h + 15;

        let mut svg = String::new();
        svg.push_str(&format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\" style=\"width:100%; height:auto;\">\n",
            width, height
        ));
        svg.push_str("  <defs>\n");
        svg.push_str(&format!(
            "    <linearGradient id=\"grad-{}\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">\n",
            title_safe
        ));
        svg.push_str(&format!(
            "      <stop offset=\"0%\" stop-color=\"{}\" stop-opacity=\"0.3\"/>\n",
            color
        ));
        svg.push_str(&format!(
            "      <stop offset=\"100%\" stop-color=\"{}\" stop-opacity=\"0.05\"/>\n",
            color
        ));
        svg.push_str("    </linearGradient>\n");
        svg.push_str("  </defs>\n");
        svg.push_str(&format!(
            "  <rect width=\"{}\" height=\"{}\" fill=\"rgb(250,250,250)\" rx=\"8\"/>\n",
            width, height
        ));
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"20\" text-anchor=\"end\" font-size=\"12\" font-weight=\"bold\" fill=\"rgb(55,65,81)\">{}</text>\n",
            width, title
        ));
        svg.push_str(&format!("  {}\n", grid_lines));
        svg.push_str(&format!(
            "  <path d=\"{}\" fill=\"url(#grad-{})\"/>\n",
            area_d, title_safe
        ));
        svg.push_str(&format!(
            "  <path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n",
            path_d, color
        ));
        svg.push_str(&format!("  {}\n", points));
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-size=\"10\" fill=\"rgb(107,114,128)\">{}</text>\n",
            pad_minus, padding, y_label
        ));
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-size=\"10\" fill=\"rgb(107,114,128)\">{}</text>\n",
            pad_minus, y_bottom, y_label_min
        ));
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"10\" fill=\"rgb(107,114,128)\">{}</text>\n",
            padding, y_bottom_plus, x_min
        ));
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"10\" fill=\"rgb(107,114,128)\">{}</text>\n",
            x_max_pos, y_bottom_plus, x_max
        ));
        svg.push_str("</svg>");

        svg
    }

    fn empty_chart(title: &str, width: u32, height: u32) -> String {
        format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">\n  <rect width=\"{}\" height=\"{}\" fill=\"rgb(243,244,246)\" rx=\"8\"/>\n  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"rgb(156,163,175)\" font-size=\"14\">{}</text>\n  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"rgb(156,163,175)\" font-size=\"11\">Yetersiz veri</text>\n</svg>",
            width, height, width, height, width / 2, height / 2 - 5, title, width / 2, height / 2 + 15
        )
    }

    fn empty_dashboard() -> String {
        String::from("<div style=\"padding: 20px; background: rgb(243,244,246); border-radius: 8px; text-align: center; color: rgb(156,163,175);\">\n  Dashboard icin yeterli veri yok\n</div>")
    }
}
