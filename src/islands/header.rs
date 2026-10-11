use iced::{Alignment, Font, Length, font::{Family, Weight}, padding};
use iced::widget::{row, text};

use crate::Element;
use crate::state::MainMessage;

pub fn main_header<'a>(today_date: impl text::IntoFragment<'a>, hours_worked: impl text::IntoFragment<'a>) -> Element<'a, MainMessage> {
    let bold = Font {
        family: Family::Name("Segoe UI"),
        weight: Weight::Bold,
        ..Default::default()
    };
    let regular = Font {
        family: Family::Name("Segoe UI"),
        weight: Weight::Normal,
        ..Default::default()
    };
    let today_date = text(today_date)
        .font(bold).size(24).width(Length::Fill);
    let hours_worked = text(hours_worked)
        .font(regular).size(24).align_x(Alignment::End).width(Length::Fill);

    row![today_date, hours_worked].padding(padding::horizontal(14)).width(Length::Fill).into()
}
