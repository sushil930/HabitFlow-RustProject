use iced::widget::{button, column, container, horizontal_space, pick_list, row, scrollable, text, text_input, Space};
use iced::{alignment, Background, Border, Element, Length};
use lucide_icons::Icon;

use crate::lucide::{featured_icons, icon_by_name, icon_matches_query, sanitize_icon_name, ICON_CATALOG};
use crate::models::HabitCategory;
use crate::theme::{self, AppTheme};
use crate::{views, AddHabitDraft, Message};

const ICON_GRID_COLUMNS: usize = 4;

pub fn view<'a>(name_input_id: text_input::Id, draft: &'a AddHabitDraft, theme: &'a AppTheme) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;

    let selected_icon = sanitize_icon_name(&draft.icon);
    let visible_icons = visible_icons(draft);

    let header = row![
        column![
            text("Add habit")
                .size(ty.size_display)
                .color(theme.colors.text_primary),
            text("Choose a category and icon before you save.")
                .size(ty.size_body_md)
                .color(theme.colors.text_secondary),
        ]
        .spacing(sp.xs)
        .width(Length::Fill),
        horizontal_space(),
        button(
            text("Cancel")
                .size(ty.size_body_sm)
                .color(theme.colors.text_secondary),
        )
        .on_press(Message::HideAddInput)
        .padding([8, 12])
        .style(|_, status| theme::ghost_button(theme, status)),
    ]
    .align_y(alignment::Alignment::Center);

    let name_field = text_input("Name this habit...", &draft.name)
        .id(name_input_id)
        .on_input(Message::AddInputChanged)
        .on_submit(Message::SubmitAddHabit)
        .padding([sp.md, sp.lg])
        .size(ty.size_body_lg)
        .style(|_, status| theme::text_field(theme, status));

    let category_picker = pick_list(
        &HabitCategory::ALL[..],
        Some(draft.category),
        Message::SelectAddCategory,
    );

    let selected_icon_preview = container(
        row![
            icon_by_name(&selected_icon, theme.colors.accent_primary, 30.0),
            column![
                text(pretty_icon_label(&selected_icon))
                    .size(ty.size_body_lg)
                    .color(theme.colors.text_primary),
                text(format!("{} icons available", visible_icons.len()))
                    .size(ty.size_caption)
                    .color(theme.colors.text_secondary),
            ]
            .spacing(2),
        ]
        .spacing(sp.md)
        .align_y(alignment::Alignment::Center),
    )
    .padding([sp.md, sp.lg])
    .style(|_| theme::section_card(theme));

    let left_panel = container(
        column![
            text("Details")
                .size(ty.size_heading_md)
                .color(theme.colors.text_primary),
            name_field,
            column![
                text("Category")
                    .size(ty.size_body_sm)
                    .color(theme.colors.text_secondary),
                category_picker,
            ]
            .spacing(8),
            column![
                text("Selected icon")
                    .size(ty.size_body_sm)
                    .color(theme.colors.text_secondary),
                selected_icon_preview,
            ]
            .spacing(8),
        ]
        .spacing(sp.lg),
    )
    .padding(sp.lg)
    .width(Length::FillPortion(1))
    .style(|_| theme::section_card(theme));

    let icon_search = text_input("Search Lucide icons...", &draft.icon_search)
        .on_input(Message::AddIconSearchChanged)
        .padding([sp.md, sp.lg])
        .size(ty.size_body_md)
        .style(|_, status| theme::text_field(theme, status));

    let icon_results_header = row![
        text("Icon picker")
            .size(ty.size_heading_md)
            .color(theme.colors.text_primary),
        horizontal_space(),
        text(if draft.icon_search.trim().is_empty() {
            format!("Featured icons • {} choices", visible_icons.len())
        } else {
            format!("{} matches", visible_icons.len())
        })
        .size(ty.size_caption)
        .color(theme.colors.text_secondary),
    ]
    .align_y(alignment::Alignment::Center);

    let icon_grid: Element<'_, Message> = if visible_icons.is_empty() {
        container(
            text("No icons matched your search.")
                .size(ty.size_body_md)
                .color(theme.colors.text_secondary),
        )
        .width(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else {
        scrollable(icon_grid_view(&visible_icons, &selected_icon, theme))
            .direction(views::hidden_scrollbar())
            .height(Length::Fixed(384.0))
            .into()
    };

    let right_panel = container(
        column![icon_results_header, icon_search, icon_grid]
            .spacing(sp.md)
            .width(Length::Fill),
    )
    .padding(sp.lg)
    .width(Length::FillPortion(1))
    .style(|_| theme::section_card(theme));

    let footer = row![
        horizontal_space(),
        button(
            text("Create habit")
                .size(ty.size_body_sm)
                .color(theme.colors.text_inverse),
        )
        .on_press(Message::SubmitAddHabit)
        .padding([10, 16])
        .style(|_, status| theme::primary_button(theme, status)),
    ]
    .align_y(alignment::Alignment::Center);

    container(
        column![header, row![left_panel, right_panel].spacing(sp.xl), footer]
            .spacing(sp.xl)
            .width(Length::Fill),
    )
    .padding(sp.xl)
    .width(Length::Fill)
    .style(|_| theme::dialog(theme))
    .into()
}

