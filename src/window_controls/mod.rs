use iced::{
    Background, Color, Length, alignment::{Horizontal, Vertical}, widget::{
        Container, button, row, svg::{self, Svg, Handle},
    }, window,
};
use iced_fluent_theme::button::medium;
use std::sync::LazyLock;

use crate::state::MainMessage;
use crate::Element;
use crate::handle;

handle!(MINIMIZE_ICON, "win/minimize.svg");
handle!(MAXIMIZE_ICON, "win/maximize.svg");
handle!(MAXIMIZE_RESTORE_ICON, "win/maximize_restore.svg");
handle!(CLOSE_ICON, "win/close.svg");

#[derive(Clone)]
enum ControlType {
    Minimize,
    Maximize(bool),
    Close,
}

fn transparent_button<'a>(ctype: ControlType, message: MainMessage) -> Element<'a, MainMessage> {
    let icon = match &ctype {
        ControlType::Minimize => MINIMIZE_ICON.clone(),
        ControlType::Maximize(restore) => {
            if *restore {
                MAXIMIZE_RESTORE_ICON.clone()
            } else {
                MAXIMIZE_ICON.clone()
            }
        }
        ControlType::Close => CLOSE_ICON.clone(),
    };

    let icon_color = |is_hovered| Color::WHITE.scale_alpha(if is_hovered { 1.0 } else { 0.8956 });

    let icon: Element<'a, MainMessage> = match &ctype {
        // fix for minimize icon not rendered because 1x10, empty button with same white background fill as svg color
        ControlType::Minimize => {
            button("").width(10).height(1).style(move |_, status| button::Style {
                background: Some(Background::Color(icon_color(status == button::Status::Hovered))),
                ..Default::default()
            }).into()
        },
        _ => {
            Svg::new(icon).width(10).style(move |_, status| {
                svg::Style {
                    color: Some(icon_color(status == svg::Status::Hovered)),
                }
            }).into()
        }
    };

    let container = Container::new(icon)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .height(Length::Fill)
        .width(Length::Fill);

    medium(container)
        .style(move |_, status| button::Style {
            background: match (ctype.clone(), status) {
                (ControlType::Close, button::Status::Hovered) => {
                    Some(iced::Background::Color(Color::from_rgb8(196, 43, 28)))
                }
                (_, button::Status::Hovered) => {
                    Some(iced::Background::Color(Color::WHITE.scale_alpha(0.20)))
                }
                (_, _) => None,
            },
            ..Default::default()
        })
        .width(46)
        .height(32)
        .padding(0)
        .on_press(message)
        .into()
}

pub fn window_controls<'a>(
    show_minimze: bool,
    show_maximize: bool,
    is_maximized: bool,
    id: window::Id,
) -> Element<'a, MainMessage> {
    row![
        show_minimze.then_some(transparent_button(
            ControlType::Minimize,
            MainMessage::WindowMinimize(id)
        )),
        show_maximize.then_some(transparent_button(
            ControlType::Maximize(is_maximized),
            MainMessage::WindowMaximize(id)
        )),
        transparent_button(ControlType::Close, MainMessage::WindowClose(id))
    ]
    .into()
}
