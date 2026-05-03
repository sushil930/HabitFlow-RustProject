# CODEBASE KNOWLEDGE: Habit Flow

## 1. 🗂️ PROJECT OVERVIEW

| Item | Details |
|---|---|
| Project name | `habit-flow` crate, user-facing app title `Habit Flow` |
| Purpose | A calm, offline-first desktop habit tracker for adding daily habits, checking them off, viewing streaks, reviewing weekly progress, exporting/importing backups, and managing local data without accounts or cloud sync. |
| Problem solved | Replaces bloated account-based habit trackers with a local native desktop app that opens directly to daily tracking. |
| Target platform | Native desktop GUI. The PRD says Windows first; the current build target is `x86_64-pc-windows-msvc`; Iced keeps the code mostly cross-platform, but the crate uses `#![windows_subsystem = "windows"]` in `src/main.rs`. |
| Status | Active WIP/beta-style desktop app at version `0.1.1`. It has no README, no CI config, no installer pipeline, and no production deployment docs. |

Tech stack from `Cargo.toml` and `Cargo.lock`:

| Layer | Technology |
|---|---|
| Language | Rust 2021 edition |
| GUI framework | `iced` requested as `0.13`, locked to `0.13.1`, with features `canvas`, `svg`, `tokio`, `image` |
| Storage | SQLite through `rusqlite` requested as `0.31`, locked to `0.31.0`, with `bundled` SQLite |
| Date/time | `chrono` requested as `0.4`, locked to `0.4.44`, with `serde` |
| Serialization | `serde` locked to `1.0.228`, `serde_json` locked to `1.0.149` |
| IDs | `uuid` locked to `1.23.0`, using v4 UUIDs |
| App data directory | `dirs` locked to `5.0.1` |
| File dialogs | `rfd` locked to `0.14.1` |
| Icons | Static SVG assets plus `lucide-icons` locked to `1.8.0` with its `iced` feature |
| Windows dependency | `winapi` locked to `0.3.9` with `winuser` |
| Font asset | `assets/fonts/Inter-Regular.ttf` embedded into the Iced app |

Important product notes from `docs/prd.md`:

| PRD requirement | Current implementation status |
|---|---|
| Offline-only, no accounts, no network calls | Implemented: no HTTP clients or network APIs are present. |
| Local SQLite storage | Implemented in `src/db.rs`. |
| Add/edit/delete/check habits | Implemented; delete is hard delete, archive/unarchive is also implemented. |
| Weekly overview and stats | Implemented in `src/views/weekly.rs` and `src/views/stats.rs`. |
| JSON export/import | Implemented in `src/export.rs` and wired from Settings. |
| Categories/tags out of MVP scope | Categories are implemented in `HabitCategory`, the add-habit dialog, DB schema, and stats pie chart. |
| Dark/light mode toggle out of MVP scope | A full theme preset selector with 22 presets is implemented. |
| Dates stored in UTC | Not implemented: app uses `chrono::Local::now().date_naive()` and stores local `YYYY-MM-DD` strings. |

## 2. 📁 DIRECTORY STRUCTURE

Annotated tree of project-authored files:

```text
habit-tracker/
  .gitignore                         # Ignores Cargo build output, backup files, temp files, and OS metadata
  Cargo.toml                         # Rust package metadata, dependencies, release profile, bundle metadata
  Cargo.lock                         # Locked dependency graph; lockfile format version 4
  build.rs                           # Build script that generates a Lucide icon catalog into OUT_DIR
  CODEBASE_KNOWLEDGE.md              # This generated architecture and behavior reference
  assets/                            # Embedded visual/font assets
    habit_logo.svg                   # Five ascending orange rounded bars used as app logo SVG
    fonts/
      Inter-Regular.ttf              # Embedded default UI font loaded in src/main.rs
    icons/                           # Static Lucide-style SVGs used by src/icons.rs
      archive-restore.svg            # Static unarchive action icon
      archive.svg                    # Static archive/nav action icon
      bar-chart-2.svg                # Static stats nav icon
      calendar-days.svg              # Static weekly nav icon
      calendar.svg                   # Static today nav icon
      check.svg                      # Static completed checkbox icon
      download.svg                   # Static export action icon
      fire.svg                       # Static streak badge icon
      pencil.svg                     # Static rename action icon
      plus.svg                       # Static add action icon
      settings.svg                   # Static settings nav icon
      trash-2.svg                    # Static delete/factory-reset action icon
      upload.svg                     # Static import action icon
  docs/
    prd.md                           # Product requirements document for offline-first desktop habit tracker
  src/                               # Rust application source
    main.rs                          # Iced application entrypoint, state machine, message handling, dialogs
    app_icon.rs                      # Procedural RGBA window icon renderer matching habit_logo.svg
    db.rs                            # SQLite repository, schema initialization, queries, export/import/reset logic, unit tests
    export.rs                        # JSON backup read/write and validation helpers
    icons.rs                         # Static SVG asset loading helpers for fixed app icons/logo
    lucide.rs                        # Dynamic Lucide icon helpers and generated ICON_CATALOG include
    models.rs                        # Habit categories, persisted models, derived stats model, export model
    repository.rs                    # Type alias exposing crate::db::Database
    theme.rs                         # Theme preset enum, design tokens, Iced style functions
    components/                      # Reusable UI components
      mod.rs                         # Component module declarations
      add_habit_popup.rs             # Modal add-habit form with category and Lucide icon search
      category_pie_chart.rs          # Canvas pie chart for habit category distribution
      confirm_dialog.rs              # Generic confirm/cancel modal
      factory_reset_dialog.rs        # Factory reset modal with delete or backup-and-delete actions
      habit_card.rs                  # Habit list row/card with checkbox, stats, actions
      habit_input.rs                 # Reusable inline text input row for rename-style forms
      line_chart.rs                  # Canvas trend chart for completion rates and hover tooltip
      streak_badge.rs                # Fire icon plus current streak display
    views/                           # Top-level app screens and shared layout helpers
      mod.rs                         # View module declarations, content shell, divider, banner, hidden scrollbar helper
      archived.rs                    # Archived habits screen
      settings.rs                    # Appearance, export/import, factory reset, about screen
      sidebar.rs                     # Left navigation rail and add-habit button
      stats.rs                       # Dashboard, summary cards, heatmap, charts, habit breakdown
      today.rs                       # Today list screen and empty state
      weekly.rs                      # Weekly seven-day habit grid and legend
```

Generated and repository-management directories:

```text
.git/                                # Git metadata; 112 files locally, including config, refs, hooks, objects, logs, index
target/                              # Cargo build output; 17,780 files locally, about 10 GB
  debug/                             # Debug artifacts and test binaries
  release/                           # Release executable artifacts, including habit-flow.exe
  x86_64-pc-windows-msvc/             # Target-specific release output
  build-verify/                      # Additional local build output directory
  doc/                               # Generated rustdoc output
  flycheck0/                         # Rust analyzer/flycheck output
  .rustc_info.json                   # Local rustc fingerprint; shows rustc 1.92.0, host x86_64-pc-windows-msvc
  .rustdoc_fingerprint.json          # Generated rustdoc cache metadata
  CACHEDIR.TAG                       # Cargo cache directory marker
```

