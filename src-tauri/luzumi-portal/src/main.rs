mod indexer;

use std::collections::HashMap;
use std::path::PathBuf;
use tokio::process::Command;
use zbus::object_server::SignalEmitter;
use zbus::{connection, interface, zvariant};

/// XDG Desktop Portal FileChooser backend for Luzumi.
///
/// Implements `org.freedesktop.impl.portal.FileChooser` which is called by
/// `xdg-desktop-portal` when an application requests a file-open or file-save dialog.
///
/// The portal launches `luzumi` in picker mode, waits for the user to choose files,
/// then reads the result from a temporary JSON file and returns it via D-Bus.
struct FileChooserBackend;

/// org.freedesktop.FileManager1 — used by browsers for "Show in folder"
struct FileManagerBackend;

/// Find the luzumi binary. Check:
/// 1. Next to this binary (luzumi-portal)
/// 2. In $HOME/.local/bin/
/// 3. In $PATH
fn find_luzumi() -> Result<PathBuf, String> {
    // Next to this binary
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join("luzumi");
            if candidate.exists() {
                return Ok(candidate);
            }
        }
    }
    // In ~/.local/bin
    if let Some(home) = dirs_path() {
        let candidate = home.join(".local/bin/luzumi");
        if candidate.exists() {
            return Ok(candidate);
        }
    }
    // In PATH via `which`
    if let Ok(output) = std::process::Command::new("which").arg("luzumi").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
    }
    Err("Could not find luzumi binary".into())
}

fn dirs_path() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}

#[derive(Debug, serde::Deserialize)]
struct PickerResponse {
    response: u32,
    uris: Vec<String>,
}

/// Extract common options from the D-Bus options dictionary.
struct PickerOptions {
    multiple: bool,
    directory: bool,
    filters: Vec<serde_json::Value>,
    current_folder: Option<String>,
    current_name: Option<String>,
}

fn extract_options(options: &HashMap<String, zvariant::OwnedValue>) -> PickerOptions {
    let multiple = options
        .get("multiple")
        .and_then(|v| bool::try_from(v.clone()).ok())
        .unwrap_or(false);

    let directory = options
        .get("directory")
        .and_then(|v| bool::try_from(v.clone()).ok())
        .unwrap_or(false);

    let current_name = options
        .get("current_name")
        .and_then(|v| String::try_from(v.clone()).ok());

    // current_folder comes as a byte array (path)
    let current_folder = options.get("current_folder").and_then(|v| {
        // Try as byte array first (spec says ay)
        if let Ok(bytes) = <Vec<u8>>::try_from(v.clone()) {
            let trimmed: Vec<u8> = bytes.into_iter().take_while(|&b| b != 0).collect();
            String::from_utf8(trimmed).ok()
        } else {
            String::try_from(v.clone()).ok()
        }
    });

    // Filters: try to extract but don't fail if the type is too complex
    let filters = options
        .get("filters")
        .and_then(extract_filters)
        .unwrap_or_default();

    PickerOptions {
        multiple,
        directory,
        filters,
        current_folder,
        current_name,
    }
}

