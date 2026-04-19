#![windows_subsystem = "windows"]

mod components;
mod app_icon;
mod db;
mod export;
mod lucide;
mod models;
mod repository;
mod theme;
mod views;
mod icons;

use chrono::NaiveDate;
use iced::widget::{container, opaque, stack, text, text_input};
use iced::{Font, Length, Size, Subscription, Task};
use rfd::FileDialog;
use std::path::PathBuf;
use std::time::Duration as StdDuration;

use crate::components::{add_habit_popup, confirm_dialog, factory_reset_dialog};
use crate::models::{ExportData, HabitCategory, HabitWithStats};
use crate::repository::Database;
use crate::theme::{AppTheme, BannerKind, ThemePreset};

fn main() -> iced::Result {
    iced::application(
        "Minimal Habit Tracker",
        HabitTracker::update,
        HabitTracker::view,
    )
    .subscription(HabitTracker::subscription)
    .theme(|state| state.app_theme.iced_theme())
    .style(|state, _| state.app_theme.application_style())
    .font(include_bytes!("../assets/fonts/Inter-Regular.ttf").as_slice())
    .font(lucide_icons::LUCIDE_FONT_BYTES)
    .default_font(Font::with_name("Inter"))
    .window(iced::window::Settings {
        icon: Some(app_icon::window_icon()),
        size: Size::new(1100.0, 700.0),
        min_size: Some(Size::new(900.0, 600.0)),
        position: iced::window::Position::Centered,
        ..Default::default()
    })
    .run_with(HabitTracker::initialize)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Today,
    Weekly,
    Stats,
    Archived,
    Settings,
}

#[derive(Debug, Clone)]
pub struct BannerMessage {
    pub kind: BannerKind,
    pub text: String,
    pub dismissible: bool,
}

#[derive(Debug, Clone)]
enum Dialog {
    DeleteHabit {
        habit_id: String,
        habit_name: String,
        history_entries: u32,
    },
    ImportBackup {
        file_name: String,
        data: ExportData,
    },
    FactoryReset {
        habit_count: usize,
        log_entries: u32,
    },
}

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(View),
    ToggleHabit(String),
    ShowAddInput,
    HideAddInput,
    AddInputChanged(String),
    SelectAddCategory(HabitCategory),
    SelectAddIcon(String),
    AddIconSearchChanged(String),
    SubmitAddHabit,
    StartRename(String),
    RenameInputChanged(String),
    SubmitRename,
    CancelRename,
    ArchiveHabit(String),
    UnarchiveHabit(String),
    DeleteHabitRequest(String),
    ConfirmDeleteHabit,
    RequestExport,
    RequestImport,
    ConfirmImport,
    RequestFactoryReset,
    ConfirmFactoryReset,
    ConfirmFactoryResetWithBackup,
    CancelDialog,
    DismissNotice,
    SelectTheme(ThemePreset),
    SelectStatsHabit(Option<String>),
    SelectStatsRange(i64),
    ChartAnimationTick,
}

struct HabitTracker {
    current_view: View,
    db: Option<Database>,
    habits: Vec<HabitWithStats>,
    archived_habits: Vec<HabitWithStats>,
    today: NaiveDate,
    show_add_input: bool,
    add_habit: AddHabitDraft,
    editing_habit_id: Option<String>,
    rename_input_value: String,
    active_dialog: Option<Dialog>,
    notice: Option<BannerMessage>,
    rollback_warning: Option<String>,
    checkins_blocked: bool,
    fatal_error: Option<String>,
    app_theme: AppTheme,
    stats_selected_habit: Option<String>,
    stats_range_days: i64,
    stats_chart_reveal_progress: f32,
}

