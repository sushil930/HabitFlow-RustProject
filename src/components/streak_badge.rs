use iced::widget::{row, text};
use iced::{Alignment, Element, Font};

use crate::icons;
use crate::theme::AppTheme;
use crate::Message;

pub fn view<'a>(streak: u32, theme: &AppTheme) -> Element<'a, Message> {
    let ty = &theme.typography;
    let colors = &theme.colors;

    let (color, size) = if streak >= 30 {
        (colors.success_fg, ty.size_mono_lg)
    } else if streak >= 3 {
        (colors.text_secondary, ty.size_mono_md)
    } else {
        (colors.text_tertiary, ty.size_mono_md)
    };

    row![
        icons::fire(color, size),
        text(streak.to_string())
            .size(size)
            .font(Font::MONOSPACE)
            .color(color),
    ]
    .spacing(theme.spacing.xs)
    .align_y(Alignment::Center)
    .into()
}
