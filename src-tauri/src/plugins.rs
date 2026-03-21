use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn plugins_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("luzumi")
        .join("plugins")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    #[serde(default = "default_main")]
    pub main: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub hooks: Vec<String>,
}

fn default_main() -> String {
    "index.js".to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub enabled: bool,
    pub path: String,
    pub permissions: Vec<String>,
    pub hooks: Vec<String>,
}

fn enabled_file() -> PathBuf {
    plugins_dir().join(".enabled.json")
}

fn load_enabled_set() -> Vec<String> {
    let path = enabled_file();
    if !path.exists() {
        return Vec::new();
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn validate_plugin_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Plugin name cannot be empty".into());
    }
    if name.contains('/') || name.contains('\\') || name.contains('\0') || name.contains("..") {
        return Err("Plugin name contains invalid characters".into());
    }
    if name.starts_with('.') {
        return Err("Plugin name cannot start with '.'".into());
    }
    Ok(())
}

fn save_enabled_set(set: &[String]) {
    let path = enabled_file();
    if let Ok(json) = serde_json::to_string_pretty(set) {
        let _ = fs::write(path, json);
    }
}

pub fn list_plugins() -> Result<Vec<PluginInfo>, String> {
    let dir = plugins_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        return Ok(Vec::new());
    }

    let enabled = load_enabled_set();
    let mut plugins = Vec::new();

    let entries = fs::read_dir(&dir).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let manifest_path = path.join("manifest.json");
        if !manifest_path.exists() {
            continue;
        }
        let content = match fs::read_to_string(&manifest_path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let manifest: PluginManifest = match serde_json::from_str(&content) {
            Ok(m) => m,
            Err(_) => continue,
        };

        plugins.push(PluginInfo {
            enabled: enabled.contains(&manifest.name),
            path: path.to_string_lossy().to_string(),
            name: manifest.name,
            version: manifest.version,
            description: manifest.description,
            author: manifest.author,
            permissions: manifest.permissions,
            hooks: manifest.hooks,
        });
    }

    plugins.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(plugins)
}

pub fn enable_plugin(name: &str) -> Result<(), String> {
    validate_plugin_name(name)?;
    let mut set = load_enabled_set();
    if !set.contains(&name.to_string()) {
        set.push(name.to_string());
        save_enabled_set(&set);
    }
    Ok(())
}

pub fn disable_plugin(name: &str) -> Result<(), String> {
    validate_plugin_name(name)?;
    let mut set = load_enabled_set();
    set.retain(|n| n != name);
    save_enabled_set(&set);
    Ok(())
}

pub fn get_plugin_script(name: &str) -> Result<String, String> {
    validate_plugin_name(name)?;
    let dir = plugins_dir().join(name);
    let manifest_path = dir.join("manifest.json");
    if !manifest_path.exists() {
        return Err(format!("Plugin '{}' not found", name));
    }
    let content = fs::read_to_string(&manifest_path).map_err(|e| e.to_string())?;
    let manifest: PluginManifest = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    // Validate main field to prevent path traversal
    if manifest.main.contains("..") || manifest.main.contains('/') || manifest.main.contains('\\') {
        return Err("Plugin main field contains invalid path".into());
    }
    let script_path = dir.join(&manifest.main);
    // Verify the resolved path is still inside the plugin dir
    let canonical = script_path.canonicalize().map_err(|e| e.to_string())?;
    let canonical_dir = dir.canonicalize().map_err(|e| e.to_string())?;
    if !canonical.starts_with(&canonical_dir) {
        return Err("Plugin script path escapes plugin directory".into());
    }
    fs::read_to_string(&canonical).map_err(|e| e.to_string())
}

pub fn install_plugin_from_dir(source: &str) -> Result<String, String> {
    let source_path = PathBuf::from(source);
    let manifest_path = source_path.join("manifest.json");
    if !manifest_path.exists() {
        return Err("No manifest.json found in source directory".to_string());
    }
    let content = fs::read_to_string(&manifest_path).map_err(|e| e.to_string())?;
    let manifest: PluginManifest = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    let dest = plugins_dir().join(&manifest.name);
    if dest.exists() {
        fs::remove_dir_all(&dest).map_err(|e| e.to_string())?;
    }

    copy_dir_recursive(&source_path, &dest)?;
    Ok(manifest.name)
}

pub fn uninstall_plugin(name: &str) -> Result<(), String> {
    validate_plugin_name(name)?;
    disable_plugin(name)?;
    let dir = plugins_dir().join(name);
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn copy_dir_recursive(src: &PathBuf, dest: &PathBuf) -> Result<(), String> {
    fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
