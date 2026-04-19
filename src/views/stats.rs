use chrono::{Datelike, Duration, NaiveDate};
use iced::widget::{column, container, horizontal_space, pick_list, row, scrollable, text, Space, vertical_space};
use iced::{Alignment, Element, Font, Length};
use std::collections::HashMap;

use crate::components::{category_pie_chart, line_chart::LineChart};
use crate::models::HabitWithStats;
use crate::theme::AppTheme;
use crate::{views, BannerMessage, Message};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChartFilter {
    AllTasks,
    Habit(String, String),
}

impl std::fmt::Display for ChartFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartFilter::AllTasks => write!(f, "All tasks"),
            ChartFilter::Habit(_, name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartRange {
    Days7,
    Days14,
    Days30,
}

impl ChartRange {
    pub const ALL: [Self; 3] = [Self::Days7, Self::Days14, Self::Days30];

    pub fn days(self) -> i64 {
        match self {
            Self::Days7 => 7,
            Self::Days14 => 14,
            Self::Days30 => 30,
        }
    }

    pub fn from_days(days: i64) -> Self {
        match days {
            7 => Self::Days7,
            30 => Self::Days30,
            _ => Self::Days14,
        }
    }
}

impl std::fmt::Display for ChartRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartRange::Days7 => write!(f, "7 days"),
            ChartRange::Days14 => write!(f, "14 days"),
            ChartRange::Days30 => write!(f, "30 days"),
        }
    }
}

pub fn view<'a>(
    habits: &'a [HabitWithStats],
    today: NaiveDate,
    banner: Option<BannerMessage>,
    theme: &'a AppTheme,
    selected_habit: &'a Option<String>,
    selected_range_days: i64,
    chart_reveal_progress: f32,
) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;

    let mut content = column![
        text("Dashboard")
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
                    text("No stats to show yet")
                        .size(ty.size_heading_lg)
                        .color(theme.colors.text_primary),
                    text("Track a habit for a day or two and your stats will show up here.")
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

    // Top Level Summary Cards
    let top_stats = top_level_stats(habits, theme);

    // Category distribution chart
    let category_chart = category_pie_chart::view(habits, theme);

    // Global Heatmap
    let heatmap = global_heatmap(habits, today, theme);

    // Trend line chart with task and date range filters
    let mut filter_options = vec![ChartFilter::AllTasks];
    for h in habits {
        filter_options.push(ChartFilter::Habit(h.habit.id.clone(), h.habit.name.clone()));
    }

    let selected_filter = match selected_habit {
        Some(id) => habits
            .iter()
            .find(|h| h.habit.id == *id)
            .map(|h| ChartFilter::Habit(id.clone(), h.habit.name.clone()))
            .unwrap_or(ChartFilter::AllTasks),
        None => ChartFilter::AllTasks,
    };

    let filter_dropdown = pick_list(
        filter_options,
        Some(selected_filter),
        |filter| match filter {
            ChartFilter::AllTasks => Message::SelectStatsHabit(None),
            ChartFilter::Habit(id, _) => Message::SelectStatsHabit(Some(id)),
        },
    );

    let selected_range = ChartRange::from_days(selected_range_days);
    let range_dropdown = pick_list(
        &ChartRange::ALL[..],
        Some(selected_range),
        |range| Message::SelectStatsRange(range.days()),
    );

    let program = LineChart::new(
        habits,
        selected_habit.clone(),
        today,
        theme,
        selected_range.days(),
        chart_reveal_progress,
    );

    let chart_container = container(
        column![
            row![
                text(format!("{} Completion Trend", selected_range))
                    .size(theme.typography.size_heading_md)
                    .color(theme.colors.text_primary),
                horizontal_space(),
                filter_dropdown,
                range_dropdown,
            ]
            .align_y(Alignment::Center),
            text("Hover the line to inspect day-by-day completion details.")
                .size(theme.typography.size_caption)
                .color(theme.colors.text_secondary),
            Space::new(0, 8.0),
            iced::widget::canvas::Canvas::new(program)
                .width(Length::Fill)
                .height(Length::Fixed(200.0))
        ]
        .spacing(16)
    )
    .padding(24)
    .width(Length::Fill)
    .style(|_| crate::theme::section_card(theme));

    let mut cards = column![].spacing(sp.lg);

    for habit in habits {
        cards = cards.push(card(habit, theme));
    }

    let inner_content = column![
        top_stats,
        category_chart,
        heatmap,
        chart_container,
        views::divider(theme),
        text("Habit Breakdown")
            .size(ty.size_heading_lg)
            .color(theme.colors.text_primary),
        cards
    ]
    .spacing(sp.lg);

    let scrollable_content = scrollable(inner_content)
        .direction(crate::views::hidden_scrollbar())
        .height(Length::Fill);

    content = content.push(scrollable_content);

    views::content_shell(content, theme)
}

