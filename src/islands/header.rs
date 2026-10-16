use iced::{Alignment, Length, padding};
use iced::widget::{row, text};

use crate::Element;
use crate::assets::fonts::{BOLD, REGULAR};
use crate::state::MainMessage;

pub fn main_header<'a>(today_date: impl text::IntoFragment<'a>, hours_worked: impl text::IntoFragment<'a>) -> Element<'a, MainMessage> {
    let today_date = text(today_date)
        .font(BOLD).size(24).width(Length::Fill);
    let hours_worked = text(hours_worked)
        .font(REGULAR).size(24).align_x(Alignment::End).width(Length::Fill);

    row![today_date, hours_worked].padding(padding::horizontal(14)).width(Length::Fill).into()
}