fn extract_filters(value: &zvariant::OwnedValue) -> Option<Vec<serde_json::Value>> {
    // Portal filters are a(sa(us)) — array of (name, array of (type, pattern))
    // type 0 = glob, type 1 = mime
    // Use the Value enum for pattern matching
    use zvariant::Value;
    let val: Value = value.clone().into();

    if let Value::Array(arr) = val {
        let mut result = Vec::new();
        for item in arr.iter() {
            if let Value::Structure(s) = item {
                let fields = s.fields();
                if fields.len() >= 2 {
                    let name = match &fields[0] {
                        Value::Str(s) => s.to_string(),
                        _ => continue,
                    };
                    let mut patterns = Vec::new();
                    if let Value::Array(pat_arr) = &fields[1] {
                        for pat_item in pat_arr.iter() {
                            if let Value::Structure(ps) = pat_item {
                                let pf = ps.fields();
                                if pf.len() >= 2 {
                                    if let (Value::U32(typ), Value::Str(pattern)) =
                                        (&pf[0], &pf[1])
                                    {
                                        if *typ == 0 {
                                            patterns.push(pattern.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if !patterns.is_empty() {
                        result.push(serde_json::json!({
                            "name": name,
                            "patterns": patterns,
                        }));
                    }
                }
            }
        }
        if !result.is_empty() {
            return Some(result);
        }
    }
    None
}

async fn run_picker(
    mode: &str,
    title: &str,
    options: PickerOptions,
) -> Result<PickerResponse, String> {
    let luzumi = find_luzumi()?;

    // Create temp file for response
    let response_file = {
        let f = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
        let path = f.path().to_path_buf();
        // Keep the file alive but let luzumi write to it
        f.keep().map_err(|e| e.to_string())?;
        path
    };

    let mut args = vec![
        format!("--picker-{}", mode),
        format!("--response-file={}", response_file.display()),
        format!("--title={}", title),
    ];

    if options.multiple {
        args.push("--multiple".into());
    }
    if options.directory {
        args.push("--directory".into());
    }
    if let Some(folder) = &options.current_folder {
        args.push(format!("--current-folder={}", folder));
    }
    if let Some(name) = &options.current_name {
        args.push(format!("--current-name={}", name));
    }
    if !options.filters.is_empty() {
        let json = serde_json::to_string(&options.filters).unwrap_or_default();
        args.push(format!("--filters={}", json));
    }

    // Launch luzumi and wait for it to exit
    let status = Command::new(&luzumi)
        .args(&args)
        .status()
        .await
        .map_err(|e| format!("Failed to launch luzumi: {}", e))?;

    // Read the response file
    let response_data = tokio::fs::read_to_string(&response_file)
        .await
        .unwrap_or_default();

    // Clean up
    let _ = tokio::fs::remove_file(&response_file).await;

    if response_data.is_empty() || !status.success() {
        return Ok(PickerResponse {
            response: 1,
            uris: vec![],
        });
    }

    serde_json::from_str::<PickerResponse>(&response_data)
        .map_err(|e| format!("Invalid picker response: {}", e))
}

type PortalResult = HashMap<String, zvariant::OwnedValue>;

fn build_response(response: &PickerResponse) -> (u32, PortalResult) {
    let mut results = HashMap::new();
    if response.response == 0 && !response.uris.is_empty() {
        // Must produce D-Bus type "as" (array of string), not "av" (array of variant)
        if let Ok(sig) = zvariant::Signature::from_bytes(b"s") {
            let mut arr = zvariant::Array::new(&sig);
            for uri in &response.uris {
                arr.append(zvariant::Value::from(uri.as_str())).ok();
            }
            if let Ok(val) = zvariant::OwnedValue::try_from(zvariant::Value::Array(arr)) {
                results.insert("uris".to_string(), val);
            }
        }

        if let Ok(val) = zvariant::OwnedValue::try_from(zvariant::Value::Bool(true)) {
            results.insert("writable".to_string(), val);
        }
    }
    eprintln!("luzumi-portal: response={}, uris={:?}", response.response, response.uris);
    (response.response, results)
}

#[interface(name = "org.freedesktop.impl.portal.FileChooser")]
impl FileChooserBackend {
    async fn open_file(
        &self,
        #[zbus(signal_emitter)] _emitter: SignalEmitter<'_>,
        _handle: zvariant::ObjectPath<'_>,
        _app_id: &str,
        _parent_window: &str,
        title: &str,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> zbus::fdo::Result<(u32, PortalResult)> {
        let opts = extract_options(&options);
        match run_picker("open", title, opts).await {
            Ok(resp) => Ok(build_response(&resp)),
            Err(_) => Ok((2, HashMap::new())),
        }
    }

    async fn save_file(
        &self,
        #[zbus(signal_emitter)] _emitter: SignalEmitter<'_>,
        _handle: zvariant::ObjectPath<'_>,
        _app_id: &str,
        _parent_window: &str,
        title: &str,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> zbus::fdo::Result<(u32, PortalResult)> {
        let opts = extract_options(&options);
        match run_picker("save", title, opts).await {
            Ok(resp) => Ok(build_response(&resp)),
            Err(_) => Ok((2, HashMap::new())),
        }
    }

    async fn save_files(
        &self,
        #[zbus(signal_emitter)] _emitter: SignalEmitter<'_>,
        _handle: zvariant::ObjectPath<'_>,
        _app_id: &str,
        _parent_window: &str,
        title: &str,
        options: HashMap<String, zvariant::OwnedValue>,
    ) -> zbus::fdo::Result<(u32, PortalResult)> {
        // SaveFiles is similar to SaveFile but for multiple files at once
        // We treat it the same way — show the save picker
        let opts = extract_options(&options);
        match run_picker("save", title, opts).await {
            Ok(resp) => Ok(build_response(&resp)),
            Err(_) => Ok((2, HashMap::new())),
        }
    }
}

#[interface(name = "org.freedesktop.FileManager1")]
impl FileManagerBackend {
    /// ShowItems: open the file manager and highlight the given files.
    /// Called by browsers for "Show in folder" / "Open file location".
    async fn show_items(&self, uris: Vec<String>, _startup_id: &str) {
        eprintln!("luzumi-portal: ShowItems called with {:?}", uris);
        if let Ok(luzumi) = find_luzumi() {
            // Extract the parent folder from the first URI and select the file
            for uri in &uris {
                let path = uri.strip_prefix("file://").unwrap_or(uri);
                let path = PathBuf::from(path);
                if let Some(parent) = path.parent() {
                    let mut cmd = Command::new(&luzumi);
                    cmd.arg(parent.to_string_lossy().as_ref());
                    // Pass --select=filename to highlight the file
                    if let Some(name) = path.file_name() {
                        cmd.arg(format!("--select={}", name.to_string_lossy()));
                    }
                    let _ = cmd.spawn();
                    return;
                }
            }
            // Fallback: just open luzumi
            let _ = Command::new(&luzumi).spawn();
        }
    }

    /// ShowFolders: open the file manager at the given folders.
    async fn show_folders(&self, uris: Vec<String>, _startup_id: &str) {
        eprintln!("luzumi-portal: ShowFolders called with {:?}", uris);
        if let Ok(luzumi) = find_luzumi() {
            for uri in &uris {
                let path = uri.strip_prefix("file://").unwrap_or(uri);
                let _ = Command::new(&luzumi).arg(path).spawn();
                return;
            }
        }
    }

    /// ShowItemProperties: not implemented, just open the folder.
    async fn show_item_properties(&self, uris: Vec<String>, _startup_id: &str) {
        self.show_items(uris, _startup_id).await;
    }

    /// SortDirectories: not implemented — just return empty.
    async fn sort_uris(&self, uris: Vec<String>) -> Vec<String> {
        uris
    }
}

/// Desktop context menu backend — called by the KDE Plasma ContainmentAction plugin
/// to show Luzumi's desktop context menu at a given screen position.
struct DesktopMenuBackend;

#[interface(name = "com.luzumi.DesktopMenu")]
impl DesktopMenuBackend {
    async fn show(&self, x: i32, y: i32) {
        eprintln!("luzumi-portal: DesktopMenu.Show({}, {})", x, y);
        // Kill any existing desktop-menu instance to avoid duplicates
        let _ = Command::new("pkill")
            .args(["-f", "luzumi --desktop-menu"])
            .status()
            .await;
        if let Ok(luzumi) = find_luzumi() {
            let _ = Command::new(&luzumi)
                .args([
                    "--desktop-menu",
                    &format!("--menu-x={}", x),
                    &format!("--menu-y={}", y),
                ])
                .spawn();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let daemon_mode = std::env::args().any(|a| a == "--daemon");

    let _conn = connection::Builder::session()?
        .name("org.freedesktop.impl.portal.desktop.luzumi")?
        .name("org.freedesktop.FileManager1")?
        .serve_at("/org/freedesktop/portal/desktop", FileChooserBackend)?
        .serve_at("/org/freedesktop/FileManager1", FileManagerBackend)?
        .serve_at("/com/luzumi/DesktopMenu", DesktopMenuBackend)?
        .build()
        .await?;

    eprintln!("luzumi-portal: D-Bus service started (FileChooser + FileManager1 + DesktopMenu)");

    if daemon_mode {
        // In daemon mode, also run the file indexer for fast search
        eprintln!("luzumi-portal: daemon mode — starting file indexer");
        tokio::spawn(async {
            indexer::run_indexer().await;
        });
    }

    // Keep the service running — the connection handles incoming method calls
    std::future::pending::<()>().await;
    Ok(())
}