fn top_level_stats<'a>(habits: &'a [HabitWithStats], theme: &'a AppTheme) -> Element<'a, Message> {
    let total_tasks = habits.len();
    let total_completions: u32 = habits.iter().map(|h| h.total_completions).sum();
    let avg_rate = if total_tasks > 0 {
        habits.iter().map(|h| h.last_30_days_rate).sum::<f32>() / (total_tasks as f32)
    } else {
        0.0
    };
    let level = 1 + (total_completions / 10);

    row![
        stat_summary_card("Total Habits", &total_tasks.to_string(), theme),
        stat_summary_card("Total Completions", &total_completions.to_string(), theme),
        stat_summary_card("Completion Rate", &format!("{:.0}%", avg_rate), theme),
        stat_summary_card("Level", &format!("Lvl {}", level), theme),
    ]
    .spacing(16)
    .width(Length::Fill)
    .into()
}

fn stat_summary_card<'a>(label: &str, value: &str, theme: &'a AppTheme) -> Element<'a, Message> {
    container(
        column![
            text(label.to_string())
                .size(theme.typography.size_body_sm)
                .color(theme.colors.text_secondary),
            vertical_space().height(4),
            text(value.to_string())
                .size(28) // Large highly-visible display number
                .color(theme.colors.text_primary),
        ]
    )
    .padding(20)
    .width(Length::Fill)
    .style(|_| crate::theme::section_card(theme))
    .into()
}

fn global_heatmap<'a>(
    habits: &'a [HabitWithStats],
    today: NaiveDate,
    theme: &'a AppTheme,
) -> Element<'a, Message> {
    let start_date = today - Duration::days(364);
    
    // Aggregation
    let mut completion_counts = HashMap::new();
    for h in habits {
        for l in &h.history_logs {
            if l.completed {
                *completion_counts.entry(l.date).or_insert(0) += 1;
            }
        }
    }
    
    let mut columns: Vec<Element<'a, Message>> = Vec::new();
    
    let days_from_monday = start_date.weekday().num_days_from_monday() as i64;
    let grid_start = start_date - Duration::days(days_from_monday);
    let mut date_cursor = grid_start;
    
    while date_cursor <= today {
        let mut week_col = column![].spacing(3);
        for _ in 0..7 {
            if date_cursor > today || date_cursor < start_date {
                // Invisible padding cell
                week_col = week_col.push(container(Space::new(10, 10)));
            } else {
                let count = completion_counts.get(&date_cursor).copied().unwrap_or(0);
                
                let bg_color = if count == 0 {
                    theme.colors.surface_active // light gray / zinc 200
                } else if count == 1 {
                    theme.colors.success_fg.scale_alpha(0.3)
                } else if count == 2 {
                    theme.colors.success_fg.scale_alpha(0.6)
                } else {
                    theme.colors.success_fg
                };
                
                let cell = container(Space::new(10, 10))
                    .style(move |_| iced::widget::container::Style {
                        background: Some(iced::Background::Color(bg_color)),
                        border: iced::Border {
                            radius: 2.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                
                week_col = week_col.push(cell);
            }
            date_cursor += Duration::days(1);
        }
        columns.push(week_col.into());
    }
    
    let heatmap_row = row(columns).spacing(3);
    
    container(
        column![
            text("Activity (Last Year)")
                .size(theme.typography.size_heading_md)
                .color(theme.colors.text_primary),
            heatmap_row
        ]
        .spacing(16)
    )
    .padding(24)
    .width(Length::Fill)
    .style(|_| crate::theme::section_card(theme))
    .into()
}



fn card<'a>(habit: &'a HabitWithStats, theme: &'a AppTheme) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;

    let current_streak = if habit.current_streak >= 3 {
        format!("{} days 🔥", habit.current_streak)
    } else {
        format!("{} days", habit.current_streak)
    };

    container(
        column![
            text(habit.habit.name.clone())
                .size(ty.size_heading_md)
                .color(theme.colors.text_primary),
            views::divider(theme),
            stat_row("Current streak", &current_streak, theme),
            stat_row(
                "Longest streak",
                &format!("{} days", habit.longest_streak),
                theme,
            ),
            stat_row(
                "Total completions",
                &habit.total_completions.to_string(),
                theme,
            ),
            stat_row(
                "Last 30 days",
                &format!("{:.0}%", habit.last_30_days_rate),
                theme,
            ),
        ]
        .spacing(sp.md),
    )
    .padding(sp.lg)
    .style(|_| crate::theme::section_card(theme))
    .into()
}

fn stat_row<'a>(label: &str, value: &str, theme: &'a AppTheme) -> Element<'a, Message> {
    row![
        text(label.to_string())
            .size(theme.typography.size_body_sm)
            .color(theme.colors.text_secondary),
        horizontal_space(),
        text(value.to_string())
            .size(theme.typography.size_mono_md)
            .font(Font::MONOSPACE)
            .color(theme.colors.text_primary),
    ]
    .align_y(Alignment::Center)
    .into()
}
