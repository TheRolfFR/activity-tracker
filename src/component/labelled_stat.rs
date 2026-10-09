//! LabelledStat component with day of week and labeled island
//!
//! This component renders:
//! - Day of week (light font, centered)
//! - An island container (rounded rectangle with background)
//! - Two text labels inside the island:
//!   * Medium weight text
//!   * Bold text
//! All texts styled appropriately

use iced::{Alignment, Color, Font, Length::Fill, alignment::Vertical, border, font::{Family, Weight}, widget::{container, text}};
use iced::widget::column;

use crate::{Element, state::MainMessage};

pub fn labbeled_stat<'a>(dow: String, hour: String) -> Element<'a, MainMessage> {
    let semibold = Font {
        family: Family::Name("Segoe UI"),
        weight: Weight::Semibold,
        ..Default::default()
    };
    let bold = Font {
        family: Family::Name("Segoe UI"),
        weight: Weight::Bold,
        ..Default::default()
    };

    let dow_text = text(dow)
        .size(12.0)
        .line_height(1.2)
        .font(semibold)
        .color(Color::from_rgb8(0xc6, 0xc6, 0xc6)) // Light gray/light font
        .align_x(Alignment::Center)
        .align_y(Vertical::Center)
        .height(15)
        .width(Fill);

    let hour_text = text(hour)
        .size(24.)
        .line_height(1.2)
        .font(bold)
        .color(Color::WHITE)
        .align_x(Alignment::Center)
        .align_y(Vertical::Center)
        .height(Fill)
        .width(Fill);

    let grey_container = container(hour_text)
        .width(Fill)
        .height(37)
        .max_height(37)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::WHITE.scale_alpha(0.0720))),
            border: border::rounded(4),
            ..Default::default()
        });

    column![
        dow_text,
        grey_container
    ].spacing(6).into()
}