Inventory counts excluding `.git/` and `target/`:

| Extension | Count | Bytes | Notes |
|---|---:|---:|---|
| `.rs` | 26 | 180,244 | All Rust source, including tests in `src/db.rs` |
| `.svg` | 14 | 7,496 | Logo and static action/navigation icons |
| `.ttf` | 1 | 111,268 | Embedded Inter font |
| `.md` | 1 before this file | 14,037 | `docs/prd.md` |
| `.toml` | 1 | 964 | `Cargo.toml` |
| `.lock` | 1 | 133,697 | `Cargo.lock` |
| `.gitignore` | 1 | 103 | Ignore rules |

## 3. 🏗️ ARCHITECTURE & DESIGN PATTERNS

The app uses the Iced Elm architecture:

| Concept | Implementation |
|---|---|
| Model/state | `HabitTracker` struct in `src/main.rs` stores current view, database handle, active/archived habit lists, current local date, dialogs, banners, theme, and stats chart controls. |
| Messages | `Message` enum in `src/main.rs` is the single event/action surface for navigation, check-ins, add/rename/archive/delete, import/export/reset, theme selection, stats filters, and chart animation. |
| Update | `HabitTracker::update` mutates state and performs synchronous side effects such as DB writes and file dialogs. |
| View | `HabitTracker::view` delegates to `src/views/*.rs`, then overlays add-habit or confirm dialogs with `iced::widget::stack`. |
| Subscription | `HabitTracker::subscription` drives only the stats chart reveal animation using `iced::time::every(16ms)` while the Stats view animation is incomplete. |

High-level architecture:

```text
User input
  -> Iced widget event
  -> Message enum
  -> HabitTracker::update
  -> Database/export/theme/file-dialog side effect when needed
  -> HabitTracker::reload_habits
  -> HabitWithStats derived state
  -> HabitTracker::view
  -> views/components render Iced Element<Message>
```

Design patterns and local idioms:

| Pattern | Where | Notes |
|---|---|---|
| Elm/update loop | `src/main.rs` | All user actions pass through `Message` and `HabitTracker::update`. |
| Repository-style persistence wrapper | `src/db.rs`, `src/repository.rs` | `Database` wraps a `rusqlite::Connection`; `repository.rs` is only `pub type Database = crate::db::Database`. |
| Value object/data transfer models | `src/models.rs` | `Habit`, `DailyLog`, `HabitWithStats`, and `ExportData` carry data between DB, UI, and JSON. |
| Design-token theme module | `src/theme.rs` | Central `AppTheme`, `Colors`, `Spacing`, `Typography`, and style functions keep visual styling out of most views. |
| Modal/dialog composition | `src/main.rs`, `src/components/confirm_dialog.rs`, `src/components/factory_reset_dialog.rs`, `src/components/add_habit_popup.rs` | `active_dialog` and `show_add_input` create overlay layers. |
| Build-time generated source | `build.rs`, `src/lucide.rs` | `build.rs` generates `lucide_catalog.rs`; `src/lucide.rs` includes it from `OUT_DIR`. |
| Canvas custom rendering | `src/components/category_pie_chart.rs`, `src/components/line_chart.rs` | Pie chart, callouts, trend line, heatmap-like data visualization are custom Iced canvas/widgets. |

State management:

| State | Owner |
|---|---|
| Current screen | `HabitTracker.current_view: View` |
| Active habits with stats | `HabitTracker.habits: Vec<HabitWithStats>` |
| Archived habits with stats | `HabitTracker.archived_habits: Vec<HabitWithStats>` |
| Current date | `HabitTracker.today: NaiveDate`, set once at app initialization from local time |
| Add form | `HabitTracker.show_add_input` and `AddHabitDraft` |
| Rename form | `editing_habit_id` and `rename_input_value` |
| Dialogs | `active_dialog: Option<Dialog>` |
| Notices/date rollback | `notice`, `rollback_warning`, `checkins_blocked`, `fatal_error` |
| Theme | `app_theme: AppTheme`; persisted as `app_meta.theme_preset` |
| Stats controls | `stats_selected_habit`, `stats_range_days`, `stats_chart_reveal_progress` |

Communication patterns:

| Pattern | Present? | Details |
|---|---|---|
| REST/GraphQL/WebSocket | No | There are no network endpoints, HTTP clients, servers, or sockets. |
| File I/O | Yes | SQLite DB in OS app data directory; JSON backups via `rfd::FileDialog` and `std::fs`. |
| Database | Yes | Local SQLite through `rusqlite`; all DB operations are synchronous. |
| Queues/background jobs | No | No background workers beyond Iced subscription timer for chart animation. |

## 4. 🔑 CORE MODULES & KEY FILES

