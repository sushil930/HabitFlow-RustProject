use chrono::{Datelike, Duration, NaiveDate};
use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

use crate::models::HabitWithStats;
use crate::theme::{self, AppTheme, WeekCircle};
use crate::{views, BannerMessage, Message};

pub fn view<'a>(
    habits: &'a [HabitWithStats],
    today: NaiveDate,
    banner: Option<BannerMessage>,
    theme: &'a AppTheme,
) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;
    let week_start = today - Duration::days(today.weekday().num_days_from_monday() as i64);

    let mut content = column![
        text("This Week")
            .size(ty.size_display)
            .color(theme.colors.text_primary),
        views::divider(theme),
    ]
    .spacing(sp.lg);

    if let Some(banner) = banner {
        content = content.push(views::banner(banner, theme));
    }

    if habits.is_empty() {
        content = content.push(
            container(
                column![
                    text("No habits to show")
                        .size(ty.size_heading_lg)
                        .color(theme.colors.text_primary),
                    text("Create a habit in the Today view to see the weekly grid.")
                        .size(ty.size_body_md)
                        .color(theme.colors.text_secondary),
                ]
                .spacing(sp.md)
                .align_x(Alignment::Center)
                .width(Length::Fill),
            )
            .padding([72, 0]),
        );

        return views::content_shell(content, theme);
    }

    let mut grid = column![header_row(week_start, today, theme)].spacing(sp.md);

    for habit in habits {
        let mut cells = row![container(
            text(habit.habit.name.clone())
                .size(ty.size_body_sm)
                .color(theme.colors.text_primary)
        )
        .width(200)]
        .spacing(sp.sm)
        .align_y(Alignment::Center);

        let mut week_completions = 0;
        let mut max_possible_completions = 0;

        for offset in 0..7 {
            let date = week_start + Duration::days(offset);

            if date <= today && date >= habit.habit.created_date {
                max_possible_completions += 1;
                if habit.is_completed_on(date) {
                    week_completions += 1;
                }
            }

            let cell: Element<'a, Message> = if date > today || date < habit.habit.created_date {
                container("").width(24).height(24).into()
            } else {
                let state = if habit.is_completed_on(date) {
                    WeekCircle::Completed
                } else if date == today {
                    WeekCircle::TodayPending
                } else {
                    WeekCircle::Missed
                };

                let size = if date == today { 14 } else { 12 };

                container("")
                    .width(size)
                    .height(size)
                    .style(move |_| theme::week_circle(theme, state))
                    .into()
            };

            cells = cells.push(container(cell).center_x(52));
        }

        let progress_percent = if max_possible_completions > 0 {
            (week_completions as f32 / max_possible_completions as f32) * 100.0
        } else {
            0.0
        };

        cells = cells.push(
            container(
                column![
                    text(format!("{:.0}%", progress_percent))
                        .size(ty.size_caption)
                        .color(if progress_percent >= 100.0 { theme.colors.success_fg } else { theme.colors.text_secondary }),
                    iced::widget::progress_bar(0.0..=100.0, progress_percent)
                        .height(4)
                ]
                .spacing(4)
                .align_x(Alignment::End)
            )
            .width(80)
            .align_y(Alignment::Center)
        );

        let row_card = container(cells)
            .padding([12, 16])
            .style(|_| theme::section_card(theme));

        grid = grid.push(row_card);
    }

    let legend = row![
        legend_chip("Completed", WeekCircle::Completed, theme),
        legend_chip("Missed", WeekCircle::Missed, theme),
        legend_chip("Today", WeekCircle::TodayPending, theme),
    ]
    .spacing(sp.lg)
    .align_y(Alignment::Center);

    content = content.push(scrollable(column![grid, legend].spacing(sp.xl)).direction(crate::views::hidden_scrollbar()).height(Length::Fill));

    views::content_shell(content, theme)
}

fn header_row<'a>(
    week_start: NaiveDate,
    today: NaiveDate,
    theme: &'a AppTheme,
) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;
    let mut row = row![container("").width(200)]
        .spacing(sp.sm)
        .align_y(Alignment::Center);

    for offset in 0..7 {
        let date = week_start + Duration::days(offset);
        let day_name = date.format("%a").to_string();
        let is_today = date == today;

        let content = column![
            text(day_name)
                .size(ty.size_caption)
                .color(if is_today { theme.colors.text_primary } else { theme.colors.text_secondary }),
            text(date.format("%d").to_string())
                .size(ty.size_body_md)
                .color(if is_today { theme.colors.text_primary } else { theme.colors.text_secondary })
        ]
        .align_x(Alignment::Center)
        .spacing(2);

        let chip: Element<'a, Message> = if is_today {
            container(content)
                .padding([4, 8])
                .style(|_| theme::today_marker(theme))
                .into()
        } else {
            container(content)
                .padding([4, 8])
                .into()
        };

        row = row.push(container(chip).center_x(52));
    }

    row = row.push(
        container(
            text("Progress")
                .size(ty.size_caption)
                .color(theme.colors.text_secondary)
        )
        .width(80)
        .align_x(Alignment::End)
    );

    container(row).padding([8, 16]).into()
}

fn legend_chip<'a>(label: &str, state: WeekCircle, theme: &'a AppTheme) -> Element<'a, Message> {
    row![
        container("")
            .width(12)
            .height(12)
            .style(move |_| theme::week_circle(theme, state)),
        text(label.to_string())
            .size(theme.typography.size_caption)
            .color(theme.colors.text_secondary),
    ]
    .spacing(theme.spacing.xs)
    .align_y(Alignment::Center)
    .into()
}
