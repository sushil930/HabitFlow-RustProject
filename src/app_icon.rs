use iced::window::icon;
use iced::window::Icon;

pub fn window_icon() -> Icon {
    icon::from_file_data(include_bytes!("../assets/app_icon.png"), None)
        .expect("generated app window icon should be valid and loaded from assets/app_icon.png")
}