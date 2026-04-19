use iced::widget::Text;
use lucide_icons::Icon;
use std::convert::TryFrom;

include!(concat!(env!("OUT_DIR"), "/lucide_catalog.rs"));

pub const DEFAULT_ICON_NAME: &str = "sparkles";

pub const FEATURED_ICON_NAMES: &[&str] = &[
    "sparkles",
    "heart-pulse",
    "dumbbell",
    "brain-circuit",
    "book-open",
    "graduation-cap",
    "briefcase-business",
    "home",
    "users",
    "wallet",
    "palette",
    "leaf",
    "sun-moon",
    "coffee",
    "medal",
    "target",
    "rocket",
    "calendar",
    "calendar-check-2",
    "check-circle-2",
    "flame",
    "flower-2",
    "fish",
    "trophy",
    "watch",
    "sprout",
    "tree-pine",
    "cloud",
    "alarm-clock",
    "clock-3",
    "zap",
    "star",
    "gift",
    "guitar",
    "camera",
    "car",
    "code-2",
    "cpu",
    "database",
    "file-text",
    "map",
    "mountain",
    "music",
    "notebook-pen",
    "pen-line",
    "shield-check",
    "shopping-bag",
    "smile",
    "squirrel",
    "gamepad-2",
    "bar-chart-3",
    "monitor-smartphone",
    "clipboard-list",
    "atom",
    "laptop",
];

pub fn icon_by_name(name: &str, color: iced::Color, size: f32) -> Text<'static> {
    Text::from(Icon::try_from(name).unwrap_or(Icon::Sparkles))
        .size(size)
        .color(color)
}

pub fn sanitize_icon_name(name: &str) -> String {
    Icon::try_from(name)
        .map(|icon| icon.to_string())
        .unwrap_or_else(|_| DEFAULT_ICON_NAME.to_string())
}

pub fn icon_matches_query(icon: Icon, query: &str) -> bool {
    let normalized_query = query.trim().to_lowercase();

    if normalized_query.is_empty() {
        return true;
    }

    let icon_name = icon.to_string();
    let humanized = icon_name.replace('-', " ");

    icon_name.contains(&normalized_query) || humanized.contains(&normalized_query)
}

pub fn featured_icons() -> Vec<Icon> {
    FEATURED_ICON_NAMES
        .iter()
        .filter_map(|name| Icon::try_from(*name).ok())
        .collect()
}