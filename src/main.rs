use iced::{
    Font,
    font::Family,
    widget::{
        Checkbox, checkbox,
        checkbox::Icon,
        column,
        text::{LineHeight, Shaping},
    },
};

fn main() -> iced::Result {
    iced::run( MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    DoNothing,
    Update4(bool),
    Update5(bool),
}

#[derive(Default)]
struct MyApp {
    checkbox4: bool,
    checkbox5: bool,
}

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::DoNothing => {}
            Message::Update4(b) => self.checkbox4 = b,
            Message::Update5(b) => self.checkbox5 = b,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            Checkbox::new(false)
              .label("Construct from struct"),
            checkbox(false)
              .label("Construct from function"),
            checkbox(false)
                .label("Enabled checkbox")
              .on_toggle(|_| Message::DoNothing),
            checkbox(self.checkbox4)
                .label("Functional checkbox")
              .on_toggle(|b| Message::Update4(b)),
            checkbox(self.checkbox5)
                .label("Shorter parameter")
              .on_toggle(Message::Update5),
            checkbox(false)
                .label("Larger box")
                .on_toggle(|_| Message::DoNothing)
                .size(30),
            checkbox(true)
                .label("Different icon")
                .on_toggle(|_| Message::DoNothing)
                .icon(Icon {
                    font: Font::DEFAULT,
                    code_point: '*',
                    size: None,
                    line_height: LineHeight::default(),
                    shaping: Shaping::default()
                }),
            checkbox(false)
                .label("Different font")
                .on_toggle(|_| Message::DoNothing)
                .font(Font {
                    family: Family::Fantasy,
                    ..Font::DEFAULT
                }),
            checkbox(false)
                .label("Larger text")
                .on_toggle(|_| Message::DoNothing)
                .text_size(24),
            checkbox(false)
                .label("Special character 😊")
                .on_toggle(|_| Message::DoNothing)
                .text_shaping(Shaping::Advanced),
            checkbox(false)
                .label("Space between box and text")
                .on_toggle(|_| Message::DoNothing)
                .spacing(30),
            checkbox(false)
                .label("Space between box and text")
                .on_toggle(|_| Message::DoNothing)
                .spacing(0),
        ]
        .into()
    }
}
