use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
use rusqlite::{Connection, params};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use walkdir::WalkDir;

/// Directories to skip during indexing.
const SKIP_DIRS: &[&str] = &[
    ".cache", ".local/share/Trash", "node_modules", ".git", "target",
    "__pycache__", ".venv", "build", ".npm", ".cargo/registry",
    ".rustup", ".mozilla", ".config/chromium", ".config/google-chrome",
    ".config/BraveSoftware", "snap",
];

/// Get the database path.
pub fn db_path() -> PathBuf {
    let data_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    data_dir.join("luzumi").join("index.db")
}

/// Initialize the database schema.
fn init_db(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch("
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA cache_size = -8000;

        CREATE TABLE IF NOT EXISTS files (
            path TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            name_lower TEXT NOT NULL,
            parent TEXT NOT NULL,
            kind TEXT NOT NULL,
            size INTEGER,
            modified INTEGER,
            extension TEXT,
            indexed_at INTEGER NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_name_lower ON files(name_lower);
        CREATE INDEX IF NOT EXISTS idx_parent ON files(parent);
        CREATE INDEX IF NOT EXISTS idx_extension ON files(extension);
    ")
}

/// Check if a path should be skipped.
fn should_skip(path: &Path, home: &Path) -> bool {
    for skip in SKIP_DIRS {
        let skip_path = home.join(skip);
        if path.starts_with(&skip_path) {
            return true;
        }
    }
    // Skip hidden directories (but not files in them)
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if name.starts_with('.') && path.is_dir() && path != home {
            // Allow .local but skip most hidden dirs
            if name != ".local" {
                return true;
            }
        }
    }
    false
}

/// Perform a full scan of the home directory.
fn full_scan(conn: &Connection, home: &Path) -> rusqlite::Result<usize> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let mut stmt = conn.prepare_cached(
        "INSERT OR REPLACE INTO files (path, name, name_lower, parent, kind, size, modified, extension, indexed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
    )?;

    let mut count = 0usize;
    let mut batch_count = 0;

    conn.execute_batch("BEGIN TRANSACTION")?;

    for entry in WalkDir::new(home)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !should_skip(e.path(), home))
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path == home {
            continue;
        }

        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        let parent = path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
        let kind = if entry.file_type().is_dir() {
            "directory"
        } else if entry.file_type().is_symlink() {
            "symlink"
        } else {
            "file"
        };

        let meta = entry.metadata().ok();
        let size: Option<i64> = meta.as_ref().map(|m| m.len() as i64);
        let modified: Option<i64> = meta.as_ref()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64);
        let extension = path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase());

        stmt.execute(params![
            path.to_string_lossy().as_ref(),
            name,
            name.to_lowercase(),
            parent,
            kind,
            size,
            modified,
            extension,
            now,
        ])?;

        count += 1;
        batch_count += 1;

        // Commit every 10000 entries to avoid holding the lock too long
        if batch_count >= 10000 {
            conn.execute_batch("COMMIT; BEGIN TRANSACTION")?;
            batch_count = 0;
        }
    }

    conn.execute_batch("COMMIT")?;

    // Remove stale entries from previous scans
    conn.execute("DELETE FROM files WHERE indexed_at < ?1", params![now])?;

    Ok(count)
}

/// Index a single path (file or directory).
fn index_path(conn: &Connection, path: &Path) -> rusqlite::Result<()> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let meta = match std::fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => {
            // File doesn't exist anymore, remove from index
            conn.execute("DELETE FROM files WHERE path = ?1", params![path.to_string_lossy().as_ref()])?;
            return Ok(());
        }
    };

    let name = match path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return Ok(()),
    };
    let parent = path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
    let kind = if meta.is_dir() { "directory" } else if meta.file_type().is_symlink() { "symlink" } else { "file" };
    let size = meta.len() as i64;
    let modified = meta.modified().ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64);
    let extension = path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase());

    conn.execute(
        "INSERT OR REPLACE INTO files (path, name, name_lower, parent, kind, size, modified, extension, indexed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            path.to_string_lossy().as_ref(),
            name,
            name.to_lowercase(),
            parent,
            kind,
            size,
            modified,
            extension,
            now,
        ],
    )?;

    Ok(())
}

