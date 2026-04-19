use chrono::{Datelike, Duration, NaiveDate};
use rusqlite::{params, Connection, OptionalExtension, Result as SqlResult};
use std::path::{Path, PathBuf};

use crate::lucide::{sanitize_icon_name, DEFAULT_ICON_NAME};
use crate::models::{DailyLog, ExportData, Habit, HabitCategory, HabitWithStats};
use crate::theme::ThemePreset;
use crate::APP_DATA_DIR_NAME;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> SqlResult<Self> {
        let db_path = Self::database_path();

        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(io_to_sql_error)?;
        }

        Self::open(db_path)
    }

    pub fn database_path() -> PathBuf {
        let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(APP_DATA_DIR_NAME);
        path.push("data.db");
        path
    }

    pub fn detect_date_rollback(&self, today: NaiveDate) -> SqlResult<Option<NaiveDate>> {
        let stored: Option<String> = self
            .conn
            .query_row(
                "SELECT value FROM app_meta WHERE key = 'last_used_date'",
                [],
                |row| row.get(0),
            )
            .optional()?;

        if let Some(value) = stored {
            let last_used = parse_date(&value)?;

            if last_used > today {
                return Ok(Some(last_used));
            }
        }

        self.touch_last_used_date(today)?;

        Ok(None)
    }

    pub fn touch_last_used_date(&self, today: NaiveDate) -> SqlResult<()> {
        self.conn.execute(
            "
            INSERT INTO app_meta (key, value) VALUES ('last_used_date', ?1)
            ON CONFLICT(key) DO UPDATE SET value = excluded.value
            ",
            params![today.to_string()],
        )?;

        Ok(())
    }

    pub fn get_theme_preset(&self) -> SqlResult<Option<ThemePreset>> {
        let stored: Option<String> = self
            .conn
            .query_row(
                "SELECT value FROM app_meta WHERE key = 'theme_preset'",
                [],
                |row| row.get(0),
            )
            .optional()?;

        Ok(stored.and_then(|value| ThemePreset::from_storage_key(&value)))
    }

    pub fn set_theme_preset(&self, preset: ThemePreset) -> SqlResult<()> {
        self.conn.execute(
            "
            INSERT INTO app_meta (key, value) VALUES ('theme_preset', ?1)
            ON CONFLICT(key) DO UPDATE SET value = excluded.value
            ",
            params![preset.storage_key()],
        )?;

        Ok(())
    }

    pub fn add_habit(
        &self,
        name: &str,
        today: NaiveDate,
        category: HabitCategory,
        icon: &str,
    ) -> SqlResult<Habit> {
        self.ensure_habit_metadata_schema()?;

        let id = uuid::Uuid::new_v4().to_string();
        let next_sort_order: i32 = self.conn.query_row(
            "SELECT COALESCE(MAX(sort_order), -1) + 1 FROM habits",
            [],
            |row| row.get(0),
        )?;

        let icon_name = sanitize_icon_name(icon);

        self.conn.execute(
            "INSERT INTO habits (id, name, created_date, sort_order, category, icon, archived) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)",
            params![
                id,
                name.trim(),
                today.to_string(),
                next_sort_order,
                category.key(),
                icon_name,
            ],
        )?;

        Ok(Habit {
            id,
            name: name.trim().to_string(),
            created_date: today,
            sort_order: next_sort_order,
            category,
            icon: icon_name,
            archived: false,
        })
    }

    pub fn rename_habit(&self, id: &str, new_name: &str) -> SqlResult<()> {
        self.conn.execute(
            "UPDATE habits SET name = ?1 WHERE id = ?2",
            params![new_name.trim(), id],
        )?;

        Ok(())
    }

    pub fn archive_habit(&self, id: &str) -> SqlResult<()> {
        self.conn
            .execute("UPDATE habits SET archived = 1 WHERE id = ?1", params![id])?;

        Ok(())
    }

    pub fn unarchive_habit(&self, id: &str) -> SqlResult<()> {
        self.conn
            .execute("UPDATE habits SET archived = 0 WHERE id = ?1", params![id])?;

        Ok(())
    }

    pub fn delete_habit(&self, id: &str) -> SqlResult<()> {
        self.conn
            .execute("DELETE FROM habits WHERE id = ?1", params![id])?;

        Ok(())
    }

    pub fn toggle_day(&self, habit_id: &str, date: NaiveDate) -> SqlResult<bool> {
        let existing: Option<i64> = self
            .conn
            .query_row(
                "SELECT completed FROM daily_logs WHERE habit_id = ?1 AND date = ?2",
                params![habit_id, date.to_string()],
                |row| row.get(0),
            )
            .optional()?;

        let new_state = match existing {
            Some(value) => value == 0,
            None => true,
        };

        self.conn.execute(
            "
            INSERT INTO daily_logs (habit_id, date, completed)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(habit_id, date) DO UPDATE SET completed = excluded.completed
            ",
            params![habit_id, date.to_string(), i64::from(new_state)],
        )?;

        Ok(new_state)
    }

    pub fn get_all_habits(&self, archived: bool) -> SqlResult<Vec<Habit>> {
        self.ensure_habit_metadata_schema().ok();

        let has_category = self.habits_table_has_column("category")?;
        let has_icon = self.habits_table_has_column("icon")?;

        if has_category && has_icon {
            let mut stmt = self.conn.prepare(
                "SELECT id, name, created_date, sort_order, category, icon, archived FROM habits WHERE archived = ?1 ORDER BY sort_order, created_date",
            )?;

            let habits = stmt.query_map(params![i64::from(archived)], |row| {
                Ok(Habit {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_date: parse_date(&row.get::<_, String>(2)?)?,
                    sort_order: row.get(3)?,
                    category: HabitCategory::from_db_value(&row.get::<_, String>(4)?),
                    icon: sanitize_icon_name(&row.get::<_, String>(5)?),
                    archived: row.get::<_, i64>(6)? != 0,
                })
            })?;

            return habits.collect();
        }

        let mut stmt = self.conn.prepare(
            "SELECT id, name, created_date, sort_order, archived FROM habits WHERE archived = ?1 ORDER BY sort_order, created_date",
        )?;

        let habits = stmt.query_map(params![i64::from(archived)], |row| {
            Ok(Habit {
                id: row.get(0)?,
                name: row.get(1)?,
                created_date: parse_date(&row.get::<_, String>(2)?)?,
                sort_order: row.get(3)?,
                category: HabitCategory::General,
                icon: DEFAULT_ICON_NAME.to_string(),
                archived: row.get::<_, i64>(4)? != 0,
            })
        })?;

        habits.collect()
    }

    pub fn get_logs_between(
        &self,
        habit_id: &str,
        start: NaiveDate,
        end: NaiveDate,
    ) -> SqlResult<Vec<DailyLog>> {
        let mut stmt = self.conn.prepare(
            "
            SELECT habit_id, date, completed
            FROM daily_logs
            WHERE habit_id = ?1 AND date >= ?2 AND date <= ?3
            ORDER BY date
            ",
        )?;

        let logs = stmt.query_map(
            params![habit_id, start.to_string(), end.to_string()],
            |row| {
                Ok(DailyLog {
                    habit_id: row.get(0)?,
                    date: parse_date(&row.get::<_, String>(1)?)?,
                    completed: row.get::<_, i64>(2)? != 0,
                })
            },
        )?;

        logs.collect()
    }

    pub fn calculate_streaks(&self, habit_id: &str, today: NaiveDate) -> SqlResult<(u32, u32)> {
        let mut stmt = self.conn.prepare(
            "
            SELECT date
            FROM daily_logs
            WHERE habit_id = ?1 AND completed = 1
            ORDER BY date ASC
            ",
        )?;

        let dates: Vec<NaiveDate> = stmt
            .query_map(params![habit_id], |row| {
                parse_date(&row.get::<_, String>(0)?)
            })?
            .collect::<SqlResult<Vec<_>>>()?;

        if dates.is_empty() {
            return Ok((0, 0));
        }

        let longest_streak = longest_streak(&dates);
        let last_completed = *dates.last().unwrap();
        let current_anchor = if last_completed == today {
            today
        } else if last_completed == today - Duration::days(1) {
            today - Duration::days(1)
        } else {
            return Ok((0, longest_streak));
        };

        let mut current_streak = 0_u32;
        let mut expected = current_anchor;

        for date in dates.iter().rev() {
            if *date == expected {
                current_streak += 1;
                expected -= Duration::days(1);
            } else if *date < expected {
                break;
            }
        }

        Ok((current_streak, longest_streak.max(current_streak)))
    }

    pub fn get_all_habits_with_stats(
        &self,
        today: NaiveDate,
        archived: bool,
    ) -> SqlResult<Vec<HabitWithStats>> {
        let habits = self.get_all_habits(archived)?;
        let week_start = start_of_week(today);
        let week_end = week_start + Duration::days(6);
        let thirty_days_ago = today - Duration::days(29);
        let sixty_days_ago = today - Duration::days(59);
        let mut result = Vec::with_capacity(habits.len());

        for habit in habits {
            let week_logs = self.get_logs_between(&habit.id, week_start, week_end)?;
            let history_logs = self.get_logs_between(&habit.id, sixty_days_ago, today)?;
            let (current_streak, longest_streak) = self.calculate_streaks(&habit.id, today)?;

            let total_completions: u32 = self.conn.query_row(
                "SELECT COUNT(*) FROM daily_logs WHERE habit_id = ?1 AND completed = 1",
                params![habit.id.as_str()],
                |row| row.get(0),
            )?;

            let total_logged_days: u32 = self.conn.query_row(
                "SELECT COUNT(*) FROM daily_logs WHERE habit_id = ?1",
                params![habit.id.as_str()],
                |row| row.get(0),
            )?;

            let completed_last_30: u32 = self.conn.query_row(
                "
                SELECT COUNT(*) FROM daily_logs
                WHERE habit_id = ?1 AND completed = 1 AND date >= ?2
                ",
                params![habit.id.as_str(), thirty_days_ago.to_string()],
                |row| row.get(0),
            )?;

            let tracking_days = ((today - habit.created_date).num_days() + 1).clamp(1, 30) as f32;
            let last_30_days_rate = (completed_last_30 as f32 / tracking_days) * 100.0;

            let completed_today = week_logs
                .iter()
                .find(|log| log.date == today)
                .map(|log| log.completed)
                .unwrap_or(false);

            let last_completed_date: Option<NaiveDate> = self
                .conn
                .query_row(
                    "
                    SELECT date FROM daily_logs
                    WHERE habit_id = ?1 AND completed = 1
                    ORDER BY date DESC
                    LIMIT 1
                    ",
                    params![habit.id.as_str()],
                    |row| parse_date(&row.get::<_, String>(0)?),
                )
                .optional()?;

            result.push(HabitWithStats {
                habit,
                current_streak,
                longest_streak,
                total_completions,
                total_logged_days,
                last_30_days_rate,
                completed_today,
                last_completed_date,
                week_logs,
                history_logs,
            });
        }

        Ok(result)
    }

    pub fn export_data(&self) -> SqlResult<ExportData> {
        let active_habits = self.get_all_habits(false)?;
        let archived_habits = self.get_all_habits(true)?;
        let habits = active_habits.into_iter().chain(archived_habits).collect();
        let mut stmt = self
            .conn
            .prepare("SELECT habit_id, date, completed FROM daily_logs ORDER BY date, habit_id")?;

        let daily_logs = stmt
            .query_map([], |row| {
                Ok(DailyLog {
                    habit_id: row.get(0)?,
                    date: parse_date(&row.get::<_, String>(1)?)?,
                    completed: row.get::<_, i64>(2)? != 0,
                })
            })?
            .collect::<SqlResult<Vec<_>>>()?;

        Ok(ExportData {
            version: 2,
            exported_at: chrono::Local::now().to_rfc3339(),
            habits,
            daily_logs,
        })
    }

    pub fn import_data(&mut self, data: &ExportData) -> SqlResult<()> {
        let tx = self.conn.transaction()?;

        tx.execute_batch("DELETE FROM daily_logs; DELETE FROM habits;")?;

        for habit in &data.habits {
            tx.execute(
                "INSERT INTO habits (id, name, created_date, sort_order, category, icon, archived) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    habit.id,
                    habit.name,
                    habit.created_date.to_string(),
                    habit.sort_order,
                    habit.category.key(),
                    sanitize_icon_name(&habit.icon),
                    i64::from(habit.archived)
                ],
            )?;
        }

        for log in &data.daily_logs {
            tx.execute(
                "INSERT INTO daily_logs (habit_id, date, completed) VALUES (?1, ?2, ?3)",
                params![log.habit_id, log.date.to_string(), i64::from(log.completed)],
            )?;
        }

        tx.commit()?;

        Ok(())
    }

    pub fn factory_reset(&mut self) -> SqlResult<()> {
        let tx = self.conn.transaction()?;

        tx.execute_batch("DELETE FROM daily_logs; DELETE FROM habits; DELETE FROM app_meta;")?;

        tx.commit()?;

        Ok(())
    }

    fn open(path: impl AsRef<Path>) -> SqlResult<Self> {
        let conn = Connection::open(path)?;
        Self::from_connection(conn)
    }

    fn from_connection(conn: Connection) -> SqlResult<Self> {
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Self { conn };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> SqlResult<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS habits (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_date TEXT NOT NULL,
                sort_order INTEGER NOT NULL DEFAULT 0,
                category TEXT NOT NULL DEFAULT 'general',
                icon TEXT NOT NULL DEFAULT 'sparkles',
                archived INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS daily_logs (
                habit_id TEXT NOT NULL,
                date TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (habit_id, date),
                FOREIGN KEY (habit_id) REFERENCES habits(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS app_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            ",
        )?;

        self.ensure_habits_column("category", "TEXT NOT NULL DEFAULT 'general'")?;
        self.ensure_habits_column("icon", &format!("TEXT NOT NULL DEFAULT '{}'", DEFAULT_ICON_NAME))?;
        self.ensure_habits_column("archived", "INTEGER NOT NULL DEFAULT 0")?;

        Ok(())
    }

    fn ensure_habits_column(&self, column: &str, definition: &str) -> SqlResult<()> {
        let mut stmt = self.conn.prepare("PRAGMA table_info(habits)")?;
        let columns = stmt.query_map([], |row| row.get::<_, String>(1))?;

        for existing in columns {
            if existing? == column {
                return Ok(());
            }
        }

        self.conn.execute(
            &format!("ALTER TABLE habits ADD COLUMN {column} {definition}"),
            [],
        )?;

        Ok(())
    }

    fn habits_table_has_column(&self, column: &str) -> SqlResult<bool> {
        let mut stmt = self.conn.prepare("PRAGMA table_info(habits)")?;
        let columns = stmt.query_map([], |row| row.get::<_, String>(1))?;

        for existing in columns {
            if existing? == column {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn ensure_habit_metadata_schema(&self) -> SqlResult<()> {
        self.ensure_habits_column("category", "TEXT NOT NULL DEFAULT 'general'")?;
        self.ensure_habits_column("icon", &format!("TEXT NOT NULL DEFAULT '{}'", DEFAULT_ICON_NAME))?;
        self.ensure_habits_column("archived", "INTEGER NOT NULL DEFAULT 0")?;

        Ok(())
    }

    #[cfg(test)]
    fn in_memory() -> SqlResult<Self> {
        Self::from_connection(Connection::open_in_memory()?)
    }
}

fn parse_date(value: &str) -> SqlResult<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))
}

fn start_of_week(today: NaiveDate) -> NaiveDate {
    today - Duration::days(today.weekday().num_days_from_monday() as i64)
}

fn longest_streak(dates: &[NaiveDate]) -> u32 {
    let mut longest = 1_u32;
    let mut current = 1_u32;

    for pair in dates.windows(2) {
        if pair[1] - pair[0] == Duration::days(1) {
            current += 1;
            longest = longest.max(current);
        } else {
            current = 1;
        }
    }

    longest
}

fn io_to_sql_error(error: std::io::Error) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(error))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streak_uses_yesterday_when_today_is_unchecked() {
        let db = Database::in_memory().expect("db");
        let today = NaiveDate::from_ymd_opt(2026, 4, 7).unwrap();
        let habit = db
            .add_habit("Read", today, HabitCategory::General, "sparkles")
            .expect("habit");

        db.toggle_day(&habit.id, today - Duration::days(1)).unwrap();
        db.toggle_day(&habit.id, today - Duration::days(2)).unwrap();
        db.toggle_day(&habit.id, today - Duration::days(3)).unwrap();

        let (current, longest) = db.calculate_streaks(&habit.id, today).unwrap();

        assert_eq!(current, 3);
        assert_eq!(longest, 3);
    }

    #[test]
    fn missed_day_resets_current_streak() {
        let db = Database::in_memory().expect("db");
        let today = NaiveDate::from_ymd_opt(2026, 4, 7).unwrap();
        let habit = db
            .add_habit("Run", today, HabitCategory::Fitness, "dumbbell")
            .expect("habit");

        db.toggle_day(&habit.id, today - Duration::days(1)).unwrap();
        db.toggle_day(&habit.id, today - Duration::days(3)).unwrap();

        let (current, longest) = db.calculate_streaks(&habit.id, today).unwrap();

        assert_eq!(current, 1);
        assert_eq!(longest, 1);
    }

    #[test]
    fn delete_cascades_history() {
        let db = Database::in_memory().expect("db");
        let today = NaiveDate::from_ymd_opt(2026, 4, 7).unwrap();
        let habit = db
            .add_habit("Walk", today, HabitCategory::Fitness, "dumbbell")
            .expect("habit");

        db.toggle_day(&habit.id, today).unwrap();
        db.delete_habit(&habit.id).unwrap();

        let exported = db.export_data().unwrap();

        assert!(exported.habits.is_empty());
        assert!(exported.daily_logs.is_empty());
    }

    #[test]
    fn factory_reset_clears_habits_logs_and_meta() {
        let mut db = Database::in_memory().expect("db");
        let today = NaiveDate::from_ymd_opt(2026, 4, 7).unwrap();
        let habit = db
            .add_habit("Reset me", today, HabitCategory::General, "sparkles")
            .expect("habit");

        db.toggle_day(&habit.id, today).unwrap();
        db.touch_last_used_date(today).unwrap();
        db.factory_reset().unwrap();

        let exported = db.export_data().unwrap();
        assert!(exported.habits.is_empty());
        assert!(exported.daily_logs.is_empty());

        let last_used: Option<String> = db
            .conn
            .query_row(
                "SELECT value FROM app_meta WHERE key = 'last_used_date'",
                [],
                |row| row.get(0),
            )
            .optional()
            .unwrap();

        assert!(last_used.is_none());
    }

    #[test]
    fn theme_preset_round_trips_through_app_meta() {
        let db = Database::in_memory().expect("db");

        assert_eq!(db.get_theme_preset().unwrap(), None);

        db.set_theme_preset(ThemePreset::TokyoNightStorm).unwrap();

        assert_eq!(db.get_theme_preset().unwrap(), Some(ThemePreset::TokyoNightStorm));
    }

    #[test]
    fn archive_moves_habit_between_lists() {
        let db = Database::in_memory().expect("db");
        let today = NaiveDate::from_ymd_opt(2026, 4, 7).unwrap();
        let habit = db
            .add_habit("Archive me", today, HabitCategory::Work, "briefcase-business")
            .expect("habit");

        db.archive_habit(&habit.id).unwrap();

        let active = db.get_all_habits_with_stats(today, false).unwrap();
        let archived = db.get_all_habits_with_stats(today, true).unwrap();

        assert!(active.is_empty());
        assert_eq!(archived.len(), 1);
        assert!(archived[0].habit.archived);
    }
}
