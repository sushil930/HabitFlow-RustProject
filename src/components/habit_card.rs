use chrono::NaiveDate;

use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Alignment, Element, Length};

use crate::icons;
use crate::lucide::icon_by_name;
use crate::components::streak_badge;
use crate::models::HabitWithStats;
use crate::theme::{self, AppTheme};
use crate::Message;

pub fn view<'a>(
    habit: &'a HabitWithStats,
    today: NaiveDate,
    checkins_blocked: bool,
    archived_view: bool,
    theme: &'a AppTheme,
) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;
    let colors = &theme.colors;
    let habit_id = habit.habit.id.clone();

    let checkbox_content: Element<'a, Message> = if habit.completed_today {
        icons::check(colors.success_fg, 18.0)
    } else {
        text("○").size(18).color(colors.border_default).into()
    };

    let checkbox = button(checkbox_content)
    .on_press_maybe(
        (!checkins_blocked && !archived_view).then_some(Message::ToggleHabit(habit_id.clone())),
    )
    .padding(2)
    .style(|_, status| theme::ghost_button(theme, status));

    let title_color = if habit.completed_today {
        colors.text_secondary
    } else {
        colors.text_primary
    };

    let icon_color = if habit.completed_today {
        colors.success_fg
    } else {
        colors.text_secondary
    };

    let title = row![
        icon_by_name(&habit.habit.icon, icon_color, 18.0),
        text(habit.habit.name.clone())
            .size(ty.size_body_lg)
            .color(title_color),
    ]
    .spacing(sp.sm)
    .align_y(Alignment::Center);

    let subtitle = text(format!("{} · {}", habit.habit.category, subtitle(habit, today, archived_view)))
        .size(ty.size_caption)
        .color(subtitle_color(habit, today, archived_view, theme));

    let mut actions = row![].spacing(sp.xs).align_y(Alignment::Center);

    if archived_view {
        actions = actions.push(
            button(
                row![
                    icons::unarchive(colors.text_secondary, 12.0),
                    text("Unarchive".to_string())
                        .size(ty.size_caption)
                        .color(colors.text_secondary),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .on_press(Message::UnarchiveHabit(habit_id.clone()))
            .padding([6, 8])
            .style(|_, status| theme::icon_button(theme, status)),
        );
    } else {
        actions = actions
            .push(
                button(
                    row![
                        icons::edit(colors.text_secondary, 12.0),
                        text("Rename".to_string())
                            .size(ty.size_caption)
                            .color(colors.text_secondary),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                )
                .on_press(Message::StartRename(habit_id.clone()))
                .padding([6, 8])
                .style(|_, status| theme::icon_button(theme, status)),
            )
            .push(
                button(
                    row![
                        icons::archive(colors.text_secondary, 12.0),
                        text("Archive".to_string())
                            .size(ty.size_caption)
                            .color(colors.text_secondary),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                )
                .on_press(Message::ArchiveHabit(habit_id.clone()))
                .padding([6, 8])
                .style(|_, status| theme::icon_button(theme, status)),
            );
    }

    actions = actions.push(
        button(
            row![
                icons::trash(colors.text_secondary, 12.0),
                text("Delete".to_string())
                    .size(ty.size_caption)
                    .color(colors.text_secondary),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .on_press(Message::DeleteHabitRequest(habit_id))
        .padding([6, 8])
        .style(|_, status| theme::icon_button(theme, status)),
    );

    let content = row![
        checkbox,
        column![title, subtitle].spacing(2).width(Length::Fill),
        horizontal_space(),
        streak_badge::view(habit.current_streak, theme),
        actions,
    ]
    .align_y(Alignment::Center)
    .spacing(sp.md)
    .width(Length::Fill);

    container(content)
        .padding([sp.md, sp.lg])
        .width(Length::Fill)
        .style(|_| theme::habit_card(theme, habit.completed_today))
        .into()
}

fn subtitle(habit: &HabitWithStats, today: NaiveDate, archived_view: bool) -> String {
    if archived_view {
        return "Archived habit".to_string();
    }

    if habit.completed_today {
        return "Done today".to_string();
    }

    match habit.last_completed_date {
        Some(last_completed) => {
            let days_ago = (today - last_completed).num_days();

            if days_ago == 1 {
                "Last done: yesterday".to_string()
            } else if days_ago > 1 {
                let missed_days = days_ago - 1;
                if missed_days == 1 {
                    "Missed 1 day".to_string()
                } else {
                    format!("Missed {missed_days} days")
                }
            } else {
                "Done today".to_string()
            }
        }
        None => "No history yet".to_string(),
    }
}

fn subtitle_color(
    habit: &HabitWithStats,
    today: NaiveDate,
    archived_view: bool,
    theme: &AppTheme,
) -> iced::Color {
    let colors = &theme.colors;

    if archived_view {
        return colors.text_tertiary;
    }

    if habit.completed_today {
        return colors.success_fg;
    }

    match habit.last_completed_date {
        None => colors.text_tertiary,
        Some(last_completed) => {
            let days_ago = (today - last_completed).num_days();

            if days_ago > 2 {
                colors.warning_fg
            } else {
                colors.text_secondary
            }
        }
    }
}