/// Remove a path from the index.
fn remove_path(conn: &Connection, path: &Path) -> rusqlite::Result<()> {
    let path_str = path.to_string_lossy().to_string();
    // Remove the path itself and any children (if directory was removed)
    conn.execute("DELETE FROM files WHERE path = ?1 OR path LIKE ?2", params![path_str, format!("{}/%", path_str)])?;
    Ok(())
}

/// Start the indexer. This runs forever: does an initial full scan, then watches for changes.
pub async fn run_indexer() {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => {
            eprintln!("luzumi-indexer: cannot determine home directory");
            return;
        }
    };

    // Ensure data directory exists
    let db = db_path();
    if let Some(parent) = db.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let conn = match Connection::open(&db) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("luzumi-indexer: cannot open database: {}", e);
            return;
        }
    };

    if let Err(e) = init_db(&conn) {
        eprintln!("luzumi-indexer: cannot initialize database: {}", e);
        return;
    }

    let conn = Arc::new(Mutex::new(conn));

    // Initial full scan
    eprintln!("luzumi-indexer: starting initial scan of {}", home.display());
    if let Ok(conn) = conn.lock() {
        match full_scan(&conn, &home) {
            Ok(count) => eprintln!("luzumi-indexer: indexed {} files", count),
            Err(e) => eprintln!("luzumi-indexer: scan error: {}", e),
        }
    } else {
        eprintln!("luzumi-indexer: lock poisoned during initial scan");
    }

    // Set up file watcher
    let (tx, mut rx) = mpsc::channel::<Event>(1000);

    let mut watcher: RecommendedWatcher = match notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            let _ = tx.blocking_send(event);
        }
    }) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("luzumi-indexer: cannot create watcher: {}", e);
            // Fall back to periodic rescans only
            let conn = conn.clone();
            let home = home.clone();
            loop {
                tokio::time::sleep(Duration::from_secs(1800)).await;
                let Ok(conn) = conn.lock() else { eprintln!("luzumi-indexer: lock poisoned"); continue };
                match full_scan(&conn, &home) {
                    Ok(count) => eprintln!("luzumi-indexer: rescan indexed {} files", count),
                    Err(e) => eprintln!("luzumi-indexer: rescan error: {}", e),
                }
            }
        }
    };

    if let Err(e) = watcher.watch(&home, RecursiveMode::Recursive) {
        eprintln!("luzumi-indexer: watch error: {}", e);
    }

    eprintln!("luzumi-indexer: watching for changes in {}", home.display());

    // Spawn periodic rescan
    let conn_rescan = conn.clone();
    let home_rescan = home.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1800)).await;
            let conn = conn_rescan.lock().unwrap();
            match full_scan(&conn, &home_rescan) {
                Ok(count) => eprintln!("luzumi-indexer: periodic rescan indexed {} files", count),
                Err(e) => eprintln!("luzumi-indexer: periodic rescan error: {}", e),
            }
        }
    });

    // Process file change events
    // Debounce: collect events for 500ms, then process
    loop {
        let mut paths: Vec<PathBuf> = Vec::new();

        // Wait for first event
        if let Some(event) = rx.recv().await {
            collect_paths(&event, &mut paths, &home);
        }

        // Collect more events for 500ms
        let deadline = tokio::time::Instant::now() + Duration::from_millis(500);
        loop {
            match tokio::time::timeout_at(deadline, rx.recv()).await {
                Ok(Some(event)) => collect_paths(&event, &mut paths, &home),
                _ => break,
            }
        }

        // Process collected paths
        if !paths.is_empty() {
            paths.sort();
            paths.dedup();
            let Ok(conn) = conn.lock() else { eprintln!("luzumi-indexer: lock poisoned"); continue };
            for path in &paths {
                if path.exists() {
                    let _ = index_path(&conn, path);
                } else {
                    let _ = remove_path(&conn, path);
                }
            }
        }
    }
}

fn collect_paths(event: &Event, paths: &mut Vec<PathBuf>, home: &Path) {
    match event.kind {
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
            for path in &event.paths {
                if !should_skip(path, home) {
                    paths.push(path.clone());
                }
            }
        }
        _ => {}
    }
}
