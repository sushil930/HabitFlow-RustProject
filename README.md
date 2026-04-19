# 🌿 Habit Flow

> A calm, offline-first desktop habit tracker built with Rust and the Iced GUI framework.

---

## 📖 Overview

**Habit Flow** is a minimal, distraction-free desktop application designed to help you build and maintain daily habits. It runs entirely offline — your data never leaves your machine. Built with Rust for performance and reliability, and powered by the [Iced](https://github.com/iced-rs/iced) GUI framework for a smooth, native-feeling interface.

---

## ✨ Features

- ✅ **Today View** — See and check off all your habits for the current day at a glance
- 📅 **Weekly View** — Review your habit completion across the past week in a clean grid layout
- 📊 **Statistics Dashboard** — Track streaks, completion rates, and long-term progress per habit
- 🗂️ **Categories** — Organize habits into 11 categories: General, Health, Fitness, Mindfulness, Learning, Productivity, Work, Home, Social, Finance, and Creative
- 🎨 **Custom Icons** — Each habit and category gets a Lucide icon for quick visual recognition
- 🗄️ **Archived Habits** — Archive habits you've paused without losing their history
- ⚙️ **Settings** — Manage app preferences, data export, and import
- 💾 **JSON Export / Import** — Back up and restore all your habits and logs via a portable JSON file
- 🔒 **Fully Offline** — All data stored locally using SQLite; no accounts, no cloud, no tracking
- 🪟 **Clean Windows Build** — Console window hidden in release builds for a polished desktop experience

---

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| Language | [Rust](https://www.rust-lang.org/) (Edition 2021) |
| GUI Framework | [Iced](https://github.com/iced-rs/iced) v0.13 |
| Database | [SQLite](https://www.sqlite.org/) via `rusqlite` (bundled) |
| Icons | [Lucide Icons](https://lucide.dev/) via `lucide-icons` |
| Serialization | `serde` + `serde_json` |
| Date/Time | `chrono` |
| ID Generation | `uuid` v4 |
| File Dialogs | `rfd` (Rusty File Dialogs) |
| Platform Dirs | `dirs` |

---

## 📁 Project Structure

```
Desktop-Application-RustProject/
├── src/
│   ├── main.rs           # App entry point, state management, Iced update/view loop
│   ├── models.rs         # Core data types: Habit, DailyLog, HabitCategory, HabitWithStats
│   ├── db.rs             # All SQLite database operations (CRUD for habits and logs)
│   ├── export.rs         # JSON export and import logic
│   ├── theme.rs          # Custom Iced theme and styling definitions
│   ├── icons.rs          # Icon helpers and mappings
│   ├── lucide.rs         # Lucide icon integration with Iced
│   ├── app_icon.rs       # Application window icon setup
│   ├── repository.rs     # Repository module re-exports
│   ├── views/
│   │   ├── mod.rs        # View module definitions
│   │   ├── today.rs      # Today's habit checklist view
│   │   ├── weekly.rs     # Weekly completion grid view
│   │   ├── stats.rs      # Per-habit statistics and streak view
│   │   ├── archived.rs   # Archived habits management view
│   │   ├── sidebar.rs    # Navigation sidebar
│   │   └── settings.rs   # Settings and data management view
│   └── components/       # Reusable UI components
├── assets/               # Static assets (app icon, SVGs)
├── docs/                 # Additional documentation
├── build.rs              # Build script (Windows icon embedding, etc.)
├── Cargo.toml            # Project manifest and dependencies
└── Cargo.lock            # Locked dependency versions
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable, edition 2021 or later)
- Cargo (included with Rust)
- On **Linux**: you may need system libraries for GUI rendering (e.g., `libxkbcommon`, `libvulkan` or `libGL`)

### Clone the Repository

```bash
git clone https://github.com/sushil930/Desktop-Application-RustProject.git
cd Desktop-Application-RustProject
```

### Run in Development Mode

```bash
cargo run
```

### Build for Release

```bash
cargo build --release
```

The optimized binary will be located at `target/release/habit-flow` (or `habit-flow.exe` on Windows).

> **Note:** The release build applies LTO, symbol stripping, and size optimization (`opt-level = "s"`) for a lean final binary.

---

## 💡 How It Works

### Data Storage

Habit Flow uses an embedded **SQLite** database stored in your system's local app data directory (resolved via the `dirs` crate). No setup required — the database is created automatically on first launch.

### Habit Categories

Each habit belongs to one of 11 categories, each with a dedicated Lucide icon:

| Category | Icon |
|---|---|
| General | ✨ Sparkles |
| Health | 💓 Heart Pulse |
| Fitness | 🏋️ Dumbbell |
| Mindfulness | 🧠 Brain Circuit |
| Learning | 📖 Book Open |
| Productivity | 📋 Clipboard List |
| Work | 💼 Briefcase |
| Home | 🏠 Home |
| Social | 👥 Users |
| Finance | 👛 Wallet |
| Creative | 🎨 Palette |

### Statistics Tracked Per Habit

- 🔥 **Current Streak** — consecutive days completed up to today
- 🏆 **Longest Streak** — all-time best streak
- ✅ **Total Completions** — total days the habit was completed
- 📆 **Total Logged Days** — total days the habit has been tracked
- 📈 **Last 30-Day Rate** — completion percentage over the past 30 days

### Export & Import

You can export all your habits and daily logs to a `.json` file from the **Settings** view. This file can be used to migrate data between machines or restore from a backup using the import option.

---

## 🔧 Build Configuration

The `Cargo.toml` release profile is tuned for a compact, fast binary:

```toml
[profile.release]
lto = true           # Link-Time Optimization for smaller binary
strip = true         # Strip debug symbols
opt-level = "s"      # Optimize for size
codegen-units = 1    # Single codegen unit for better optimization
```

On Windows, `winapi` is used to suppress the console window in release builds, giving the app a native desktop feel.

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to open an issue or submit a pull request.

1. Fork the repository
2. Create your feature branch: `git checkout -b feature/your-feature`
3. Commit your changes: `git commit -m 'feat: add your feature'`
4. Push to the branch: `git push origin feature/your-feature`
5. Open a Pull Request

---

## 📄 License

This project is open source. Please check the repository for license details.

---

## 👤 Author

**sushi** — [@sushil930](https://github.com/sushil930)

---

*Built with 🦀 Rust and a calm mindset.*
