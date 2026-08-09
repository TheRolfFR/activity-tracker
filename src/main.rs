use std::collections::BTreeMap;

use iced::widget::{
    button, center, center_x, column, container, operation, scrollable, space, text, text_input,
};
use iced::window;
use iced::{Center, Fill, Function, Subscription, Task, Vector};

use iced_fluent_theme::{
    BrandVariants, Theme,
    font::{self}
};

mod state;
mod tray;

use crate::tray::{tray_icon, tray_subscription};
use crate::state::MainMessage;

const ICON: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/Images/icon.png"));

// Type alias to save specifying the theme every time
pub type Element<'a, Message> = iced::Element<'a, Message, iced_fluent_theme::Theme>;

fn main() -> iced::Result {
    let settings = iced::Settings {
        fonts: font::load(),
        antialiasing: true,
        default_font: font::REGULAR,
        ..Default::default()
    };

            let _tray_icon = tray_icon("Activity tracker", ICON);

    iced::daemon(MyApp::new, MyApp::update, MyApp::view)
        .subscription(|state| Subscription::batch([
            state.subscription(),
            tray_subscription(),
        ]))
        .settings(settings)
        .title(MyApp::title)
        .theme(MyApp::theme)
        .scale_factor(MyApp::scale_factor)
        .run()
}

struct MyApp {
    windows: BTreeMap<window::Id, Window>,
}

#[derive(Debug)]
struct Window {
    title: String,
    scale_input: String,
    current_scale: f32,
}

impl MyApp {
    fn new() -> (Self, Task<MainMessage>) {
        let (_, open) = window::open(window::Settings::default());

        (
            Self {
                windows: BTreeMap::new(),
            },
            open.map(MainMessage::WindowOpened),
        )
    }

    fn update(&mut self, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::OpenWindow => {
                let Some(last_window) = self.windows.keys().last() else {
                    return Task::none();
                };

                window::position(*last_window)
                    .then(|last_position| {
                        let position =
                            last_position.map_or(window::Position::Default, |last_position| {
                                window::Position::Specific(last_position + Vector::new(20.0, 20.0))
                            });

                        let (_, open) = window::open(window::Settings {
                            position,
                            ..window::Settings::default()
                        });

                        open
                    })
                    .map(MainMessage::WindowOpened)
            }
            MainMessage::WindowOpened(id) => {
                let window = Window::new(self.windows.len() + 1);
                let focus_input = operation::focus(format!("input-{id}"));

                self.windows.insert(id, window);

                focus_input
            }
            MainMessage::WindowClosed(id) => {
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
            MainMessage::ScaleInputChanged(id, scale) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.scale_input = scale;
                }

                Task::none()
            }
            MainMessage::ScaleChanged(id, scale) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.current_scale = scale
                        .parse()
                        .unwrap_or(window.current_scale)
                        .clamp(0.5, 5.0);
                }

                Task::none()
            }
            MainMessage::TitleChanged(id, title) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.title = title;
                }

                Task::none()
            },
            MainMessage::TrayEvent(name) => {
                match name.as_str() {
                    "quit" => { dbg!("quit"); },
                    "week_data" => { dbg!("week_data"); },
                    "hide" => { dbg!("hide"); },
                    _ => {}
                };
                Task::none()
            }
        }
    }

    fn view(&self, window_id: window::Id) -> Element<'_, MainMessage> {
        if let Some(window) = self.windows.get(&window_id) {
            center(window.view(window_id)).into()
        } else {
            space().into()
        }
    }


    fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|window| window.title.clone())
            .unwrap_or_default()
    }

    fn theme(&self, _window: window::Id) -> Theme {
        Theme::light(Some(BrandVariants::DEFAULT))
    }

    fn scale_factor(&self, window: window::Id) -> f32 {
        self.windows
            .get(&window)
            .map(|window| window.current_scale)
            .unwrap_or(1.0)
    }

    fn subscription(&self) -> Subscription<MainMessage> {
        window::close_events().map(MainMessage::WindowClosed)
    }
}

impl Window {
    fn new(count: usize) -> Self {
        Self {
            title: format!("Window_{count}"),
            scale_input: "1.0".to_string(),
            current_scale: 1.0,
        }
    }

    fn view(&self, id: window::Id) -> Element<'_, MainMessage> {
        let scale_input = column![
            text("Window scale factor:"),
            text_input("Window Scale", &self.scale_input)
                .on_input(MainMessage::ScaleInputChanged.with(id))
                .on_submit(MainMessage::ScaleChanged(id, self.scale_input.to_string()))
        ];

        let title_input = column![
            text("Window title:"),
            text_input("Window Title", &self.title)
                .on_input(MainMessage::TitleChanged.with(id))
                .id(format!("input-{id}"))
        ];

        let new_window_button = button(text("New Window")).on_press(MainMessage::OpenWindow);

        let content = column![scale_input, title_input, new_window_button]
            .spacing(50)
            .width(Fill)
            .align_x(Center)
            .width(200);

        container(scrollable(center_x(content))).padding(10).into()
    }
}
