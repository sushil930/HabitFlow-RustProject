use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Alignment, Element, Length};

use crate::theme::{self, AppTheme};
use crate::Message;

pub fn view<'a>(title: &str, body: &str, theme: &'a AppTheme) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;

    let cancel = button(
        text("Cancel")
            .size(ty.size_body_md)
            .color(theme.colors.text_secondary),
    )
    .on_press(Message::CancelDialog)
    .padding([10, 16])
    .style(|_, status| theme::ghost_button(theme, status));

    let delete = button(
        text("Delete")
            .size(ty.size_body_md)
            .color(theme.colors.text_inverse),
    )
    .on_press(Message::ConfirmFactoryReset)
    .padding([10, 16])
    .style(|_, status| theme::destructive_button(theme, status));

    let backup_and_delete = button(
        text("Backup and delete")
            .size(ty.size_body_md)
            .color(theme.colors.text_inverse),
    )
    .on_press(Message::ConfirmFactoryResetWithBackup)
    .padding([10, 16])
    .style(|_, status| theme::primary_button(theme, status));

    let dialog = container(
        column![
            text(title.to_string())
                .size(ty.size_heading_lg)
                .color(theme.colors.text_primary),
            text(body.to_string())
                .size(ty.size_body_md)
                .color(theme.colors.text_secondary),
            row![horizontal_space(), cancel, delete, backup_and_delete]
                .spacing(sp.sm)
                .align_y(Alignment::Center),
        ]
        .spacing(sp.lg)
        .width(Length::Fill),
    )
    .padding(sp.xxl)
    .max_width(620)
    .style(|_| theme::dialog(theme));

    container(dialog)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_| theme::scrim())
        .into()
}
