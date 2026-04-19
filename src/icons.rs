use iced::widget::svg::Handle;
use iced::{Color, Element, Length};

pub const APP_LOGO_BYTES: &[u8] = include_bytes!("../assets/habit_logo.svg");

fn lucide_svg(filename: &str) -> &'static [u8] {
    match filename {
        "archive.svg" => include_bytes!("../assets/icons/archive.svg"),
        "archive-restore.svg" => include_bytes!("../assets/icons/archive-restore.svg"),
        "bar-chart-2.svg" => include_bytes!("../assets/icons/bar-chart-2.svg"),
        "calendar-days.svg" => include_bytes!("../assets/icons/calendar-days.svg"),
        "calendar.svg" => include_bytes!("../assets/icons/calendar.svg"),
        "check.svg" => include_bytes!("../assets/icons/check.svg"),
        "download.svg" => include_bytes!("../assets/icons/download.svg"),
        "fire.svg" => include_bytes!("../assets/icons/fire.svg"),
        "pencil.svg" => include_bytes!("../assets/icons/pencil.svg"),
        "plus.svg" => include_bytes!("../assets/icons/plus.svg"),
        "settings.svg" => include_bytes!("../assets/icons/settings.svg"),
        "trash-2.svg" => include_bytes!("../assets/icons/trash-2.svg"),
        "upload.svg" => include_bytes!("../assets/icons/upload.svg"),
        _ => include_bytes!("../assets/icons/check.svg"),
    }
}

pub fn icon<'a, Message>(filename: &str, color: Color, size: f32) -> Element<'a, Message> {
    iced::widget::svg(Handle::from_memory(lucide_svg(filename)))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |_, _| iced::widget::svg::Style {
            color: Some(color),
        })
        .into()
}

pub fn app_logo<'a, Message>(size: f32) -> Element<'a, Message> {
    iced::widget::svg(Handle::from_memory(APP_LOGO_BYTES))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .into()
}

pub fn check<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("check.svg", color, size)
}

pub fn settings<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("settings.svg", color, size)
}

pub fn edit<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("pencil.svg", color, size)
}

pub fn trash<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("trash-2.svg", color, size)
}

pub fn archive<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("archive.svg", color, size)
}

pub fn unarchive<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("archive-restore.svg", color, size)
}

pub fn plus<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("plus.svg", color, size)
}

pub fn calendar<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("calendar.svg", color, size)
}

pub fn calendar_days<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("calendar-days.svg", color, size)
}

pub fn bar_chart<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("bar-chart-2.svg", color, size)
}

pub fn download<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("download.svg", color, size)
}

pub fn upload<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("upload.svg", color, size)
}

pub fn fire<'a, Message>(color: Color, size: f32) -> Element<'a, Message> {
    icon("fire.svg", color, size)
}
