mod filesystem;
mod desktop_apps;
mod errors;
mod search;
mod preview;
mod tags;
mod compare;
mod logging;
mod operation_queue;
mod plugins;
mod vault;

use std::path::Path;
use std::fs;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use tauri::Emitter;
use tauri::menu::{MenuBuilder, MenuItem};
use notify::{Watcher, RecursiveMode};
use filesystem::{
    BookmarkEntry, DriveInfo, FileEntry, FileDetails, DuplicateGroup, TrashItemInfo,
    read_directory, elevated_read_directory, create_directory, rename_entry,
    delete_entries, copy_entry, move_entry,
    get_bookmarks, get_drives, mount_drive, unlock_drive, eject_drive,
    read_thumbnail, get_file_details,
    elevated_move, elevated_copy, elevated_delete, elevated_rename, elevated_create_directory,
    authenticate_admin, kill_root_shell, check_path_readable,
    list_all_trash, get_all_trash_size, empty_all_trash, restore_from_trash,
    get_quick_access, add_quick_access, remove_quick_access,
    count_secure_targets, secure_delete_streamed,
    calculate_dir_size, find_duplicates,
    copy_dir_all,
    set_permissions, create_symlink, get_trash_item_info,
};

static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

#[derive(Clone, serde::Serialize)]
struct DeleteProgress {
    current: String,
    done: usize,
    total: usize,
}

#[tauri::command]
fn cmd_cancel_operation() {
    CANCEL_FLAG.store(true, Ordering::Relaxed);
}

#[tauri::command]
async fn list_directory(path: String, show_hidden: bool) -> Result<Vec<FileEntry>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        read_directory(Path::new(&path), show_hidden)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_elevated_list_directory(path: String, show_hidden: bool) -> Result<Vec<FileEntry>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        elevated_read_directory(Path::new(&path), show_hidden)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let p = std::path::Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {}", path));
    }
    // Check if the file is a native executable (not a .desktop file or script handled by xdg-open)
    if let Ok(meta) = p.metadata() {
        let mode = meta.permissions().mode();
        let is_executable = mode & 0o111 != 0;
        let is_regular = meta.is_file();
        if is_executable && is_regular {
            // Check if it's a binary (not a text file that xdg-open should handle)
            let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
            let text_exts = ["sh", "bash", "zsh", "py", "pl", "rb", "js", "ts", "lua",
                             "desktop", "txt", "md", "html", "css", "json", "xml", "yaml", "yml",
                             "toml", "ini", "cfg", "conf", "log", "csv"];
            if !text_exts.contains(&ext) {
                // Launch the executable directly
                std::process::Command::new(&path)
                    .current_dir(p.parent().unwrap_or(std::path::Path::new("/")))
                    .spawn()
                    .map_err(|e| e.to_string())?;
                return Ok(());
            }
        }
    }
    // For everything else, use xdg-open
    open::that(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_list_open_with_apps(path: String) -> Vec<desktop_apps::AppInfo> {
    desktop_apps::list_apps_for_path(std::path::Path::new(&path))
}

#[tauri::command]
fn cmd_open_with_app(path: String, desktop_id: String) -> Result<(), String> {
    desktop_apps::open_with_app(&path, &desktop_id)
}

#[tauri::command]
fn cmd_open_with_command(path: String, command: String) -> Result<(), String> {
    use std::process::Command;
    // Validate: command must be a simple executable name (no shell metacharacters)
    let cmd_trimmed = command.trim();
    if cmd_trimmed.is_empty() {
        return Err("Command cannot be empty".into());
    }
    // Block shell metacharacters to prevent injection
    const BLOCKED: &[char] = &[';', '|', '&', '$', '`', '(', ')', '{', '}', '<', '>', '\n', '\r', '\\', '\'', '"', '!', '#'];
    if cmd_trimmed.chars().any(|c| BLOCKED.contains(&c)) {
        return Err("Command contains invalid characters".into());
    }
    // Validate the path exists
    if !std::path::Path::new(&path).exists() {
        return Err(format!("File not found: {}", path));
    }
    // Split command into program + args safely (no shell involved)
    let parts: Vec<&str> = cmd_trimmed.split_whitespace().collect();
    let program = parts[0];
    let mut cmd = Command::new(program);
    for arg in &parts[1..] {
        cmd.arg(arg);
    }
    cmd.arg(&path);
    cmd.spawn().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn cmd_set_default_app(mime_type: String, desktop_id: String) -> Result<(), String> {
    use std::process::Command;
    // Validate inputs to prevent argument injection
    if mime_type.is_empty() || desktop_id.is_empty() {
        return Err("mime_type and desktop_id are required".into());
    }
    // desktop_id must look like "app.desktop"
    if !desktop_id.ends_with(".desktop") || desktop_id.contains('/') || desktop_id.contains('\0') {
        return Err("Invalid desktop_id format".into());
    }
    // mime_type must look like "type/subtype"
    if !mime_type.contains('/') || mime_type.contains('\0') || mime_type.contains(' ') {
        return Err("Invalid mime_type format".into());
    }
    let output = Command::new("xdg-mime")
        .arg("default")
        .arg(&desktop_id)
        .arg(&mime_type)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(())
}

#[tauri::command]
fn cmd_search_index(query: String, limit: Option<usize>) -> Result<Vec<FileEntry>, String> {
    let limit = limit.unwrap_or(200);
    let db_path = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("luzumi")
        .join("index.db");

    if !db_path.exists() {
        return Ok(Vec::new());
    }

    let conn = rusqlite::Connection::open_with_flags(
        &db_path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
    ).map_err(|e| e.to_string())?;

    let query_lower = query.to_lowercase();
    let pattern = format!("%{}%", query_lower);

    let mut stmt = conn.prepare(
        "SELECT path, name, kind, size, modified, extension FROM files WHERE name_lower LIKE ?1 LIMIT ?2"
    ).map_err(|e| e.to_string())?;

    let entries = stmt.query_map(rusqlite::params![pattern, limit as i64], |row| {
        let path: String = row.get(0)?;
        let name: String = row.get(1)?;
        let kind: String = row.get(2)?;
        let size: Option<i64> = row.get(3)?;
        let modified: Option<i64> = row.get(4)?;
        let extension: Option<String> = row.get(5)?;

        let is_vault = extension.as_deref() == Some("luzumi-vault") && kind == "file";
        Ok(FileEntry {
            name: name.clone(),
            path,
            kind: kind.clone(),
            size: if kind == "file" || kind == "symlink" { size.map(|s| s as u64) } else { None },
            modified,
            is_hidden: name.starts_with('.'),
            extension,
            is_writable: true,
            permissions_mode: None,
            is_broken_link: false,
            is_vault,
        })
    }).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for entry in entries {
        if let Ok(e) = entry {
            result.push(e);
        }
    }

    Ok(result)
}

#[tauri::command]
fn cmd_create_directory(parent: String, name: String) -> Result<(), String> {
    create_directory(Path::new(&parent), &name)
}

#[tauri::command]
fn cmd_rename_entry(from: String, new_name: String) -> Result<(), String> {
    rename_entry(Path::new(&from), &new_name)
}

#[tauri::command]
fn cmd_delete_entries(paths: Vec<String>) -> Result<(), String> {
    let owned: Vec<std::path::PathBuf> = paths.iter().map(|p| std::path::PathBuf::from(p)).collect();
    let refs: Vec<&Path> = owned.iter().map(|p| p.as_path()).collect();
    delete_entries(&refs)
}

#[tauri::command]
async fn cmd_secure_delete_streamed(app: tauri::AppHandle, paths: Vec<String>) -> Result<(), String> {
    let owned: Vec<std::path::PathBuf> = paths.iter().map(|p| std::path::PathBuf::from(p)).collect();
    let refs: Vec<&Path> = owned.iter().map(|p| p.as_path()).collect();
    let total = count_secure_targets(&refs);
    let done = Arc::new(AtomicUsize::new(0));
    let app2 = app.clone();
    let done2 = done.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let refs2: Vec<&Path> = owned.iter().map(|p| p.as_path()).collect();
        secure_delete_streamed(&refs2, |current| {
            let n = done2.fetch_add(1, Ordering::Release);
            app2.emit("delete-progress", DeleteProgress {
                current: current.to_string(),
                done: n,
                total,
            }).ok();
        })
    }).await.map_err(|e| e.to_string())??;
    app.emit("delete-progress", DeleteProgress {
        current: String::new(),
        done: total,
        total,
    }).ok();
    Ok(())
}

#[derive(Clone, serde::Serialize)]
struct CopyMoveProgress {
    done: usize,
    total: usize,
    current: String,
}

