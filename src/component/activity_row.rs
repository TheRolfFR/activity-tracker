//! Activity Bar Component

use iced::{Background, Color, Length, border, padding};
use iced::widget::{text, container, Row, row, tooltip};
use iced_core::text::Alignment;

use crate::assets::fonts::SEMIBOLD;
use crate::{Element, state::MainMessage};

fn create_container<'a>(opacity: f32) -> Element<'a, MainMessage> {
    container("")
        .width(4)
        .height(Length::Fill)
        .style(move |_| container::Style {
            border: border::rounded(1),
            background: Some(Background::Color(Color::WHITE.scale_alpha(opacity))),
            ..Default::default()
        })
        .into()
}

pub fn draw_activity_row<'a>(hour_label: String, hour_activity: &[(f32, u32); 60])-> Element<'a, MainMessage> {

    let dots_row = Row::with_children(
        (0..60usize).map(|i| {
            let (opacity, value) = hour_activity[i];
            tooltip::Tooltip::new(
                create_container(opacity),
                text(value.to_string()).center(),
                tooltip::Position::Bottom
            ).into()
        })
    ).spacing(4).width(Length::Shrink);

    row![
        text(hour_label).font(SEMIBOLD).align_x(Alignment::Right).size(16).width(30),
        dots_row
    ].spacing(12).width(Length::Shrink).into()
}
