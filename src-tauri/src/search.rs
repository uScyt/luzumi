use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use serde::{Serialize, Deserialize};
use strsim::jaro_winkler;

use crate::filesystem::FileEntry;

pub static SEARCH_CANCEL: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchFilter {
    #[serde(default)]
    pub types: Vec<String>,
    pub size_min: Option<u64>,
    pub size_max: Option<u64>,
    pub date_after: Option<i64>,
    pub date_before: Option<i64>,
    pub name_pattern: Option<String>,
    pub content_query: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SearchResult {
    #[serde(flatten)]
    pub entry: FileEntry,
    pub score: f32,
    pub match_ranges: Vec<(usize, usize)>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ContentMatch {
    pub path: String,
    pub line_number: usize,
    pub line_text: String,
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
}

fn matches_filter(entry: &FileEntry, filters: &SearchFilter) -> bool {
    // Size filter
    if let Some(min) = filters.size_min {
        if entry.size.unwrap_or(0) < min { return false; }
    }
    if let Some(max) = filters.size_max {
        if entry.size.unwrap_or(0) > max { return false; }
    }

    // Date filter
    if let Some(after) = filters.date_after {
        if entry.modified.unwrap_or(0) < after { return false; }
    }
    if let Some(before) = filters.date_before {
        if entry.modified.unwrap_or(0) > before { return false; }
    }

    // Type filter
    if !filters.types.is_empty() {
        let ext = entry.extension.as_deref().unwrap_or("").to_lowercase();
        let kind = &entry.kind;
        let matches = filters.types.iter().any(|t| {
            match t.as_str() {
                "directory" => kind == "directory",
                "image" => ["png","jpg","jpeg","gif","webp","bmp","svg","ico","tiff","heic","avif","jxl"].contains(&ext.as_str()),
                "video" => ["mp4","mkv","avi","mov","webm","flv","wmv","mpg","mpeg","ogv"].contains(&ext.as_str()),
                "audio" => ["mp3","flac","wav","ogg","m4a","aac","wma","opus","aiff"].contains(&ext.as_str()),
                "document" => ["pdf","doc","docx","odt","txt","rtf","tex","md","rst"].contains(&ext.as_str()),
                "code" => ["rs","py","js","ts","c","cpp","go","java","rb","php","svelte","vue","jsx","tsx","html","css"].contains(&ext.as_str()),
                "archive" => ["zip","tar","gz","bz2","xz","7z","rar","zst"].contains(&ext.as_str()),
                _ => ext == t.to_lowercase(),
            }
        });
        if !matches { return false; }
    }

    // Name pattern filter (glob)
    if let Some(ref pattern) = filters.name_pattern {
        let pat = pattern.to_lowercase();
        let name = entry.name.to_lowercase();
        if !glob_match(&pat, &name) { return false; }
    }

    true
}

fn glob_match(pattern: &str, text: &str) -> bool {
    if pattern.len() > 200 {
        return false; // Reject excessively long patterns to prevent ReDoS
    }
    let regex_str = format!(
        "^{}$",
        pattern
            .replace('.', "\\.")
            .replace('*', ".*")
            .replace('?', ".")
    );
    regex::RegexBuilder::new(&regex_str)
        .size_limit(1 << 20) // 1MB compiled regex limit
        .build()
        .map(|r| r.is_match(text))
        .unwrap_or(false)
}

fn fuzzy_score(query: &str, name: &str) -> f32 {
    let q = query.to_lowercase();
    let n = name.to_lowercase();

    // Exact match bonus
    if n == q { return 100.0; }

    // Prefix match bonus
    if n.starts_with(&q) { return 90.0 + (q.len() as f32 / n.len() as f32) * 10.0; }

    // Contains match
    if n.contains(&q) { return 70.0 + (q.len() as f32 / n.len() as f32) * 10.0; }

    // Jaro-Winkler similarity
    let jw = jaro_winkler(&q, &n) as f32;
    if jw > 0.7 { return jw * 60.0; }

    0.0
}

fn find_match_ranges(query: &str, name: &str) -> Vec<(usize, usize)> {
    let q = query.to_lowercase();
    let n = name.to_lowercase();
    let mut ranges = Vec::new();
    let mut start = 0;
    while let Some(pos) = n[start..].find(&q) {
        let abs_pos = start + pos;
        ranges.push((abs_pos, abs_pos + q.len()));
        start = abs_pos + q.len();
    }
    ranges
}

pub fn search_advanced(
    query: &str,
    filters: &SearchFilter,
    path: &Path,
    recursive: bool,
) -> Result<Vec<SearchResult>, String> {
    SEARCH_CANCEL.store(false, Ordering::Relaxed);
    let mut results = Vec::new();
    search_dir(query, filters, path, recursive, &mut results, 0)?;
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    if results.len() > 1000 { results.truncate(1000); }
    Ok(results)
}

fn search_dir(
    query: &str,
    filters: &SearchFilter,
    dir: &Path,
    recursive: bool,
    results: &mut Vec<SearchResult>,
    depth: u32,
) -> Result<(), String> {
    if SEARCH_CANCEL.load(Ordering::Relaxed) { return Ok(()); }
    if depth > 50 { return Ok(()); } // Prevent infinite recursion

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };

    for entry in entries {
        if SEARCH_CANCEL.load(Ordering::Relaxed) { return Ok(()); }

        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        let kind = if metadata.is_dir() { "directory" } else if metadata.is_symlink() { "symlink" } else { "file" };
        let ext = path.extension().map(|e| e.to_string_lossy().to_string());
        let is_vault = ext.as_deref() == Some("luzumi-vault") && kind == "file";

        let file_entry = FileEntry {
            name: name.clone(),
            path: path.to_string_lossy().to_string(),
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

        // Apply filters
        if !matches_filter(&file_entry, filters) { continue; }

        // Score against query
        let score = if query.is_empty() { 50.0 } else { fuzzy_score(query, &name) };

        if score > 0.0 || query.is_empty() {
            let match_ranges = if query.is_empty() { vec![] } else { find_match_ranges(query, &name) };
            results.push(SearchResult { entry: file_entry, score, match_ranges });
        }

        // Recurse into directories
        if recursive && metadata.is_dir() {
            search_dir(query, filters, &path, recursive, results, depth + 1)?;
        }
    }

    Ok(())
}

pub fn search_content(
    query: &str,
    path: &Path,
    recursive: bool,
    max_matches: usize,
) -> Result<Vec<ContentMatch>, String> {
    SEARCH_CANCEL.store(false, Ordering::Relaxed);
    let mut matches = Vec::new();
    search_content_dir(query, path, recursive, max_matches, &mut matches, 0)?;
    Ok(matches)
}

fn is_binary(data: &[u8]) -> bool {
    data.iter().take(512).any(|&b| b == 0)
}

fn search_content_dir(
    query: &str,
    dir: &Path,
    recursive: bool,
    max_matches: usize,
    matches: &mut Vec<ContentMatch>,
    depth: u32,
) -> Result<(), String> {
    if SEARCH_CANCEL.load(Ordering::Relaxed) || matches.len() >= max_matches { return Ok(()); }
    if depth > 50 { return Ok(()); }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };

    let query_lower = query.to_lowercase();

    for entry in entries {
        if SEARCH_CANCEL.load(Ordering::Relaxed) || matches.len() >= max_matches { return Ok(()); }

        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let path = entry.path();

        if metadata.is_dir() {
            if recursive {
                search_content_dir(query, &path, recursive, max_matches, matches, depth + 1)?;
            }
            continue;
        }

        if !metadata.is_file() { continue; }

        // Skip large files (>10MB)
        if metadata.len() > 10 * 1024 * 1024 { continue; }

        // Read file and search
        let data = match fs::read(&path) {
            Ok(d) => d,
            Err(_) => continue,
        };

        if is_binary(&data) { continue; }

        let content = match String::from_utf8(data) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let lines: Vec<&str> = content.lines().collect();
        let path_str = path.to_string_lossy().to_string();

        for (i, line) in lines.iter().enumerate() {
            if matches.len() >= max_matches { break; }
            if line.to_lowercase().contains(&query_lower) {
                let ctx_before: Vec<String> = lines[i.saturating_sub(2)..i]
                    .iter().map(|s| s.to_string()).collect();
                let ctx_after: Vec<String> = lines[i + 1..std::cmp::min(i + 3, lines.len())]
                    .iter().map(|s| s.to_string()).collect();

                matches.push(ContentMatch {
                    path: path_str.clone(),
                    line_number: i + 1,
                    line_text: line.to_string(),
                    context_before: ctx_before,
                    context_after: ctx_after,
                });
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob_match_basic() {
        assert!(glob_match("hello", "hello"));
        assert!(!glob_match("hello", "world"));
        assert!(glob_match("*.txt", "file.txt"));
        assert!(!glob_match("*.txt", "file.rs"));
        assert!(glob_match("test*", "testing"));
        assert!(glob_match("test?", "tests"));
        assert!(!glob_match("test?", "testing"));
    }

    #[test]
    fn test_glob_match_rejects_long_patterns() {
        let long_pattern = "a".repeat(201);
        assert!(!glob_match(&long_pattern, "a"));
    }

    #[test]
    fn test_fuzzy_score() {
        assert_eq!(fuzzy_score("hello", "hello"), 100.0);
        assert!(fuzzy_score("hel", "hello") >= 90.0);
        assert!(fuzzy_score("ell", "hello") >= 70.0);
        assert_eq!(fuzzy_score("xyz", "hello"), 0.0);
    }

    #[test]
    fn test_find_match_ranges() {
        let ranges = find_match_ranges("lo", "hello world lo");
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0], (3, 5));
        assert_eq!(ranges[1], (12, 14));
    }

    #[test]
    fn test_matches_filter_size() {
        let entry = FileEntry {
            name: "test.txt".into(), path: "/test.txt".into(), kind: "file".into(),
            size: Some(1000), modified: Some(1000000), is_hidden: false,
            extension: Some("txt".into()), is_writable: true,
            permissions_mode: None, is_broken_link: false, is_vault: false,
        };
        let pass = SearchFilter {
            types: vec![], size_min: Some(500), size_max: Some(2000),
            date_after: None, date_before: None, name_pattern: None, content_query: None,
        };
        assert!(matches_filter(&entry, &pass));

        let fail = SearchFilter {
            types: vec![], size_min: Some(2000), size_max: None,
            date_after: None, date_before: None, name_pattern: None, content_query: None,
        };
        assert!(!matches_filter(&entry, &fail));
    }

    #[test]
    fn test_matches_filter_type() {
        let entry = FileEntry {
            name: "photo.jpg".into(), path: "/photo.jpg".into(), kind: "file".into(),
            size: Some(100), modified: Some(100), is_hidden: false,
            extension: Some("jpg".into()), is_writable: true,
            permissions_mode: None, is_broken_link: false, is_vault: false,
        };
        let image_filter = SearchFilter {
            types: vec!["image".into()], size_min: None, size_max: None,
            date_after: None, date_before: None, name_pattern: None, content_query: None,
        };
        assert!(matches_filter(&entry, &image_filter));

        let video_filter = SearchFilter {
            types: vec!["video".into()], size_min: None, size_max: None,
            date_after: None, date_before: None, name_pattern: None, content_query: None,
        };
        assert!(!matches_filter(&entry, &video_filter));
    }

    #[test]
    fn test_search_advanced_temp_dir() {
        let dir = std::env::temp_dir().join("luzumi_test_search");
        let _ = std::fs::create_dir_all(&dir);
        std::fs::write(dir.join("hello.txt"), "content").unwrap();
        std::fs::write(dir.join("world.rs"), "fn main()").unwrap();

        let filter = SearchFilter {
            types: vec![], size_min: None, size_max: None,
            date_after: None, date_before: None, name_pattern: None, content_query: None,
        };
        let results = search_advanced("hello", &filter, &dir, false).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].entry.name.contains("hello"));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
