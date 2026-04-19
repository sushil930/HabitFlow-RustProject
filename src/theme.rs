use iced::application;
use iced::widget::{button, container, text_input};
use iced::{Background, Border, Color, Theme};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemePreset {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

impl ThemePreset {
    pub const ALL: [Self; 22] = [
        Self::Light,
        Self::Dark,
        Self::Dracula,
        Self::Nord,
        Self::SolarizedLight,
        Self::SolarizedDark,
        Self::GruvboxLight,
        Self::GruvboxDark,
        Self::CatppuccinLatte,
        Self::CatppuccinFrappe,
        Self::CatppuccinMacchiato,
        Self::CatppuccinMocha,
        Self::TokyoNight,
        Self::TokyoNightStorm,
        Self::TokyoNightLight,
        Self::KanagawaWave,
        Self::KanagawaDragon,
        Self::KanagawaLotus,
        Self::Moonfly,
        Self::Nightfly,
        Self::Oxocarbon,
        Self::Ferra,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::Dracula => "Dracula",
            Self::Nord => "Nord",
            Self::SolarizedLight => "Solarized Light",
            Self::SolarizedDark => "Solarized Dark",
            Self::GruvboxLight => "Gruvbox Light",
            Self::GruvboxDark => "Gruvbox Dark",
            Self::CatppuccinLatte => "Catppuccin Latte",
            Self::CatppuccinFrappe => "Catppuccin Frappe",
            Self::CatppuccinMacchiato => "Catppuccin Macchiato",
            Self::CatppuccinMocha => "Catppuccin Mocha",
            Self::TokyoNight => "Tokyo Night",
            Self::TokyoNightStorm => "Tokyo Night Storm",
            Self::TokyoNightLight => "Tokyo Night Light",
            Self::KanagawaWave => "Kanagawa Wave",
            Self::KanagawaDragon => "Kanagawa Dragon",
            Self::KanagawaLotus => "Kanagawa Lotus",
            Self::Moonfly => "Moonfly",
            Self::Nightfly => "Nightfly",
            Self::Oxocarbon => "Oxocarbon",
            Self::Ferra => "Ferra",
        }
    }

    pub fn storage_key(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::Dracula => "dracula",
            Self::Nord => "nord",
            Self::SolarizedLight => "solarized_light",
            Self::SolarizedDark => "solarized_dark",
            Self::GruvboxLight => "gruvbox_light",
            Self::GruvboxDark => "gruvbox_dark",
            Self::CatppuccinLatte => "catppuccin_latte",
            Self::CatppuccinFrappe => "catppuccin_frappe",
            Self::CatppuccinMacchiato => "catppuccin_macchiato",
            Self::CatppuccinMocha => "catppuccin_mocha",
            Self::TokyoNight => "tokyo_night",
            Self::TokyoNightStorm => "tokyo_night_storm",
            Self::TokyoNightLight => "tokyo_night_light",
            Self::KanagawaWave => "kanagawa_wave",
            Self::KanagawaDragon => "kanagawa_dragon",
            Self::KanagawaLotus => "kanagawa_lotus",
            Self::Moonfly => "moonfly",
            Self::Nightfly => "nightfly",
            Self::Oxocarbon => "oxocarbon",
            Self::Ferra => "ferra",
        }
    }

    pub fn from_storage_key(value: &str) -> Option<Self> {
        let normalized = value.trim().to_ascii_lowercase().replace([' ', '-'], "_");

        Some(match normalized.as_str() {
            "light" => Self::Light,
            "dark" => Self::Dark,
            "dracula" => Self::Dracula,
            "nord" => Self::Nord,
            "solarized_light" => Self::SolarizedLight,
            "solarized_dark" => Self::SolarizedDark,
            "gruvbox_light" => Self::GruvboxLight,
            "gruvbox_dark" => Self::GruvboxDark,
            "catppuccin_latte" => Self::CatppuccinLatte,
            "catppuccin_frappe" => Self::CatppuccinFrappe,
            "catppuccin_macchiato" => Self::CatppuccinMacchiato,
            "catppuccin_mocha" => Self::CatppuccinMocha,
            "tokyo_night" => Self::TokyoNight,
            "tokyo_night_storm" => Self::TokyoNightStorm,
            "tokyo_night_light" => Self::TokyoNightLight,
            "kanagawa_wave" => Self::KanagawaWave,
            "kanagawa_dragon" => Self::KanagawaDragon,
            "kanagawa_lotus" => Self::KanagawaLotus,
            "moonfly" => Self::Moonfly,
            "nightfly" => Self::Nightfly,
            "oxocarbon" => Self::Oxocarbon,
            "ferra" => Self::Ferra,
            _ => return None,
        })
    }
}

