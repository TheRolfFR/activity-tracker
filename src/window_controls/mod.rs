
use iced::{Color, widget::{button, row, svg::{Handle, Svg}}, window};
use iced_fluent_theme::button::medium;
use std::sync::LazyLock;

use crate::Element;
use crate::state::MainMessage;

macro_rules! icon {
    ($name:ident, $svg:literal) => {
        static $name: LazyLock<Handle> = LazyLock::new(|| {
            Handle::from_memory(include_bytes!(concat!($svg)))
        });
    };
}

icon!(MINIMIZE_ICON, "win/minimize.svg");
icon!(MAXIMIZE_ICON, "win/maximize.svg");
icon!(MAXIMIZE_RESTORE_ICON, "win/maximize_restore.svg");
icon!(CLOSE_ICON, "win/close.svg");

fn transparent_button<'a>(icon: Handle, message: MainMessage) -> Element<'a, MainMessage>
{
    medium(Svg::new(icon))
        .style(|theme, status| {
            button::Style {
                background: None,
                text_color: Color::WHITE,
                ..Default::default()
            }
        })
        .on_press(message).into()
}

pub fn window_controls<'a>(
    show_minimze: bool,
    show_maximize: bool,
    is_maximized: bool,
    id: window::Id
) -> Element<'a, MainMessage>
{
    row![
        show_minimze.then_some(
            transparent_button(MINIMIZE_ICON.clone(), MainMessage::WindowMinimize(id))
        ),
        show_maximize.then_some(transparent_button(
            if is_maximized {
                MAXIMIZE_RESTORE_ICON.clone()
            } else {
                MAXIMIZE_ICON.clone()
            }, MainMessage::WindowMaximize(id))
        ),
        transparent_button(CLOSE_ICON.clone(), MainMessage::WindowClose(id))
    ].into()
}
