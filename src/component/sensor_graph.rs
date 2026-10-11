use std::collections::VecDeque;

use chrono::{DateTime, Utc};
use iced::{Length::{self}, widget::{Space, row, text}};
use iced::{widget::column};
use plotters::prelude::ChartBuilder;
use plotters_iced2::{Chart, ChartWidget, DrawingBackend};

use crate::{Element, state::MainMessage};

const PLOT_SECONDS: usize = 3600; // 1h

pub type SensorSample = (DateTime<Utc>, u32);

pub struct SensorGraph {
    title: &'static str,
    data: VecDeque<SensorSample>
}

impl SensorGraph {

    pub fn new(title: &'static str) -> Self {
        Self {
            title,
            data: VecDeque::default()
        }
    }

    pub fn push_data(&mut self, data: SensorSample) {
        self.data.push_back(data);
    }

    fn rate_per_minute(&self) -> f64 {
        let sum: u32 = Iterator::sum(self.data.iter().map(|s| s.1));
        f64::from(sum) / (self.data.len() as f64)
    }

    fn last_rate_per_minute(&self) -> String {
        return self.data.front()
            .map(|d| d.1.to_string())
            .unwrap_or("-".into())
    }

    pub fn view(&self)-> Element<'_, MainMessage> {
        let title_text = text(self.title).width(Length::Fill);
        let rate_text = text(format!("{:.2}/min", self.rate_per_minute())).width(Length::Shrink);
        let header_row = row![title_text, rate_text];

        let spacing = Space::new().height(14);

        let last_activity_rate_text = text(format!("Last minute ({}) (122)", self.last_rate_per_minute()));

        let chart = ChartWidget::new(self);

        let container = column![
            header_row,
            spacing,
            chart,
            last_activity_rate_text
        ];

        container.into()
    }
}

impl Chart<MainMessage> for SensorGraph {
    type State = ();
    fn build_chart<DB:DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        use plotters::prelude::*;

                const PLOT_LINE_COLOR: RGBColor = RGBColor(0, 175, 255);

                // Acquire time range
                let newest_time = self
                    .data
                    .front()
                    .map(|s| s.0)
                    .unwrap_or(DateTime::from_timestamp(0, 0).unwrap());
                let oldest_time = newest_time - chrono::Duration::seconds(PLOT_SECONDS as i64);
                let mut chart = builder
                    .x_label_area_size(0)
                    .y_label_area_size(28)
                    .margin(20)
                    .build_cartesian_2d(oldest_time..newest_time, 0u32..100u32)
                    .expect("failed to build chart");

                chart
                    .draw_series(
                        AreaSeries::new(
                            self.data.iter().map(|x| (x.0, x.1)),
                            0u32,
                            PLOT_LINE_COLOR.mix(0.175),
                        )
                        .border_style(ShapeStyle::from(PLOT_LINE_COLOR).stroke_width(2)),
                    )
                    .expect("failed to draw chart data");
    }
}