| File path | Purpose | Key exports/classes/functions | Dependencies | Side effects |
|---|---|---|---|---|
| `src/main.rs` | Application entrypoint, Iced state machine, top-level view routing, dialog orchestration, import/export/reset flows. | `main`, `View`, `BannerMessage`, `Message`, `HabitTracker`, `AddHabitDraft`, `validate_name`, `error_banner`, `file_name`. | `iced`, `chrono`, `rfd`, `components`, `models`, `repository::Database`, `theme`, `views`, `export`, `app_icon`, `lucide_icons`. | Opens GUI window; opens SQLite DB; shows native file dialogs; writes/reads backups through `export`; writes DB for tracking, theme, import/reset. |
| `src/db.rs` | All SQLite schema setup, migrations-by-column-addition, CRUD, stats derivation, import/export data access, unit tests. | `Database` with methods `new`, `database_path`, `add_habit`, `rename_habit`, `archive_habit`, `unarchive_habit`, `delete_habit`, `toggle_day`, `get_all_habits_with_stats`, `export_data`, `import_data`, `factory_reset`; helper tests. | `rusqlite`, `chrono`, `dirs`, `uuid`, `lucide`, `models`, `theme::ThemePreset`, `APP_DATA_DIR_NAME`. | Creates `%APPDATA%/HabitFlow`; opens `data.db`; creates/alters tables; executes SQLite reads/writes; uses WAL. |
| `src/models.rs` | Central data model definitions shared by DB, UI, and export JSON. | `HabitCategory`, `Habit`, `DailyLog`, `HabitWithStats`, `ExportData`, `HabitWithStats::is_completed_on`. | `chrono::NaiveDate`, `serde`, `std::fmt`, `FromStr`. | None directly. |
| `src/export.rs` | JSON backup serialization/deserialization and validation. | `write_export`, `load_export`, private `validate_export`. | `serde_json`, `std::fs`, `std::path::Path`, `ExportData`, `HashSet`. | Reads/writes chosen JSON backup files. |
| `src/theme.rs` | Theme presets, design tokens, and Iced style callbacks. | `ThemePreset`, `Colors`, `Spacing`, `Typography`, `AppTheme`, `BannerKind`, `WeekCircle`, style functions such as `primary_button`, `section_card`, `week_circle`, `text_field`. | `iced::{Theme, Color, Background, Border}`, `iced::widget::{button, container, text_input}`. | None directly; pure style computation. |
| `src/lucide.rs` | Dynamic Lucide icon rendering and icon search helpers. | `DEFAULT_ICON_NAME`, `FEATURED_ICON_NAMES`, generated `ICON_CATALOG`, `icon_by_name`, `sanitize_icon_name`, `icon_matches_query`, `featured_icons`. | `lucide_icons::Icon`, `iced::widget::Text`, generated `OUT_DIR/lucide_catalog.rs`. | Compile-time include of generated file. |
| `src/icons.rs` | Static SVG icon/logo asset loader for common navigation/action icons. | `APP_LOGO_BYTES`, `icon`, `app_logo`, `check`, `settings`, `edit`, `trash`, `archive`, `unarchive`, `plus`, `calendar`, `calendar_days`, `bar_chart`, `download`, `upload`, `fire`. | `iced::widget::svg`, embedded SVG assets under `assets/`. | Compile-time `include_bytes!` for SVG assets. |
| `src/app_icon.rs` | Generates a 512x512 RGBA window icon from hard-coded bar geometry matching the logo. | `window_icon`; private `render_logo_rgba`, `point_in_bars`, `point_in_rounded_rect`. | `iced::window::icon`, `iced::window::Icon`. | Allocates RGBA buffer at startup; panics only if generated icon bytes are invalid. |
| `src/repository.rs` | Thin compatibility alias for the persistence type. | `pub type Database = crate::db::Database`. | `crate::db::Database`. | None. |
| `build.rs` | Build script that extracts all Lucide `Icon` enum variants from the local Cargo registry and writes `lucide_catalog.rs`. | `main`, `find_lucide_source`, `search_for_lucide_dir`, `extract_icon_variants`. | `std::env`, `std::fs`, `std::io::Write`, `std::path`. | Reads Cargo registry source; writes generated Rust file into `OUT_DIR`; emits `cargo:rerun-if-changed`. |
| `src/views/mod.rs` | Shared view helpers and view module declarations. | `hidden_scrollbar`, `content_shell`, `divider`, `banner`. | `iced`, `theme`, `BannerMessage`, `Message`. | None. |
| `src/views/sidebar.rs` | Left navigation rail with brand, screen links, and add-habit button. | `view`; private `nav_button`. | `iced`, `icons`, `theme`, `Message`, `View`, `APP_TITLE`. | Emits navigation/add messages through buttons. |
| `src/views/today.rs` | Main Today screen, empty state, active habit list, inline rename rows. | `view`; private `header`. | `iced`, `chrono`, `components::habit_card`, `components::habit_input`, `views`, `HabitWithStats`. | Emits add, toggle, rename, archive, delete messages through child components. |
| `src/views/weekly.rs` | Weekly grid showing Monday-to-Sunday completion circles and progress percentage per habit. | `view`; private `header_row`, `legend_chip`. | `chrono`, `iced`, `theme::WeekCircle`, `HabitWithStats`. | None except UI messages from shared banner. |
| `src/views/stats.rs` | Dashboard with summary cards, category pie chart, yearly heatmap, completion trend chart, and habit breakdown cards. | `ChartFilter`, `ChartRange`, `view`; private `top_level_stats`, `global_heatmap`, `card`, `stat_row`. | `chrono`, `iced`, `HashMap`, `category_pie_chart`, `LineChart`, `HabitWithStats`. | Emits stats filter/range selection messages. |
| `src/views/archived.rs` | Archived habits screen with restore/delete actions. | `view`. | `chrono`, `iced`, `habit_card`, `views`, `HabitWithStats`. | Emits unarchive/delete messages through child component. |
| `src/views/settings.rs` | Settings screen for theme selection, export/import, factory reset, and About. | `view`. | `iced`, `icons`, `theme::ThemePreset`, `views`, `APP_TITLE`, `APP_VERSION`. | Emits theme, export, import, reset messages. |
| `src/components/add_habit_popup.rs` | Modal form for adding a habit with name, category, selected icon, featured icon grid, and icon search. | `view`; private `visible_icons`, `icon_grid_view`, `icon_button`, `icon_button_style`, `pretty_icon_label`. | `iced`, `lucide_icons`, `crate::lucide`, `HabitCategory`, `AddHabitDraft`, `Message`, `views`. | Emits add-form messages; no direct persistence. |
| `src/components/habit_card.rs` | Habit row/card with check control, category/icon/title/subtitle, streak badge, and rename/archive/delete/unarchive buttons. | `view`; private `subtitle`, `subtitle_color`. | `chrono`, `iced`, `icons`, `lucide`, `streak_badge`, `HabitWithStats`, `theme`, `Message`. | Emits toggle/rename/archive/unarchive/delete messages. |
| `src/components/habit_input.rs` | Reusable text field with submit and cancel controls, used for renaming. | `view`. | `iced`, `theme`, `Message`. | Emits input/submit/cancel messages. |
| `src/components/confirm_dialog.rs` | Generic modal confirmation dialog. | `view`. | `iced`, `theme`, `Message`. | Emits confirm or cancel messages. |
| `src/components/factory_reset_dialog.rs` | Specialized modal for destructive factory reset, with optional backup first. | `view`. | `iced`, `theme`, `Message`. | Emits reset, backup-and-reset, or cancel messages. |
| `src/components/streak_badge.rs` | Compact streak display using a fire icon and monospace count. | `view`. | `iced`, `icons`, `theme`, `Message`. | None. |
| `src/components/category_pie_chart.rs` | Canvas pie chart and legend for category distribution. | `view`; private `CategoryPieChart`, `CategorySlice`, `category_slices`, `category_color`, callout helpers. | `iced::widget::canvas`, `HabitCategory`, `HabitWithStats`, `theme`, `lucide`. | Custom canvas drawing only. |
| `src/components/line_chart.rs` | Canvas completion-rate trend chart with smoothed line, reveal animation support, hover guide, and tooltip. | `LineChart::new`; private `DailyPoint`, `add_smooth_segments`, `revealed_polyline_points`, `habit_completed_on`. | `chrono`, `iced::widget::canvas`, `HabitWithStats`, `theme`. | Custom canvas drawing only. |

Asset files:

| File path | Purpose | Notes |
|---|---|---|
| `assets/habit_logo.svg` | Main app logo SVG. | Five orange bars with `#F97316`, transparent background. |
| `assets/fonts/Inter-Regular.ttf` | Embedded UI font. | Loaded by `main.rs` through `include_bytes!`. |
| `assets/icons/*.svg` | Static action/nav icons. | Loaded by filename switch in `src/icons.rs`; unknown filename falls back to `check.svg`. |

