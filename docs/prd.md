# Product Requirements Document: Minimal Habit Tracker

**Version**: 1.0  
**Date**: 2026-04-07  
**Author**: Sarah (Product Owner)  
**Quality Score**: 92/100

---

## Executive Summary

Minimal Habit Tracker is a desktop-first, offline-only application designed to help individuals track daily habits with zero friction. The core thesis is simple: existing habit trackers are bloated, slow, and distraction-heavy — they've become dopamine casinos rather than productivity tools. This product strips habit tracking down to its essence: add a habit, check it off, see your streak.

The target audience is students, developers, and productivity-focused individuals who want a calm, fast, keyboard-friendly tool that stays out of their way. There are no social features, no cloud sync, no AI suggestions — just a clean interface that loads in under a second and does exactly one thing well.

Success is not measured by downloads or engagement metrics. It is measured by daily usage, zero crashes, and the ability to track a habit in under three clicks.

---

## Problem Statement

**Current Situation**: Existing habit tracking apps are over-engineered. They push notifications, require account creation, sync to the cloud, suggest habits via AI, and present dashboards that feel more like productivity theater than actual tools. Many are web-based or Electron apps that take 3–5 seconds to start. Users with a focus on simplicity and local-first workflows have no good desktop option.

**Proposed Solution**: A native desktop application (starting with Windows, cross-platform later) built in Rust with a minimal UI. Local storage only. Fast startup. No accounts. No internet required.

**Business Impact**: The product wins by being the fastest, quietest habit tracker available on desktop. Users who value calm productivity will adopt it precisely because it lacks features.

---

## Success Metrics

**Primary KPIs:**
- **Startup Time**: Cold launch completes in under 1 second
- **Daily Active Use**: User checks at least one habit within 30 seconds of opening the app
- **Crash Rate**: Zero crashes during normal usage (add/edit/delete/check habits)
- **Click-to-Track**: A habit can be marked complete in ≤ 2 clicks from app launch

**Validation**: Measured through local telemetry-free self-assessment during beta testing. No data leaves the device.

---

## User Personas

### Primary: The Focused Developer
- **Role**: Software developer, power user
- **Goals**: Track a small set of daily habits (exercise, reading, coding side project) without leaving their workflow
- **Pain Points**: Existing apps are Electron-based and slow; cloud sync they don't want; notifications that interrupt focus
- **Technical Level**: Advanced — appreciates keyboard shortcuts, local data, and fast tools

### Secondary: The Student
- **Role**: University student managing routines during high-pressure periods
- **Goals**: Build and maintain simple habits (study sessions, water intake, sleep schedule)
- **Pain Points**: Feature-heavy apps are distracting; they want something that takes 5 seconds to use, not 5 minutes
- **Technical Level**: Intermediate — comfortable with desktop apps but doesn't want to configure anything

### Tertiary: The Offline-First User
- **Role**: Anyone who prefers privacy-respecting, local software
- **Goals**: Track personal habits without any data leaving their machine
- **Pain Points**: Every productivity app now requires an account and cloud sync
- **Technical Level**: Novice to Advanced — the "offline-first" principle is the primary draw

---

## User Stories & Acceptance Criteria

### Story 1: Add a New Habit

**As a** focused developer  
**I want to** add a new habit in under 5 seconds  
**So that** I can start tracking it immediately without friction

**Acceptance Criteria:**
- [ ] User can create a new habit with only a name (no other required fields)
- [ ] Habit appears in the habit list immediately after creation
- [ ] Empty state shows a clear prompt to add the first habit

---

### Story 2: Daily Check-In

**As a** student  
**I want to** mark a habit as complete for today  
**So that** I can see my progress and maintain my streak

**Acceptance Criteria:**
- [ ] Each habit shows a checkbox or toggle on the Today view
- [ ] Checking a habit updates its streak count immediately
- [ ] A habit can be unchecked on the same day if checked by mistake
- [ ] A habit cannot be retroactively checked for past dates (enforces honest tracking)

---

### Story 3: Streak Tracking

**As a** productivity-focused user  
**I want to** see my current streak for each habit  
**So that** I stay motivated to maintain consistency

**Acceptance Criteria:**
- [ ] Current streak (consecutive days completed) is visible on each habit card
- [ ] Streak resets to 0 if a day is missed (no grace periods in MVP)
- [ ] Longest streak is stored and visible in habit stats

