use std::default::Default;

use iced::{
    Color, Degrees, Gradient, Length, Radians, alignment::{Horizontal, Vertical}, gradient::{ColorStop, Linear}, widget::{Container, column, container},
};

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
            Container::new("Construct from struct"),
            container("Construct from function"),
            container("With padding").padding(20),
            container("Different alignment")
                .width(Length::Fill)
                .align_x(Horizontal::Center),
            container("Different alignment for vertical axis")
                .height(Length::Fill)
                .align_y(Vertical::Center),
            container("Center")
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill).style(|_| {
                    container::Style {
                        background: Some(Gradient::Linear(Linear::new(Degrees(-35.)).add_stops(vec![
                            ColorStop { offset: 0., color: Color::from_rgb8(0xff, 0xbc, 0x00) },
                            ColorStop { offset: 1., color: Color::from_rgb8(0xff, 0x00, 0x58) },
                        ])).scale_alpha(1.).into()),
                        ..Default::default()
                    }
                }),
        ]
        .into()
    }
}
