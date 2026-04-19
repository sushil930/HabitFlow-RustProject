use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let source = find_lucide_source().unwrap_or_else(|| {
        panic!(
            "Could not locate the lucide-icons source file in the local Cargo registry."
        )
    });

    println!("cargo:rerun-if-changed={}", source.display());

    let contents = fs::read_to_string(&source).unwrap_or_else(|error| {
        panic!("Failed to read {}: {error}", source.display());
    });

    let variants = extract_icon_variants(&contents);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    let destination = out_dir.join("lucide_catalog.rs");
    let mut file = File::create(&destination).unwrap_or_else(|error| {
        panic!("Failed to create {}: {error}", destination.display());
    });

    writeln!(file, "pub const ICON_CATALOG: &[Icon] = &[").unwrap();
    for variant in variants {
        writeln!(file, "    Icon::{variant},").unwrap();
    }
    writeln!(file, "];\n").unwrap();
}

fn find_lucide_source() -> Option<PathBuf> {
    let mut roots = Vec::new();

    if let Some(cargo_home) = env::var_os("CARGO_HOME") {
        roots.push(PathBuf::from(cargo_home));
    }

    if let Some(profile) = env::var_os("USERPROFILE") {
        roots.push(PathBuf::from(profile).join(".cargo"));
    }

    if let Some(home) = env::var_os("HOME") {
        roots.push(PathBuf::from(home).join(".cargo"));
    }

    for root in roots {
        let registry_src = root.join("registry").join("src");
        if !registry_src.exists() {
            continue;
        }

        if let Some(found) = search_for_lucide_dir(&registry_src) {
            return Some(found.join("src").join("icon.rs"));
        }
    }

    None
}

fn search_for_lucide_dir(path: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(path).ok()?;

    for entry in entries.flatten() {
        let entry_path = entry.path();

        if entry_path.is_dir() {
            if entry_path
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with("lucide-icons-"))
                .unwrap_or(false)
                && entry_path.join("src").join("icon.rs").exists()
            {
                return Some(entry_path);
            }

            if let Some(found) = search_for_lucide_dir(&entry_path) {
                return Some(found);
            }
        }
    }

    None
}

fn extract_icon_variants(contents: &str) -> Vec<String> {
    let mut in_enum = false;
    let mut variants = Vec::new();

    for line in contents.lines() {
        let trimmed = line.trim();

        if !in_enum {
            if trimmed.starts_with("pub enum Icon {") {
                in_enum = true;
            }

            continue;
        }

        if trimmed.starts_with('}') {
            break;
        }

        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with("///")
            || trimmed.starts_with("//")
        {
            continue;
        }

        let candidate = trimmed
            .split(['=', '(', '{'])
            .next()
            .unwrap_or("")
            .trim_end_matches(',')
            .trim();

        if !candidate.is_empty()
            && candidate
                .chars()
                .all(|character| character.is_ascii_alphanumeric())
        {
            variants.push(candidate.to_string());
        }
    }

    variants.sort();
    variants.dedup();
    variants
}