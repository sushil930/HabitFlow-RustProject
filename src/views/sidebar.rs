use iced::widget::{button, column, container, row, text, vertical_space};
use iced::{Alignment, Element, Length};

use crate::icons;
use crate::theme::{self, AppTheme};
use crate::{Message, View};

pub fn view<'a>(current_view: View, theme: &'a AppTheme) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;

    let brand = column![
        text("Minimal Habit Tracker")
            .size(ty.size_heading_md)
            .color(theme.colors.text_primary),
        text("Offline-first daily habits")
            .size(ty.size_caption)
            .color(theme.colors.text_secondary),
    ]
    .spacing(sp.xs);

    let nav = column![
        nav_button("Today", View::Today, current_view, theme),
        nav_button("Weekly", View::Weekly, current_view, theme),
        nav_button("Stats", View::Stats, current_view, theme),
        nav_button("Archived", View::Archived, current_view, theme),
        nav_button("Settings", View::Settings, current_view, theme),
    ]
    .spacing(sp.xs);

    let add_button = button(
        row![
            icons::plus(theme.colors.text_inverse, 16.0),
            text("Add habit")
                .size(ty.size_body_md)
                .color(theme.colors.text_inverse),
        ]
        .spacing(sp.sm)
        .align_y(Alignment::Center),
    )
    .on_press(Message::ShowAddInput)
    .width(Length::Fill)
    .padding([10, 16])
    .style(|_, status| theme::primary_button(theme, status));

    container(
        column![brand, nav, vertical_space(), add_button]
            .spacing(sp.xxl)
            .padding([sp.xxxl, sp.lg])
            .height(Length::Fill)
            .align_x(Alignment::Start),
    )
    .width(220)
    .height(Length::Fill)
    .style(|_| theme::sidebar_container(theme))
    .into()
}

fn nav_button<'a>(
    label: &'a str,
    target: View,
    current_view: View,
    theme: &'a AppTheme,
) -> Element<'a, Message> {
    let active = target == current_view;
    let icon_color = if active {
        theme.colors.text_primary
    } else {
        theme.colors.text_secondary
    };

    let icon = match target {
        View::Today => icons::calendar(icon_color, 16.0),
        View::Weekly => icons::calendar_days(icon_color, 16.0),
        View::Stats => icons::bar_chart(icon_color, 16.0),
        View::Archived => icons::archive(icon_color, 16.0),
        View::Settings => icons::settings(icon_color, 16.0),
    };

    button(
        row![
            icon,
            text(label.to_string())
                .size(theme.typography.size_body_md)
                .color(icon_color),
        ]
        .spacing(theme.spacing.sm)
        .align_y(Alignment::Center),
    )
    .on_press(Message::NavigateTo(target))
    .width(Length::Fill)
    .padding([10, 14])
    .style(move |_, status| theme::nav_button(theme, active, status))
    .into()
}
