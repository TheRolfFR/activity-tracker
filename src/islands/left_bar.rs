use std::str::FromStr;

use crate::Element;
use crate::state::MainMessage;
use iced::{{alignment::Vertical}, Length, Color, Border, border::Radius};
use iced::font::{Family, Font, Weight};
use iced::widget::{column, Column, row, space};
use iced::widget::{button::{self, Button}, text, image};
use iced_core::{image::FilterMethod, text::LineHeight};

fn top_logo<'a>() -> Element<'a, MainMessage> {
    let image_icon = image("Images/icon.png").width(24).filter_method(FilterMethod::Linear);

    let font = Font {
        family: Family::Name("Segoe UI"),
        weight: Weight::Semibold,
        ..Default::default()
    };
    let title = text("Activity\nTracker").size(14).color(Color::WHITE).font(font);

    row![
        image_icon,
        title,
    ].spacing(10).align_y(Vertical::Center).into()
}

fn week_stats<'a>() -> Element<'a, MainMessage> {
    let font = Font {
        family: Family::Name("Segoe UI"),
        weight: Weight::Normal,
        ..Default::default()
    };
    let button_text = text("Week stats")
        .size(12).color(Color::WHITE).font(font)
        .line_height(LineHeight::Absolute(21.into()))
        .center()
        .width(Length::Fill);

    column![
        text("week"),
        Button::new(button_text).width(Length::Fill).style(|_, _| button::Style {
            background: Some(iced::Background::Color(Color::from_str("#2d2d2d").unwrap())),
            text_color: Color::WHITE,
            border: Border {
                width: 1.0f32,
                color: Color::from_rgba8(255, 255, 255, 0.1),
                radius: Radius::new(3),
            },
            ..Default::default()
        }).height(21).padding(0)
    ].into()
}

pub (crate) fn left_bar<'a>() -> Element<'a, MainMessage> {
    column![
        top_logo(),
        space::Space::new().height(20),

        Column::with_children(
            (0..7).map(|i| {
                text(format!("Day {}", i)).into()
            })
        ),
        space::vertical(),

        week_stats(),
    ].padding(12).into()
}
