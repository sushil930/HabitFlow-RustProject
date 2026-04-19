use chrono::NaiveDate;
use iced::widget::{button, column, container, scrollable, text};
use iced::{Alignment, Element, Length};

use crate::components::{habit_card, habit_input};
use crate::models::HabitWithStats;
use crate::theme::AppTheme;
use crate::{views, BannerMessage, Message};

pub fn view<'a>(
    habits: &'a [HabitWithStats],
    today: NaiveDate,
    editing_habit_id: Option<&str>,
    rename_input_value: &'a str,
    banner: Option<BannerMessage>,
    checkins_blocked: bool,
    theme: &'a AppTheme,
) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;

    let mut content = column![
        header("Today", &today.format("%B %-d, %Y").to_string(), theme),
        views::divider(theme)
    ]
    .spacing(sp.lg);

    if let Some(banner) = banner {
        content = content.push(views::banner(banner, theme));
    }

    if habits.is_empty() {
        let empty_state = column![
            text("No habits yet")
                .size(ty.size_heading_lg)
                .color(theme.colors.text_primary),
            text("Add your first habit to start tracking today.")
                .size(ty.size_body_md)
                .color(theme.colors.text_secondary),
            button(
                text("Add habit")
                    .size(ty.size_body_md)
                    .color(theme.colors.text_inverse),
            )
            .on_press(Message::ShowAddInput)
            .padding([10, 16])
            .style(|_, status| crate::theme::primary_button(theme, status)),
        ]
        .spacing(sp.md)
        .align_x(Alignment::Center)
        .width(Length::Fill);

        content = content.push(container(empty_state).padding([72, 0]));
        return views::content_shell(content, theme);
    }

    let mut list = column![].spacing(sp.sm);

    for habit in habits {
        if editing_habit_id == Some(habit.habit.id.as_str()) {
            list = list.push(habit_input::view(
                format!("rename-{}", habit.habit.id).into(),
                "Rename habit",
                rename_input_value,
                "Save",
                Message::RenameInputChanged,
                Message::SubmitRename,
                Message::CancelRename,
                theme,
            ));
        } else {
            list = list.push(habit_card::view(
                habit,
                today,
                checkins_blocked,
                false,
                theme,
            ));
        }
    }

    content = content.push(scrollable(list).direction(crate::views::hidden_scrollbar()).height(Length::Fill));

    views::content_shell(content, theme)
}

fn header<'a>(title: &str, date: &str, theme: &'a AppTheme) -> Element<'a, Message> {
    iced::widget::row![
        text(title.to_string())
            .size(theme.typography.size_display)
            .color(theme.colors.text_primary),
        iced::widget::horizontal_space(),
        text(date.to_string())
            .size(theme.typography.size_body_md)
            .color(theme.colors.text_secondary),
    ]
    .align_y(Alignment::End)
    .into()
}