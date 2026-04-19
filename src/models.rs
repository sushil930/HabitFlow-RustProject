use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum HabitCategory {
    #[default]
    General,
    Health,
    Fitness,
    Mindfulness,
    Learning,
    Productivity,
    Work,
    Home,
    Social,
    Finance,
    Creative,
}

impl HabitCategory {
    pub const ALL: [Self; 11] = [
        Self::General,
        Self::Health,
        Self::Fitness,
        Self::Mindfulness,
        Self::Learning,
        Self::Productivity,
        Self::Work,
        Self::Home,
        Self::Social,
        Self::Finance,
        Self::Creative,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Health => "Health",
            Self::Fitness => "Fitness",
            Self::Mindfulness => "Mindfulness",
            Self::Learning => "Learning",
            Self::Productivity => "Productivity",
            Self::Work => "Work",
            Self::Home => "Home",
            Self::Social => "Social",
            Self::Finance => "Finance",
            Self::Creative => "Creative",
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::General => "general",
            Self::Health => "health",
            Self::Fitness => "fitness",
            Self::Mindfulness => "mindfulness",
            Self::Learning => "learning",
            Self::Productivity => "productivity",
            Self::Work => "work",
            Self::Home => "home",
            Self::Social => "social",
            Self::Finance => "finance",
            Self::Creative => "creative",
        }
    }

    pub fn icon_name(self) -> &'static str {
        match self {
            Self::General => "sparkles",
            Self::Health => "heart-pulse",
            Self::Fitness => "dumbbell",
            Self::Mindfulness => "brain-circuit",
            Self::Learning => "book-open",
            Self::Productivity => "clipboard-list",
            Self::Work => "briefcase-business",
            Self::Home => "home",
            Self::Social => "users",
            Self::Finance => "wallet",
            Self::Creative => "palette",
        }
    }

    pub fn from_db_value(value: &str) -> Self {
        Self::from_str(value).unwrap_or_default()
    }
}

impl fmt::Display for HabitCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

impl FromStr for HabitCategory {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = value.trim().to_ascii_lowercase().replace([' ', '-'], "_");

        Ok(match normalized.as_str() {
            "general" => Self::General,
            "health" => Self::Health,
            "fitness" => Self::Fitness,
            "mindfulness" => Self::Mindfulness,
            "learning" => Self::Learning,
            "productivity" => Self::Productivity,
            "work" => Self::Work,
            "home" => Self::Home,
            "social" => Self::Social,
            "finance" => Self::Finance,
            "creative" => Self::Creative,
            _ => Self::General,
        })
    }
}

fn default_icon_name() -> String {
    HabitCategory::General.icon_name().to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub id: String,
    pub name: String,
    pub created_date: NaiveDate,
    pub sort_order: i32,
    #[serde(default)]
    pub category: HabitCategory,
    #[serde(default = "default_icon_name")]
    pub icon: String,
    #[serde(default)]
    pub archived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DailyLog {
    pub habit_id: String,
    pub date: NaiveDate,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub struct HabitWithStats {
    pub habit: Habit,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub total_completions: u32,
    pub total_logged_days: u32,
    pub last_30_days_rate: f32,
    pub completed_today: bool,
    pub last_completed_date: Option<NaiveDate>,
    pub week_logs: Vec<DailyLog>,
    pub history_logs: Vec<DailyLog>,
}

impl HabitWithStats {
    pub fn is_completed_on(&self, date: NaiveDate) -> bool {
        self.week_logs
            .iter()
            .find(|log| log.date == date)
            .map(|log| log.completed)
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    pub version: u32,
    pub exported_at: String,
    pub habits: Vec<Habit>,
    pub daily_logs: Vec<DailyLog>,
}