impl HabitTracker {
    fn initialize() -> (Self, Task<Message>) {
        let today = chrono::Local::now().date_naive();
        let app_theme = AppTheme::light();

        let mut app = Self {
            current_view: View::Today,
            db: None,
            habits: Vec::new(),
            archived_habits: Vec::new(),
            today,
            show_add_input: false,
            add_habit: AddHabitDraft::new(),
            editing_habit_id: None,
            rename_input_value: String::new(),
            active_dialog: None,
            notice: None,
            rollback_warning: None,
            checkins_blocked: false,
            fatal_error: None,
            app_theme,
            stats_selected_habit: None,
            stats_range_days: 14,
            stats_chart_reveal_progress: 0.0,
        };

        match Database::new() {
            Ok(db) => {
                match db.detect_date_rollback(today) {
                    Ok(Some(last_used)) => {
                        app.checkins_blocked = true;
                        app.rollback_warning = Some(format!(
                            "Check-ins are blocked because the last recorded app date ({}) is ahead of today ({}). Fix your system clock before tracking today.",
                            last_used.format("%B %-d, %Y"),
                            today.format("%B %-d, %Y"),
                        ));
                    }
                    Ok(None) => {}
                    Err(error) => {
                        app.notice = Some(BannerMessage {
                            kind: BannerKind::Error,
                            text: format!("Could not validate app date: {error}"),
                            dismissible: true,
                        });
                    }
                }

                if let Ok(Some(preset)) = db.get_theme_preset() {
                    app.app_theme = AppTheme::from_preset(preset);
                }

                app.db = Some(db);

                if let Err(error) = app.reload_habits() {
                    app.fatal_error = Some(error);
                }
            }
            Err(error) => {
                app.fatal_error = Some(format!("Failed to open the local database: {error}"));
            }
        }

        (app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NavigateTo(view) => {
                self.current_view = view;
                self.show_add_input = false;
                self.editing_habit_id = None;
                self.reset_add_habit_draft();
                self.rename_input_value.clear();

                if view == View::Stats {
                    self.stats_chart_reveal_progress = 0.0;
                }
            }
            Message::ToggleHabit(habit_id) => {
                if self.checkins_blocked {
                    self.notice = Some(BannerMessage {
                        kind: BannerKind::Warning,
                        text: "Check-ins are blocked until your system date is corrected."
                            .to_string(),
                        dismissible: true,
                    });
                } else if let Some(db) = &self.db {
                    if let Err(error) = db.toggle_day(&habit_id, self.today) {
                        self.notice =
                            Some(error_banner(format!("Could not update habit: {error}")));
                    } else if let Err(error) = self.reload_habits() {
                        self.notice = Some(error_banner(error));
                    }
                }
            }
            Message::ShowAddInput => {
                self.show_add_input = true;
                self.editing_habit_id = None;
                self.reset_add_habit_draft();

                return Task::batch([
                    text_input::focus("add-habit-name"),
                    text_input::move_cursor_to_end("add-habit-name"),
                ]);
            }
            Message::HideAddInput => {
                self.show_add_input = false;
                self.reset_add_habit_draft();
            }
            Message::AddInputChanged(value) => {
                self.add_habit.name = value;
            }
            Message::SelectAddCategory(category) => {
                let previous_default = self.add_habit.category.icon_name().to_string();

                if self.add_habit.icon == previous_default {
                    self.add_habit.icon = category.icon_name().to_string();
                }

                self.add_habit.category = category;
            }
            Message::SelectAddIcon(icon) => {
                self.add_habit.icon = icon;
            }
            Message::AddIconSearchChanged(value) => {
                self.add_habit.icon_search = value;
            }
            Message::SubmitAddHabit => {
                let name = self.add_habit.name.trim();

                if let Err(error) = validate_name(name) {
                    self.notice = Some(error_banner(error));
                } else if let Some(db) = &self.db {
                    if let Err(error) = db.add_habit(
                        name,
                        self.today,
                        self.add_habit.category,
                        &self.add_habit.icon,
                    ) {
                        self.notice =
                            Some(error_banner(format!("Could not create habit: {error}")));
                    } else {
                        self.show_add_input = false;
                        self.reset_add_habit_draft();

                        if let Err(error) = self.reload_habits() {
                            self.notice = Some(error_banner(error));
                        }
                    }
                }
            }
            Message::StartRename(habit_id) => {
                self.show_add_input = false;
                self.reset_add_habit_draft();
                self.editing_habit_id = Some(habit_id.clone());
                self.rename_input_value = self
                    .habits
                    .iter()
                    .chain(self.archived_habits.iter())
                    .find(|habit| habit.habit.id == habit_id)
                    .map(|habit| habit.habit.name.clone())
                    .unwrap_or_default();

                return Task::batch([
                    text_input::focus(format!("rename-{habit_id}")),
                    text_input::move_cursor_to_end(format!("rename-{habit_id}")),
                ]);
            }
            Message::RenameInputChanged(value) => {
                self.rename_input_value = value;
            }
            Message::SubmitRename => {
                if let Some(habit_id) = self.editing_habit_id.clone() {
                    let new_name = self.rename_input_value.trim();

                    if let Err(error) = validate_name(new_name) {
                        self.notice = Some(error_banner(error));
                    } else if let Some(db) = &self.db {
                        if let Err(error) = db.rename_habit(&habit_id, new_name) {
                            self.notice =
                                Some(error_banner(format!("Could not rename habit: {error}")));
                        } else {
                            self.editing_habit_id = None;
                            self.rename_input_value.clear();

                            if let Err(error) = self.reload_habits() {
                                self.notice = Some(error_banner(error));
                            }
                        }
                    }
                }
            }
            Message::CancelRename => {
                self.editing_habit_id = None;
                self.rename_input_value.clear();
            }
            Message::ArchiveHabit(habit_id) => {
                if let Some(db) = &self.db {
                    if let Err(error) = db.archive_habit(&habit_id) {
                        self.notice =
                            Some(error_banner(format!("Could not archive habit: {error}")));
                    } else {
                        if self.editing_habit_id.as_deref() == Some(habit_id.as_str()) {
                            self.editing_habit_id = None;
                            self.rename_input_value.clear();
                        }

                        if let Err(error) = self.reload_habits() {
                            self.notice = Some(error_banner(error));
                        } else {
                            self.notice = Some(BannerMessage {
                                kind: BannerKind::Success,
                                text: "Habit archived.".to_string(),
                                dismissible: true,
                            });
                        }
                    }
                }
            }
            Message::UnarchiveHabit(habit_id) => {
                if let Some(db) = &self.db {
                    if let Err(error) = db.unarchive_habit(&habit_id) {
                        self.notice =
                            Some(error_banner(format!("Could not unarchive habit: {error}")));
                    } else if let Err(error) = self.reload_habits() {
                        self.notice = Some(error_banner(error));
                    } else {
                        self.notice = Some(BannerMessage {
                            kind: BannerKind::Success,
                            text: "Habit restored to active habits.".to_string(),
                            dismissible: true,
                        });
                    }
                }
            }
            Message::DeleteHabitRequest(habit_id) => {
                if let Some(habit) = self
                    .habits
                    .iter()
                    .chain(self.archived_habits.iter())
                    .find(|habit| habit.habit.id == habit_id)
                {
                    self.active_dialog = Some(Dialog::DeleteHabit {
                        habit_id: habit.habit.id.clone(),
                        habit_name: habit.habit.name.clone(),
                        history_entries: habit.total_logged_days,
                    });
                }
            }
            Message::ConfirmDeleteHabit => {
                if let Some(Dialog::DeleteHabit { habit_id, .. }) = self.active_dialog.clone() {
                    if let Some(db) = &self.db {
                        if let Err(error) = db.delete_habit(&habit_id) {
                            self.notice =
                                Some(error_banner(format!("Could not delete habit: {error}")));
                        } else {
                            self.active_dialog = None;

                            if let Err(error) = self.reload_habits() {
                                self.notice = Some(error_banner(error));
                            }
                        }
                    }
                }
            }
            Message::RequestExport => {
                if let Some(db) = &self.db {
                    if let Some(path) = FileDialog::new()
                        .set_title("Export Minimal Habit Tracker data")
                        .add_filter("JSON", &["json"])
                        .set_file_name(&format!(
                            "habit-tracker-backup-{}.json",
                            self.today.format("%Y-%m-%d")
                        ))
                        .save_file()
                    {
                        match db.export_data() {
                            Ok(data) => match export::write_export(&path, &data) {
                                Ok(()) => {
                                    self.notice = Some(BannerMessage {
                                        kind: BannerKind::Success,
                                        text: format!("Backup exported to {}", path.display()),
                                        dismissible: true,
                                    });
                                }
                                Err(error) => {
                                    self.notice = Some(error_banner(error));
                                }
                            },
                            Err(error) => {
                                self.notice = Some(error_banner(format!(
                                    "Could not read local data for export: {error}"
                                )));
                            }
                        }
                    }
                }
            }
            Message::RequestImport => {
                if let Some(path) = FileDialog::new()
                    .set_title("Import Minimal Habit Tracker data")
                    .add_filter("JSON", &["json"])
                    .pick_file()
                {
                    match export::load_export(&path) {
                        Ok(data) => {
                            self.active_dialog = Some(Dialog::ImportBackup {
                                file_name: file_name(&path),
                                data,
                            });
                        }
                        Err(error) => {
                            self.notice = Some(error_banner(error));
                        }
                    }
                }
            }
            Message::RequestFactoryReset => {
                let habit_count = self.habits.len() + self.archived_habits.len();
                let log_entries = self
                    .habits
                    .iter()
                    .chain(self.archived_habits.iter())
                    .map(|habit| habit.total_logged_days)
                    .sum();

                self.active_dialog = Some(Dialog::FactoryReset {
                    habit_count,
                    log_entries,
                });
            }
            Message::ConfirmImport => {
                if let Some(Dialog::ImportBackup { data, .. }) = self.active_dialog.clone() {
                    if let Some(db) = &mut self.db {
                        match db.import_data(&data) {
                            Ok(()) => {
                                if let Err(error) = db.touch_last_used_date(self.today) {
                                    self.notice = Some(error_banner(format!(
                                        "Imported data, but could not refresh app metadata: {error}"
                                    )));
                                } else if let Err(error) = self.reload_habits() {
                                    self.notice = Some(error_banner(error));
                                } else {
                                    self.notice = Some(BannerMessage {
                                        kind: BannerKind::Success,
                                        text: format!(
                                            "Imported {} habits and {} log entries.",
                                            data.habits.len(),
                                            data.daily_logs.len()
                                        ),
                                        dismissible: true,
                                    });
                                }

                                self.active_dialog = None;
                                self.show_add_input = false;
                                self.editing_habit_id = None;
                                self.reset_add_habit_draft();
                                self.rename_input_value.clear();
                            }
                            Err(error) => {
                                self.notice =
                                    Some(error_banner(format!("Could not import backup: {error}")));
                            }
                        }
                    }
                }
            }
            Message::ConfirmFactoryReset => {
                if matches!(self.active_dialog, Some(Dialog::FactoryReset { .. })) {
                    match self.run_factory_reset() {
                        Ok(()) => {
                            self.notice = Some(BannerMessage {
                                kind: BannerKind::Success,
                                text: "All local data has been reset.".to_string(),
                                dismissible: true,
                            });
                        }
                        Err(error) => {
                            self.notice = Some(error_banner(error));
                        }
                    }
                }
            }
            Message::ConfirmFactoryResetWithBackup => {
                if matches!(self.active_dialog, Some(Dialog::FactoryReset { .. })) {
                    let Some(path) = FileDialog::new()
                        .set_title("Backup before factory reset")
                        .add_filter("JSON", &["json"])
                        .set_file_name(&format!(
                            "habit-tracker-backup-{}.json",
                            self.today.format("%Y-%m-%d")
                        ))
                        .save_file()
                    else {
                        return Task::none();
                    };

                    if let Some(db) = &self.db {
                        match db.export_data() {
                            Ok(data) => {
                                if let Err(error) = export::write_export(&path, &data) {
                                    self.notice = Some(error_banner(error));
                                    return Task::none();
                                }
                            }
                            Err(error) => {
                                self.notice = Some(error_banner(format!(
                                    "Could not read local data for backup: {error}"
                                )));
                                return Task::none();
                            }
                        }
                    }

                    match self.run_factory_reset() {
                        Ok(()) => {
                            self.notice = Some(BannerMessage {
                                kind: BannerKind::Success,
                                text: format!(
                                    "Backup exported to {} and all local data has been reset.",
                                    path.display()
                                ),
                                dismissible: true,
                            });
                        }
                        Err(error) => {
                            self.notice = Some(error_banner(format!(
                                "{error} Backup was exported to {}.",
                                path.display()
                            )));
                        }
                    }
                }
            }
            Message::CancelDialog => {
                self.active_dialog = None;
            }
            Message::DismissNotice => {
                self.notice = None;
            }
            Message::SelectTheme(preset) => {
                self.app_theme = AppTheme::from_preset(preset);

                if let Some(db) = &self.db {
                    if let Err(error) = db.set_theme_preset(preset) {
                        self.notice = Some(error_banner(format!(
                            "Could not save theme selection: {error}"
                        )));
                    }
                }
            }
            Message::SelectStatsHabit(selection) => {
                self.stats_selected_habit = selection;
                self.stats_chart_reveal_progress = 0.0;
            }
            Message::SelectStatsRange(days) => {
                self.stats_range_days = days.clamp(7, 30);
                self.stats_chart_reveal_progress = 0.0;
            }
            Message::ChartAnimationTick => {
                if self.current_view == View::Stats && self.stats_chart_reveal_progress < 1.0 {
                    self.stats_chart_reveal_progress =
                        (self.stats_chart_reveal_progress + 0.08).min(1.0);
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        if let Some(error) = &self.fatal_error {
            return self.error_view(error);
        }

        let banner = self.current_banner();
        let sidebar = views::sidebar::view(self.current_view, &self.app_theme);
        let content = match self.current_view {
            View::Today => views::today::view(
                &self.habits,
                self.today,
                self.editing_habit_id.as_deref(),
                &self.rename_input_value,
                banner.clone(),
                self.checkins_blocked,
                &self.app_theme,
            ),
            View::Weekly => {
                views::weekly::view(&self.habits, self.today, banner.clone(), &self.app_theme)
            }
            View::Stats => views::stats::view(
                &self.habits,
                self.today,
                banner.clone(),
                &self.app_theme,
                &self.stats_selected_habit,
                self.stats_range_days,
                self.stats_chart_reveal_progress,
            ),
            View::Archived => views::archived::view(
                &self.archived_habits,
                self.today,
                banner.clone(),
                &self.app_theme,
            ),
            View::Settings => views::settings::view(banner, &self.app_theme, self.app_theme.preset),
        };

        let base: iced::Element<'_, Message> = iced::widget::row![sidebar, content]
            .height(Length::Fill)
            .width(Length::Fill)
            .into();

        let mut layers = stack![base];

        if self.show_add_input {
            layers = layers.push(opaque(self.add_habit_overlay()));
        }

        if let Some(dialog) = &self.active_dialog {
            layers = layers.push(opaque(self.dialog_view(dialog)));
        }

        layers.into()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.current_view == View::Stats && self.stats_chart_reveal_progress < 1.0 {
            iced::time::every(StdDuration::from_millis(16)).map(|_| Message::ChartAnimationTick)
        } else {
            Subscription::none()
        }
    }

    fn reload_habits(&mut self) -> Result<(), String> {
        if let Some(db) = &self.db {
            self.habits = db
                .get_all_habits_with_stats(self.today, false)
                .map_err(|error| format!("Could not load active habits: {error}"))?;
            self.archived_habits = db
                .get_all_habits_with_stats(self.today, true)
                .map_err(|error| format!("Could not load archived habits: {error}"))?;
        }

        Ok(())
    }

    fn reset_add_habit_draft(&mut self) {
        self.add_habit = AddHabitDraft::new();
    }

    fn current_banner(&self) -> Option<BannerMessage> {
        if let Some(message) = &self.rollback_warning {
            return Some(BannerMessage {
                kind: BannerKind::Warning,
                text: message.clone(),
                dismissible: false,
            });
        }

        self.notice.clone()
    }

    fn dialog_view(&self, dialog: &Dialog) -> iced::Element<'_, Message> {
        match dialog {
            Dialog::DeleteHabit {
                habit_name,
                history_entries,
                ..
            } => {
                let body = if *history_entries == 0 {
                    "This will permanently delete the habit. This cannot be undone.".to_string()
                } else if *history_entries == 1 {
                    "This will permanently delete the habit and 1 day of history. This cannot be undone.".to_string()
                } else {
                    format!(
                        "This will permanently delete the habit and {} days of history. This cannot be undone.",
                        history_entries
                    )
                };

                confirm_dialog::view(
                    &format!("Delete \"{}\"?", habit_name),
                    &body,
                    "Delete",
                    Message::ConfirmDeleteHabit,
                    true,
                    &self.app_theme,
                )
            }
            Dialog::ImportBackup { file_name, data } => confirm_dialog::view(
                "Import backup?",
                &format!(
                    "{} contains {} habits and {} log entries. Importing will replace your current local data.",
                    file_name,
                    data.habits.len(),
                    data.daily_logs.len()
                ),
                "Import",
                Message::ConfirmImport,
                false,
                &self.app_theme,
            ),
            Dialog::FactoryReset {
                habit_count,
                log_entries,
            } => {
                let habit_label = if *habit_count == 1 { "habit" } else { "habits" };
                let log_label = if *log_entries == 1 {
                    "log entry"
                } else {
                    "log entries"
                };

                factory_reset_dialog::view(
                    "Factory reset all data?",
                    &format!(
                        "This will permanently remove {} {} and {} {} from this device. This cannot be undone.",
                        habit_count, habit_label, log_entries, log_label
                    ),
                    &self.app_theme,
                )
            }
        }
    }

    fn add_habit_overlay(&self) -> iced::Element<'_, Message> {
        let form = add_habit_popup::view("add-habit-name".into(), &self.add_habit, &self.app_theme);

        container(container(form).max_width(980))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| crate::theme::scrim())
            .into()
    }

