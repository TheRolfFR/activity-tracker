use std::str::FromStr;

use crate::Element;
use crate::state::MainMessage;
use iced::{Radians, border};
use iced::gradient::Linear;
use iced::{Border, Color, Length, alignment::Vertical, border::Radius};
use iced::font::{Family, Font, Weight};
use iced::widget::{Column, column, container, row, space};
use iced::widget::{button::{self, Button}, text, image};
use iced_core::{image::FilterMethod, text::LineHeight};

use crate::component::labelled_stat;

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

    let button_background_solid = container(button_text)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb8(0x2d, 0x2d, 0x2d))),
            border: border::rounded(2),
            ..Default::default()
        }).height(Length::Fill)
        .width(Length::Fill);

    let linear = Linear::new(Radians::PI)
        .add_stop(0., Color::BLACK.scale_alpha(0.09))
        .add_stop(0.1, Color::WHITE.scale_alpha(0.1))
        .add_stop(1., Color::WHITE.scale_alpha(0.1));

    let button_background_gradient = container(button_background_solid)
        .style(move |_| container::Style {
            background: Some(iced::Background::Gradient(iced::Gradient::Linear(linear))),
            border: border::rounded(3),
            ..Default::default()
        })
        .padding(1)
        .width(Length::Fill)
        .height(21);

    column![
        labelled_stat::labbeled_stat("Week".into(), "XXhXX".into()),

        Button::new(button_background_gradient).width(Length::Fill).style(|_, _| button::Style {
            background: Some(iced::Background::Color(Color::from_str("#2d2d2d").unwrap())),
            text_color: Color::WHITE,
            border: Border {
                width: 1.0f32,
                color: Color::from_rgba8(255, 255, 255, 0.1),
                radius: Radius::new(3),
            },
            ..Default::default()
        }).height(21).padding(0)
    ].spacing(6).into()
}

pub (crate) fn left_bar<'a>() -> Element<'a, MainMessage> {
    column![
        top_logo(),
        space::Space::new().height(20),

        Column::with_children(
            (1..=5).map(|i| {
                labelled_stat::labbeled_stat(format!("Day {}", i), "XhXX".into()).into()
            })
        ).spacing(6),

        space::vertical(),

        week_stats(),
    ].padding(12).into()
}