#[tauri::command]
async fn cmd_copy_entries(app: tauri::AppHandle, paths: Vec<String>, dest: String) -> Result<(), String> {
    CANCEL_FLAG.store(false, Ordering::Relaxed);
    let total = paths.len();
    tauri::async_runtime::spawn_blocking(move || {
        let dest = Path::new(&dest);
        for (i, p) in paths.iter().enumerate() {
            if CANCEL_FLAG.load(Ordering::Relaxed) {
                app.emit("copy-move-progress", CopyMoveProgress { done: total, total, current: String::new() }).ok();
                return Err("Operation cancelled".into());
            }
            let name = Path::new(p).file_name().unwrap_or_default().to_string_lossy().to_string();
            app.emit("copy-move-progress", CopyMoveProgress { done: i, total, current: name }).ok();
            copy_entry(Path::new(p), dest)?;
        }
        app.emit("copy-move-progress", CopyMoveProgress { done: total, total, current: String::new() }).ok();
        Ok(())
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_move_entries(app: tauri::AppHandle, paths: Vec<String>, dest: String) -> Result<(), String> {
    CANCEL_FLAG.store(false, Ordering::Relaxed);
    let total = paths.len();
    tauri::async_runtime::spawn_blocking(move || {
        let dest_path = Path::new(&dest);
        for (i, p) in paths.iter().enumerate() {
            if CANCEL_FLAG.load(Ordering::Relaxed) {
                app.emit("copy-move-progress", CopyMoveProgress { done: total, total, current: String::new() }).ok();
                return Err("Operation cancelled".into());
            }
            let name = Path::new(p).file_name().unwrap_or_default().to_string_lossy().to_string();
            app.emit("copy-move-progress", CopyMoveProgress { done: i, total, current: name }).ok();
            move_entry(Path::new(p), dest_path)?;
        }
        app.emit("copy-move-progress", CopyMoveProgress { done: total, total, current: String::new() }).ok();
        Ok(())
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_copy_as(source: String, dest: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let src = Path::new(&source);
        let dst = Path::new(&dest);
        if src.is_dir() {
            copy_dir_all(src, dst)
        } else {
            fs::copy(src, dst).map(|_| ()).map_err(|e| e.to_string())
        }
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
fn cmd_create_file(parent: String, name: String) -> Result<(), String> {
    if name.contains('/') || name.contains("..") || name.contains('\0') {
        return Err("Invalid file name".into());
    }
    let path = Path::new(&parent).join(&name);
    fs::File::create(&path).map(|_| ()).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_open_terminal(path: String) -> Result<(), String> {
    let env_term = std::env::var("TERMINAL").unwrap_or_default();
    let candidates: Vec<&str> = {
        let mut v: Vec<&str> = Vec::new();
        if !env_term.is_empty() { v.push(&env_term); }
        v.extend(["kitty","alacritty","gnome-terminal","konsole","xfce4-terminal","xterm"]);
        v
    };
    for term in candidates {
        if std::process::Command::new(term).current_dir(&path).spawn().is_ok() {
            return Ok(());
        }
    }
    Err("No terminal emulator found. Install kitty, alacritty, or set $TERMINAL.".into())
}

#[tauri::command]
fn cmd_get_bookmarks() -> Vec<BookmarkEntry> {
    get_bookmarks()
}

#[tauri::command]
fn cmd_get_drives() -> Vec<DriveInfo> {
    get_drives()
}

#[tauri::command]
fn cmd_mount_drive(device: String) -> Result<String, String> {
    mount_drive(&device)
}

#[tauri::command]
fn cmd_unlock_drive(device: String, password: String) -> Result<String, String> {
    unlock_drive(&device, &password)
}

#[tauri::command]
fn cmd_eject_drive(device: String) -> Result<(), String> {
    eject_drive(&device)
}

#[tauri::command]
async fn cmd_read_thumbnail(path: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        read_thumbnail(Path::new(&path))
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_get_file_details(path: String) -> Result<FileDetails, String> {
    tauri::async_runtime::spawn_blocking(move || {
        get_file_details(Path::new(&path))
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
fn cmd_authenticate_admin() -> Result<(), String> {
    authenticate_admin()
}

#[tauri::command]
fn cmd_deauthenticate_admin() {
    kill_root_shell();
}

#[tauri::command]
fn cmd_check_path_readable(path: String) -> bool {
    check_path_readable(Path::new(&path))
}

#[tauri::command]
fn cmd_elevated_move(paths: Vec<String>, dest: String) -> Result<(), String> {
    let dest_path = Path::new(&dest);
    for p in &paths {
        elevated_move(Path::new(p), dest_path)?;
    }
    Ok(())
}

#[tauri::command]
fn cmd_elevated_copy(paths: Vec<String>, dest: String) -> Result<(), String> {
    let dest_path = Path::new(&dest);
    for p in &paths {
        elevated_copy(Path::new(p), dest_path)?;
    }
    Ok(())
}

#[tauri::command]
fn cmd_elevated_delete(paths: Vec<String>) -> Result<(), String> {
    let owned: Vec<std::path::PathBuf> = paths.iter().map(|p| std::path::PathBuf::from(p)).collect();
    let refs: Vec<&Path> = owned.iter().map(|p| p.as_path()).collect();
    elevated_delete(&refs)
}

#[tauri::command]
fn cmd_elevated_rename(from: String, new_name: String) -> Result<(), String> {
    elevated_rename(Path::new(&from), &new_name)
}

#[tauri::command]
fn cmd_elevated_create_directory(parent: String, name: String) -> Result<(), String> {
    elevated_create_directory(Path::new(&parent), &name)
}

#[tauri::command]
fn cmd_list_trash() -> Result<Vec<FileEntry>, String> {
    list_all_trash()
}

#[tauri::command]
fn cmd_get_trash_size() -> u64 {
    get_all_trash_size()
}

#[tauri::command]
fn cmd_empty_trash() -> Result<(), String> {
    empty_all_trash()
}

#[tauri::command]
fn cmd_restore_from_trash(file_name: String) -> Result<(), String> {
    restore_from_trash(&file_name)
}

#[tauri::command]
fn cmd_get_quick_access() -> Vec<BookmarkEntry> {
    get_quick_access()
}

#[tauri::command]
fn cmd_add_quick_access(name: String, path: String, icon: String) -> Result<(), String> {
    add_quick_access(&name, &path, &icon)
}

#[tauri::command]
fn cmd_remove_quick_access(path: String) -> Result<(), String> {
    remove_quick_access(&path)
}

#[tauri::command]
fn get_home_dir() -> String {
    dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/"))
        .to_string_lossy()
        .to_string()
}

#[tauri::command]
fn cmd_check_default_file_manager() -> bool {
    if let Ok(o) = std::process::Command::new("xdg-mime")
        .args(["query", "default", "inode/directory"])
        .output()
    {
        return String::from_utf8_lossy(&o.stdout)
            .trim().to_lowercase().contains("luzumi");
    }
    let home = dirs::home_dir().unwrap_or_default();
    std::fs::read_to_string(home.join(".config/mimeapps.list"))
        .map(|c| c.contains("inode/directory=luzumi"))
        .unwrap_or(false)
}

fn set_mime_default(content: String, mime: &str, desktop: &str) -> String {
    let entry = format!("{}={}", mime, desktop);
    if content.contains("[Default Applications]") {
        let mut result = String::new();
        let mut in_section = false;
        let mut written = false;
        for line in content.lines() {
            if line.starts_with('[') { in_section = line == "[Default Applications]"; }
            if in_section && line.starts_with(&format!("{}=", mime)) {
                result.push_str(&entry); result.push('\n');
                written = true; continue;
            }
            result.push_str(line); result.push('\n');
        }
        if !written { result.push_str(&entry); result.push('\n'); }
        result
    } else {
        format!("{}\n[Default Applications]\n{}\n", content.trim_end(), entry)
    }
}

#[tauri::command]
fn cmd_get_startup_path() -> String {
    let args: Vec<String> = std::env::args().collect();
    for arg in args.iter().skip(1) {
        if arg.starts_with("--") {
            continue;
        }
        let cleaned = if let Some(stripped) = arg.strip_prefix("file://") {
            percent_decode(stripped)
        } else {
            arg.clone()
        };
        if std::path::Path::new(&cleaned).is_dir() {
            return cleaned;
        }
    }
    String::new()
}

#[tauri::command]
fn cmd_get_startup_select() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .find_map(|a| a.strip_prefix("--select=").map(|s| s.to_string()))
}

fn percent_decode(s: &str) -> String {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(val) = u8::from_str_radix(
                &s[i + 1..i + 3], 16
            ) {
                result.push(val);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&result).to_string()
}

#[tauri::command]
fn cmd_set_as_default_file_manager() -> Result<(), String> {
    let home = dirs::home_dir().ok_or("Failed to find home directory")?;
    let apps_dir = home.join(".local/share/applications");
    std::fs::create_dir_all(&apps_dir).map_err(|e| e.to_string())?;
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let release_exe = exe.parent()
        .and_then(|p| p.parent()) // target/debug -> target
        .map(|target| target.join("release").join("luzumi"))
        .filter(|p| p.exists());
    let effective_exe = release_exe.as_deref().unwrap_or(&exe);
    let icons_dir = home.join(".local/share/icons/hicolor/256x256/apps");
    std::fs::create_dir_all(&icons_dir).ok();
    let icon_src = exe.parent()
        .and_then(|p| p.parent()) // target/debug -> target
        .and_then(|p| p.parent()) // target -> src-tauri
        .map(|src_tauri| src_tauri.join("icons").join("archive.png"))
        .filter(|p| p.exists());
    let icon_dest = icons_dir.join("luzumi.png");
    if let Some(src) = &icon_src {
        std::fs::copy(src, &icon_dest).ok();
    }
    let icon_value = if icon_dest.exists() {
        icon_dest.to_string_lossy().to_string()
    } else {
        "system-file-manager".to_string()
    };
    let desktop = format!(
        "[Desktop Entry]\nName=Luzumi\nExec={} %u\nIcon={}\nType=Application\nMimeType=inode/directory;\nCategories=System;FileTools;FileManager;\nTerminal=false\nStartupNotify=true\n",
        effective_exe.display(), icon_value
    );
    std::fs::write(apps_dir.join("luzumi.desktop"), desktop).map_err(|e| e.to_string())?;
    let ok = std::process::Command::new("xdg-mime")
        .args(["default", "luzumi.desktop", "inode/directory"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !ok {
        let mimeapps_path = home.join(".config/mimeapps.list");
        let content = std::fs::read_to_string(&mimeapps_path).unwrap_or_default();
        let updated = set_mime_default(content, "inode/directory", "luzumi.desktop");
        std::fs::write(mimeapps_path, updated).map_err(|e| e.to_string())?;
    }

    if !cmd_check_portal_installed() {
        cmd_install_portal()?;
    }

    Ok(())
}

#[tauri::command]
async fn cmd_calculate_dir_size(path: String) -> Result<u64, String> {
    tauri::async_runtime::spawn_blocking(move || {
        calculate_dir_size(Path::new(&path))
    }).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn cmd_find_duplicates(path: String, recursive: bool) -> Result<Vec<DuplicateGroup>, String> {
    let p = std::path::PathBuf::from(path);
    tauri::async_runtime::spawn_blocking(move || find_duplicates(&p, recursive))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_read_text_preview(path: String, max_lines: usize) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        use std::io::{BufRead, BufReader};
        let p = std::path::Path::new(&path);
        let meta = fs::symlink_metadata(p).map_err(|e| e.to_string())?;
        if !meta.is_file() {
            return Err("Not a regular file".into());
        }
        if meta.len() > 10 * 1024 * 1024 {
            return Err("File too large for preview (>10MB)".into());
        }
        let file = fs::File::open(&path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines()
            .take(max_lines)
            .filter_map(|l| l.ok())
            .collect();
        Ok(lines.join("\n"))
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_create_archive(paths: Vec<String>, dest: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let dest_path = std::path::PathBuf::from(&dest);
        let file = fs::File::create(&dest_path).map_err(|e| e.to_string())?;
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        for p in &paths {
            let path = std::path::PathBuf::from(p);
            let base_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            if path.is_dir() {
                add_dir_to_zip(&mut zip, &path, &base_name, options)?;
            } else {
                zip.start_file(&base_name, options).map_err(|e| e.to_string())?;
                let f = fs::File::open(&path).map_err(|e| e.to_string())?;
                let mut reader = std::io::BufReader::new(f);
                std::io::copy(&mut reader, &mut zip).map_err(|e| e.to_string())?;
            }
        }
        zip.finish().map_err(|e| e.to_string())?;
        Ok(())
    }).await.map_err(|e| e.to_string())?
}

fn add_dir_to_zip(
    zip: &mut zip::ZipWriter<fs::File>,
    dir: &std::path::Path,
    prefix: &str,
    options: zip::write::SimpleFileOptions,
) -> Result<(), String> {
    zip.add_directory(format!("{}/", prefix), options).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = format!("{}/{}", prefix, entry.file_name().to_string_lossy());
        if path.is_dir() {
            add_dir_to_zip(zip, &path, &name, options)?;
        } else {
            zip.start_file(&name, options).map_err(|e| e.to_string())?;
            let f = fs::File::open(&path).map_err(|e| e.to_string())?;
            let mut reader = std::io::BufReader::new(f);
            std::io::copy(&mut reader, zip).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
async fn cmd_extract_archive(archive: String, dest: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let file = fs::File::open(&archive).map_err(|e| e.to_string())?;
        let mut zip = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
        let dest_path = std::path::Path::new(&dest);
        let canonical_dest = dest_path.canonicalize().map_err(|e| e.to_string())?;

        for i in 0..zip.len() {
            let mut entry = zip.by_index(i).map_err(|e| e.to_string())?;
            let name = entry.mangled_name();
            let out_path = canonical_dest.join(&name);

            if !out_path.starts_with(&canonical_dest) {
                return Err(format!("Archive entry escapes destination: {}", name.display()));
            }

            if entry.is_dir() {
                fs::create_dir_all(&out_path).map_err(|e| e.to_string())?;
            } else {
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                let mut out_file = fs::File::create(&out_path).map_err(|e| e.to_string())?;
                std::io::copy(&mut entry, &mut out_file).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }).await.map_err(|e| e.to_string())?
}

// ── Picker mode (XDG Desktop Portal integration) ──

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct FileFilter {
    name: String,
    patterns: Vec<String>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PickerConfig {
    mode: String,            // "open" | "save"
    response_file: String,
    multiple: bool,
    directory: bool,
    title: String,
    filters: Vec<FileFilter>,
    current_folder: Option<String>,
    current_name: Option<String>,
}

fn parse_picker_args() -> Option<PickerConfig> {
    let args: Vec<String> = std::env::args().collect();
    let is_open = args.iter().any(|a| a == "--picker-open");
    let is_save = args.iter().any(|a| a == "--picker-save");
    if !is_open && !is_save {
        return None;
    }
    let mode = if is_open { "open" } else { "save" }.to_string();
    let get_val = |prefix: &str| -> Option<String> {
        args.iter().find_map(|a| a.strip_prefix(prefix).map(|s| s.to_string()))
    };
    let response_file = get_val("--response-file=").unwrap_or_default();
    let multiple = args.iter().any(|a| a == "--multiple");
    let directory = args.iter().any(|a| a == "--directory");
    let title = get_val("--title=").unwrap_or_else(|| {
        if is_open { "Open".to_string() } else { "Save".to_string() }
    });
    let filters: Vec<FileFilter> = get_val("--filters=")
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    let current_folder = get_val("--current-folder=");
    let current_name = get_val("--current-name=");
    Some(PickerConfig {
        mode, response_file, multiple, directory, title, filters,
        current_folder, current_name,
    })
}

#[tauri::command]
fn cmd_get_picker_config(state: tauri::State<'_, Mutex<Option<PickerConfig>>>) -> Result<Option<PickerConfig>, String> {
    Ok(state.lock().map_err(|e| format!("Lock error: {}", e))?.clone())
}

#[tauri::command]
fn cmd_submit_picker_result(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<Option<PickerConfig>>>,
    uris: Vec<String>,
) -> Result<(), String> {
    let config = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let response_file = config.as_ref().map(|c| c.response_file.clone()).unwrap_or_default();
    drop(config);
    if response_file.is_empty() {
        return Err("No response file configured".into());
    }
    let result = serde_json::json!({ "response": 0, "uris": uris });
    {
        use std::io::Write;
        let mut f = fs::File::create(&response_file).map_err(|e| e.to_string())?;
        f.write_all(result.to_string().as_bytes()).map_err(|e| e.to_string())?;
        f.sync_all().map_err(|e| e.to_string())?;
    }
    app.exit(0);
    Ok(())
}

#[tauri::command]
fn cmd_cancel_picker(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<Option<PickerConfig>>>,
) -> Result<(), String> {
    let config = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let response_file = config.as_ref().map(|c| c.response_file.clone()).unwrap_or_default();
    drop(config);
    if !response_file.is_empty() {
        let empty: Vec<String> = vec![];
        let result = serde_json::json!({ "response": 1, "uris": empty });
        {
            use std::io::Write;
            let mut f = fs::File::create(&response_file).map_err(|e| e.to_string())?;
            f.write_all(result.to_string().as_bytes()).map_err(|e| e.to_string())?;
            f.sync_all().map_err(|e| e.to_string())?;
        }
    }
    app.exit(0);
    Ok(())
}

// ── Desktop Menu mode ──

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DesktopMenuConfig {
    x: i32,
    y: i32,
}

fn parse_desktop_menu_args() -> Option<DesktopMenuConfig> {
    let args: Vec<String> = std::env::args().collect();
    if !args.iter().any(|a| a == "--desktop-menu") {
        return None;
    }
    let get_val = |prefix: &str| -> Option<String> {
        args.iter().find_map(|a| a.strip_prefix(prefix).map(|s| s.to_string()))
    };
    let x = get_val("--menu-x=").and_then(|s| s.parse().ok()).unwrap_or(0);
    let y = get_val("--menu-y=").and_then(|s| s.parse().ok()).unwrap_or(0);
    Some(DesktopMenuConfig { x, y })
}

#[tauri::command]
fn cmd_get_desktop_menu_config(state: tauri::State<'_, Mutex<Option<DesktopMenuConfig>>>) -> Result<Option<DesktopMenuConfig>, String> {
    Ok(state.lock().map_err(|e| format!("Lock error: {}", e))?.clone())
}

#[tauri::command]
async fn cmd_desktop_action(action: String) -> Result<(), String> {
    match action.as_str() {
        "lock-screen" => {
            std::process::Command::new("dbus-send")
                .args(["--session", "--dest=org.freedesktop.ScreenSaver",
                       "--type=method_call", "/ScreenSaver",
                       "org.freedesktop.ScreenSaver.Lock"])
                .spawn().map_err(|e| e.to_string())?;
        }
        "logout" => {
            std::process::Command::new("dbus-send")
                .args(["--session", "--dest=org.kde.Shutdown",
                       "--type=method_call", "/Shutdown",
                       "org.kde.Shutdown.logout",
                       "int32:0", "int32:0", "int32:0"])
                .spawn().map_err(|e| e.to_string())?;
        }
        "wallpaper-settings" => {
            std::process::Command::new("plasma-open-settings")
                .arg("kcm_wallpaper")
                .spawn()
                .or_else(|_| {
                    std::process::Command::new("kcmshell6")
                        .arg("kcm_lookandfeel")
                        .spawn()
                })
                .map_err(|e| e.to_string())?;
        }
        "display-config" => {
            std::process::Command::new("plasma-open-settings")
                .arg("kcm_kscreen")
                .spawn()
                .or_else(|_| {
                    std::process::Command::new("kcmshell6")
                        .arg("kcm_kscreen")
                        .spawn()
                })
                .map_err(|e| e.to_string())?;
        }
        _ => return Err(format!("Unknown desktop action: {}", action)),
    }
    Ok(())
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct DesktopExtensionItem {
    label: String,
    icon: Option<String>,
    exec: String,
    separator_before: Option<bool>,
}

#[tauri::command]
fn cmd_get_desktop_menu_extensions() -> Vec<DesktopExtensionItem> {
    let ext_dir = dirs::home_dir()
        .map(|h| h.join(".local/share/luzumi/desktop-menu.d"));
    let Some(dir) = ext_dir else { return vec![] };
    if !dir.exists() { return vec![]; }

    let mut items = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    #[derive(serde::Deserialize)]
                    struct ExtFile { items: Vec<DesktopExtensionItem> }
                    if let Ok(parsed) = serde_json::from_str::<ExtFile>(&content) {
                        items.extend(parsed.items);
                    }
                }
            }
        }
    }
    items
}

#[tauri::command]
fn cmd_exec_desktop_extension(exec: String) -> Result<(), String> {
    let parts: Vec<&str> = exec.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty command".into());
    }
    let mut cmd = std::process::Command::new(parts[0]);
    if parts.len() > 1 {
        cmd.args(&parts[1..]);
    }
    cmd.spawn().map_err(|e| e.to_string())?;
    Ok(())
}

// ── Portal install/uninstall ──

fn inject_filechooser_line(conf_path: &std::path::Path, desktop: &str) -> Result<(), String> {
    let existing = fs::read_to_string(conf_path).unwrap_or_default();
    if existing.contains("FileChooser=luzumi") {
        return Ok(());
    }
    if existing.contains("org.freedesktop.impl.portal.FileChooser") {
        let updated: String = existing.lines().map(|line| {
            if line.trim_start().starts_with("org.freedesktop.impl.portal.FileChooser") {
                "org.freedesktop.impl.portal.FileChooser=luzumi"
            } else {
                line
            }
        }).collect::<Vec<_>>().join("\n") + "\n";
        fs::write(conf_path, updated).map_err(|e| e.to_string())
    } else if existing.contains("[preferred]") {
        let updated = existing.replace(
            "[preferred]",
            "[preferred]\norg.freedesktop.impl.portal.FileChooser=luzumi"
        );
        fs::write(conf_path, updated).map_err(|e| e.to_string())
    } else {
        let default_backend = if desktop.contains("kde") { "kde" }
            else if desktop.contains("gnome") { "gtk" }
            else { "gtk" };
        fs::write(
            conf_path,
            format!("[preferred]\ndefault={}\norg.freedesktop.impl.portal.FileChooser=luzumi\n", default_backend),
        ).map_err(|e| e.to_string())
    }
}

fn remove_filechooser_line(conf_path: &std::path::Path) {
    if let Ok(content) = fs::read_to_string(conf_path) {
        if content.contains("FileChooser=luzumi") {
            let updated: String = content.lines()
                .filter(|line| !line.trim_start().starts_with("org.freedesktop.impl.portal.FileChooser=luzumi"))
                .collect::<Vec<_>>()
                .join("\n") + "\n";
            fs::write(conf_path, updated).ok();
        }
    }
}

#[tauri::command]
fn cmd_check_portal_installed() -> bool {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return false,
    };
    let portal_exists = home.join(".local/share/xdg-desktop-portal/portals/luzumi.portal").exists();
    let conf_dir = home.join(".config/xdg-desktop-portal");
    let has_config = ["portals.conf", "kde-portals.conf", "gnome-portals.conf"]
        .iter()
        .any(|f| {
            fs::read_to_string(conf_dir.join(f))
                .map(|c| c.contains("FileChooser=luzumi"))
                .unwrap_or(false)
        });
    portal_exists && has_config
}

#[tauri::command]
fn cmd_install_portal() -> Result<(), String> {
    let home = dirs::home_dir().ok_or("Failed to find home directory")?;
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;

    let portal_bin = exe.parent()
        .map(|p| p.join("luzumi-portal"))
        .filter(|p| p.exists());
    let portal_bin = portal_bin.ok_or("luzumi-portal binary not found next to the main executable")?;

    let portal_dir = home.join(".local/share/xdg-desktop-portal/portals");
    let service_dir = home.join(".local/share/dbus-1/services");
    let conf_dir = home.join(".config/xdg-desktop-portal");
    let bin_dir = home.join(".local/bin");
    for d in [&portal_dir, &service_dir, &conf_dir, &bin_dir] {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }

    let dest_bin = bin_dir.join("luzumi-portal");
    fs::copy(&portal_bin, &dest_bin).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dest_bin, fs::Permissions::from_mode(0o755)).ok();
    }

    let dest_luzumi = bin_dir.join("luzumi");
    if !dest_luzumi.exists() {
        let release_exe = exe.parent()
            .and_then(|p| p.parent())
            .map(|target| target.join("release").join("luzumi"))
            .filter(|p| p.exists());
        let src_exe = release_exe.as_deref().unwrap_or(&exe);
        fs::copy(src_exe, &dest_luzumi).ok();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&dest_luzumi, fs::Permissions::from_mode(0o755)).ok();
        }
    }

    fs::write(
        portal_dir.join("luzumi.portal"),
        "[portal]\nDBusName=org.freedesktop.impl.portal.desktop.luzumi\nInterfaces=org.freedesktop.impl.portal.FileChooser\n",
    ).map_err(|e| e.to_string())?;

    fs::write(
        service_dir.join("org.freedesktop.impl.portal.desktop.luzumi.service"),
        format!(
            "[D-BUS Service]\nName=org.freedesktop.impl.portal.desktop.luzumi\nExec={}\n",
            dest_bin.display()
        ),
    ).map_err(|e| e.to_string())?;

    fs::write(
        service_dir.join("org.freedesktop.FileManager1.service"),
        format!(
            "[D-BUS Service]\nName=org.freedesktop.FileManager1\nExec={}\n",
            dest_bin.display()
        ),
    ).map_err(|e| e.to_string())?;

    std::process::Command::new("systemctl")
        .args(["--user", "mask", "plasma-dolphin.service"])
        .status()
        .ok();

    std::process::Command::new("pkill")
        .args(["-f", "dolphin --daemon"])
        .status()
        .ok();

    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_lowercase();
    let conf_filename = if desktop.contains("kde") {
        "kde-portals.conf"
    } else if desktop.contains("gnome") {
        "gnome-portals.conf"
    } else {
        "portals.conf"
    };
    let conf_path = conf_dir.join(conf_filename);
    inject_filechooser_line(&conf_path, desktop.as_str())?;

    std::process::Command::new("systemctl")
        .args(["--user", "restart", "xdg-desktop-portal"])
        .status()
        .ok();

    let systemd_dir = home.join(".config/systemd/user");
    fs::create_dir_all(&systemd_dir).map_err(|e| e.to_string())?;
    fs::write(
        systemd_dir.join("luzumi-daemon.service"),
        format!(
            "[Unit]\nDescription=Luzumi File Manager Daemon\nAfter=graphical-session.target\n\n[Service]\nType=simple\nExecStart={} --daemon\nRestart=on-failure\nRestartSec=5\n\n[Install]\nWantedBy=default.target\n",
            dest_bin.display()
        ),
    ).map_err(|e| e.to_string())?;

    std::process::Command::new("systemctl")
        .args(["--user", "daemon-reload"])
        .status()
        .ok();
    std::process::Command::new("systemctl")
        .args(["--user", "enable", "--now", "luzumi-daemon.service"])
        .status()
        .ok();

    // ── KDE Plasma ContainmentAction plugin ──
    // Copy the pre-compiled plugin .so if available
    let plugin_src = exe.parent()
        .map(|p| p.join("com.luzumi.desktopmenu.so"))
        .filter(|p| p.exists());
    if let Some(plugin_path) = plugin_src {
        // Detect Qt6 plugin dir (usually /usr/lib64/qt6/plugins on Fedora)
        let qt_plugin_dir = std::process::Command::new("qtpaths6")
            .arg("--plugin-dir")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| std::path::PathBuf::from(s.trim()))
            .unwrap_or_else(|| std::path::PathBuf::from("/usr/lib64/qt6/plugins"));
        let plugin_dir = qt_plugin_dir.join("plasma/containmentactions");
        // This requires root — use pkexec to copy and set permissions
        let dest = plugin_dir.join("com.luzumi.desktopmenu.so");
        std::process::Command::new("pkexec")
            .args(["install", "-m", "755", &plugin_path.to_string_lossy(), &dest.to_string_lossy()])
            .status()
            .ok();
    }

    // Create extension directory for third-party desktop menu items
    let ext_dir = home.join(".local/share/luzumi/desktop-menu.d");
    fs::create_dir_all(&ext_dir).ok();

    Ok(())
}

#[tauri::command]
fn cmd_uninstall_portal() -> Result<(), String> {
    let home = dirs::home_dir().ok_or("Failed to find home directory")?;

    let files = [
        home.join(".local/share/xdg-desktop-portal/portals/luzumi.portal"),
        home.join(".local/share/dbus-1/services/org.freedesktop.impl.portal.desktop.luzumi.service"),
        home.join(".local/share/dbus-1/services/org.freedesktop.FileManager1.service"),
        home.join(".local/bin/luzumi-portal"),
    ];
    for f in &files {
        if f.exists() {
            fs::remove_file(f).ok();
        }
    }

    let conf_dir = home.join(".config/xdg-desktop-portal");
    for f in ["portals.conf", "kde-portals.conf", "gnome-portals.conf"] {
        remove_filechooser_line(&conf_dir.join(f));
    }

    std::process::Command::new("systemctl")
        .args(["--user", "disable", "--now", "luzumi-daemon.service"])
        .status()
        .ok();

    let systemd_service = home.join(".config/systemd/user/luzumi-daemon.service");
    if systemd_service.exists() {
        fs::remove_file(&systemd_service).ok();
    }

    std::process::Command::new("systemctl")
        .args(["--user", "daemon-reload"])
        .status()
        .ok();

    std::process::Command::new("systemctl")
        .args(["--user", "unmask", "plasma-dolphin.service"])
        .status()
        .ok();

    std::process::Command::new("systemctl")
        .args(["--user", "restart", "xdg-desktop-portal"])
        .status()
        .ok();

    // Remove KDE ContainmentAction plugin
    let qt_plugin_dir = std::process::Command::new("qtpaths6")
        .arg("--plugin-dir")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| std::path::PathBuf::from(s.trim()))
        .unwrap_or_else(|| std::path::PathBuf::from("/usr/lib64/qt6/plugins"));
    let plugin_file = qt_plugin_dir.join("plasma/containmentactions/com.luzumi.desktopmenu.so");
    if plugin_file.exists() {
        std::process::Command::new("pkexec")
            .args(["rm", &plugin_file.to_string_lossy()])
            .status()
            .ok();
    }

    let index_db = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("luzumi")
        .join("index.db");
    if index_db.exists() {
        fs::remove_file(&index_db).ok();
    }

    Ok(())
}

fn themes_dir() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("luzumi")
        .join("themes")
}

#[tauri::command]
fn cmd_list_themes() -> Vec<String> {
    let dir = themes_dir();
    fs::create_dir_all(&dir).ok();
    fs::read_dir(&dir)
        .ok()
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter_map(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    name.strip_suffix(".css").map(|n| n.to_string())
                })
                .collect()
        })
        .unwrap_or_default()
}

#[tauri::command]
fn cmd_read_theme(name: String) -> Result<String, String> {
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        return Err("Invalid theme name".into());
    }
    let path = themes_dir().join(format!("{}.css", name));
    fs::read_to_string(&path).map_err(|e| format!("Failed to read theme: {}", e))
}

#[tauri::command]
fn cmd_get_themes_dir() -> String {
    themes_dir().to_string_lossy().to_string()
}

#[derive(Clone, serde::Serialize)]
struct ChecksumResult {
    md5: String,
    sha256: String,
}

#[tauri::command]
fn cmd_compute_checksum(path: String) -> Result<ChecksumResult, String> {
    use sha2::Digest;
    use std::io::Read;
    let mut file = fs::File::open(&path).map_err(|e| e.to_string())?;
    let mut md5_ctx = md5::Context::new();
    let mut sha256_ctx = sha2::Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 { break; }
        md5_ctx.consume(&buf[..n]);
        sha256_ctx.update(&buf[..n]);
    }
    Ok(ChecksumResult {
        md5: format!("{:x}", md5_ctx.compute()),
        sha256: format!("{:x}", sha256_ctx.finalize()),
    })
}

