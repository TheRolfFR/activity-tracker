use iced::{
    ContentFit,
    widget::{Svg, column, svg, svg::Handle, text},
};

fn main() -> iced::Result {
    iced::run(MyApp::update, MyApp::view)
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
            Svg::from_path("Images/ferris.svg")
                .width(100)
                .height(100),
            text("Construct from function"),
            svg(Handle::from_path("Images/ferris.svg"))
                .width(100)
                .height(100),
            svg(Handle::from_path("Images/ferris.svg"))
                .width(500),
            text("Different content fit"),
            Svg::from_path("Images/ferris.svg")
                .content_fit(ContentFit::Cover)
                .width(100)
                .height(100),
        ]
        .into()
    }
}