impl fmt::Display for ThemePreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Debug, Clone)]
pub struct Colors {
    pub surface_base: Color,
    pub surface_page: Color,
    pub surface_hover: Color,
    pub surface_active: Color,
    pub surface_sidebar: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_tertiary: Color,
    pub text_inverse: Color,
    pub border_subtle: Color,
    pub border_default: Color,
    pub border_strong: Color,
    pub accent_primary: Color,
    pub success_bg: Color,
    pub success_fg: Color,
    pub warning_bg: Color,
    pub warning_fg: Color,
    pub error_bg: Color,
    pub error_fg: Color,
}

#[derive(Debug, Clone)]
pub struct Spacing {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub xxl: f32,
    pub xxxl: f32,
    pub xxxxl: f32,
}

#[derive(Debug, Clone)]
pub struct Typography {
    pub size_display: f32,
    pub size_heading_lg: f32,
    pub size_heading_md: f32,
    pub size_body_lg: f32,
    pub size_body_md: f32,
    pub size_body_sm: f32,
    pub size_caption: f32,
    pub size_mono_lg: f32,
    pub size_mono_md: f32,
}

#[derive(Debug, Clone)]
pub struct AppTheme {
    pub preset: ThemePreset,
    pub colors: Colors,
    pub spacing: Spacing,
    pub typography: Typography,
}

impl AppTheme {
    pub fn light() -> Self {
        Self::from_preset(ThemePreset::Light)
    }

    pub fn from_preset(preset: ThemePreset) -> Self {
        Self {
            preset,
            colors: colors_for_preset(preset),
            spacing: default_spacing(),
            typography: default_typography(),
        }
    }

    pub fn iced_theme(&self) -> Theme {
        match self.preset {
            ThemePreset::Light => Theme::Light,
            ThemePreset::Dark => Theme::Dark,
            ThemePreset::Dracula => Theme::Dracula,
            ThemePreset::Nord => Theme::Nord,
            ThemePreset::SolarizedLight => Theme::SolarizedLight,
            ThemePreset::SolarizedDark => Theme::SolarizedDark,
            ThemePreset::GruvboxLight => Theme::GruvboxLight,
            ThemePreset::GruvboxDark => Theme::GruvboxDark,
            ThemePreset::CatppuccinLatte => Theme::CatppuccinLatte,
            ThemePreset::CatppuccinFrappe => Theme::CatppuccinFrappe,
            ThemePreset::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            ThemePreset::CatppuccinMocha => Theme::CatppuccinMocha,
            ThemePreset::TokyoNight => Theme::TokyoNight,
            ThemePreset::TokyoNightStorm => Theme::TokyoNightStorm,
            ThemePreset::TokyoNightLight => Theme::TokyoNightLight,
            ThemePreset::KanagawaWave => Theme::KanagawaWave,
            ThemePreset::KanagawaDragon => Theme::KanagawaDragon,
            ThemePreset::KanagawaLotus => Theme::KanagawaLotus,
            ThemePreset::Moonfly => Theme::Moonfly,
            ThemePreset::Nightfly => Theme::Nightfly,
            ThemePreset::Oxocarbon => Theme::Oxocarbon,
            ThemePreset::Ferra => Theme::Ferra,
        }
    }

    pub fn application_style(&self) -> application::Appearance {
        application::Appearance {
            background_color: self.colors.surface_page,
            text_color: self.colors.text_primary,
        }
    }
}

pub fn sidebar_container(theme: &AppTheme) -> container::Style {
    container::Style::default().background(theme.colors.surface_sidebar)
}

pub fn page_container(theme: &AppTheme) -> container::Style {
    container::Style::default().background(theme.colors.surface_page)
}

