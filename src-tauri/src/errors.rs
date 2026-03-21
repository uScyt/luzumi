use serde::Serialize;
use std::io;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum AppError {
    #[serde(rename = "io")]
    Io { path: String, message: String },
    #[serde(rename = "permission")]
    Permission { path: String, message: String },
    #[serde(rename = "not_found")]
    NotFound { path: String, message: String },
    #[serde(rename = "conflict")]
    Conflict { path: String, existing: String, message: String },
    #[serde(rename = "cancelled")]
    Cancelled { message: String },
    #[serde(rename = "custom")]
    Custom { message: String },
}

impl AppError {
    pub fn io(path: impl Into<String>, msg: impl Into<String>) -> Self {
        AppError::Io { path: path.into(), message: msg.into() }
    }

    pub fn permission(path: impl Into<String>) -> Self {
        let p = path.into();
        AppError::Permission { path: p.clone(), message: format!("Permission denied: {}", p) }
    }

    pub fn not_found(path: impl Into<String>) -> Self {
        let p = path.into();
        AppError::NotFound { path: p.clone(), message: format!("Not found: {}", p) }
    }

    pub fn conflict(path: impl Into<String>, existing: impl Into<String>) -> Self {
        let p = path.into();
        let e = existing.into();
        AppError::Conflict {
            path: p.clone(),
            existing: e.clone(),
            message: format!("File already exists: {}", e),
        }
    }

    pub fn cancelled() -> Self {
        AppError::Cancelled { message: "Operation cancelled".into() }
    }

    pub fn custom(msg: impl Into<String>) -> Self {
        AppError::Custom { message: msg.into() }
    }

    pub fn from_io(err: io::Error, path: impl Into<String>) -> Self {
        let p = path.into();
        match err.kind() {
            io::ErrorKind::PermissionDenied => AppError::permission(&p),
            io::ErrorKind::NotFound => AppError::not_found(&p),
            io::ErrorKind::AlreadyExists => AppError::conflict(&p, &p),
            _ => AppError::io(&p, err.to_string()),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io { message, .. } => write!(f, "{}", message),
            AppError::Permission { message, .. } => write!(f, "{}", message),
            AppError::NotFound { message, .. } => write!(f, "{}", message),
            AppError::Conflict { message, .. } => write!(f, "{}", message),
            AppError::Cancelled { message } => write!(f, "{}", message),
            AppError::Custom { message } => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for AppError {}

// Convert to String for Tauri command returns (serialized JSON)
impl From<AppError> for String {
    fn from(err: AppError) -> String {
        serde_json::to_string(&err).unwrap_or_else(|_| err.to_string())
    }
}