#[derive(Clone, serde::Serialize)]
struct DiskSpaceInfo {
    total: u64,
    available: u64,
    used: u64,
}

#[tauri::command]
fn cmd_get_disk_space(path: String) -> Result<DiskSpaceInfo, String> {
    use std::mem::MaybeUninit;
    let c_path = std::ffi::CString::new(path).map_err(|e| e.to_string())?;
    let mut stat = MaybeUninit::<libc::statvfs>::uninit();
    let ret = unsafe { libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr()) };
    if ret != 0 {
        return Err("Failed to get disk space".into());
    }
    let stat = unsafe { stat.assume_init() };
    let total = stat.f_blocks as u64 * stat.f_frsize as u64;
    let available = stat.f_bavail as u64 * stat.f_frsize as u64;
    Ok(DiskSpaceInfo {
        total,
        available,
        used: total - (stat.f_bfree as u64 * stat.f_frsize as u64),
    })
}

#[derive(Clone, serde::Serialize)]
struct GitStatusInfo {
    is_repo: bool,
    branch: String,
    modified: Vec<String>,
    staged: Vec<String>,
    untracked: Vec<String>,
}

#[tauri::command]
fn cmd_get_git_status(path: String) -> Option<GitStatusInfo> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(&path)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let branch = std::process::Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(&path)
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    let status_output = std::process::Command::new("git")
        .args(["status", "--porcelain", "-uall"])
        .current_dir(&path)
        .output()
        .ok()?;
    let status_str = String::from_utf8_lossy(&status_output.stdout);

    let mut modified = Vec::new();
    let mut staged = Vec::new();
    let mut untracked = Vec::new();

    for line in status_str.lines() {
        if line.len() < 4 { continue; }
        let (index, work) = (line.as_bytes()[0], line.as_bytes()[1]);
        let file = if line.is_char_boundary(3) {
            line[3..].to_string()
        } else {
            continue;
        };
        match (index, work) {
            (b'?', b'?') => untracked.push(file),
            (b' ', b'M') | (b' ', b'D') => modified.push(file),
            (b'M', _) | (b'A', _) | (b'D', _) | (b'R', _) => staged.push(file),
            _ => {
                if work == b'M' || work == b'D' { modified.push(file); }
            }
        }
    }

    Some(GitStatusInfo {
        is_repo: true,
        branch,
        modified,
        staged,
        untracked,
    })
}

