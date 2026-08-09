use iced::{
    widget::{slider, column, progress_bar, text, ProgressBar},
};

fn main() -> iced::Result {
    iced::run(MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    UpdateValue(u32),
}

struct MyApp {
    value: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            value: 50
        }
    }
}

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::UpdateValue(v) => self.value = v,
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        column![
            text("Construct from struct"),
            ProgressBar::new(0.0..=100.0, self.value as f32 + 20.),
            text("Construct from function"),
            progress_bar(0.0..=100.0, self.value as f32),
            text(format!("Functional slider {}", self.value)),
            slider(0..=100, self.value, Message::UpdateValue),
        ]
        .into()
    }
}