fn visible_icons(draft: &AddHabitDraft) -> Vec<Icon> {
    let query = draft.icon_search.trim();

    if query.is_empty() {
        return featured_icons();
    }

    ICON_CATALOG
        .iter()
        .copied()
        .filter(|icon| icon_matches_query(*icon, query))
        .take(72)
        .collect()
}

fn icon_grid_view<'a>(icons: &[Icon], selected_icon: &str, theme: &'a AppTheme) -> Element<'a, Message> {
    let sp = &theme.spacing;

    let mut rows = column![].spacing(sp.sm);

    for chunk in icons.chunks(ICON_GRID_COLUMNS) {
        let mut icon_row = row![].spacing(sp.sm).width(Length::Fill);

        for icon in chunk {
            icon_row = icon_row.push(icon_button(*icon, selected_icon, theme));
        }

        if chunk.len() < ICON_GRID_COLUMNS {
            for _ in chunk.len()..ICON_GRID_COLUMNS {
                icon_row = icon_row.push(Space::with_width(Length::FillPortion(1)));
            }
        }

        rows = rows.push(icon_row);
    }

    rows.into()
}

fn icon_button<'a>(icon: Icon, selected_icon: &str, theme: &'a AppTheme) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;
    let icon_name = icon.to_string();
    let selected = icon_name == selected_icon;
    let icon_color = if selected {
        theme.colors.accent_primary
    } else {
        theme.colors.text_primary
    };

    let label_color = if selected {
        theme.colors.text_primary
    } else {
        theme.colors.text_secondary
    };

    button(
        column![
            icon_by_name(&icon_name, icon_color, 24.0),
            text(pretty_icon_label(&icon_name))
                .size(ty.size_caption)
                .color(label_color),
        ]
        .spacing(6)
        .align_x(alignment::Alignment::Center),
    )
    .on_press(Message::SelectAddIcon(icon_name))
    .padding([sp.md, sp.sm])
    .width(Length::FillPortion(1))
    .style(move |_, status| icon_button_style(theme, selected, status))
    .into()
}

fn icon_button_style(theme: &AppTheme, selected: bool, status: button::Status) -> button::Style {
    let background = if selected {
        match status {
            button::Status::Hovered => theme.colors.surface_active,
            button::Status::Pressed => theme.colors.surface_active.scale_alpha(0.95),
            _ => theme.colors.surface_base,
        }
    } else {
        match status {
            button::Status::Hovered => theme.colors.surface_hover,
            button::Status::Pressed => theme.colors.surface_active,
            _ => theme.colors.surface_base,
        }
    };

    let border_color = if selected {
        theme.colors.accent_primary
    } else {
        theme.colors.border_default
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: if selected {
            theme.colors.text_primary
        } else {
            theme.colors.text_secondary
        },
        border: Border {
            radius: 12.0.into(),
            width: if selected { 1.5 } else { 1.0 },
            color: border_color,
        },
        ..button::Style::default()
    }
}

fn pretty_icon_label(name: &str) -> String {
    name.split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut characters = part.chars();
            match characters.next() {
                Some(first) => {
                    let mut word = first.to_uppercase().to_string();
                    word.push_str(characters.as_str());
                    word
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}