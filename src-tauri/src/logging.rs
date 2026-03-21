use std::fs;
use std::path::PathBuf;
use serde::Serialize;
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Serialize)]
pub struct LogEntry {
    pub timestamp: i64,
    pub level: String,
    pub message: String,
    pub target: String,
}

fn log_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("luzumi")
        .join("logs")
}

pub fn init_logging() {
    let log_path = log_dir();
    let _ = fs::create_dir_all(&log_path);

    let file_appender = tracing_appender::rolling::daily(&log_path, "luzumi.log");

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer().with_writer(file_appender).with_ansi(false))
        .try_init()
        .ok();
}

pub fn get_logs(lines: usize) -> Result<Vec<LogEntry>, String> {
    let log_path = log_dir();
    let mut entries = Vec::new();

    let dir = fs::read_dir(&log_path).map_err(|e| e.to_string())?;
    let mut files: Vec<_> = dir
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with("luzumi"))
        .collect();

    files.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

    for file in files.iter().take(3) {
        let content = fs::read_to_string(file.path()).unwrap_or_default();
        for line in content.lines().rev().take(lines.saturating_sub(entries.len())) {
            entries.push(LogEntry {
                timestamp: chrono::Utc::now().timestamp(),
                level: "info".into(),
                message: line.to_string(),
                target: "backend".into(),
            });
        }
        if entries.len() >= lines { break; }
    }

    entries.reverse();
    Ok(entries)
}

pub fn export_logs(path: &str) -> Result<(), String> {
    let log_path = log_dir();
    let mut combined = String::new();

    let dir = fs::read_dir(&log_path).map_err(|e| e.to_string())?;
    let mut files: Vec<_> = dir
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with("luzumi"))
        .collect();

    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for file in &files {
        let content = fs::read_to_string(file.path()).unwrap_or_default();
        combined.push_str(&content);
        combined.push('\n');
    }

    fs::write(path, combined).map_err(|e| e.to_string())?;
    Ok(())
}