pub fn section_card(theme: &AppTheme) -> container::Style {
    panel(theme.colors.surface_base, theme.colors.border_subtle, 8.0)
}

pub fn habit_card(theme: &AppTheme, completed: bool) -> container::Style {
    let background = if completed {
        theme.colors.success_bg
    } else {
        theme.colors.surface_base
    };

    panel(background, theme.colors.border_subtle, 8.0)
}

pub fn scrim() -> container::Style {
    container::Style::default().background(Color::from_rgba(0.0, 0.0, 0.0, 0.15))
}

pub fn dialog(theme: &AppTheme) -> container::Style {
    panel(theme.colors.surface_base, theme.colors.border_default, 8.0)
}

pub fn banner(theme: &AppTheme, kind: BannerKind) -> container::Style {
    match kind {
        BannerKind::Info => panel(theme.colors.surface_base, theme.colors.border_default, 6.0),
        BannerKind::Success => panel(theme.colors.success_bg, theme.colors.success_fg.scale_alpha(0.3), 6.0),
        BannerKind::Warning => panel(theme.colors.warning_bg, theme.colors.warning_fg.scale_alpha(0.3), 6.0),
        BannerKind::Error => panel(theme.colors.error_bg, theme.colors.error_fg.scale_alpha(0.3), 6.0),
    }
}

pub fn primary_button(theme: &AppTheme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active | button::Status::Pressed => button::Style {
            background: Some(Background::Color(theme.colors.accent_primary)),
            text_color: theme.colors.text_inverse,
            border: rounded_border(Color::TRANSPARENT, 0.0, 6.0),
            ..button::Style::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(theme.colors.text_primary)),
            text_color: theme.colors.text_inverse,
            border: rounded_border(Color::TRANSPARENT, 0.0, 6.0),
            ..button::Style::default()
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(
                theme.colors.accent_primary.scale_alpha(0.4),
            )),
            text_color: theme.colors.text_inverse.scale_alpha(0.6),
            border: rounded_border(Color::TRANSPARENT, 0.0, 6.0),
            ..button::Style::default()
        },
    }
}

pub fn secondary_button(theme: &AppTheme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered => theme.colors.surface_hover,
        button::Status::Pressed => theme.colors.surface_active,
        _ => theme.colors.surface_base,
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: if matches!(status, button::Status::Disabled) {
            theme.colors.text_tertiary
        } else {
            theme.colors.text_primary
        },
        border: rounded_border(theme.colors.border_default, 1.0, 6.0),
        ..button::Style::default()
    }
}

pub fn destructive_button(theme: &AppTheme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered => theme.colors.error_fg.scale_alpha(0.9),
        button::Status::Disabled => theme.colors.error_fg.scale_alpha(0.4),
        _ => theme.colors.error_fg,
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: theme.colors.text_inverse,
        border: rounded_border(Color::TRANSPARENT, 0.0, 6.0),
        ..button::Style::default()
    }
}

pub fn ghost_button(theme: &AppTheme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered => Some(Background::Color(theme.colors.surface_hover)),
        button::Status::Pressed => Some(Background::Color(theme.colors.surface_active)),
        _ => None,
    };

    button::Style {
        background,
        text_color: if matches!(status, button::Status::Disabled) {
            theme.colors.text_tertiary
        } else {
            theme.colors.text_secondary
        },
        border: rounded_border(Color::TRANSPARENT, 0.0, 6.0),
        ..button::Style::default()
    }
}

pub fn nav_button(theme: &AppTheme, active: bool, status: button::Status) -> button::Style {
    let background = if active {
        theme.colors.surface_active
    } else {
        match status {
            button::Status::Hovered => theme.colors.surface_hover,
            button::Status::Pressed => theme.colors.surface_active,
            _ => Color::TRANSPARENT,
        }
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: if active {
            theme.colors.text_primary
        } else {
            theme.colors.text_secondary
        },
        border: rounded_border(
            if active {
                theme.colors.border_default
            } else {
                Color::TRANSPARENT
            },
            if active { 1.0 } else { 0.0 },
            6.0,
        ),
        ..button::Style::default()
    }
}

