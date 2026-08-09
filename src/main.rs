use iced::widget::{button, row, text, Row};

struct Counter {
    value: u8,
}

impl Default for Counter {
  fn default() -> Self {
      Self { value: 255 }
  }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value = self.value.checked_add(1).unwrap_or(self.value);
            }
            Message::Decrement => {
                self.value = self.value.checked_sub(1).unwrap_or(self.value);
            }
        }
    }
}

#[test]
fn it_counts_properly() {
    let mut counter = Counter { value: 0 };

    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);

    assert_eq!(counter.value, 1);
}

impl Counter {
    fn view(&self) -> Row<Message> {
        row![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
        ]
    }
}

pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
