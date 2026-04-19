use iced::widget::{button, column, container, pick_list, row, text};
use iced::{Alignment, Element, Length};

use crate::icons;
use crate::theme::{self, AppTheme, ThemePreset};
use crate::{views, BannerMessage, Message};

pub fn view<'a>(
    banner: Option<BannerMessage>,
    theme: &'a AppTheme,
    selected_theme: ThemePreset,
) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;

    let appearance_section = container(
        column![
            text("Appearance")
                .size(ty.size_heading_lg)
                .color(theme.colors.text_primary),
            text("Choose from the built-in Iced theme collection.")
                .size(ty.size_body_md)
                .color(theme.colors.text_secondary),
            pick_list(
                &ThemePreset::ALL[..],
                Some(selected_theme),
                Message::SelectTheme,
            )
            .placeholder("Select theme")
            .padding([8, 12]),
        ]
        .spacing(sp.md)
        .width(Length::Fill),
    )
    .padding(sp.xl)
    .style(|_| theme::section_card(theme));

    let mut content = column![
        text("Settings")
            .size(ty.size_display)
            .color(theme.colors.text_primary),
        views::divider(theme),
    ]
    .spacing(sp.lg);

    if let Some(banner) = banner {
        content = content.push(views::banner(banner, theme));
    }

    let data_section = container(
        column![
            text("Data")
                .size(ty.size_heading_lg)
                .color(theme.colors.text_primary),
            text("Export your habits and full history as a JSON backup.")
                .size(ty.size_body_md)
                .color(theme.colors.text_secondary),
            button(
                row![
                    icons::download(theme.colors.text_inverse, 16.0),
                    text("Export data")
                        .size(ty.size_body_md)
                        .color(theme.colors.text_inverse),
                ]
                .spacing(sp.sm)
                .align_y(Alignment::Center),
            )
            .on_press(Message::RequestExport)
            .padding([10, 16])
            .style(|_, status| theme::primary_button(theme, status)),
            text("Import a previous JSON backup and replace the current local data.")
                .size(ty.size_body_md)
                .color(theme.colors.text_secondary),
            button(
                row![
                    icons::upload(theme.colors.text_primary, 16.0),
                    text("Import data")
                        .size(ty.size_body_md)
                        .color(theme.colors.text_primary),
                ]
                .spacing(sp.sm)
                .align_y(Alignment::Center),
            )
            .on_press(Message::RequestImport)
            .padding([10, 16])
            .style(|_, status| theme::secondary_button(theme, status)),
            text("Need a clean slate? Factory reset removes every habit and all history.")
                .size(ty.size_body_md)
                .color(theme.colors.text_secondary),
            button(
                row![
                    icons::trash(theme.colors.text_inverse, 16.0),
                    text("Factory reset all data")
                        .size(ty.size_body_md)
                        .color(theme.colors.text_inverse),
                ]
                .spacing(sp.sm)
                .align_y(Alignment::Center),
            )
            .on_press(Message::RequestFactoryReset)
            .padding([10, 16])
            .style(|_, status| theme::destructive_button(theme, status)),
        ]
        .spacing(sp.md)
        .width(Length::Fill),
    )
    .padding(sp.xl)
    .style(|_| theme::section_card(theme));

    let about_section = container(
        column![
            text("About")
                .size(ty.size_heading_lg)
                .color(theme.colors.text_primary),
            text("Minimal Habit Tracker v0.1.0")
                .size(ty.size_body_md)
                .color(theme.colors.text_secondary),
            text("Offline-first. No accounts. No cloud. Just habits.")
                .size(ty.size_body_sm)
                .color(theme.colors.text_tertiary),
        ]
        .spacing(sp.md)
        .width(Length::Fill),
    )
    .padding(sp.xl)
    .style(|_| theme::section_card(theme));

    content = content.push(
        column![appearance_section, data_section, about_section]
            .spacing(sp.xl)
            .align_x(Alignment::Start),
    );

    views::content_shell(content, theme)
}
