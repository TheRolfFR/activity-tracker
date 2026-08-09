use iced::{
    ContentFit,
    widget::{Image, column, image, text},
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
            Image::new("Images/icon.png").width(100).height(100),
            text("Construct from function"),
            image("Images/icon.png").width(100).height(100),
            text("Different content fit"),
            image("Images/activity-tracker.png")
                .content_fit(ContentFit::Contain)
                .width(500)
        ]
        .into()
    }
}