## 5. 🗃️ DATA MODELS & DATABASE SCHEMA

Rust models in `src/models.rs`:

| Type | Fields/variants | Notes |
|---|---|---|
| `HabitCategory` | `General`, `Health`, `Fitness`, `Mindfulness`, `Learning`, `Productivity`, `Work`, `Home`, `Social`, `Finance`, `Creative` | Serialized as snake_case. `label()` returns UI title text; `key()` returns DB key; `icon_name()` returns default Lucide icon; `from_db_value()` silently defaults unknown values to `General`. |
| `Habit` | `id: String`, `name: String`, `created_date: NaiveDate`, `sort_order: i32`, `category: HabitCategory`, `icon: String`, `archived: bool` | Persisted entity. `category`, `icon`, and `archived` have Serde defaults for older backups. |
| `DailyLog` | `habit_id: String`, `date: NaiveDate`, `completed: bool` | Persisted completion state for one habit on one date. |
| `HabitWithStats` | `habit: Habit`, `current_streak: u32`, `longest_streak: u32`, `total_completions: u32`, `total_logged_days: u32`, `last_30_days_rate: f32`, `completed_today: bool`, `last_completed_date: Option<NaiveDate>`, `week_logs: Vec<DailyLog>`, `history_logs: Vec<DailyLog>` | Derived view model loaded by `Database::get_all_habits_with_stats`. `history_logs` currently covers only the last 60 days. |
| `ExportData` | `version: u32`, `exported_at: String`, `habits: Vec<Habit>`, `daily_logs: Vec<DailyLog>` | JSON backup format. New exports use `version = 2`; imports accept versions 1 and 2. |

Other important app state types in `src/main.rs`:

| Type | Fields/variants | Notes |
|---|---|---|
| `View` | `Today`, `Weekly`, `Stats`, `Archived`, `Settings` | Top-level route enum for sidebar navigation. |
| `BannerMessage` | `kind: BannerKind`, `text: String`, `dismissible: bool` | Global notice/banner model. |
| `Dialog` | `DeleteHabit`, `ImportBackup`, `FactoryReset` | Private modal state; stores habit IDs/names, import data, or reset counts. |
| `Message` | Navigation, add, rename, toggle, archive, delete, export/import/reset, theme, stats, animation variants | Central Iced event enum. |
| `AddHabitDraft` | `name`, `category`, `icon`, `icon_search` | Add-habit modal form state. |

SQLite database:

| Item | Details |
|---|---|
| DB file path | `Database::database_path()` builds `dirs::data_dir()/HabitFlow/data.db`; on Windows this normally resolves under `%APPDATA%\HabitFlow\data.db`. |
| ORM/query builder | None; direct SQL through `rusqlite`. |
| Schema owner | `Database::init_tables` in `src/db.rs`. |
| Migrations | No migration table. Missing `category`, `icon`, and `archived` columns are added with `ALTER TABLE` through `ensure_habits_column`. |
| Pragmas | `PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;`. |

Schema created by `src/db.rs`:

```sql
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
```

Relationships:

| Relationship | Implementation |
|---|---|
| Habit to DailyLog | One-to-many through `daily_logs.habit_id`; hard delete cascades from `habits` to `daily_logs`. |
| App metadata | Key/value table independent from habit data. Known keys are `last_used_date` and `theme_preset`. |

Indexes and constraints:

| Table | Constraint/index |
|---|---|
| `habits` | Primary key on `id`; no uniqueness constraint on `name`; no index on `archived` or `sort_order`. |
| `daily_logs` | Composite primary key on `(habit_id, date)`; foreign key cascade to `habits(id)`. |
| `app_meta` | Primary key on `key`. |

Date and boolean storage:

| Value | Storage representation |
|---|---|
| Dates | `NaiveDate` serialized as `YYYY-MM-DD` strings. The app uses local date, not UTC. |
| Booleans | SQLite integers: `0` false, non-zero true. |
| UUIDs | Stored as text strings from `uuid::Uuid::new_v4().to_string()`. |

## 6. 🔌 API REFERENCE

This is a binary desktop app. It exposes no HTTP endpoints, no CLI commands, and no external library API. The public surfaces below are crate-internal Rust interfaces used between modules.

Database interface from `src/db.rs`:

| Function signature | Purpose | Request shape | Response shape | Auth | Error cases |
|---|---|---|---|---|---|
| `Database::new() -> SqlResult<Self>` | Open/create the app DB at the OS data path and initialize schema. | No args. | `Database`. | None. | App data directory creation failure; SQLite open/schema failure. |
| `Database::database_path() -> PathBuf` | Compute local SQLite file path. | No args. | Path ending in `HabitFlow/data.db`. | None. | No error; falls back to `.` if OS data dir is unavailable. |
| `detect_date_rollback(&self, today: NaiveDate) -> SqlResult<Option<NaiveDate>>` | Detect if `app_meta.last_used_date` is later than today. | `today` local date. | `Some(last_used)` blocks check-ins; `None` updates last-used date. | None. | Missing/invalid DB; date parse failure; metadata write failure. |
| `touch_last_used_date(&self, today: NaiveDate) -> SqlResult<()>` | Upsert `app_meta.last_used_date`. | `today`. | Unit. | None. | SQLite write failure. |
| `get_theme_preset(&self) -> SqlResult<Option<ThemePreset>>` | Read persisted theme preset. | No args. | `Some(ThemePreset)` for recognized key; `None` for missing/unknown. | None. | SQLite read failure. |
| `set_theme_preset(&self, preset: ThemePreset) -> SqlResult<()>` | Upsert `app_meta.theme_preset`. | Theme preset. | Unit. | None. | SQLite write failure. |
| `add_habit(&self, name: &str, today: NaiveDate, category: HabitCategory, icon: &str) -> SqlResult<Habit>` | Insert a new active habit. | Trimmed name, local date, category, icon name. | Persisted `Habit` with UUID and next sort order. | None. | SQLite write failure; schema alteration failure. Name validation is done by UI, not this method. |
| `rename_habit(&self, id: &str, new_name: &str) -> SqlResult<()>` | Update habit name. | Habit ID and new name. | Unit. | None. | SQLite write failure. No rows changed is not treated as an error. |
| `archive_habit(&self, id: &str) -> SqlResult<()>` | Set `archived = 1`. | Habit ID. | Unit. | None. | SQLite write failure. |
| `unarchive_habit(&self, id: &str) -> SqlResult<()>` | Set `archived = 0`. | Habit ID. | Unit. | None. | SQLite write failure. |
| `delete_habit(&self, id: &str) -> SqlResult<()>` | Hard-delete a habit; logs cascade. | Habit ID. | Unit. | None. | SQLite write failure. |
| `toggle_day(&self, habit_id: &str, date: NaiveDate) -> SqlResult<bool>` | Toggle one habit/date completion. | Habit ID and date. | New completion state. | None. | SQLite read/write failure; foreign key failure for unknown habit. |
| `get_all_habits(&self, archived: bool) -> SqlResult<Vec<Habit>>` | Load active or archived habits ordered by sort order and created date. | `archived` flag. | Vec of `Habit`. | None. | SQLite read/parse failure. |
| `get_logs_between(&self, habit_id: &str, start: NaiveDate, end: NaiveDate) -> SqlResult<Vec<DailyLog>>` | Load logs for one habit in inclusive date range. | Habit ID, start date, end date. | Ordered Vec of `DailyLog`. | None. | SQLite read/parse failure. |
| `calculate_streaks(&self, habit_id: &str, today: NaiveDate) -> SqlResult<(u32, u32)>` | Calculate current and longest streak from completed logs. | Habit ID and current local date. | `(current_streak, longest_streak)`. | None. | SQLite read/parse failure. |
| `get_all_habits_with_stats(&self, today: NaiveDate, archived: bool) -> SqlResult<Vec<HabitWithStats>>` | Load habits plus weekly logs, 60-day history, streaks, totals, last-30-day rate, and last completed date. | Date and archived flag. | Vec of `HabitWithStats`. | None. | SQLite read/parse failure. |
| `export_data(&self) -> SqlResult<ExportData>` | Load all active/archived habits and all daily logs for backup. | No args. | `ExportData { version: 2, exported_at, habits, daily_logs }`. | None. | SQLite read/parse failure. |
| `import_data(&mut self, data: &ExportData) -> SqlResult<()>` | Replace all habits/logs with backup contents. | Validated `ExportData`. | Unit. | None. | SQLite transaction/constraint failure; malformed data should be caught by `load_export` first. |
| `factory_reset(&mut self) -> SqlResult<()>` | Delete habits, logs, and metadata. | No args. | Unit. | None. | SQLite transaction failure. |

