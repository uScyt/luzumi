use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Clone, Serialize, Debug)]
pub struct AppInfo {
    pub desktop_id: String,
    pub name: String,
    pub icon: String,
    pub is_default: bool,
}

/// Get the MIME type for a file or directory.
fn get_mime_type(path: &Path) -> String {
    if path.is_dir() {
        return "inode/directory".to_string();
    }

    // Try xdg-mime first (most accurate)
    if let Ok(output) = Command::new("xdg-mime")
        .args(["query", "filetype", &path.to_string_lossy()])
        .output()
    {
        if output.status.success() {
            let mime = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !mime.is_empty() {
                return mime;
            }
        }
    }

    // Fallback to mime_guess
    mime_guess::from_path(path)
        .first()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string())
}

/// Get the default application desktop ID for a MIME type.
fn get_default_app(mime_type: &str) -> Option<String> {
    let output = Command::new("xdg-mime")
        .args(["query", "default", mime_type])
        .output()
        .ok()?;
    if output.status.success() {
        let id = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !id.is_empty() {
            return Some(id);
        }
    }
    None
}

/// Directories to scan for .desktop files.
fn desktop_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![
        PathBuf::from("/usr/share/applications"),
        PathBuf::from("/usr/local/share/applications"),
    ];
    if let Some(home) = dirs::home_dir() {
        dirs.push(home.join(".local/share/applications"));
        dirs.push(home.join(".local/share/flatpak/exports/share/applications"));
    }
    dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));

    // Also check XDG_DATA_DIRS
    if let Ok(xdg) = std::env::var("XDG_DATA_DIRS") {
        for dir in xdg.split(':') {
            let app_dir = PathBuf::from(dir).join("applications");
            if !dirs.contains(&app_dir) {
                dirs.push(app_dir);
            }
        }
    }

    dirs
}

/// Parse a single .desktop file and extract relevant fields.
fn parse_desktop_file(path: &Path) -> Option<DesktopEntry> {
    let content = fs::read_to_string(path).ok()?;
    let mut name = String::new();
    let mut icon = String::new();
    let mut exec = String::new();
    let mut mime_types = Vec::new();
    let mut no_display = false;
    let mut hidden = false;
    let mut entry_type = String::new();
    let mut in_desktop_entry = false;

    for line in content.lines() {
        let line = line.trim();
        if line == "[Desktop Entry]" {
            in_desktop_entry = true;
            continue;
        }
        if line.starts_with('[') && line != "[Desktop Entry]" {
            if in_desktop_entry {
                break; // We've left the [Desktop Entry] section
            }
            continue;
        }
        if !in_desktop_entry {
            continue;
        }

        if let Some(val) = line.strip_prefix("Name=") {
            if name.is_empty() { // Only take first Name= (not localized ones)
                name = val.to_string();
            }
        } else if let Some(val) = line.strip_prefix("Icon=") {
            icon = val.to_string();
        } else if let Some(val) = line.strip_prefix("Exec=") {
            exec = val.to_string();
        } else if let Some(val) = line.strip_prefix("MimeType=") {
            mime_types = val.split(';').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        } else if let Some(val) = line.strip_prefix("NoDisplay=") {
            no_display = val.eq_ignore_ascii_case("true");
        } else if let Some(val) = line.strip_prefix("Hidden=") {
            hidden = val.eq_ignore_ascii_case("true");
        } else if let Some(val) = line.strip_prefix("Type=") {
            entry_type = val.to_string();
        }
    }

    if entry_type != "Application" || name.is_empty() || exec.is_empty() {
        return None;
    }
    if no_display || hidden {
        return None;
    }

    Some(DesktopEntry {
        name,
        icon,
        exec,
        mime_types,
    })
}

struct DesktopEntry {
    name: String,
    icon: String,
    exec: String,
    mime_types: Vec<String>,
}

/// Also check mimeapps.list files for additional associations.
fn get_mimeapps_associations(mime_type: &str) -> Vec<String> {
    let mut desktop_ids = Vec::new();
    let mut paths = vec![
        PathBuf::from("/usr/share/applications/mimeapps.list"),
        PathBuf::from("/usr/share/applications/defaults.list"),
    ];
    if let Some(config) = dirs::config_dir() {
        paths.insert(0, config.join("mimeapps.list"));
    }
    if let Some(home) = dirs::home_dir() {
        paths.insert(0, home.join(".local/share/applications/mimeapps.list"));
    }

    for path in paths {
        if let Ok(content) = fs::read_to_string(&path) {
            let mut in_section = false;
            for line in content.lines() {
                let line = line.trim();
                if line == "[Added Associations]" || line == "[Default Applications]" {
                    in_section = true;
                    continue;
                }
                if line.starts_with('[') {
                    in_section = false;
                    continue;
                }
                if !in_section {
                    continue;
                }
                if let Some(val) = line.strip_prefix(&format!("{}=", mime_type)) {
                    for id in val.split(';').filter(|s| !s.is_empty()) {
                        if !desktop_ids.contains(&id.to_string()) {
                            desktop_ids.push(id.to_string());
                        }
                    }
                }
            }
        }
    }

    desktop_ids
}

