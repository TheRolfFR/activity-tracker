use iced::{theme::Base, widget::{button, column, text}};

fn main() -> iced::Result {
    iced::application(MyApp::default, MyApp::update, MyApp::view)
        .theme(MyApp::theme)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    ToggleTheme,
}

#[derive(Default)]
struct MyApp {
    mode: iced::theme::Mode,
}

impl MyApp {
    fn update(&mut self, _message: Message) {
        match _message {
            Message::ToggleTheme => {
                self.mode = match self.mode {
                    iced::theme::Mode::Dark => iced::theme::Mode::Light,
                    iced::theme::Mode::Light => iced::theme::Mode::Dark,
                    _ => iced::theme::Mode::Dark,
                };
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        column![
            button("Toggle Theme").on_press(Message::ToggleTheme),
            text("Hello, world!"),
            text(format!("Current mode: {:?}", self.theme().mode())),
        ]
        .into()
    }

    fn theme(&self) -> iced::Theme {
        dbg!(&self.mode);
        iced::Theme::default(self.mode)
    }
}
