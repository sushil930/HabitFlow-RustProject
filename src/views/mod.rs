pub mod archived;
pub mod settings;
pub mod sidebar;
pub mod stats;
pub mod today;
pub mod weekly;

use iced::widget::{button, container, horizontal_space, row, text};
use iced::{Alignment, Element, Length, Theme};

use crate::theme::{self, AppTheme, BannerKind};
use crate::{BannerMessage, Message};

pub fn hidden_scrollbar() -> iced::widget::scrollable::Direction {
    iced::widget::scrollable::Direction::Vertical(
        iced::widget::scrollable::Scrollbar::new()
            .width(0)
            .margin(0)
            .scroller_width(0),
    )
}

pub fn content_shell<'a>(
    content: impl Into<Element<'a, Message>>,
    theme: &'a AppTheme,
) -> Element<'a, Message> {
    let sp = &theme.spacing;

    container(
        container(content)
            .padding([sp.xxxxl, sp.xxxl + sp.sm]),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .style(|_| theme::page_container(theme))
    .into()
}

pub fn divider<'a>(theme: &'a AppTheme) -> Element<'a, Message> {
    container("")
        .height(1)
        .width(Length::Fill)
        .style(|_| theme::divider(theme))
        .into()
}

pub fn banner<'a>(banner: BannerMessage, theme: &'a AppTheme) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let colors = &theme.colors;
    let kind = banner.kind;
    let text_color = match kind {
        BannerKind::Info => colors.text_primary,
        BannerKind::Success => colors.success_fg,
        BannerKind::Warning => colors.warning_fg,
        BannerKind::Error => colors.error_fg,
    };

    let dismiss: Element<'a, Message> = if banner.dismissible {
        button(
            text("Dismiss")
                .size(theme.typography.size_caption)
                .color(colors.text_secondary),
        )
        .on_press(Message::DismissNotice)
        .padding([4, 8])
        .style(|_: &Theme, status| theme::ghost_button(theme, status))
        .into()
    } else {
        horizontal_space().into()
    };

    container(
        row![
            text(banner.text)
                .size(theme.typography.size_body_sm)
                .color(text_color),
            horizontal_space(),
            dismiss,
        ]
        .align_y(Alignment::Center)
        .spacing(sp.sm),
    )
    .padding([sp.md, sp.lg])
    .style(move |_| theme::banner(theme, kind))
    .into()
}