/// List applications that can open the given file/directory.
pub fn list_apps_for_path(path: &Path) -> Vec<AppInfo> {
    let mime_type = get_mime_type(path);
    let default_app = get_default_app(&mime_type);

    // Collect all desktop files
    let mut all_entries: HashMap<String, DesktopEntry> = HashMap::new();
    for dir in desktop_dirs() {
        if !dir.exists() {
            continue;
        }
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let file_path = entry.path();
                if file_path.extension().map(|e| e == "desktop").unwrap_or(false) {
                    let Some(fname) = file_path.file_name() else { continue };
                    let desktop_id = fname.to_string_lossy().to_string();
                    if all_entries.contains_key(&desktop_id) {
                        continue; // First found wins (user dirs checked first)
                    }
                    if let Some(de) = parse_desktop_file(&file_path) {
                        all_entries.insert(desktop_id, de);
                    }
                }
            }
        }
    }

    // Get apps associated via mimeapps.list
    let mimeapps_ids = get_mimeapps_associations(&mime_type);

    // Build result: apps that declare this MIME type OR are in mimeapps.list
    let mut matched_ids: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    // First: mimeapps.list associations (in order)
    for id in &mimeapps_ids {
        if all_entries.contains_key(id) && seen.insert(id.clone()) {
            matched_ids.push(id.clone());
        }
    }

    // Then: apps that declare this MIME type in their MimeType= field
    for (id, entry) in &all_entries {
        if entry.mime_types.contains(&mime_type) && seen.insert(id.clone()) {
            matched_ids.push(id.clone());
        }
    }

    // Build AppInfo list
    let mut apps: Vec<AppInfo> = matched_ids
        .into_iter()
        .filter_map(|id| {
            let entry = all_entries.get(&id)?;
            Some(AppInfo {
                is_default: default_app.as_ref() == Some(&id),
                desktop_id: id,
                name: entry.name.clone(),
                icon: entry.icon.clone(),
            })
        })
        .collect();

    // Sort: default first, then alphabetical
    apps.sort_by(|a, b| {
        match (a.is_default, b.is_default) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    apps
}

/// Open a file/directory with a specific application identified by its .desktop file ID.
pub fn open_with_app(path: &str, desktop_id: &str) -> Result<(), String> {
    // Resolve the full path to the .desktop file
    let desktop_file_path = desktop_dirs()
        .iter()
        .map(|dir| dir.join(desktop_id))
        .find(|p| p.exists());

    // Try gio launch with the full .desktop file path (non-blocking)
    if let Some(ref desktop_path) = desktop_file_path {
        if Command::new("gio")
            .args(["launch", &desktop_path.to_string_lossy(), path])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .is_ok()
        {
            return Ok(());
        }
    }

    // Try gtk-launch (accepts desktop ID without .desktop extension)
    let id_no_ext = desktop_id.strip_suffix(".desktop").unwrap_or(desktop_id);
    if Command::new("gtk-launch")
        .args([id_no_ext, path])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok()
    {
        return Ok(());
    }

    // Final fallback: parse the .desktop file and run Exec manually
    if let Some(ref desktop_path) = desktop_file_path {
        if let Some(entry) = parse_desktop_file(desktop_path) {
            return launch_exec(&entry.exec, path);
        }
    }

    Err(format!("Could not find or launch application: {}", desktop_id))
}

/// Parse Exec= field and launch with the given file path.
fn launch_exec(exec: &str, file_path: &str) -> Result<(), String> {
    // Remove field codes: %f %F %u %U %i %c %k
    let mut parts: Vec<String> = Vec::new();
    let mut added_path = false;

    for token in exec.split_whitespace() {
        match token {
            "%f" | "%F" => {
                parts.push(file_path.to_string());
                added_path = true;
            }
            "%u" | "%U" => {
                // Convert to file:// URI if it's a local path
                if file_path.starts_with('/') {
                    parts.push(format!("file://{}", file_path));
                } else {
                    parts.push(file_path.to_string());
                }
                added_path = true;
            }
            "%i" | "%c" | "%k" => {} // Skip these field codes
            _ if token.starts_with('%') => {} // Skip unknown field codes
            _ => parts.push(token.to_string()),
        }
    }

    // If no field code was found, append the path at the end
    if !added_path {
        parts.push(file_path.to_string());
    }

    if parts.is_empty() {
        return Err("Empty Exec command".to_string());
    }

    Command::new(&parts[0])
        .args(&parts[1..])
        .spawn()
        .map_err(|e| format!("Failed to launch {}: {}", parts[0], e))?;

    Ok(())
}