    fn run_factory_reset(&mut self) -> Result<(), String> {
        {
            let Some(db) = &mut self.db else {
                return Err("Database is not available.".to_string());
            };

            db.factory_reset()
                .map_err(|error| format!("Could not reset local data: {error}"))?;
            db.touch_last_used_date(self.today).map_err(|error| {
                format!("Data reset completed, but could not refresh app metadata: {error}")
            })?;
            db.set_theme_preset(self.app_theme.preset).map_err(|error| {
                format!("Data reset completed, but could not save theme preference: {error}")
            })?;
        }

        self.reload_habits()?;

        self.active_dialog = None;
        self.show_add_input = false;
        self.editing_habit_id = None;
        self.reset_add_habit_draft();
        self.rename_input_value.clear();
        self.stats_selected_habit = None;
        self.checkins_blocked = false;
        self.rollback_warning = None;

        Ok(())
    }

    fn error_view(&self, error: &str) -> iced::Element<'_, Message> {
        let error_text = error.to_string();
        let database_path = Database::database_path().display().to_string();

        container(
            container(
                iced::widget::column![
                    text("The app could not start")
                        .size(self.app_theme.typography.size_heading_lg)
                        .color(self.app_theme.colors.text_primary),
                    text(error_text)
                        .size(self.app_theme.typography.size_body_md)
                        .color(self.app_theme.colors.error_fg),
                    text(format!("Database location: {database_path}"))
                        .size(self.app_theme.typography.size_body_sm)
                        .color(self.app_theme.colors.text_secondary),
                ]
                .spacing(self.app_theme.spacing.md),
            )
            .padding(self.app_theme.spacing.xxl)
            .max_width(640)
            .style(|_| crate::theme::section_card(&self.app_theme)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
}

fn validate_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Habit names cannot be empty.".to_string());
    }

    if name.len() > 100 {
        return Err("Habit names must stay under 100 characters.".to_string());
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct AddHabitDraft {
    pub name: String,
    pub category: HabitCategory,
    pub icon: String,
    pub icon_search: String,
}

impl AddHabitDraft {
    fn new() -> Self {
        Self {
            name: String::new(),
            category: HabitCategory::General,
            icon: HabitCategory::General.icon_name().to_string(),
            icon_search: String::new(),
        }
    }
}

fn error_banner(text: String) -> BannerMessage {
    BannerMessage {
        kind: BannerKind::Error,
        text,
        dismissible: true,
    }
}

fn file_name(path: &PathBuf) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .unwrap_or_else(|| path.display().to_string())
}
