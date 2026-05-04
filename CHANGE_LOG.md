# CHANGE_LOG

## CURRENT STATE
- Project: `habit-flow`, Rust/Iced desktop habit tracker.
- Application has been successfully rebuilt from source.
- Generated `assets/habit_logo.ico` from SVG.
- Configured Windows Resource compiler to embed the logo into the `.exe`.
- Rewrote the `cargo-wix` configuration to use a `perUser` install scope (no admin needed), proper Start Menu/Desktop shortcuts, and proper cleanup.
- A fully working, user-scoped MSI installer with an application icon is now generated at `target\wix\habit-flow-0.1.1-x86_64.msi`.

## OPEN TASKS
- None currently. Waiting for the next feature or bug fix request.

## 2026-05-04 Session
- Decision: Treat "current build files" as Cargo build artifacts under `target/`, not source/config/docs.
- Safety check: Resolved `target` to `C:\Users\sushi\Documents\proJects\da\habit-tracker\target`, which is inside the workspace.
- Safety check: No running `cargo`, `rustc`, or `habit-flow` process was found before cleaning.
- Build cleanup: Ran `cargo clean`; Cargo removed 17,780 generated files totaling 9.4 GiB.
- Build: Ran `cargo build` which successfully completed compiling the crate `habit-flow v0.1.1` and all dependencies.