Export interface from `src/export.rs`:

| Function signature | Purpose | Request shape | Response shape | Auth | Error cases |
|---|---|---|---|---|---|
| `write_export(path: &Path, data: &ExportData) -> Result<(), String>` | Serialize backup JSON and write it to a chosen file. | Path and export model. | Unit. | None. | Serialization or filesystem write failure. |
| `load_export(path: &Path) -> Result<ExportData, String>` | Read JSON backup, deserialize it, validate schema consistency. | Path. | `ExportData`. | None. | Read failure, invalid JSON, unsupported version, empty/too-long names, duplicate habit IDs, logs for unknown habits. |

Model/helper interface from `src/models.rs` and `src/lucide.rs`:

| Function/signature | Purpose |
|---|---|
| `HabitCategory::label(self) -> &'static str` | Human label for UI. |
| `HabitCategory::key(self) -> &'static str` | Storage key for DB/export. |
| `HabitCategory::icon_name(self) -> &'static str` | Default Lucide icon for category. |
| `HabitCategory::from_db_value(value: &str) -> Self` | Parse category-like value; unknown values become `General`. |
| `HabitWithStats::is_completed_on(&self, date: NaiveDate) -> bool` | Check whether `week_logs` contains a completed log for a date. |
| `icon_by_name(name: &str, color: Color, size: f32) -> Text<'static>` | Render Lucide font glyph by icon name, fallback to `Sparkles`. |
| `sanitize_icon_name(name: &str) -> String` | Return valid Lucide icon name or `sparkles`. |
| `icon_matches_query(icon: Icon, query: &str) -> bool` | Search icon names with hyphen and humanized space matching. |
| `featured_icons() -> Vec<Icon>` | Resolve the fixed featured icon list into Lucide enum values. |

UI view/component interface:

| Function/signature | Purpose |
|---|---|
| `views::content_shell(content, theme) -> Element<Message>` | Common full-page container and padding. |
| `views::banner(banner, theme) -> Element<Message>` | Render dismissible/non-dismissible app banner. |
| `views::today::view(...) -> Element<Message>` | Render active habits for current day. |
| `views::weekly::view(...) -> Element<Message>` | Render weekly progress grid. |
| `views::stats::view(...) -> Element<Message>` | Render dashboard and charts. |
| `views::archived::view(...) -> Element<Message>` | Render archived habits. |
| `views::settings::view(...) -> Element<Message>` | Render appearance/data/about settings. |
| `views::sidebar::view(current_view, theme) -> Element<Message>` | Render navigation sidebar. |
| `components::add_habit_popup::view(...) -> Element<Message>` | Render modal add form. |
| `components::habit_card::view(...) -> Element<Message>` | Render one habit row/card. |
| `components::habit_input::view(...) -> Element<Message>` | Render text input row for rename. |
| `components::confirm_dialog::view(...) -> Element<Message>` | Render confirm modal. |
| `components::factory_reset_dialog::view(...) -> Element<Message>` | Render reset modal. |
| `components::streak_badge::view(streak, theme) -> Element<Message>` | Render streak badge. |
| `components::category_pie_chart::view(habits, theme) -> Element<Message>` | Render category pie chart. |
| `LineChart::new(habits, selected_habit, today, theme, days, reveal_progress) -> LineChart` | Build completion trend canvas program. |

## 7. ⚙️ CONFIGURATION & ENVIRONMENT

Runtime configuration:

| Variable/file | Required | Default | Controls |
|---|---|---|---|
| `.env` / `.env.example` | No | None | No env file exists; no runtime env vars are read by the app. |
| OS data directory from `dirs::data_dir()` | Yes for normal persistence | Falls back to current directory `.` if unavailable | Parent directory for `HabitFlow/data.db`. |
| `%APPDATA%/HabitFlow/data.db` on Windows | Created automatically | N/A | SQLite database file for habits, logs, and app metadata. |
| JSON backup path chosen through `rfd::FileDialog` | User-selected | File name defaults to `habit-flow-backup-YYYY-MM-DD.json` on export | Export/import file location. |

Compile/build-time environment:

| Variable | Used by | Required | Meaning |
|---|---|---|---|
| `OUT_DIR` | `build.rs`, `src/lucide.rs` | Yes during Cargo build | Destination/source location for generated `lucide_catalog.rs`. |
| `CARGO_HOME` | `build.rs` | Optional | First searched root for local Cargo registry source. |
| `USERPROFILE` | `build.rs` | Optional | Windows fallback root for `.cargo/registry/src`. |
| `HOME` | `build.rs` | Optional | Unix-like fallback root for `.cargo/registry/src`. |
| `CARGO_PKG_VERSION` | `src/main.rs` through `env!` | Cargo-provided | Defines `APP_VERSION` shown in Settings About. |

Constants in `src/main.rs`:

| Constant | Value | Used for |
|---|---|---|
| `APP_TITLE` | `Habit Flow` | Window title, sidebar brand, Settings About. |
| `APP_VERSION` | `env!("CARGO_PKG_VERSION")` | Settings About. |
| `APP_DATA_DIR_NAME` | `HabitFlow` | DB directory name under OS data dir. |
| `APP_BACKUP_PREFIX` | `habit-flow-backup` | Export backup default file names. |

