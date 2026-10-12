use std::collections::VecDeque;

use chrono::{DateTime, Utc};
use iced::{Background, Color, Length, border, padding, widget::{Space, container, row, text}};
use iced::font::{Family, Font, Weight};
use iced::{widget::column};
use iced_fluent_theme::shadow;
use plotters::{prelude::ChartBuilder, style::RGBColor};
use plotters_iced2::{Chart, ChartWidget, DrawingBackend};

use crate::{Element, state::MainMessage};

pub type SensorSample = (DateTime<Utc>, u32);

pub struct SensorGraph {
    title: &'static str,
    data: VecDeque<SensorSample>,
    color: RGBColor,
}

impl SensorGraph {

    pub fn new(title: &'static str) -> Self {
        Self {
            title,
            data: VecDeque::default(),
            color: RGBColor(255,255,255),
        }
    }

    pub fn push_data(&mut self, data: SensorSample) {
        self.data.push_front(data);
    }

    fn rate_per_minute(&self) -> String {
        if self.data.is_empty() {
            return "-/min".to_string();
        }

        let sum: u32 = Iterator::sum(self.data.iter().map(|s| s.1));
        let avg = f64::from(sum) / (self.data.len() as f64);

        format!("{:.2}/min", avg)
    }

    fn last_rate_per_minute(&self) -> String {
        return self.data.front()
            .map(|d| d.1.to_string())
            .unwrap_or("-".into())
    }

    pub fn view(&self)-> Element<'_, MainMessage> {
        let semibold = Font {
            family: Family::Name("Segoe UI"),
            weight: Weight::Semibold,
            ..Default::default()
        };

        let rate_text = text(self.rate_per_minute()).width(Length::Shrink).size(16).line_height(1.2);
        let title_text = text(self.title).width(Length::Fill).font(semibold).size(16).line_height(1.2);
        let header_row = row![title_text, rate_text];

        let spacing = Space::new().height(14);

        let last_activity_rate_text = text(format!("Last minute ({}) (122)", self.last_rate_per_minute())).font(semibold).style(|_| text::Style {
            color: Some(Color::WHITE.scale_alpha(0.7)),
        }).size(12).height(18).center();

        let chart = ChartWidget::new(self).height(Length::Fill).width(Length::Fill);

        let chart_backdrop = container(chart).height(167).width(Length::Fill).style(|_| container::Style {
            border: border::rounded(7),
            background: Background::Color(Color::WHITE.scale_alpha(0.05)).into(),
            shadow: shadow::DARK.shadow2,
            snap: true,
            ..Default::default()
        }).clip(true);

        let container = column![
            header_row,
            spacing,
            chart_backdrop,
            last_activity_rate_text
        ];

        container.into()
    }
}

impl Chart<MainMessage> for SensorGraph {
    type State = ();
    fn build_chart<DB:DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        use plotters::prelude::*;

        // Acquire time range
        let oldest_time = self
            .data
            .iter()
            .map(|s| s.0)
            .min()
            .unwrap_or(DateTime::from_timestamp(0, 0).unwrap());
        let newest_time = self
            .data
            .iter()
            .map(|s| s.0)
            .max()
            .unwrap_or(DateTime::from_timestamp(0, 0).unwrap());

        let max_value = self
            .data
            .iter()
            .map(|s| s.1)
            .max()
            .unwrap_or(100);

        let mut chart = builder
            .margin(0)
            .margin_top(10)
            .set_all_label_area_size(0)
            .build_cartesian_2d(oldest_time..newest_time, 0u32..max_value)
            .expect("failed to build chart");

        chart
            .draw_series(
                AreaSeries::new(
                    self.data.iter().map(|x| (x.0, x.1)),
                    0u32,
                    self.color.mix(0.3),
                )
                .border_style(ShapeStyle::from(self.color).stroke_width(2)),
            )
            .expect("failed to draw chart data");
    }
}
