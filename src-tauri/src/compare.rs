use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Serialize;

use crate::filesystem::FileEntry;

#[derive(Debug, Serialize)]
pub struct ComparisonResult {
    pub only_left: Vec<FileEntry>,
    pub only_right: Vec<FileEntry>,
    pub modified: Vec<(FileEntry, FileEntry)>,
    pub identical: Vec<FileEntry>,
}

#[derive(Debug, Serialize)]
pub struct FileComparisonResult {
    pub identical: bool,
    pub size_diff: i64,
    pub date_diff: i64,
}

pub fn compare_directories(left: &str, right: &str) -> Result<ComparisonResult, String> {
    let left_path = Path::new(left);
    let right_path = Path::new(right);

    let left_entries = read_dir_map(left_path)?;
    let right_entries = read_dir_map(right_path)?;

    let mut only_left = Vec::new();
    let mut only_right = Vec::new();
    let mut modified = Vec::new();
    let mut identical = Vec::new();

    for (name, entry) in &left_entries {
        match right_entries.get(name) {
            Some(right_entry) => {
                if entries_differ(entry, right_entry) {
                    modified.push((entry.clone(), right_entry.clone()));
                } else {
                    identical.push(entry.clone());
                }
            }
            None => {
                only_left.push(entry.clone());
            }
        }
    }

    for (name, entry) in &right_entries {
        if !left_entries.contains_key(name) {
            only_right.push(entry.clone());
        }
    }

    // Sort all arrays by name
    only_left.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    only_right.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    modified.sort_by(|a, b| a.0.name.to_lowercase().cmp(&b.0.name.to_lowercase()));
    identical.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(ComparisonResult { only_left, only_right, modified, identical })
}

fn entries_differ(a: &FileEntry, b: &FileEntry) -> bool {
    if a.kind != b.kind { return true; }
    if a.size != b.size { return true; }
    if a.modified != b.modified { return true; }
    false
}

fn read_dir_map(path: &Path) -> Result<HashMap<String, FileEntry>, String> {
    let mut map = HashMap::new();
    let entries = fs::read_dir(path).map_err(|e| format!("Cannot read {}: {}", path.display(), e))?;

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let name = entry.file_name().to_string_lossy().to_string();
        let entry_path = entry.path();
        let kind = if metadata.is_dir() { "directory" } else if metadata.is_symlink() { "symlink" } else { "file" };
        let ext = entry_path.extension().map(|e| e.to_string_lossy().to_string());
        let is_vault = ext.as_deref() == Some("luzumi-vault") && kind == "file";

        let file_entry = FileEntry {
            name: name.clone(),
            path: entry_path.to_string_lossy().to_string(),
            kind: kind.to_string(),
            size: if metadata.is_file() { Some(metadata.len()) } else { None },
            modified: metadata.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs() as i64),
            is_hidden: name.starts_with('.'),
            extension: ext,
            is_writable: true,
            permissions_mode: None,
            is_broken_link: false,
            is_vault,
        };

        map.insert(name, file_entry);
    }

    Ok(map)
}

pub fn compare_files(path1: &str, path2: &str) -> Result<FileComparisonResult, String> {
    let meta1 = fs::metadata(path1).map_err(|e| e.to_string())?;
    let meta2 = fs::metadata(path2).map_err(|e| e.to_string())?;

    let size1 = meta1.len() as i64;
    let size2 = meta2.len() as i64;

    let mod1 = meta1.modified().ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let mod2 = meta2.modified().ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let identical = size1 == size2 && {
        // Only compare content if same size
        if size1 == 0 { true }
        else if size1 > 100 * 1024 * 1024 { false } // Skip content check for files > 100MB
        else {
            match (fs::read(path1), fs::read(path2)) {
                (Ok(d1), Ok(d2)) => d1 == d2,
                _ => false,
            }
        }
    };

    Ok(FileComparisonResult {
        identical,
        size_diff: size1 - size2,
        date_diff: mod1 - mod2,
    })
}