Cargo configuration:

| Section | Details |
|---|---|
| `[profile.release]` | `lto = true`, `strip = true`, `opt-level = "s"`, `codegen-units = 1`. Optimizes for small release binary. |
| `[target.'cfg(windows)'.dependencies]` | Adds `winapi` with `winuser`. |
| `[package.metadata.bundle]` | Bundle name `Habit Flow`, identifier `io.github.sushi.habitflow`, icon `assets/habit_logo.svg`. No bundling command/script exists in repo. |

External services:

| Service | Status |
|---|---|
| Database server | None. SQLite is local and bundled. |
| Cloud sync/storage | None. |
| Authentication provider | None. |
| Email/SMS/payments/analytics | None. |
| Network APIs | None found. |

## 8. 🔐 AUTHENTICATION & AUTHORIZATION

| Topic | Details |
|---|---|
| Auth mechanism | None. No JWT, sessions, OAuth, passwords, API keys, or account model. |
| Authorization | None. All local app actions are available to whoever can run the desktop process and access the local files. |
| Protected routes/resources | None. |
| Sensitive data handling | Habits and logs are stored in plaintext SQLite and exported as plaintext JSON. |
| Security boundary | OS user account and filesystem permissions only. |
| Date rollback protection | `Database::detect_date_rollback` blocks check-ins if stored `last_used_date` is later than current local `today`; this is integrity protection, not authentication. |

## 9. 🧪 TESTING STRATEGY

Framework:

| Item | Details |
|---|---|
| Test framework | Rust built-in unit tests with `#[test]`. |
| Test location | `src/db.rs` inside `#[cfg(test)] mod tests`. |
| Test DB | In-memory SQLite through private `Database::in_memory()`. |
| UI tests | None. |
| Integration/e2e tests | None. |

Existing tests:

| Test name | Coverage |
|---|---|
| `streak_uses_yesterday_when_today_is_unchecked` | Current streak can anchor to yesterday when today is unchecked. |
| `missed_day_resets_current_streak` | A gap resets current streak. |
| `delete_cascades_history` | Deleting a habit removes its daily logs through cascade. |
| `factory_reset_clears_habits_logs_and_meta` | Factory reset deletes habits, logs, and app metadata. |
| `theme_preset_round_trips_through_app_meta` | Theme preset persists and loads through `app_meta`. |
| `archive_moves_habit_between_lists` | Archiving removes a habit from active list and puts it in archived list. |

Commands:

| Command | Purpose | Current observed result |
|---|---|---|
| `cargo test` | Run all tests. | Passed: 6 passed, 0 failed. |
| `cargo test --no-run` | Compile tests without executing. | Passed. |
| `cargo build` | Compile debug app. | Passed. |
| `cargo fmt -- --check` | Check Rust formatting. | Failed: many files need rustfmt formatting changes. |

Coverage gaps:

| Area | Missing tests |
|---|---|
| Export/import validation | No direct tests for malformed JSON, duplicate habit IDs, unknown log habit IDs, version checks, or import overwrite behavior. |
| UI update flow | No tests for `HabitTracker::update`, add/rename/archive/delete dialog transitions, notices, or check-in blocking. |
| Theme catalog | No tests for all `ThemePreset` storage keys or palette-derived colors. |
| Lucide build script/catalog | No tests for `extract_icon_variants` or missing Cargo registry behavior. |
| Stats/charts | No tests for last-30-day rate, heatmap aggregation, line chart point generation, or category pie slices. |
| Date edge cases | Date rollback has no direct unit test; timezone/local-date behavior has no tests. |

## 10. 🚀 BUILD, RUN & DEPLOY

Install dependencies:

```powershell
cargo fetch
```

Run locally:

```powershell
cargo run
```

Build debug:

```powershell
cargo build
```

Build release:

```powershell
cargo build --release
```

Run tests:

```powershell
cargo test
```

Check formatting:

```powershell
cargo fmt -- --check
```

Inspect dependencies:

```powershell
cargo tree --prefix depth --charset ascii
cargo metadata --no-deps --format-version 1
```

Important build behavior:

| Item | Details |
|---|---|
| Build script | `build.rs` runs before compiling the main crate. It finds `lucide-icons-*/src/icon.rs` in the local Cargo registry, extracts `Icon` enum variants, and writes `lucide_catalog.rs` to `OUT_DIR`. |
| Generated include | `src/lucide.rs` uses `include!(concat!(env!("OUT_DIR"), "/lucide_catalog.rs"))`; compile through Cargo, not raw `rustc`. |
| Window settings | `src/main.rs` creates an 1100x700 centered window, minimum 900x600, with generated icon from `src/app_icon.rs`. |
| Console behavior | `#![windows_subsystem = "windows"]` hides the console on Windows. |
| Existing local artifacts | `target/release/habit-flow.exe`, `target/x86_64-pc-windows-msvc/release/habit-flow.exe`, and debug/test artifacts exist locally. |

Deployment:

| Topic | Current state |
|---|---|
| Pipeline | No `.github/`, CI config, release workflow, or Makefile found. |
| Docker/container | None. |
| Installer/bundler | `package.metadata.bundle` exists, but no bundling tool config or command is documented. |
| Hosting | Not applicable; local desktop app. |
| Git remote | `origin` points to `https://github.com/sushil930/Desktop-Application-RustProject.git`. |

## 11. 🔗 EXTERNAL DEPENDENCIES & INTEGRATIONS

No third-party services are integrated. All dependencies are local crates/libraries.

Critical direct dependencies:

| Crate | Locked version | Why it matters |
|---|---:|---|
| `iced` | `0.13.1` | Core GUI runtime, widgets, theme, canvas, SVG, image support. |
| `rusqlite` | `0.31.0` | Local SQLite connection and queries; bundled SQLite avoids external DB install. |
| `lucide-icons` | `1.8.0` | Dynamic icon font and `Icon` enum for habit icons. |
| `chrono` | `0.4.44` | Local date handling, streak calculations, date formatting. |
| `serde` | `1.0.228` | Serialize/deserialize models and backups. |
| `serde_json` | `1.0.149` | JSON export/import format. |
| `uuid` | `1.23.0` | Habit IDs. |
| `dirs` | `5.0.1` | OS data directory lookup. |
| `rfd` | `0.14.1` | Native file save/open dialogs. |
| `winapi` | `0.3.9` | Windows-specific dependency enabled in Cargo config. |

Important transitive dependency families from `cargo tree`:

| Family | Used by |
|---|---|
| `wgpu`, `winit`, `softbuffer`, `tiny-skia`, `resvg`, `usvg` | Iced rendering, windowing, SVG/canvas drawing. |
| `image`, `png`, `jpeg-decoder`, `gif`, `tiff` | Iced image/SVG feature stack. |
| `tokio`, `futures` | Iced async/time subscription support. |
| `libsqlite3-sys`, `cc`, `pkg-config`, `vcpkg` | Bundled SQLite build. |

