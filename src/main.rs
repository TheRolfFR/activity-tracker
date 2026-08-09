use iced::{Color, border::Radius, widget::{Rule, column, rule::{self, FillMode, horizontal as horizontal_rule, vertical as vertical_rule}, text}};

fn main() -> iced::Result {
    iced::run( MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct MyApp;

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        column![
            text("Construct from struct"),
            text("Construct from function"),
            horizontal_rule(0),
            text("Different space"),
            horizontal_rule(5).style(|_| rule::Style {
                radius: Radius::new(5),
                snap: true,
                fill_mode: FillMode::AsymmetricPadding(50, 20),
                color: Color::from_rgb8(255, 0, 0)
            }),
            text("Vertical rule"),
            vertical_rule(100),
        ]
        .into()
    }
}
