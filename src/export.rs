use crate::models::ExportData;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn write_export(path: &Path, data: &ExportData) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data)
        .map_err(|error| format!("Failed to serialize backup: {error}"))?;

    fs::write(path, json).map_err(|error| format!("Failed to write backup: {error}"))?;

    Ok(())
}

pub fn load_export(path: &Path) -> Result<ExportData, String> {
    let contents =
        fs::read_to_string(path).map_err(|error| format!("Failed to read backup: {error}"))?;
    let data: ExportData = serde_json::from_str(&contents)
        .map_err(|error| format!("Backup file is not valid JSON: {error}"))?;

    validate_export(&data)?;

    Ok(data)
}

fn validate_export(data: &ExportData) -> Result<(), String> {
    if !matches!(data.version, 1 | 2) {
        return Err(format!(
            "Unsupported backup version {}. This app expects version 1 or 2.",
            data.version
        ));
    }

    let mut habit_ids = HashSet::new();

    for habit in &data.habits {
        if habit.name.trim().is_empty() {
            return Err("Backup contains a habit with an empty name.".to_string());
        }

        if habit.name.len() > 100 {
            return Err(format!(
                "Habit \"{}\" is longer than the 100 character limit.",
                habit.name
            ));
        }

        if !habit_ids.insert(habit.id.clone()) {
            return Err(format!("Duplicate habit id found in backup: {}", habit.id));
        }
    }

    for log in &data.daily_logs {
        if !habit_ids.contains(&log.habit_id) {
            return Err(format!(
                "Backup contains history for an unknown habit id: {}",
                log.habit_id
            ));
        }
    }

    Ok(())
}