Vendor-specific quirks:

| Area | Quirk |
|---|---|
| `lucide-icons` | The app does not hard-code the full catalog. `build.rs` scrapes the local crate source to generate `ICON_CATALOG`, so a clean build depends on Cargo having downloaded the `lucide-icons` source into the registry. |
| `iced` | Many UI APIs are compile-time typed and return `Element<'a, Message>`. Widget callbacks are wired directly to `Message` variants. |
| `rfd` | File dialog calls are synchronous in `HabitTracker::update`; canceling a dialog returns `None` and leaves state mostly unchanged. |
| SQLite | `rusqlite` operations are synchronous on the UI thread. This is acceptable for small local data, but long imports or huge DBs can block the GUI. |

## 12. ⚠️ KNOWN ISSUES, TECH DEBT & GOTCHAS

Explicit TODO/FIXME scan:

| Pattern | Result |
|---|---|
| `TODO`, `FIXME`, `HACK`, `XXX` | None found outside generated artifacts. |

Concrete issues and gotchas:

| Area | Details |
|---|---|
| Formatting | `cargo fmt -- --check` currently fails across `build.rs`, `src/app_icon.rs`, `src/components/*`, `src/db.rs`, `src/icons.rs`, `src/lucide.rs`, `src/main.rs`, `src/theme.rs`, and several views. Do not assume rustfmt-clean style until formatting is applied. |
| Heatmap data window | `src/views/stats.rs` labels the heatmap `Activity (Last Year)`, but `Database::get_all_habits_with_stats` only loads `history_logs` from the last 60 days, so older heatmap days always appear empty even if older DB logs exist. |
| PRD date model mismatch | `docs/prd.md` says dates should be UTC; `src/main.rs` uses local `chrono::Local::now().date_naive()` and `src/db.rs` stores local `NaiveDate` strings. |
| PRD current streak mismatch | PRD describes current streak as ending today; `calculate_streaks` and its test allow the current streak to anchor to yesterday when today is not checked yet. |
| Import behavior | Import is replace-all: `import_data` deletes all `daily_logs` and `habits`, then inserts backup contents. There is no merge strategy despite the PRD mentioning conflict prompts. |
| Import metadata | `import_data` does not import `app_meta`; `ConfirmImport` touches `last_used_date` afterward. Theme is unchanged by import. |
| Schema migrations | There is no versioned migration system. New columns are added opportunistically with `ALTER TABLE`, but constraints/indexes/more complex migrations have no framework. |
| Name length | `validate_name` uses `String::len()`, which counts bytes, not user-visible graphemes. Non-ASCII names can hit the 100-byte limit earlier than expected. |
| Category parsing | Unknown DB/export category strings silently become `General`. This preserves import resilience but can hide backup/schema mistakes. |
| Icon parsing | Unknown icon names become `sparkles` through `sanitize_icon_name`. |
| DB writes in UI thread | Add, rename, toggle, archive, delete, import, reset, and theme persistence run synchronously inside `HabitTracker::update`. |
| Future logs | `last_30_days_rate` counts logs with `date >= thirty_days_ago` and no `date <= today` bound, so malformed/imported future completed logs could inflate rates. |
| No DB index on date | `daily_logs` primary key is `(habit_id, date)`, which is good for per-habit ranges, but there is no index for global date aggregation. Current global heatmap uses in-memory loaded history, not DB aggregation. |
| `target/` size | Local `target/` is about 10 GB with 17,780 files. It is ignored by Git but expensive to scan/copy. |
| Build script fragility | `build.rs` panics if it cannot find `lucide-icons` source under `CARGO_HOME`, `USERPROFILE/.cargo`, or `HOME/.cargo`. |
| No README | There is no root `README.md`; developers must infer commands from Cargo conventions or this file. |
| No CI | There is no automated test/build/format pipeline in the repo. |
| Old generated names | Local `target/` contains stale `minimal-habit-tracker` artifacts alongside current `habit-flow` artifacts. These are generated and not part of source behavior. |

Security concerns:

| Area | Details |
|---|---|
| Plaintext storage | SQLite DB and JSON backups are unencrypted. This is consistent with current offline-first design but should be documented to users. |
| Destructive actions | Delete and factory reset are confirmed; import replaces data after confirmation. There is no undo. |
| File import trust | `load_export` validates backup structure but does not validate UUID format, duplicate logs, future dates, or category/icon semantic correctness beyond fallbacks. |

## 13. 📐 CODING CONVENTIONS & STYLE RULES

Observed conventions:

| Topic | Convention |
|---|---|
| Rust edition | 2021. |
| Type names | `PascalCase`: `HabitTracker`, `HabitWithStats`, `ThemePreset`, `BannerMessage`. |
| Function/variable names | `snake_case`: `reload_habits`, `validate_name`, `history_logs`. |
| Modules/files | Lower snake case: `add_habit_popup.rs`, `category_pie_chart.rs`, `factory_reset_dialog.rs`. |
| Enums | `PascalCase` variants: `View::Today`, `Message::ToggleHabit`, `ThemePreset::TokyoNightStorm`. |
| Constants | `SCREAMING_SNAKE_CASE`: `APP_TITLE`, `APP_DATA_DIR_NAME`, `DEFAULT_ICON_NAME`, `FEATURED_ICON_NAMES`. |
| Imports | Usually standard/external imports first, then `crate::...`; current ordering is not fully rustfmt-clean. |
| Error handling | DB layer returns `rusqlite::Result`; export layer returns `Result<_, String>`; UI converts failures into `BannerMessage` through `error_banner`. |
| UI styling | Most widget styles go through functions in `src/theme.rs`; direct container styles are used for chart/heatmap cells. |
| Logging | No `log`, `println!`, or `eprintln!` in app runtime source. `build.rs` uses `println!` only for Cargo build instructions. |
| Comments | Sparse comments. `src/components/line_chart.rs` has orienting comments for grid, area fill, trend line, and x-axis labels; `src/views/stats.rs` has section comments and one inline color comment. |
| Tests | Tests live beside DB code in `src/db.rs`; they use fixed `NaiveDate::from_ymd_opt(2026, 4, 7)` values and `unwrap/expect` inside tests. |

Formatting status:

| Command | Current status |
|---|---|
| `cargo fmt -- --check` | Fails. Run `cargo fmt` before making style-sensitive changes or before adding CI. |

## 14. 🧩 FEATURE MAP

