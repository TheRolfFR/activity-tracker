use iced::{Background, Color, border, widget::container};
use iced_fluent_theme::Theme;

pub fn activity_card_style(theme: &Theme) -> container::Style {
    container::Style {
        border: border::rounded(7),
        background: Background::Color(Color::from_rgb8(0x00, 0x00, 0x00).scale_alpha(0.04)).into(),
        shadow: theme.tokens().shadow2,
        snap: true,
        ..Default::default()
    }
}
