use chrono::NaiveDate;
use iced::widget::{column, container, scrollable, text};
use iced::{Alignment, Element, Length};

use crate::components::habit_card;
use crate::models::HabitWithStats;
use crate::theme::AppTheme;
use crate::{views, BannerMessage, Message};

pub fn view<'a>(
    habits: &'a [HabitWithStats],
    today: NaiveDate,
    banner: Option<BannerMessage>,
    theme: &'a AppTheme,
) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;

    let mut content = column![
        text("Archived")
            .size(ty.size_display)
            .color(theme.colors.text_primary),
        views::divider(theme)
    ]
    .spacing(sp.lg);

    if let Some(banner) = banner {
        content = content.push(views::banner(banner, theme));
    }

    if habits.is_empty() {
        content = content.push(
            container(
                column![
                    text("No archived habits")
                        .size(ty.size_heading_lg)
                        .color(theme.colors.text_primary),
                    text("Archived habits will show up here so your main list stays focused.")
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

    let mut list = column![].spacing(sp.sm);

    for habit in habits {
        list = list.push(habit_card::view(habit, today, true, true, theme));
    }

    content = content.push(scrollable(list).direction(crate::views::hidden_scrollbar()).height(Length::Fill));

    views::content_shell(content, theme)
}
