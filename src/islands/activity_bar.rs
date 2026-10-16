//! Activity Bar Island

use std::collections::BTreeMap;

use chrono::{Timelike, Utc};
use iced::{Alignment, Length, padding};
use iced::widget::{Column, container};
use sorted_vec::SortedVec;

use crate::Element;
use crate::component::activity_card::activity_card_style;
use crate::component::activity_row;
use crate::state::MainMessage;

const HOUR_MIN: u8 = 9; // 9h MIN
const HOUR_MAX: u8 = 18; // 18h MAX

mod color_shades {
    use std::iter::zip;

    const MAX_SHADES: usize = 6; // Maximum of shades of color

    #[derive(Debug, Default)]
    pub struct ColorShades([u32; MAX_SHADES]);

    impl ColorShades {

        pub fn compute_color_shades<'a, I>(sorted_input: I) -> ColorShades
        where
            I: Iterator<Item = &'a u32> {
            let collected: Vec<u32> = sorted_input.map(|rv| *rv).collect();
            let arr_len = collected.len();

            let mut shades = ColorShades::default();

            for (i, arr_val) in shades.0.iter_mut().enumerate() {
                let arr_position = arr_len * i / MAX_SHADES;
                *arr_val = collected[arr_position];
            }

            shades
        }

        pub fn serie_to_shades<'a, I>(&self, value_iter: I) -> [(f32, u32); 60]
        where
            I: Iterator<Item = &'a u32> {
            let mut result: [(f32, u32); 60] = [(0., 0);60];

            let mut value_res_zip = zip(value_iter, result.iter_mut());

            let mut shade_iter = self.0.iter().enumerate();
            let mut opt_shade = shade_iter.next();

            while let Some((value, result)) = value_res_zip.next() && let Some((index, threshold)) = opt_shade {
                if value <= threshold {
                    let shade_percent = ((index + 1) as f32) / (MAX_SHADES as f32);
                    *result = (shade_percent, *value);
                } else {
                    opt_shade = shade_iter.next();
                }
            }

            while let Some((value, result)) = value_res_zip.next() {
                *result = (1., *value);
            }

            result
        }
    }
}

#[derive(Debug, Default)]
pub struct ActivityBar {
    day_data: BTreeMap<u8, [u32; 60]>,
    value_data: SortedVec<u32>,
    shades: color_shades::ColorShades
}

impl ActivityBar {
    /// Create a new activity bar island container with default configuration
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_data(&mut self, date: chrono::DateTime<Utc>, activity: u32) {
        let hour = date.hour() as u8;
        let minute = date.minute() as usize;

        self.day_data.entry(hour).or_insert([0u32; 60])[minute] = activity;
        self.value_data.push(activity);
        self.shades = color_shades::ColorShades::compute_color_shades(self.value_data.iter());
    }

    pub fn view<'a>(&self) -> Element<'a, MainMessage> {
        let keys: Vec<&u8> = self.day_data.keys().collect();

        let hour_min_to_show = keys.iter().min().map(|h| if **h < HOUR_MIN {
            **h
        } else {
            HOUR_MIN
        }).unwrap_or(HOUR_MIN);

        let hour_max_to_show = keys.iter().max().map(|h| if **h > HOUR_MAX {
            **h
        } else {
            HOUR_MAX
        }).unwrap_or(HOUR_MAX);

        let rows = (hour_min_to_show..=hour_max_to_show).map(|h| {
            let serie = self.day_data.get(&h).unwrap_or(&[0u32; 60]);
            let serie_shades = self.shades.serie_to_shades(serie.iter());

            activity_row::draw_activity_row(format!("{h}h"), &serie_shades)
        });

        let hour_columns = Column::with_children(rows)
            .align_x(Alignment::Center)
            .spacing(3);

        container(
            container(hour_columns)
                .style(activity_card_style)
                .center(Length::Fill)
                .padding(padding::vertical(8))
        ).padding(padding::horizontal(14).top(14)).into()
    }
}