| Feature | Entry point | Key files involved |
|---|---|---|
| App startup and DB initialization | `HabitTracker::initialize` | `src/main.rs`, `src/db.rs`, `src/theme.rs`, `src/app_icon.rs` |
| Date rollback protection | `Database::detect_date_rollback`, `HabitTracker::initialize`, `Message::ToggleHabit` | `src/db.rs`, `src/main.rs`, `src/views/mod.rs` |
| Sidebar navigation | `Message::NavigateTo`, `views::sidebar::view` | `src/main.rs`, `src/views/sidebar.rs`, `src/icons.rs` |
| Add habit | `Message::ShowAddInput`, `Message::SubmitAddHabit` | `src/main.rs`, `src/components/add_habit_popup.rs`, `src/db.rs`, `src/models.rs`, `src/lucide.rs` |
| Category selection | `Message::SelectAddCategory` | `src/main.rs`, `src/models.rs`, `src/components/add_habit_popup.rs` |
| Icon search/selection | `Message::AddIconSearchChanged`, `Message::SelectAddIcon` | `src/main.rs`, `src/lucide.rs`, `src/components/add_habit_popup.rs`, `build.rs` |
| Today check/uncheck | `Message::ToggleHabit` | `src/main.rs`, `src/views/today.rs`, `src/components/habit_card.rs`, `src/db.rs` |
| Rename habit | `Message::StartRename`, `Message::SubmitRename` | `src/main.rs`, `src/views/today.rs`, `src/components/habit_input.rs`, `src/db.rs` |
| Archive habit | `Message::ArchiveHabit` | `src/main.rs`, `src/components/habit_card.rs`, `src/db.rs`, `src/views/archived.rs` |
| Unarchive habit | `Message::UnarchiveHabit` | `src/main.rs`, `src/views/archived.rs`, `src/components/habit_card.rs`, `src/db.rs` |
| Delete habit | `Message::DeleteHabitRequest`, `Message::ConfirmDeleteHabit` | `src/main.rs`, `src/components/confirm_dialog.rs`, `src/db.rs` |
| Weekly grid | `views::weekly::view` | `src/views/weekly.rs`, `src/models.rs`, `src/theme.rs` |
| Streak display | `Database::calculate_streaks`, `streak_badge::view` | `src/db.rs`, `src/components/streak_badge.rs`, `src/components/habit_card.rs` |
| Stats dashboard summary | `views::stats::view`, `top_level_stats` | `src/views/stats.rs`, `src/db.rs`, `src/models.rs` |
| Category pie chart | `category_pie_chart::view` | `src/components/category_pie_chart.rs`, `src/views/stats.rs`, `src/models.rs` |
| Completion trend chart | `LineChart::new`, `Message::ChartAnimationTick` | `src/components/line_chart.rs`, `src/views/stats.rs`, `src/main.rs` |
| Heatmap | `global_heatmap` | `src/views/stats.rs`, `src/db.rs` |
| Theme selection | `Message::SelectTheme` | `src/main.rs`, `src/views/settings.rs`, `src/theme.rs`, `src/db.rs` |
| Export backup | `Message::RequestExport` | `src/main.rs`, `src/export.rs`, `src/db.rs`, `src/views/settings.rs` |
| Import backup | `Message::RequestImport`, `Message::ConfirmImport` | `src/main.rs`, `src/export.rs`, `src/db.rs`, `src/components/confirm_dialog.rs` |
| Factory reset | `Message::RequestFactoryReset`, `Message::ConfirmFactoryReset`, `Message::ConfirmFactoryResetWithBackup` | `src/main.rs`, `src/db.rs`, `src/export.rs`, `src/components/factory_reset_dialog.rs` |
| Fatal DB error screen | `HabitTracker::error_view` | `src/main.rs`, `src/db.rs`, `src/theme.rs` |
| Static app icon/logo | `app_icon::window_icon`, `icons::app_logo` | `src/app_icon.rs`, `src/icons.rs`, `assets/habit_logo.svg` |

## 15. 📝 QUICK REFERENCE CHEATSHEET

| Question | Answer |
|---|---|
| First file to understand | `src/main.rs` because it owns app state, messages, routing, side effects, and dialogs. |
| Most critical type | `HabitTracker` in `src/main.rs`; it is the app model/update/view coordinator. |
| Most critical persistence type | `Database` in `src/db.rs`; it owns schema, queries, stats, import/export data, and reset behavior. |
| Main persisted entity | `Habit` in `src/models.rs`, stored in the `habits` table. |
| Main derived view model | `HabitWithStats`, produced by `Database::get_all_habits_with_stats`. |
| DB location | `dirs::data_dir()/HabitFlow/data.db`, usually `%APPDATA%\HabitFlow\data.db` on Windows. |
| Backup format | Pretty JSON `ExportData` with `version`, `exported_at`, `habits`, and `daily_logs`. |
| Test command | `cargo test` currently passes 6 DB unit tests. |
| Build command | `cargo build`; release is `cargo build --release`. |

Things that are not obvious but matter:

| Gotcha | Why it matters |
|---|---|
| `src/repository.rs` is only a type alias. | The real repository code is `src/db.rs`; do not look for a separate repository abstraction. |
| `history_logs` is only 60 days. | The Stats heatmap says last year, but it cannot show older history with the current DB load path. |
| Dates are local, not UTC. | Changing timezone/date behavior needs coordinated DB, streak, and PRD decisions. |
| Current streak can continue from yesterday. | This is tested behavior and differs from a strict "completed today" definition. |
| Import replaces all current habit/log data. | Any merge/import enhancement must change both `src/export.rs` validation and `Database::import_data` semantics. |
| Theme is persisted in `app_meta`. | Factory reset clears metadata but `run_factory_reset` immediately re-saves the current theme. |
| Static SVG icons and Lucide font icons both exist. | `src/icons.rs` handles fixed UI assets; `src/lucide.rs` handles user-selectable habit icons. |
| `build.rs` generates required Rust code. | `src/lucide.rs` depends on `OUT_DIR/lucide_catalog.rs`; builds should go through Cargo. |
| No runtime env vars or external services exist. | Adding sync/auth/networking would be a major architectural change, not an integration toggle. |
| `cargo fmt -- --check` fails today. | Formatting cleanup is a separate change; avoid mixing it with behavioral edits unless requested. |

Common pitfalls:

| Pitfall | Safer approach |
|---|---|
| Editing `target/` artifacts | Treat `target/` as generated build output; source of truth is `src/`, `assets/`, `Cargo.toml`, `Cargo.lock`, and docs. |
| Assuming delete is archive | `archive_habit` is reversible; `delete_habit` is permanent and cascades logs. |
| Assuming import keeps current data | Import deletes all current habits/logs inside a transaction. |
| Assuming theme selection is cosmetic-only | Theme selection writes to SQLite metadata. |
| Adding fields without migration logic | Add schema changes through explicit initialization/migration paths; current pattern only supports simple `ALTER TABLE ADD COLUMN`. |
| Changing `HabitCategory` keys casually | DB/export values use `key()` strings; renaming keys affects existing data. |
| Changing icon names casually | Invalid icon values sanitize to `sparkles`, which can hide mistakes. |
| Running `cargo run -- --help` | There is no CLI help path; it starts the GUI app. |