// ── File watcher ──

struct WatcherState {
    main: Option<notify::RecommendedWatcher>,
    split: Option<notify::RecommendedWatcher>,
    trash: Option<notify::RecommendedWatcher>,
}

#[tauri::command]
fn cmd_watch_directory(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<WatcherState>>,
    path: String,
    channel: Option<String>,
) -> Result<(), String> {
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    let event_name = channel.unwrap_or_else(|| "fs-changed".to_string());
    let app2 = app.clone();
    let path_clone = path.clone();
    let event_name_clone = event_name.clone();

    // Debounce: only emit if >300ms since last emit to avoid flooding
    let last_emit = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let watcher = notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
        if let Ok(_event) = res {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
            let prev = last_emit.load(std::sync::atomic::Ordering::Relaxed);
            if now.saturating_sub(prev) >= 300 {
                last_emit.store(now, std::sync::atomic::Ordering::Relaxed);
                let _ = app2.emit(&event_name_clone, &path_clone);
            }
        }
    }).map_err(|e| e.to_string())?;

    match event_name.as_str() {
        "fs-changed-split" => {
            guard.split = Some(watcher);
            if let Some(ref mut w) = guard.split {
                w.watch(Path::new(&path), RecursiveMode::NonRecursive).map_err(|e| e.to_string())?;
            }
        }
        "trash-changed" => {
            guard.trash = Some(watcher);
            if let Some(ref mut w) = guard.trash {
                w.watch(Path::new(&path), RecursiveMode::NonRecursive).map_err(|e| e.to_string())?;
            }
        }
        _ => {
            guard.main = Some(watcher);
            if let Some(ref mut w) = guard.main {
                w.watch(Path::new(&path), RecursiveMode::NonRecursive).map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

#[tauri::command]
fn cmd_unwatch_directory(
    state: tauri::State<'_, Mutex<WatcherState>>,
    channel: Option<String>,
) -> Result<(), String> {
    let mut guard = state.lock().map_err(|e| e.to_string())?;
    match channel.as_deref() {
        Some("fs-changed-split") => { guard.split = None; }
        Some("trash-changed") => { guard.trash = None; }
        _ => { guard.main = None; }
    }
    Ok(())
}

// ── Permissions ──

#[tauri::command]
fn cmd_set_permissions(path: String, mode: u32) -> Result<(), String> {
    set_permissions(Path::new(&path), mode)
}

#[tauri::command]
fn cmd_elevated_set_permissions(path: String, mode: u32) -> Result<(), String> {
    use filesystem::elevated_set_permissions;
    elevated_set_permissions(Path::new(&path), mode)
}

// ── Symlink creation ──

#[tauri::command]
fn cmd_create_symlink(target: String, link_path: String) -> Result<(), String> {
    create_symlink(Path::new(&target), Path::new(&link_path))
}

// ── Trash item info ──

#[tauri::command]
fn cmd_get_trash_item_info(file_name: String) -> TrashItemInfo {
    get_trash_item_info(&file_name)
}

#[tauri::command]
fn cmd_get_all_trash_info(file_names: Vec<String>) -> std::collections::HashMap<String, TrashItemInfo> {
    let mut result = std::collections::HashMap::new();
    for name in file_names {
        let info = get_trash_item_info(&name);
        result.insert(name, info);
    }
    result
}

#[derive(Clone, serde::Deserialize)]
struct NativeMenuItem {
    id: String,
    label: String,
    enabled: bool,
    separator: bool,
}

#[tauri::command]
fn cmd_show_context_menu(window: tauri::Window, items: Vec<NativeMenuItem>) -> Result<(), String> {
    let mut builder = MenuBuilder::new(&window);
    for item in &items {
        if item.separator {
            builder = builder.separator();
        } else {
            let mi = MenuItem::with_id(&window, &item.id, &item.label, item.enabled, None::<&str>)
                .map_err(|e| e.to_string())?;
            builder = builder.item(&mi);
        }
    }
    let menu = builder.build().map_err(|e| e.to_string())?;
    window.popup_menu(&menu).map_err(|e| e.to_string())?;
    Ok(())
}

// ── Advanced Search ──

#[tauri::command]
async fn cmd_search_advanced(
    query: String,
    filters: search::SearchFilter,
    path: String,
    recursive: bool,
) -> Result<Vec<search::SearchResult>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        search::search_advanced(&query, &filters, Path::new(&path), recursive)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_search_content(
    query: String,
    path: String,
    recursive: bool,
    max_matches: Option<usize>,
) -> Result<Vec<search::ContentMatch>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        search::search_content(&query, Path::new(&path), recursive, max_matches.unwrap_or(500))
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
fn cmd_search_cancel() {
    search::SEARCH_CANCEL.store(true, std::sync::atomic::Ordering::Relaxed);
}

// ── Preview ──

#[tauri::command]
async fn cmd_read_pdf_page(path: String, page: u32) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        preview::read_pdf_page(&path, page)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_get_pdf_page_count(path: String) -> Result<u32, String> {
    tauri::async_runtime::spawn_blocking(move || {
        preview::get_pdf_page_count(&path)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
fn cmd_render_markdown(content: String) -> String {
    preview::render_markdown(&content)
}

#[tauri::command]
async fn cmd_read_hex(path: String, offset: usize, length: usize) -> Result<preview::HexData, String> {
    tauri::async_runtime::spawn_blocking(move || {
        preview::read_hex(&path, offset, length)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_read_file_preview(path: String, max_lines: Option<usize>) -> Result<(String, String), String> {
    tauri::async_runtime::spawn_blocking(move || {
        preview::read_file_preview(&path, max_lines.unwrap_or(500))
    }).await.map_err(|e| e.to_string())?
}

// ── Tags ──

#[tauri::command]
fn cmd_create_tag(name: String, color: String) -> Result<tags::Tag, String> {
    tags::create_tag(&name, &color)
}

#[tauri::command]
fn cmd_delete_tag(id: i64) -> Result<(), String> {
    tags::delete_tag(id)
}

#[tauri::command]
fn cmd_rename_tag(id: i64, name: String) -> Result<(), String> {
    tags::rename_tag(id, &name)
}

#[tauri::command]
fn cmd_recolor_tag(id: i64, color: String) -> Result<(), String> {
    tags::recolor_tag(id, &color)
}

#[tauri::command]
fn cmd_list_tags() -> Result<Vec<tags::Tag>, String> {
    tags::list_tags()
}

#[tauri::command]
fn cmd_tag_file(path: String, tag_id: i64) -> Result<(), String> {
    tags::tag_file(&path, tag_id)
}

#[tauri::command]
fn cmd_untag_file(path: String, tag_id: i64) -> Result<(), String> {
    tags::untag_file(&path, tag_id)
}

#[tauri::command]
fn cmd_get_file_tags(path: String) -> Result<Vec<tags::Tag>, String> {
    tags::get_file_tags(&path)
}

#[tauri::command]
fn cmd_get_files_by_tag(tag_id: i64) -> Result<Vec<String>, String> {
    tags::get_files_by_tag(tag_id)
}

#[tauri::command]
fn cmd_get_all_file_tags(paths: Vec<String>) -> Result<Vec<tags::FileTag>, String> {
    tags::get_all_file_tags(&paths)
}

// ── Compare ──

#[tauri::command]
async fn cmd_compare_directories(left: String, right: String) -> Result<compare::ComparisonResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        compare::compare_directories(&left, &right)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_compare_files(path1: String, path2: String) -> Result<compare::FileComparisonResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        compare::compare_files(&path1, &path2)
    }).await.map_err(|e| e.to_string())?
}

// ── Logging ──

#[tauri::command]
fn cmd_get_logs(lines: Option<usize>) -> Result<Vec<logging::LogEntry>, String> {
    logging::get_logs(lines.unwrap_or(200))
}

#[tauri::command]
fn cmd_export_logs(path: String) -> Result<(), String> {
    logging::export_logs(&path)
}

// ── Session ──

#[tauri::command]
fn cmd_save_session(session: String) -> Result<(), String> {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("luzumi");
    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    fs::write(config_dir.join("session.json"), session).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_load_session() -> Result<Option<String>, String> {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("luzumi");
    let path = config_dir.join("session.json");
    if path.exists() {
        Ok(Some(fs::read_to_string(path).map_err(|e| e.to_string())?))
    } else {
        Ok(None)
    }
}

// ── Favorites ──

#[tauri::command]
fn cmd_toggle_favorite(path: String) -> Result<Vec<String>, String> {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("luzumi");
    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    let fav_path = config_dir.join("favorites.json");
    let mut favorites: Vec<String> = if fav_path.exists() {
        let data = fs::read_to_string(&fav_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    };

    if let Some(idx) = favorites.iter().position(|p| p == &path) {
        favorites.remove(idx);
    } else {
        favorites.push(path);
    }

    fs::write(fav_path, serde_json::to_string(&favorites).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    Ok(favorites)
}

#[tauri::command]
fn cmd_get_favorites() -> Result<Vec<String>, String> {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("luzumi");
    let fav_path = config_dir.join("favorites.json");
    if fav_path.exists() {
        let data = fs::read_to_string(&fav_path).map_err(|e| e.to_string())?;
        Ok(serde_json::from_str(&data).unwrap_or_default())
    } else {
        Ok(Vec::new())
    }
}

// ── Operation Queue ──

#[tauri::command]
fn cmd_list_operations() -> Vec<operation_queue::OperationInfo> {
    operation_queue::QUEUE.list_operations()
}

#[tauri::command]
fn cmd_pause_operation(id: String) -> Result<(), String> {
    operation_queue::QUEUE.pause_operation(&id)
}

#[tauri::command]
fn cmd_resume_operation(id: String) -> Result<(), String> {
    operation_queue::QUEUE.resume_operation(&id)
}

#[tauri::command]
fn cmd_cancel_operation_by_id(id: String) -> Result<(), String> {
    operation_queue::QUEUE.cancel_operation(&id)
}

// ── Workspaces ──

#[tauri::command]
fn cmd_save_workspace(name: String, data: String) -> Result<(), String> {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("luzumi").join("workspaces");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let safe_name = name.replace(['/', '\\', '\0'], "_");
    fs::write(dir.join(format!("{}.json", safe_name)), data).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_load_workspace(name: String) -> Result<Option<String>, String> {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("luzumi").join("workspaces");
    let safe_name = name.replace(['/', '\\', '\0'], "_");
    let path = dir.join(format!("{}.json", safe_name));
    if path.exists() {
        Ok(Some(fs::read_to_string(path).map_err(|e| e.to_string())?))
    } else {
        Ok(None)
    }
}

#[tauri::command]
fn cmd_list_workspaces() -> Result<Vec<String>, String> {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("luzumi").join("workspaces");
    if !dir.exists() { return Ok(Vec::new()); }
    let entries = fs::read_dir(&dir).map_err(|e| e.to_string())?;
    let mut names: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "json").unwrap_or(false))
        .filter_map(|e| e.path().file_stem().map(|s| s.to_string_lossy().to_string()))
        .collect();
    names.sort();
    Ok(names)
}

#[tauri::command]
fn cmd_delete_workspace(name: String) -> Result<(), String> {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("luzumi").join("workspaces");
    let safe_name = name.replace(['/', '\\', '\0'], "_");
    let path = dir.join(format!("{}.json", safe_name));
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ── Plugins ──

#[tauri::command]
fn cmd_list_plugins() -> Result<Vec<plugins::PluginInfo>, String> {
    plugins::list_plugins()
}

#[tauri::command]
fn cmd_enable_plugin(name: String) -> Result<(), String> {
    plugins::enable_plugin(&name)
}

#[tauri::command]
fn cmd_disable_plugin(name: String) -> Result<(), String> {
    plugins::disable_plugin(&name)
}

#[tauri::command]
fn cmd_get_plugin_script(name: String) -> Result<String, String> {
    plugins::get_plugin_script(&name)
}

#[tauri::command]
fn cmd_install_plugin(source: String) -> Result<String, String> {
    plugins::install_plugin_from_dir(&source)
}

#[tauri::command]
fn cmd_uninstall_plugin(name: String) -> Result<(), String> {
    plugins::uninstall_plugin(&name)
}

// ── Directory analysis (smart suggestions) ──

#[derive(serde::Serialize)]
struct SuggestedAction {
    action: String,
    description: String,
    icon: String,
}

#[derive(serde::Serialize)]
struct DirectoryAnalysis {
    suggested_actions: Vec<SuggestedAction>,
}

#[tauri::command]
async fn cmd_analyze_directory(path: String) -> Result<DirectoryAnalysis, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let dir = Path::new(&path);
        let entries: Vec<_> = fs::read_dir(dir).map_err(|e| e.to_string())?
            .filter_map(|e| e.ok())
            .collect();

        let mut suggestions = Vec::new();
        let mut ext_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let mut large_files = 0;

        for entry in &entries {
            if let Ok(meta) = entry.metadata() {
                let ext = entry.path().extension()
                    .map(|e| e.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                if !ext.is_empty() {
                    *ext_counts.entry(ext).or_default() += 1;
                }
                if meta.is_file() && meta.len() > 100 * 1024 * 1024 {
                    large_files += 1;
                }
            }
        }

        // Suggest organizing by type if mixed
        if ext_counts.len() > 5 && entries.len() > 20 {
            suggestions.push(SuggestedAction {
                action: "organize".into(),
                description: "Many file types detected. Organize by type?".into(),
                icon: "FolderOpen".into(),
            });
        }

        // Suggest finding duplicates if many files
        if entries.len() > 50 {
            suggestions.push(SuggestedAction {
                action: "duplicates".into(),
                description: "Many files here. Check for duplicates?".into(),
                icon: "Copy".into(),
            });
        }

        // Suggest archiving if large files exist
        if large_files > 0 {
            suggestions.push(SuggestedAction {
                action: "archive-large".into(),
                description: format!("{} large file(s) found. Consider archiving?", large_files),
                icon: "Archive".into(),
            });
        }

        // Suggest bulk rename if many similarly-named files
        for (ext, count) in &ext_counts {
            if *count > 10 {
                suggestions.push(SuggestedAction {
                    action: "bulk-rename".into(),
                    description: format!("{} .{} files. Bulk rename?", count, ext),
                    icon: "Edit".into(),
                });
                break;
            }
        }

        Ok(DirectoryAnalysis { suggested_actions: suggestions })
    }).await.map_err(|e| e.to_string())?
}

// ── Vault commands ──────────────────────────────────────────────

#[tauri::command]
async fn cmd_create_vault(path: String, password: String, dummy_password: Option<String>) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        vault::create_vault(&path, &password, dummy_password.as_deref())
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_unlock_vault(vault_path: String, password: String) -> Result<vault::VaultUnlockResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        vault::unlock_vault(&vault_path, &password)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_lock_vault(vault_path: String, mount_path: String, password: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        vault::lock_vault(&vault_path, &mount_path, &password)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_change_vault_password(vault_path: String, old_password: String, new_password: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        vault::change_vault_password(&vault_path, &old_password, &new_password)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn cmd_set_dummy_content(vault_path: String, main_password: String, dummy_password: String, dummy_dir: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        vault::set_dummy_content(&vault_path, &main_password, &dummy_password, &dummy_dir)
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
fn cmd_is_vault(path: String) -> bool {
    vault::is_vault(&path)
}

#[tauri::command]
fn cmd_get_vault_info(path: String) -> Result<vault::VaultInfo, String> {
    vault::get_vault_info(&path)
}

#[tauri::command]
fn cmd_list_unlocked_vaults() -> Vec<String> {
    vault::list_unlocked_vaults()
}

#[derive(serde::Serialize, Clone)]
struct IntegrityItem {
    name: String,
    path: String,
    status: String, // "ok", "missing", "repaired", "error"
    detail: String,
}

#[tauri::command]
fn cmd_check_integrity(repair: bool) -> Vec<IntegrityItem> {
    let mut results = Vec::new();
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => {
            results.push(IntegrityItem {
                name: "Home directory".into(),
                path: String::new(),
                status: "error".into(),
                detail: "Cannot find home directory".into(),
            });
            return results;
        }
    };

    let config_dir = dirs::config_dir().unwrap_or_else(|| home.join(".config")).join("luzumi");
    let data_dir = dirs::data_dir().unwrap_or_else(|| home.join(".local/share")).join("luzumi");

    // Directories that must exist
    let required_dirs: Vec<(&str, std::path::PathBuf)> = vec![
        ("Config directory", config_dir.clone()),
        ("Data directory", data_dir.clone()),
        ("Themes directory", config_dir.join("themes")),
        ("Plugins directory", config_dir.join("plugins")),
        ("Workspaces directory", config_dir.join("workspaces")),
        ("Logs directory", data_dir.join("logs")),
    ];

    for (name, path) in &required_dirs {
        if path.is_dir() {
            results.push(IntegrityItem {
                name: name.to_string(),
                path: path.to_string_lossy().into(),
                status: "ok".into(),
                detail: String::new(),
            });
        } else if repair {
            match fs::create_dir_all(path) {
                Ok(_) => results.push(IntegrityItem {
                    name: name.to_string(),
                    path: path.to_string_lossy().into(),
                    status: "repaired".into(),
                    detail: "Created missing directory".into(),
                }),
                Err(e) => results.push(IntegrityItem {
                    name: name.to_string(),
                    path: path.to_string_lossy().into(),
                    status: "error".into(),
                    detail: format!("Failed to create: {}", e),
                }),
            }
        } else {
            results.push(IntegrityItem {
                name: name.to_string(),
                path: path.to_string_lossy().into(),
                status: "missing".into(),
                detail: "Directory does not exist".into(),
            });
        }
    }

    // Tags database
    let tags_db = data_dir.join("tags.db");
    if tags_db.exists() {
        // Verify it's a valid SQLite database
        match rusqlite::Connection::open(&tags_db) {
            Ok(conn) => {
                match conn.execute_batch("SELECT 1 FROM tags LIMIT 1") {
                    Ok(_) => results.push(IntegrityItem {
                        name: "Tags database".into(),
                        path: tags_db.to_string_lossy().into(),
                        status: "ok".into(),
                        detail: String::new(),
                    }),
                    Err(_) => {
                        if repair {
                            // Re-initialize the tables
                            let init = tags::init_db();
                            results.push(IntegrityItem {
                                name: "Tags database".into(),
                                path: tags_db.to_string_lossy().into(),
                                status: if init.is_ok() { "repaired" } else { "error" }.into(),
                                detail: if init.is_ok() {
                                    "Re-initialized tables".into()
                                } else {
                                    format!("Failed to reinit: {}", init.unwrap_err())
                                },
                            });
                        } else {
                            results.push(IntegrityItem {
                                name: "Tags database".into(),
                                path: tags_db.to_string_lossy().into(),
                                status: "missing".into(),
                                detail: "Tables missing or corrupted".into(),
                            });
                        }
                    }
                }
            }
            Err(e) => results.push(IntegrityItem {
                name: "Tags database".into(),
                path: tags_db.to_string_lossy().into(),
                status: "error".into(),
                detail: format!("Cannot open: {}", e),
            }),
        }
    } else if repair {
        let init = tags::init_db();
        results.push(IntegrityItem {
            name: "Tags database".into(),
            path: tags_db.to_string_lossy().into(),
            status: if init.is_ok() { "repaired" } else { "error" }.into(),
            detail: if init.is_ok() {
                "Created database".into()
            } else {
                format!("Failed to create: {}", init.unwrap_err())
            },
        });
    } else {
        results.push(IntegrityItem {
            name: "Tags database".into(),
            path: tags_db.to_string_lossy().into(),
            status: "missing".into(),
            detail: "Database file does not exist".into(),
        });
    }

    // Desktop entry
    let desktop_file = home.join(".local/share/applications/luzumi.desktop");
    if desktop_file.exists() {
        results.push(IntegrityItem {
            name: "Desktop entry".into(),
            path: desktop_file.to_string_lossy().into(),
            status: "ok".into(),
            detail: String::new(),
        });
    } else {
        if repair {
            match cmd_set_as_default_file_manager() {
                Ok(_) => results.push(IntegrityItem {
                    name: "Desktop entry".into(),
                    path: desktop_file.to_string_lossy().into(),
                    status: "repaired".into(),
                    detail: "Registered as file manager".into(),
                }),
                Err(e) => results.push(IntegrityItem {
                    name: "Desktop entry".into(),
                    path: desktop_file.to_string_lossy().into(),
                    status: "error".into(),
                    detail: format!("Failed: {}", e),
                }),
            }
        } else {
            results.push(IntegrityItem {
                name: "Desktop entry".into(),
                path: desktop_file.to_string_lossy().into(),
                status: "missing".into(),
                detail: "Not registered as application".into(),
            });
        }
    }

    // MIME type association
    let mimeapps = home.join(".config/mimeapps.list");
    if mimeapps.exists() {
        let content = fs::read_to_string(&mimeapps).unwrap_or_default();
        if content.contains("inode/directory=luzumi") {
            results.push(IntegrityItem {
                name: "MIME association".into(),
                path: mimeapps.to_string_lossy().into(),
                status: "ok".into(),
                detail: "Set as default for directories".into(),
            });
        } else {
            results.push(IntegrityItem {
                name: "MIME association".into(),
                path: mimeapps.to_string_lossy().into(),
                status: "missing".into(),
                detail: "Not set as default file manager".into(),
            });
        }
    } else {
        results.push(IntegrityItem {
            name: "MIME association".into(),
            path: mimeapps.to_string_lossy().into(),
            status: "missing".into(),
            detail: "mimeapps.list not found".into(),
        });
    }

    // Binary in PATH
    let luzumi_bin = home.join(".local/bin/luzumi");
    if luzumi_bin.exists() {
        results.push(IntegrityItem {
            name: "Binary in ~/.local/bin".into(),
            path: luzumi_bin.to_string_lossy().into(),
            status: "ok".into(),
            detail: String::new(),
        });
    } else {
        results.push(IntegrityItem {
            name: "Binary in ~/.local/bin".into(),
            path: luzumi_bin.to_string_lossy().into(),
            status: "missing".into(),
            detail: "Binary not installed (optional)".into(),
        });
    }

    // Portal
    let portal_file = home.join(".local/share/xdg-desktop-portal/portals/luzumi.portal");
    if portal_file.exists() {
        results.push(IntegrityItem {
            name: "XDG Portal".into(),
            path: portal_file.to_string_lossy().into(),
            status: "ok".into(),
            detail: String::new(),
        });
    } else {
        results.push(IntegrityItem {
            name: "XDG Portal".into(),
            path: portal_file.to_string_lossy().into(),
            status: "missing".into(),
            detail: "Portal not installed (optional)".into(),
        });
    }

    // Plugins enabled file
    let enabled_plugins = config_dir.join("plugins").join(".enabled.json");
    if enabled_plugins.exists() {
        results.push(IntegrityItem {
            name: "Plugin registry".into(),
            path: enabled_plugins.to_string_lossy().into(),
            status: "ok".into(),
            detail: String::new(),
        });
    } else if repair {
        match fs::write(&enabled_plugins, "[]") {
            Ok(_) => results.push(IntegrityItem {
                name: "Plugin registry".into(),
                path: enabled_plugins.to_string_lossy().into(),
                status: "repaired".into(),
                detail: "Created empty plugin registry".into(),
            }),
            Err(e) => results.push(IntegrityItem {
                name: "Plugin registry".into(),
                path: enabled_plugins.to_string_lossy().into(),
                status: "error".into(),
                detail: format!("Failed to create: {}", e),
            }),
        }
    } else {
        results.push(IntegrityItem {
            name: "Plugin registry".into(),
            path: enabled_plugins.to_string_lossy().into(),
            status: "missing".into(),
            detail: "No plugin registry file".into(),
        });
    }

    results
}

pub fn run() {
    // Ignore SIGPIPE to prevent crashes when writing to closed pipes (e.g., root shell)
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    }

    // Initialize logging
    logging::init_logging();

    let picker_config = parse_picker_args();
    let desktop_menu_config = parse_desktop_menu_args();
    let dm_config_for_setup = desktop_menu_config.clone();

    tauri::Builder::default()
        .manage(Mutex::new(picker_config))
        .manage(Mutex::new(desktop_menu_config))
        .manage(Mutex::new(WatcherState { main: None, split: None, trash: None }))
        .setup(move |app| {
            // If in desktop-menu mode, reconfigure the main window as a small popup
            if let Some(config) = dm_config_for_setup {
                use tauri::Manager;
                use tauri::WebviewWindowBuilder;
                use tauri::WebviewUrl;

                // Hide the default main window
                if let Some(main_win) = app.get_webview_window("main") {
                    let _ = main_win.hide();
                    let _ = main_win.close();
                }

                // Create a new popup window at the exact click position
                let _popup = WebviewWindowBuilder::new(
                    app,
                    "desktop-menu",
                    WebviewUrl::App("index.html".into()),
                )
                .title("")
                .inner_size(280.0, 420.0)
                .position(config.x as f64, config.y as f64)
                .decorations(false)
                .transparent(true)
                .always_on_top(true)
                .skip_taskbar(true)
                .resizable(false)
                .minimizable(false)
                .focused(true)
                .visible(true)
                .initialization_script(&format!(
                    "window.__LUZUMI_DESKTOP_MENU__ = {{ x: {}, y: {} }};",
                    config.x, config.y
                ))
                .build();
            }
            Ok(())
        })
        .on_menu_event(|app, event| {
            let _ = app.emit("context-menu-action", event.id().0.clone());
        })
        .invoke_handler(tauri::generate_handler![
            list_directory,
            cmd_elevated_list_directory,
            open_file,
            cmd_list_open_with_apps,
            cmd_open_with_app,
            cmd_open_with_command,
            cmd_set_default_app,
            cmd_search_index,
            cmd_create_directory,
            cmd_rename_entry,
            cmd_delete_entries,
            cmd_secure_delete_streamed,
            cmd_copy_entries,
            cmd_move_entries,
            cmd_cancel_operation,
            cmd_get_bookmarks,
            cmd_get_drives,
            cmd_mount_drive,
            cmd_unlock_drive,
            cmd_eject_drive,
            cmd_read_thumbnail,
            cmd_get_file_details,
            cmd_authenticate_admin,
            cmd_deauthenticate_admin,
            cmd_check_path_readable,
            cmd_elevated_move,
            cmd_elevated_copy,
            cmd_elevated_delete,
            cmd_elevated_rename,
            cmd_elevated_create_directory,
            cmd_list_trash,
            cmd_get_trash_size,
            cmd_empty_trash,
            cmd_restore_from_trash,
            cmd_get_quick_access,
            cmd_add_quick_access,
            cmd_remove_quick_access,
            get_home_dir,
            cmd_copy_as,
            cmd_create_file,
            cmd_open_terminal,
            cmd_check_default_file_manager,
            cmd_set_as_default_file_manager,
            cmd_get_startup_path,
            cmd_get_startup_select,
            cmd_calculate_dir_size,
            cmd_find_duplicates,
            cmd_read_text_preview,
            cmd_create_archive,
            cmd_extract_archive,
            cmd_get_picker_config,
            cmd_submit_picker_result,
            cmd_cancel_picker,
            cmd_check_portal_installed,
            cmd_install_portal,
            cmd_uninstall_portal,
            cmd_list_themes,
            cmd_read_theme,
            cmd_get_themes_dir,
            cmd_compute_checksum,
            cmd_get_disk_space,
            cmd_get_git_status,
            cmd_show_context_menu,
            cmd_watch_directory,
            cmd_unwatch_directory,
            cmd_set_permissions,
            cmd_elevated_set_permissions,
            cmd_create_symlink,
            cmd_get_trash_item_info,
            cmd_get_all_trash_info,
            // Search
            cmd_search_advanced,
            cmd_search_content,
            cmd_search_cancel,
            // Preview
            cmd_read_pdf_page,
            cmd_get_pdf_page_count,
            cmd_render_markdown,
            cmd_read_hex,
            cmd_read_file_preview,
            // Tags
            cmd_create_tag,
            cmd_delete_tag,
            cmd_rename_tag,
            cmd_recolor_tag,
            cmd_list_tags,
            cmd_tag_file,
            cmd_untag_file,
            cmd_get_file_tags,
            cmd_get_files_by_tag,
            cmd_get_all_file_tags,
            // Compare
            cmd_compare_directories,
            cmd_compare_files,
            // Logging
            cmd_get_logs,
            cmd_export_logs,
            // Session
            cmd_save_session,
            cmd_load_session,
            // Favorites
            cmd_toggle_favorite,
            cmd_get_favorites,
            // Operation Queue
            cmd_list_operations,
            cmd_pause_operation,
            cmd_resume_operation,
            cmd_cancel_operation_by_id,
            // Workspaces
            cmd_save_workspace,
            cmd_load_workspace,
            cmd_list_workspaces,
            cmd_delete_workspace,
            // Plugins
            cmd_list_plugins,
            cmd_enable_plugin,
            cmd_disable_plugin,
            cmd_get_plugin_script,
            cmd_install_plugin,
            cmd_uninstall_plugin,
            // Smart suggestions
            cmd_analyze_directory,
            // Desktop Menu
            cmd_get_desktop_menu_config,
            cmd_desktop_action,
            cmd_get_desktop_menu_extensions,
            cmd_exec_desktop_extension,
            // Vault
            cmd_create_vault,
            cmd_unlock_vault,
            cmd_lock_vault,
            cmd_change_vault_password,
            cmd_set_dummy_content,
            cmd_is_vault,
            cmd_get_vault_info,
            cmd_list_unlocked_vaults,
            // Integrity
            cmd_check_integrity,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
