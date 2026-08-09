use iced_fluent_theme::{
    BrandVariants, Theme,
    font::{self}
};


// Type alias to save specifying the theme every time
pub type Element<'a, Message> = iced::Element<'a, Message, iced_fluent_theme::Theme>;

fn main() -> iced::Result {
    let settings = iced::Settings {
        fonts: font::load(),
        antialiasing: true,
        default_font: font::REGULAR,
        ..Default::default()
    };

    iced::application(MyApp::new, MyApp::update, MyApp::view)
        .settings(settings)
        .title(MyApp::title)
        .theme(MyApp::theme)
        .run()
}

struct MyApp {
}

#[derive(Clone, Debug)]
enum Message {
}

impl MyApp {
    fn new() -> Self {

        Self {
        }
    }

    fn update(&mut self, _message: Message) {
    }

    fn view(&self) -> Element<'_, Message> {
        "content".into()
    }

    fn title(&self) -> String {
        String::from("Iced Fluent Theme Gallery")
    }

    fn theme(&self) -> Theme {
        Theme::light(Some(BrandVariants::DEFAULT))
    }
}
