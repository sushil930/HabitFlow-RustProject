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
- Resolved `cargo-bundle` MSI generation failures by downloading portable WiX Toolset binaries and switching to `cargo-wix` for proper MSI building.
- Fixed the installer requiring admin privileges by completely rewriting `wix/main.wxs` to use `perUser` `InstallScope`, registry-based KeyPaths, and local app data directories.
- Programmatically generated a multi-size `assets/habit_logo.ico` file from the SVG source using Pillow.
- Configured the Rust build script (`build.rs`) using the `winresource` crate to embed the new ICO file directly into the Windows executable, enabling taskbar and window icons.
- Updated the MSI WiX configuration to generate Desktop and Start Menu shortcuts with the correct application icon.
- Updated `.gitignore` to track new artifacts, explicitly ignoring the `data/` folder, the temporary `wix314-binaries/` toolset directory, and stray `*.zip` or `*.msi` artifacts.
- Safety check: Resolved `target` to `C:\Users\sushi\Documents\proJects\da\habit-tracker\target`, which is inside the workspace.
- Safety check: No running `cargo`, `rustc`, or `habit-flow` process was found before cleaning.
- Build cleanup: Ran `cargo clean`; Cargo removed 17,780 generated files totaling 9.4 GiB.
- Build: Ran `cargo build` which successfully completed compiling the crate `habit-flow v0.1.1` and all dependencies.