pub fn icon_button(theme: &AppTheme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered => theme.colors.surface_hover,
        button::Status::Pressed => theme.colors.surface_active,
        _ => Color::TRANSPARENT,
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: theme.colors.text_secondary,
        border: rounded_border(Color::TRANSPARENT, 0.0, 6.0),
        ..button::Style::default()
    }
}

pub fn text_field(theme: &AppTheme, status: text_input::Status) -> text_input::Style {
    let border_color = match status {
        text_input::Status::Focused => theme.colors.accent_primary,
        text_input::Status::Hovered => theme.colors.border_strong,
        text_input::Status::Active => theme.colors.border_default,
        text_input::Status::Disabled => theme.colors.border_subtle,
    };

    text_input::Style {
        background: Background::Color(theme.colors.surface_base),
        border: rounded_border(border_color, 1.0, 6.0),
        icon: theme.colors.text_tertiary,
        placeholder: theme.colors.text_tertiary,
        value: theme.colors.text_primary,
        selection: theme.colors.surface_active,
    }
}

pub fn divider(theme: &AppTheme) -> container::Style {
    container::Style::default().background(theme.colors.border_subtle)
}

pub fn today_marker(theme: &AppTheme) -> container::Style {
    panel(
        theme.colors.surface_active,
        theme.colors.border_default,
        999.0,
    )
}

