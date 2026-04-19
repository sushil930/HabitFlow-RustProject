use iced::widget::{button, container, horizontal_space, row, text, text_input};
use iced::{Alignment, Element, Length};

use crate::theme::{self, AppTheme};
use crate::Message;

pub fn view<'a, F>(
    input_id: text_input::Id,
    placeholder: &'a str,
    value: &'a str,
    submit_label: &'a str,
    on_input: F,
    on_submit: Message,
    on_cancel: Message,
    theme: &'a AppTheme,
) -> Element<'a, Message>
where
    F: Fn(String) -> Message + 'a,
{
    let sp = &theme.spacing;
    let ty = &theme.typography;

    let field = text_input(placeholder, value)
        .id(input_id)
        .on_input(on_input)
        .on_submit(on_submit.clone())
        .padding([sp.md, sp.lg])
        .size(ty.size_body_lg)
        .style(|_, status| theme::text_field(theme, status));

    let submit = button(
        text(submit_label)
            .size(ty.size_body_sm)
            .color(theme.colors.text_inverse),
    )
    .on_press(on_submit)
    .padding([8, 12])
    .style(|_, status| theme::primary_button(theme, status));

    let cancel = button(
        text("Cancel")
            .size(ty.size_body_sm)
            .color(theme.colors.text_secondary),
    )
    .on_press(on_cancel)
    .padding([8, 12])
    .style(|_, status| theme::ghost_button(theme, status));

    container(
        row![field, horizontal_space(), submit, cancel]
            .align_y(Alignment::Center)
            .spacing(sp.sm)
            .width(Length::Fill),
    )
    .padding(sp.sm)
    .width(Length::Fill)
    .style(|_| theme::section_card(theme))
    .into()
}