---

### Story 4: Weekly Overview

**As a** developer  
**I want to** see my habit completion across the past 7 days  
**So that** I can spot patterns without digging through complex charts

**Acceptance Criteria:**
- [ ] Weekly view shows a 7-day grid per habit (filled = completed, empty = missed)
- [ ] Today is clearly highlighted
- [ ] The view is navigable without leaving the main window

---

### Story 5: Edit and Delete Habits

**As a** student  
**I want to** rename or remove habits  
**So that** my list stays relevant as my routines change

**Acceptance Criteria:**
- [ ] User can rename a habit without losing its history
- [ ] User is prompted to confirm before deleting a habit
- [ ] Deleting a habit removes all associated daily log entries
- [ ] Deleted habits cannot be recovered (no soft delete in MVP)

---

## Functional Requirements

### Core Features

**Feature 1: Habit Management**
- Description: Create, rename, and delete habits
- User flow: Click "Add Habit" → Enter name → Press Enter or confirm → Habit appears in list
- Edge cases: Empty name blocked; duplicate names allowed (users may want "Morning Run" twice)
- Error handling: If storage write fails, show inline error; do not silently discard the habit

**Feature 2: Daily Tracking (Today View)**
- Description: Main screen showing all habits for the current day with check/uncheck controls
- User flow: Open app → See today's habits → Click checkbox → Streak updates
- Edge cases: App opened past midnight after last check-in; system date rollback (see Edge Cases section)
- Error handling: If date cannot be determined, show warning banner; block check-ins until resolved

**Feature 3: Streak Tracking**
- Description: Per-habit streak counter that auto-increments on daily completion and resets on a missed day
- User flow: Automatic — no user action required beyond daily check-in
- Edge cases: Timezone changes, missed days, first-time use
- Error handling: Streak recalculated from Daily Log on app start; never inferred from system time alone

**Feature 4: Weekly View**
- Description: 7-day habit grid showing completion history
- User flow: Navigate to Weekly tab → See grid → No interaction required
- Edge cases: Habits added mid-week show partial history (empty = not yet tracked)
- Error handling: Missing log entries treated as "not completed"

**Feature 5: Simple Stats**
- Description: Per-habit stats: current streak, longest streak, total completions, completion rate (last 30 days)
- User flow: Click on a habit → See stats panel
- Edge cases: New habits with <7 days of data show partial stats with context label

**Feature 6: Local Data Export / Import**
- Description: Export all data as JSON; import from JSON backup
- User flow: Settings → Export → Save file dialog; Settings → Import → File picker → Confirm overwrite
- Edge cases: Importing malformed JSON shows error; importing a backup with conflicting dates prompts user for merge strategy
- Error handling: Validate schema before import; never silently corrupt existing data

### Out of Scope (MVP)
- Cloud sync of any kind
- Social features, sharing, or leaderboards
- AI or ML-based habit suggestions
- Push or system notifications
- Dark/Light mode toggle (single theme at launch)
- Habit categories or tags
- Mobile companion app
- Widgets or system tray integration

---

## Technical Constraints

### Performance
- Cold startup time: < 1 second on a mid-range machine (Core i5, 8GB RAM)
- Memory usage at idle: < 100MB
- UI frame rendering: No janky transitions; target 60fps for all interactions
- Storage read/write: All operations complete in < 50ms for up to 1000 habits and 5 years of logs

### Security
- No network calls — ever. The app must function with no internet access
- No account creation, email, or credentials
- Local data stored in user's OS app data directory (e.g., `%APPDATA%\MinimalHabitTracker` on Windows)
- Export files are plaintext JSON — user is responsible for their security

### Integration
- None. This product has zero external integrations by design.

### Technology Stack
- **Language**: Rust
- **UI Framework**: Iced (recommended) — native, fast, cross-platform ready
- **Storage**: SQLite via `rusqlite` (preferred over JSON for long-term data integrity and query performance)
- **Platform**: Windows first; macOS and Linux in Phase 2
- **Packaging**: Single `.exe` binary; no installer required for MVP

---

## Data Model

### Habit

| Field | Type | Notes |
|---|---|---|
| `id` | UUID | Auto-generated on creation |
| `name` | String | Max 100 chars; required |
| `created_date` | Date | UTC date of creation |
| `sort_order` | Integer | For manual reordering (future) |