pub fn week_circle(theme: &AppTheme, state: WeekCircle) -> container::Style {
    match state {
        WeekCircle::Completed => container::Style::default()
            .background(theme.colors.success_fg)
            .border(rounded_border(Color::TRANSPARENT, 0.0, 999.0)),
        WeekCircle::TodayPending => container::Style::default()
            .background(theme.colors.surface_base)
            .border(rounded_border(theme.colors.border_default, 1.5, 999.0)),
        WeekCircle::Missed => container::Style::default()
            .background(theme.colors.surface_base)
            .border(rounded_border(theme.colors.border_subtle, 1.0, 999.0)),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BannerKind {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy)]
pub enum WeekCircle {
    Completed,
    TodayPending,
    Missed,
}

fn default_spacing() -> Spacing {
    Spacing {
        xs: 4.0,
        sm: 8.0,
        md: 12.0,
        lg: 16.0,
        xl: 20.0,
        xxl: 24.0,
        xxxl: 32.0,
        xxxxl: 48.0,
    }
}

fn default_typography() -> Typography {
    Typography {
        size_display: 24.0,
        size_heading_lg: 18.0,
        size_heading_md: 15.0,
        size_body_lg: 15.0,
        size_body_md: 14.0,
        size_body_sm: 13.0,
        size_caption: 12.0,
        size_mono_lg: 18.0,
        size_mono_md: 13.0,
    }
}

fn colors_for_preset(preset: ThemePreset) -> Colors {
    match preset {
        ThemePreset::Light => Colors {
            surface_base: hex("#FFFFFF"),
            surface_page: hex("#FAFAFA"),
            surface_hover: hex("#F4F4F5"),
            surface_active: hex("#E4E4E7"),
            surface_sidebar: hex("#FAFAFA"),
            text_primary: hex("#09090B"),
            text_secondary: hex("#71717A"),
            text_tertiary: hex("#A1A1AA"),
            text_inverse: hex("#FAFAFA"),
            border_subtle: hex("#E4E4E7"),
            border_default: hex("#E4E4E7"),
            border_strong: hex("#D4D4D8"),
            accent_primary: hex("#18181B"),
            success_bg: hex("#F0FDF4"),
            success_fg: hex("#16A34A"),
            warning_bg: hex("#FEFCE8"),
            warning_fg: hex("#CA8A04"),
            error_bg: hex("#FEF2F2"),
            error_fg: hex("#DC2626"),
        },
        ThemePreset::Dark => Colors {
            surface_base: hex("#18181B"),
            surface_page: hex("#09090B"),
            surface_hover: hex("#27272A"),
            surface_active: hex("#3F3F46"),
            surface_sidebar: hex("#111113"),
            text_primary: hex("#FAFAFA"),
            text_secondary: hex("#A1A1AA"),
            text_tertiary: hex("#71717A"),
            text_inverse: hex("#09090B"),
            border_subtle: hex("#27272A"),
            border_default: hex("#3F3F46"),
            border_strong: hex("#52525B"),
            accent_primary: hex("#E4E4E7"),
            success_bg: hex("#052E16"),
            success_fg: hex("#4ADE80"),
            warning_bg: hex("#422006"),
            warning_fg: hex("#FACC15"),
            error_bg: hex("#450A0A"),
            error_fg: hex("#F87171"),
        },
        ThemePreset::Dracula => Colors {
            surface_base: hex("#282A36"),
            surface_page: hex("#1E1F29"),
            surface_hover: hex("#343746"),
            surface_active: hex("#44475A"),
            surface_sidebar: hex("#21222C"),
            text_primary: hex("#F8F8F2"),
            text_secondary: hex("#BDC1CC"),
            text_tertiary: hex("#8A90A6"),
            text_inverse: hex("#1E1F29"),
            border_subtle: hex("#44475A"),
            border_default: hex("#5A5D77"),
            border_strong: hex("#70738F"),
            accent_primary: hex("#BD93F9"),
            success_bg: hex("#1E3A2E"),
            success_fg: hex("#50FA7B"),
            warning_bg: hex("#4A3A1E"),
            warning_fg: hex("#F1FA8C"),
            error_bg: hex("#4A1E2A"),
            error_fg: hex("#FF5555"),
        },
        ThemePreset::Nord => Colors {
            surface_base: hex("#2E3440"),
            surface_page: hex("#242933"),
            surface_hover: hex("#3B4252"),
            surface_active: hex("#434C5E"),
            surface_sidebar: hex("#2B303B"),
            text_primary: hex("#ECEFF4"),
            text_secondary: hex("#D8DEE9"),
            text_tertiary: hex("#A7B1C2"),
            text_inverse: hex("#2E3440"),
            border_subtle: hex("#434C5E"),
            border_default: hex("#4C566A"),
            border_strong: hex("#5E81AC"),
            accent_primary: hex("#88C0D0"),
            success_bg: hex("#1B4332"),
            success_fg: hex("#A3BE8C"),
            warning_bg: hex("#4C3B24"),
            warning_fg: hex("#EBCB8B"),
            error_bg: hex("#4C2C38"),
            error_fg: hex("#BF616A"),
        },
        ThemePreset::SolarizedDark => Colors {
            surface_base: hex("#073642"),
            surface_page: hex("#002B36"),
            surface_hover: hex("#0A3C4A"),
            surface_active: hex("#1B4D5A"),
            surface_sidebar: hex("#06313C"),
            text_primary: hex("#EEE8D5"),
            text_secondary: hex("#93A1A1"),
            text_tertiary: hex("#657B83"),
            text_inverse: hex("#002B36"),
            border_subtle: hex("#1B4D5A"),
            border_default: hex("#2A5A68"),
            border_strong: hex("#586E75"),
            accent_primary: hex("#268BD2"),
            success_bg: hex("#073B2B"),
            success_fg: hex("#859900"),
            warning_bg: hex("#3B3211"),
            warning_fg: hex("#B58900"),
            error_bg: hex("#432024"),
            error_fg: hex("#DC322F"),
        },
        ThemePreset::SolarizedLight => colors_from_palette(Theme::SolarizedLight.palette()),
        ThemePreset::GruvboxLight => colors_from_palette(Theme::GruvboxLight.palette()),
        ThemePreset::GruvboxDark => colors_from_palette(Theme::GruvboxDark.palette()),
        ThemePreset::CatppuccinLatte => {
            colors_from_palette(Theme::CatppuccinLatte.palette())
        }
        ThemePreset::CatppuccinFrappe => {
            colors_from_palette(Theme::CatppuccinFrappe.palette())
        }
        ThemePreset::CatppuccinMacchiato => {
            colors_from_palette(Theme::CatppuccinMacchiato.palette())
        }
        ThemePreset::CatppuccinMocha => {
            colors_from_palette(Theme::CatppuccinMocha.palette())
        }
        ThemePreset::TokyoNight => colors_from_palette(Theme::TokyoNight.palette()),
        ThemePreset::TokyoNightStorm => {
            colors_from_palette(Theme::TokyoNightStorm.palette())
        }
        ThemePreset::TokyoNightLight => {
            colors_from_palette(Theme::TokyoNightLight.palette())
        }
        ThemePreset::KanagawaWave => colors_from_palette(Theme::KanagawaWave.palette()),
        ThemePreset::KanagawaDragon => {
            colors_from_palette(Theme::KanagawaDragon.palette())
        }
        ThemePreset::KanagawaLotus => {
            colors_from_palette(Theme::KanagawaLotus.palette())
        }
        ThemePreset::Moonfly => colors_from_palette(Theme::Moonfly.palette()),
        ThemePreset::Nightfly => colors_from_palette(Theme::Nightfly.palette()),
        ThemePreset::Oxocarbon => colors_from_palette(Theme::Oxocarbon.palette()),
        ThemePreset::Ferra => colors_from_palette(Theme::Ferra.palette()),
    }
}

fn colors_from_palette(palette: iced::theme::Palette) -> Colors {
    let bg = palette.background;
    let text = palette.text;
    let primary = palette.primary;
    let success = palette.success;
    let danger = palette.danger;
    let warning = hex("#D97706");
    let is_dark = luminance(bg) < 0.5;

    if is_dark {
        Colors {
            surface_base: mix(bg, Color::WHITE, 0.06),
            surface_page: mix(bg, Color::BLACK, 0.08),
            surface_hover: mix(bg, Color::WHITE, 0.10),
            surface_active: mix(bg, Color::WHITE, 0.18),
            surface_sidebar: mix(bg, Color::BLACK, 0.16),
            text_primary: text,
            text_secondary: mix(text, bg, 0.35),
            text_tertiary: mix(text, bg, 0.55),
            text_inverse: bg,
            border_subtle: mix(bg, text, 0.18),
            border_default: mix(bg, text, 0.28),
            border_strong: mix(bg, text, 0.40),
            accent_primary: primary,
            success_bg: mix(bg, success, 0.20),
            success_fg: success,
            warning_bg: mix(bg, warning, 0.20),
            warning_fg: warning,
            error_bg: mix(bg, danger, 0.20),
            error_fg: danger,
        }
    } else {
        Colors {
            surface_base: mix(bg, Color::WHITE, 0.35),
            surface_page: mix(bg, Color::WHITE, 0.20),
            surface_hover: mix(bg, Color::BLACK, 0.04),
            surface_active: mix(bg, Color::BLACK, 0.10),
            surface_sidebar: mix(bg, Color::WHITE, 0.12),
            text_primary: text,
            text_secondary: mix(text, bg, 0.45),
            text_tertiary: mix(text, bg, 0.65),
            text_inverse: Color::WHITE,
            border_subtle: mix(bg, text, 0.10),
            border_default: mix(bg, text, 0.20),
            border_strong: mix(bg, text, 0.32),
            accent_primary: primary,
            success_bg: mix(bg, success, 0.12),
            success_fg: mix(success, Color::BLACK, 0.15),
            warning_bg: mix(bg, warning, 0.12),
            warning_fg: mix(warning, Color::BLACK, 0.15),
            error_bg: mix(bg, danger, 0.12),
            error_fg: mix(danger, Color::BLACK, 0.15),
        }
    }
}

fn mix(a: Color, b: Color, amount: f32) -> Color {
    let t = amount.clamp(0.0, 1.0);

    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

fn luminance(color: Color) -> f32 {
    0.2126 * color.r + 0.7152 * color.g + 0.0722 * color.b
}

fn panel(background: Color, border_color: Color, radius: f32) -> container::Style {
    container::Style::default()
        .background(background)
        .border(rounded_border(border_color, 1.0, radius))
        .shadow(iced::Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            offset: iced::Vector { x: 0.0, y: 1.0 },
            blur_radius: 2.0,
            ..Default::default()
        })
}

fn rounded_border(color: Color, width: f32, radius: f32) -> Border {
    Border {
        color,
        width,
        radius: radius.into(),
    }
}

fn hex(value: &str) -> Color {
    let value = value.trim_start_matches('#');
    let red = u8::from_str_radix(&value[0..2], 16).unwrap_or(0);
    let green = u8::from_str_radix(&value[2..4], 16).unwrap_or(0);
    let blue = u8::from_str_radix(&value[4..6], 16).unwrap_or(0);

    Color::from_rgb8(red, green, blue)
}