### DailyLog

| Field | Type | Notes |
|---|---|---|
| `habit_id` | UUID | Foreign key → Habit.id |
| `date` | Date | UTC date (not datetime) |
| `completed` | Boolean | True = checked that day |

**Derived fields** (computed at query time, never stored):
- `current_streak`: Consecutive completed days ending today
- `longest_streak`: Max consecutive days from full log history
- `total_completions`: COUNT of completed DailyLog entries

---

## Edge Cases

| Scenario | Expected Behaviour |
|---|---|
| **Missed days** | Streak resets to 0; missed days show as empty in weekly view |
| **Timezone change** | Dates stored in UTC; displayed in local time; if timezone shifts mid-day, recalculate today's date and prompt user if discrepancy detected |
| **System date rolled back** | Detect if stored "last used date" is in the future relative to system date; show warning and block check-ins; do not auto-correct data |
| **Checking a habit twice** | Second click unchecks; no double-counting |
| **Deleting a habit with history** | Confirm dialog warns that history will be permanently deleted; DailyLog rows cascade-deleted |
| **App opened exactly at midnight** | Determine "today" once on app open; do not shift mid-session |
| **Corrupt SQLite file** | Show error screen with instructions to restore from export backup; never auto-delete the file |
| **Very long habit name** | Truncate display with ellipsis; full name shown on hover/focus |
| **Large data volume** (500+ habits, 5+ years) | Paginate or virtualize habit list; ensure query performance stays within < 50ms |

---

## MVP Scope & Phasing

### Phase 1: MVP (Launch)
- Add / Edit / Delete habits
- Daily check/uncheck (Today view)
- Streak tracking (current + longest)
- Weekly view (7-day grid)
- Simple per-habit stats
- Local SQLite storage
- JSON Export / Import
- Windows binary (.exe), no installer

**MVP Definition**: A user can open the app, add habits, check them off daily, and see their streak — all offline, all in under 1 second.

### Phase 2: Enhancements (Post-Launch)
- Dark mode / Light mode toggle
- Habit categories and tags
- Progress charts (30/90-day views)
- Keyboard shortcuts for power users
- Optional minimal reminders (OS-level, not notification spam)
- macOS and Linux builds

### Future Scope (Resist Until Phase 3+)
- Optional cloud sync (self-hosted or encrypted)
- Mobile app (iOS / Android) with local-first sync
- Home screen / desktop widgets
- Themes and custom accent colors
- Habit templates and community sharing

---

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|---|---|---|---|
| Iced UI framework immaturity | Medium | High | Evaluate iced v0.12+ stability; fallback to egui if needed |
| SQLite corruption on hard shutdown | Low | High | Use WAL mode; implement startup integrity check |
| Windows-only scope limits early adoption | Medium | Medium | Communicate cross-platform roadmap clearly; ship macOS build in Phase 2 |
| Feature creep from user feedback | High | Medium | Maintain strict Non-Goals list; defer all requests to Phase 2+ backlog |
| User confusion about no cloud backup | Medium | Low | Make export/import prominent in onboarding; explain offline-first philosophy |

---

## Dependencies & Blockers

**Dependencies:**
- Rust stable toolchain (1.75+)
- `iced` crate for UI rendering
- `rusqlite` crate for SQLite
- Windows SDK for packaging (Phase 1 only)

**Known Blockers:**
- None at time of writing. No external services, no APIs, no accounts.

---

## Appendix

### Glossary
- **Streak**: The number of consecutive days a habit has been completed, ending on today's date
- **Daily Log**: A record of whether a habit was completed on a given date
- **Offline-first**: The application functions entirely without internet access; no features are degraded when offline
- **Calm productivity**: A design philosophy prioritising focus and reduced cognitive load over engagement and feature density

### Design Principles (UI/UX)
- Minimal: Every element earns its place
- Fast: No loading spinners for local data
- No clutter: One primary action visible at a time
- Keyboard-friendly: All core actions accessible via keyboard in Phase 2
- No floating glass morphing neon nonsense

---

*This PRD was created through interactive requirements gathering with quality scoring (92/100) to ensure comprehensive coverage of business, functional, UX, and technical dimensions.*
