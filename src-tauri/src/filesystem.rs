use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::UNIX_EPOCH;
use serde::{Deserialize, Serialize};

// Run a command via pkexec with direct argv (no shell, no injection risk)
fn pkexec_run(program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new("pkexec")
        .arg(program)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run pkexec {}: {}", program, e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let code = output.status.code().unwrap_or(-1);
        if code == 126 || code == 127 {
            return Err("Authentication cancelled or pkexec not found".into());
        }
        Err(format!("{}: {}", program, stderr.trim()))
    }
}

pub fn kill_root_shell() {
    // No-op: root shell no longer exists. Kept for API compatibility.
}

pub fn check_path_readable(path: &Path) -> bool {
    fs::read_dir(path).is_ok()
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub kind: String,
    pub size: Option<u64>,
    pub modified: Option<i64>,
    pub is_hidden: bool,
    pub extension: Option<String>,
    pub is_writable: bool,
    pub permissions_mode: Option<u32>,
    pub is_broken_link: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkEntry {
    pub name: String,
    pub path: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveInfo {
    pub name: String,
    pub path: String,
    pub device: String,
    pub icon: String,
    pub is_mounted: bool,
    pub is_encrypted: bool,
    pub is_removable: bool,
    pub fstype: String,
}

#[derive(Debug, Deserialize)]
struct LsblkOutput {
    blockdevices: Vec<LsblkDevice>,
}

#[derive(Debug, Deserialize)]
struct LsblkDevice {
    name: String,
    label: Option<String>,
    mountpoint: Option<String>,
    hotplug: Option<bool>,
    #[serde(rename = "type")]
    dtype: Option<String>,
    fstype: Option<String>,
    size: Option<String>,
    children: Option<Vec<LsblkDevice>>,
}

impl FileEntry {
    pub fn from_path(path: &Path) -> Option<Self> {
        let meta = fs::symlink_metadata(path).ok()?;
        let name = path.file_name()?.to_string_lossy().to_string();
        let is_hidden = name.starts_with('.');

        let is_symlink = meta.is_symlink();
        let kind = if is_symlink {
            "symlink"
        } else if meta.is_dir() {
            "directory"
        } else if meta.is_file() {
            "file"
        } else {
            "other"
        }
        .to_string();

        let size = if kind == "file" || kind == "symlink" {
            Some(meta.len())
        } else {
            None
        };

        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase().to_string());

        let modified = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64);

        let is_writable = check_writable(path);

        use std::os::unix::fs::PermissionsExt;
        let permissions_mode = Some(meta.permissions().mode());

        // Broken symlink: read_link succeeds but target doesn't exist
        let is_broken_link = if is_symlink {
            fs::metadata(path).is_err()
        } else {
            false
        };

        Some(FileEntry {
            name,
            path: path.to_string_lossy().to_string(),
            kind,
            size,
            modified,
            is_hidden,
            extension,
            is_writable,
            permissions_mode,
            is_broken_link,
        })
    }
}

fn check_writable(path: &Path) -> bool {
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::fs::PermissionsExt;
    let meta = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return false,
    };
    let mode = meta.permissions().mode();
    let uid = meta.uid();
    let gid = meta.gid();
    let euid = unsafe { libc::geteuid() };
    let egid = unsafe { libc::getegid() };

    if euid == 0 { return true; }
    if euid == uid { return mode & 0o200 != 0; }
    if egid == gid { return mode & 0o020 != 0; }
    mode & 0o002 != 0
}

pub fn read_directory(path: &Path, show_hidden: bool) -> Result<Vec<FileEntry>, String> {
    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;

    let mut files: Vec<FileEntry> = entries
        .filter_map(|e| e.ok())
        .filter_map(|e| FileEntry::from_path(&e.path()))
        .filter(|e| show_hidden || !e.is_hidden)
        .collect();

    files.sort_by(|a, b| {
        let a_dir = a.kind == "directory";
        let b_dir = b.kind == "directory";
        match (a_dir, b_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(files)
}

pub fn elevated_read_directory(path: &Path, show_hidden: bool) -> Result<Vec<FileEntry>, String> {
    let path_str = path.to_string_lossy().to_string();
    let stdout = pkexec_run("find", &[
        &path_str, "-maxdepth", "1", "-mindepth", "1",
        "-printf", "%y\\t%s\\t%T@\\t%f\\n",
    ])?;
    let mut files: Vec<FileEntry> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(4, '\t').collect();
            if parts.len() < 4 { return None; }
            let ftype = parts[0];
            let size: u64 = parts[1].parse().unwrap_or(0);
            let mtime: f64 = parts[2].parse().unwrap_or(0.0);
            let name = parts[3].to_string();

            let is_hidden = name.starts_with('.');
            if !show_hidden && is_hidden { return None; }

            let full_path = path.join(&name);
            let kind = match ftype {
                "d" => "directory",
                "l" => "symlink",
                "f" => "file",
                _ => "other",
            }.to_string();

            let extension = if kind == "file" || kind == "symlink" {
                std::path::Path::new(&name)
                    .extension()
                    .map(|e| e.to_string_lossy().to_lowercase())
            } else {
                None
            };

            Some(FileEntry {
                name,
                path: full_path.to_string_lossy().to_string(),
                kind: kind.clone(),
                size: if kind == "file" || kind == "symlink" { Some(size) } else { None },
                modified: Some(mtime as i64),
                is_hidden,
                extension: extension.map(|s| s.to_string()),
                is_writable: true, // elevated = root, always writable
                permissions_mode: None,
                is_broken_link: false,
            })
        })
        .collect();

    files.sort_by(|a, b| {
        let a_dir = a.kind == "directory";
        let b_dir = b.kind == "directory";
        match (a_dir, b_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(files)
}

pub fn create_directory(parent: &Path, name: &str) -> Result<(), String> {
    if name.contains('/') || name.contains("..") || name.contains('\0') {
        return Err("Invalid directory name".into());
    }
    fs::create_dir(parent.join(name)).map_err(|e| e.to_string())
}

pub fn rename_entry(from: &Path, new_name: &str) -> Result<(), String> {
    if new_name.contains('/') || new_name.contains('\0') {
        return Err("Invalid file name".into());
    }
    let parent = from.parent().ok_or("Cannot rename root")?;
    fs::rename(from, parent.join(new_name)).map_err(|e| e.to_string())
}

pub fn delete_entries(paths: &[&Path]) -> Result<(), String> {
    for p in paths {
        trash::delete(p).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn count_secure_targets(paths: &[&Path]) -> usize {
    paths.iter().map(|p| {
        if p.is_dir() { count_dir_files(p) } else { 1 }
    }).sum()
}

fn count_dir_files(dir: &Path) -> usize {
    fs::read_dir(dir).ok()
        .map(|rd| rd.filter_map(|e| e.ok()).map(|e| {
            if e.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                count_dir_files(&e.path())
            } else { 1 }
        }).sum())
        .unwrap_or(0)
}

pub fn secure_delete_streamed<F>(paths: &[&Path], mut on_file: F) -> Result<(), String>
where F: FnMut(&str) {
    for p in paths {
        if p.is_dir() {
            secure_delete_dir_streamed(p, &mut on_file)?;
        } else {
            on_file(&p.to_string_lossy());
            secure_delete_file(p)?;
        }
    }
    Ok(())
}

fn secure_delete_dir_streamed<F>(path: &Path, on_file: &mut F) -> Result<(), String>
where F: FnMut(&str) {
    let entries: Vec<_> = fs::read_dir(path)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok()).collect();
    for entry in entries {
        let ft = entry.file_type().map_err(|e| e.to_string())?;
        if ft.is_dir() {
            secure_delete_dir_streamed(&entry.path(), on_file)?;
        } else {
            on_file(&entry.path().to_string_lossy());
            secure_delete_file(&entry.path())?;
        }
    }
    fs::remove_dir(path).map_err(|e| e.to_string())
}

fn secure_delete_file(path: &Path) -> Result<(), String> {
    let meta = fs::metadata(path).map_err(|e| e.to_string())?;
    let size = meta.len();

    if size > 0 {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|e| e.to_string())?;

        let chunk_size = 65536usize;
        let buf_zeros = vec![0u8; chunk_size];
        let buf_ones = vec![0xFFu8; chunk_size];

        for pass in 0..3 {
            file.seek_to_start()?;
            let pattern: &[u8] = match pass {
                0 => &buf_zeros,
                1 => &buf_ones,
                _ => &buf_zeros,
            };
            let mut written = 0u64;
            while written < size {
                let chunk = chunk_size.min((size - written) as usize);
                file.write_all(&pattern[..chunk]).map_err(|e| e.to_string())?;
                written += chunk as u64;
            }
            file.sync_all().map_err(|e| e.to_string())?;
        }

        use std::io::Seek;
        file.seek(std::io::SeekFrom::Start(0)).map_err(|e| e.to_string())?;
        let mut rng_written = 0u64;
        while rng_written < size {
            let chunk = chunk_size.min((size - rng_written) as usize);
            let random_buf: Vec<u8> = (0..chunk).map(|i| {
                ((i as u64 ^ rng_written ^ 0xDEADBEEF) & 0xFF) as u8
            }).collect();
            file.write_all(&random_buf).map_err(|e| e.to_string())?;
            rng_written += chunk as u64;
        }
        file.sync_all().map_err(|e| e.to_string())?;
    }

    let random_name = format!(".del_{}", std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos());
    let parent = path.parent().ok_or("No parent directory")?;
    let renamed = parent.join(&random_name);
    fs::rename(path, &renamed).map_err(|e| e.to_string())?;
    fs::remove_file(&renamed).map_err(|e| e.to_string())
}

fn secure_delete_dir(path: &Path) -> Result<(), String> {
    let entries: Vec<_> = fs::read_dir(path)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .collect();

    for entry in entries {
        let ft = entry.file_type().map_err(|e| e.to_string())?;
        if ft.is_dir() {
            secure_delete_dir(&entry.path())?;
        } else {
            secure_delete_file(&entry.path())?;
        }
    }
    fs::remove_dir(path).map_err(|e| e.to_string())
}

trait SeekToStart {
    fn seek_to_start(&mut self) -> Result<(), String>;
}

impl SeekToStart for fs::File {
    fn seek_to_start(&mut self) -> Result<(), String> {
        use std::io::Seek;
        self.seek(std::io::SeekFrom::Start(0)).map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub fn copy_entry(from: &Path, to_dir: &Path) -> Result<(), String> {
    let name = from.file_name().ok_or("No filename")?;
    let dest = to_dir.join(name);
    if from.is_dir() {
        copy_dir_all(from, &dest)
    } else {
        fs::copy(from, &dest).map(|_| ()).map_err(|e| e.to_string())
    }
}

pub fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), String> {
    fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let meta = entry.path().symlink_metadata().map_err(|e| e.to_string())?;
        if meta.is_symlink() {
            let target = fs::read_link(entry.path()).map_err(|e| e.to_string())?;
            // Reject symlinks that point outside (absolute or containing ..)
            if target.is_absolute() || target.components().any(|c| c == std::path::Component::ParentDir) {
                // Copy the underlying file instead of recreating a dangerous symlink
                let real = entry.path().canonicalize().map_err(|e| e.to_string())?;
                let dest = dst.join(entry.file_name());
                if real.is_dir() {
                    copy_dir_all(&real, &dest)?;
                } else {
                    fs::copy(&real, &dest).map_err(|e| e.to_string())?;
                }
            } else {
                std::os::unix::fs::symlink(&target, dst.join(entry.file_name()))
                    .map_err(|e| e.to_string())?;
            }
        } else if meta.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub fn move_entry(from: &Path, to_dir: &Path) -> Result<(), String> {
    let name = from.file_name().ok_or("No filename")?;
    let dest = to_dir.join(name);
    match fs::rename(from, &dest) {
        Ok(_) => Ok(()),
        Err(_) => {
            copy_entry(from, to_dir)?;
            if from.is_dir() {
                fs::remove_dir_all(from).map_err(|e| e.to_string())
            } else {
                fs::remove_file(from).map_err(|e| e.to_string())
            }
        }
    }
}

pub fn get_bookmarks() -> Vec<BookmarkEntry> {
    let mut out = Vec::new();
    let add = |out: &mut Vec<BookmarkEntry>, name: &str, path: Option<PathBuf>, icon: &str| {
        if let Some(p) = path {
            if p.exists() {
                out.push(BookmarkEntry {
                    name: name.to_string(),
                    path: p.to_string_lossy().to_string(),
                    icon: icon.to_string(),
                });
            }
        }
    };
    add(&mut out, "Home",      dirs::home_dir(),     "House");
    add(&mut out, "Desktop",   dirs::desktop_dir(),  "Monitor");
    add(&mut out, "Documents", dirs::document_dir(), "Files");
    add(&mut out, "Downloads", dirs::download_dir(), "DownloadSimple");
    add(&mut out, "Music",     dirs::audio_dir(),    "MusicNotes");
    add(&mut out, "Pictures",  dirs::picture_dir(),  "Image");
    add(&mut out, "Videos",    dirs::video_dir(),    "VideoCamera");
    add(&mut out, "Public",    dirs::public_dir(),   "Users");
    out
}

pub fn get_drives() -> Vec<DriveInfo> {
    let mut drives = Vec::new();

    if let Ok(output) = Command::new("lsblk")
        .args(["-J", "-o", "NAME,LABEL,MOUNTPOINT,HOTPLUG,TYPE,FSTYPE,SIZE"])
        .output()
    {
        if let Ok(parsed) = serde_json::from_slice::<LsblkOutput>(&output.stdout) {
            for dev in &parsed.blockdevices {
                collect_drives(&mut drives, dev, false);
            }
        }
    }

    if drives.is_empty() {
        if let Ok(content) = fs::read_to_string("/proc/mounts") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let mp = parts[1];
                    if mp.starts_with("/media") || mp.starts_with("/mnt") || mp.starts_with("/run/media") {
                        let path = PathBuf::from(mp);
                        if path.exists() {
                            let name = path
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| mp.to_string());
                            drives.push(DriveInfo {
                                name,
                                path: mp.to_string(),
                                device: parts[0].to_string(),
                                icon: "HardDrives".to_string(),
                                is_mounted: true,
                                is_encrypted: false,
                                is_removable: true,
                                fstype: parts.get(2).unwrap_or(&"").to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    // Detect root filesystem type dynamically from /proc/mounts
    let root_fstype = fs::read_to_string("/proc/mounts").ok()
        .and_then(|content| {
            content.lines()
                .find(|l| l.split_whitespace().nth(1) == Some("/"))
                .and_then(|l| l.split_whitespace().nth(2).map(String::from))
        })
        .unwrap_or_else(|| "ext4".to_string());

    let root_device = fs::read_to_string("/proc/mounts").ok()
        .and_then(|content| {
            content.lines()
                .find(|l| l.split_whitespace().nth(1) == Some("/"))
                .and_then(|l| l.split_whitespace().next().map(String::from))
        })
        .unwrap_or_else(|| "/dev/root".to_string());

    drives.push(DriveInfo {
        name: "Filesystem".to_string(),
        path: "/".to_string(),
        device: root_device,
        icon: "HardDrives".to_string(),
        is_mounted: true,
        is_encrypted: false,
        is_removable: false,
        fstype: root_fstype,
    });

    // Detect GVFS mounts
    let uid = unsafe { libc::getuid() };
    let gvfs_dir = PathBuf::from(format!("/run/user/{}/gvfs", uid));
    if gvfs_dir.exists() {
        if let Ok(entries) = fs::read_dir(&gvfs_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let display_name = name.split(':').next().unwrap_or(&name).replace('-', " ");
                    drives.push(DriveInfo {
                        name: display_name,
                        path: path.to_string_lossy().to_string(),
                        device: name,
                        icon: "Globe".to_string(),
                        is_mounted: true,
                        is_encrypted: false,
                        is_removable: false,
                        fstype: "gvfs".to_string(),
                    });
                }
            }
        }
    }

    drives
}

fn collect_drives(out: &mut Vec<DriveInfo>, dev: &LsblkDevice, parent_hotplug: bool) {
    let hotplug = dev.hotplug.unwrap_or(parent_hotplug);
    let dtype = dev.dtype.as_deref().unwrap_or("");
    let fstype = dev.fstype.as_deref().unwrap_or("");
    let is_encrypted = fstype == "crypto_LUKS";

    if (dtype == "part" || dtype == "lvm" || is_encrypted) && hotplug {
        let mounted = dev.mountpoint.is_some();
        let mount_path = dev.mountpoint.as_deref().unwrap_or("").to_string();
        let label = dev.label.as_deref().unwrap_or("");
        let size_str = dev.size.as_deref().unwrap_or("");

        let name = if !label.is_empty() {
            label.to_string()
        } else if !size_str.is_empty() {
            format!("USB ({})", size_str)
        } else {
            dev.name.clone()
        };

        let icon = if is_encrypted { "Lock" } else { "Usb" };

        out.push(DriveInfo {
            name,
            path: mount_path,
            device: format!("/dev/{}", dev.name),
            icon: icon.to_string(),
            is_mounted: mounted,
            is_encrypted,
            is_removable: true,
            fstype: fstype.to_string(),
        });
    }

    if let Some(children) = &dev.children {
        for child in children {
            collect_drives(out, child, hotplug);
        }
    }
}

fn validate_device_path(device: &str) -> Result<(), String> {
    if !device.starts_with("/dev/") || device.contains("..") || device.contains('\0') {
        return Err("Invalid device path".into());
    }
    Ok(())
}

pub fn mount_drive(device: &str) -> Result<String, String> {
    validate_device_path(device)?;
    let output = Command::new("udisksctl")
        .args(["mount", "-b", device, "--no-user-interaction"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let mount_point = stdout
            .split("at ")
            .nth(1)
            .unwrap_or("")
            .trim()
            .trim_end_matches('.')
            .to_string();
        Ok(mount_point)
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn unlock_drive(device: &str, password: &str) -> Result<String, String> {
    validate_device_path(device)?;
    let mut child = Command::new("udisksctl")
        .args(["unlock", "-b", device, "--no-user-interaction"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(password.as_bytes()).map_err(|e| e.to_string())?;
        stdin.write_all(b"\n").map_err(|e| e.to_string())?;
    }

    let output = child.wait_with_output().map_err(|e| e.to_string())?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let unlocked_dev = stdout
            .split("as ")
            .nth(1)
            .unwrap_or("")
            .trim()
            .trim_end_matches('.')
            .to_string();

        if !unlocked_dev.is_empty() {
            return mount_drive(&unlocked_dev);
        }
        Ok(unlocked_dev)
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn read_thumbnail(path: &Path) -> Result<String, String> {
    let ext = path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase().to_string())
        .unwrap_or_default();

    // Video formats — extract frame with ffmpeg
    match ext.as_str() {
        "mp4" | "m4v" | "mkv" | "avi" | "mov" | "webm" | "flv" | "wmv" | "mpg" | "mpeg"
        | "ogv" | "ogg" | "3gp" | "ts" | "mts" | "m2ts" | "vob" => {
            return read_video_thumbnail(path);
        }
        _ => {}
    }

    let data = fs::read(path).map_err(|e| e.to_string())?;

    if data.len() > 50 * 1024 * 1024 {
        return Err("File too large for thumbnail".to_string());
    }

    let mime = match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => return Err("Unsupported format".to_string()),
    };

    use base64::Engine;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&data);
    Ok(format!("data:{};base64,{}", mime, encoded))
}

fn read_video_thumbnail(path: &Path) -> Result<String, String> {
    use std::process::Command;

    let output = Command::new("ffmpeg")
        .args([
            "-ss", "1",
            "-i", &path.to_string_lossy(),
            "-vframes", "1",
            "-vf", "scale=320:-1",
            "-f", "image2pipe",
            "-vcodec", "png",
            "-",
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .map_err(|e| format!("ffmpeg not found: {}", e))?;

    if !output.status.success() || output.stdout.is_empty() {
        return Err("Failed to extract video thumbnail".to_string());
    }

    use base64::Engine;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&output.stdout);
    Ok(format!("data:image/png;base64,{}", encoded))
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDetails {
    pub name: String,
    pub path: String,
    pub kind: String,
    pub size: Option<u64>,
    pub modified: Option<i64>,
    pub created: Option<i64>,
    pub accessed: Option<i64>,
    pub extension: Option<String>,
    pub mime_type: String,
    pub permissions: String,
    pub permissions_mode: u32,
    pub owner: String,
    pub group: String,
    pub is_readonly: bool,
    pub is_executable: bool,
    pub link_target: Option<String>,
    pub link_target_exists: Option<bool>,
    pub children_count: Option<u64>,
    pub dir_size: Option<u64>,
}

pub fn get_file_details(path: &Path) -> Result<FileDetails, String> {
    let meta = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    let name = path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());

    let kind = if meta.is_symlink() { "symlink" }
        else if meta.is_dir() { "directory" }
        else if meta.is_file() { "file" }
        else { "other" }.to_string();

    let size = if meta.is_file() { Some(meta.len()) } else { None };

    let modified = meta.modified().ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64);

    let created = meta.created().ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64);

    let accessed = meta.accessed().ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64);

    let extension = path.extension()
        .map(|e| e.to_string_lossy().to_lowercase());

    let mime_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();

    use std::os::unix::fs::MetadataExt;
    use std::os::unix::fs::PermissionsExt;
    let mode = meta.permissions().mode();
    let permissions = format!(
        "{}{}{}{}{}{}{}{}{}",
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' },
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' },
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' },
    );

    let uid = meta.uid();
    let gid = meta.gid();

    let owner = Command::new("id")
        .args(["-nu", &uid.to_string()])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| uid.to_string());

    let group = Command::new("getent")
        .args(["group", &gid.to_string()])
        .output()
        .ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            s.split(':').next().map(|n| n.to_string())
        })
        .unwrap_or_else(|| gid.to_string());

    let is_readonly = meta.permissions().readonly();
    let is_executable = mode & 0o111 != 0;

    let link_target = if meta.is_symlink() {
        fs::read_link(path).ok().map(|p| p.to_string_lossy().to_string())
    } else {
        None
    };

    let link_target_exists = if meta.is_symlink() {
        Some(fs::metadata(path).is_ok())
    } else {
        None
    };

    let (children_count, dir_size) = if path.is_dir() {
        let count = fs::read_dir(path).ok()
            .map(|rd| rd.count() as u64);
        let ds = calculate_dir_size(path);
        (count, Some(ds))
    } else {
        (None, None)
    };

    Ok(FileDetails {
        name,
        path: path.to_string_lossy().to_string(),
        kind,
        size,
        modified,
        created,
        accessed,
        extension: extension.map(|e| e.to_string()),
        mime_type,
        permissions,
        permissions_mode: mode,
        owner,
        group,
        is_readonly,
        is_executable,
        link_target,
        link_target_exists,
        children_count,
        dir_size,
    })
}

pub fn calculate_dir_size(path: &Path) -> u64 {
    calculate_dir_size_bounded(path, 0)
}

fn calculate_dir_size_bounded(path: &Path, depth: u32) -> u64 {
    if depth > 100 { return 0; }
    let mut total = 0u64;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            let meta = match fs::symlink_metadata(&p) {
                Ok(m) => m,
                Err(_) => continue,
            };
            if meta.is_dir() {
                total += calculate_dir_size_bounded(&p, depth + 1);
            } else if meta.is_file() {
                total += meta.len();
            }
        }
    }
    total
}

pub fn authenticate_admin() -> Result<(), String> {
    pkexec_run("true", &[]).map(|_| ())
}

pub fn elevated_move(from: &Path, to_dir: &Path) -> Result<(), String> {
    let name = from.file_name().ok_or("No filename")?;
    let dest = to_dir.join(name);
    let from_s = from.to_string_lossy();
    let dest_s = dest.to_string_lossy();
    pkexec_run("mv", &["-f", "--", &from_s, &dest_s]).map(|_| ())
}

pub fn elevated_copy(from: &Path, to_dir: &Path) -> Result<(), String> {
    let name = from.file_name().ok_or("No filename")?;
    let dest = to_dir.join(name);
    let from_s = from.to_string_lossy();
    let dest_s = dest.to_string_lossy();
    pkexec_run("cp", &["-a", "--", &from_s, &dest_s]).map(|_| ())
}

pub fn elevated_delete(paths: &[&Path]) -> Result<(), String> {
    for p in paths {
        let p_s = p.to_string_lossy();
        pkexec_run("rm", &["-rf", "--", &p_s])?;
    }
    Ok(())
}

pub fn elevated_rename(from: &Path, new_name: &str) -> Result<(), String> {
    if new_name.contains('/') || new_name.contains('\0') {
        return Err("Invalid file name".into());
    }
    let parent = from.parent().ok_or("Cannot rename root")?;
    let dest = parent.join(new_name);
    let from_s = from.to_string_lossy();
    let dest_s = dest.to_string_lossy();
    pkexec_run("mv", &["--", &from_s, &dest_s]).map(|_| ())
}

pub fn elevated_create_directory(parent: &Path, name: &str) -> Result<(), String> {
    if name.contains('/') || name.contains("..") || name.contains('\0') {
        return Err("Invalid directory name".into());
    }
    let dest = parent.join(name);
    let dest_s = dest.to_string_lossy();
    pkexec_run("mkdir", &["-p", "--", &dest_s]).map(|_| ())
}

pub fn eject_drive(device: &str) -> Result<(), String> {
    validate_device_path(device)?;
    let _ = Command::new("udisksctl")
        .args(["unmount", "-b", device, "--no-user-interaction"])
        .output();

    let output = Command::new("udisksctl")
        .args(["power-off", "-b", device, "--no-user-interaction"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        let _ = Command::new("udisksctl")
            .args(["unmount", "-b", device, "--no-user-interaction"])
            .output();
        Ok(())
    }
}

// ── Trash ──

pub fn get_trash_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("Trash")
}

fn get_all_trash_paths() -> Vec<PathBuf> {
    let mut paths = vec![get_trash_path()];
    let uid = unsafe { libc::getuid() };
    if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let mp = parts[1];
                if mp.starts_with("/media") || mp.starts_with("/mnt") || mp.starts_with("/run/media") {
                    let ext_trash = PathBuf::from(mp).join(format!(".Trash-{}", uid));
                    if ext_trash.exists() {
                        paths.push(ext_trash);
                    }
                }
            }
        }
    }
    paths
}

pub fn list_all_trash() -> Result<Vec<FileEntry>, String> {
    let mut all = Vec::new();
    for trash_path in get_all_trash_paths() {
        let files_dir = trash_path.join("files");
        if files_dir.exists() {
            if let Ok(entries) = read_directory(&files_dir, true) {
                all.extend(entries);
            }
        }
    }
    Ok(all)
}

pub fn get_all_trash_size() -> u64 {
    let mut total = 0;
    for trash_path in get_all_trash_paths() {
        let files_dir = trash_path.join("files");
        if files_dir.exists() {
            total += calculate_dir_size(&files_dir);
        }
    }
    total
}

fn empty_single_trash(trash_path: &Path) -> Result<(), String> {
    let files_dir = trash_path.join("files");
    let info_dir = trash_path.join("info");

    if files_dir.exists() {
        let entries: Vec<_> = fs::read_dir(&files_dir)
            .map_err(|e| e.to_string())?
            .filter_map(|e| e.ok())
            .collect();
        for entry in entries {
            let p = entry.path();
            if p.is_dir() {
                secure_delete_dir(&p)?;
            } else {
                secure_delete_file(&p)?;
            }
        }
    }

    if info_dir.exists() {
        let entries: Vec<_> = fs::read_dir(&info_dir)
            .map_err(|e| e.to_string())?
            .filter_map(|e| e.ok())
            .collect();
        for entry in entries {
            fs::remove_file(entry.path()).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

pub fn empty_all_trash() -> Result<(), String> {
    for trash_path in get_all_trash_paths() {
        empty_single_trash(&trash_path)?;
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrashItemInfo {
    pub original_path: Option<String>,
    pub deletion_date: Option<String>,
}

pub fn get_trash_item_info(file_name: &str) -> TrashItemInfo {
    for trash_path in get_all_trash_paths() {
        let info_path = trash_path.join("info").join(format!("{}.trashinfo", file_name));
        if info_path.exists() {
            if let Ok(content) = fs::read_to_string(&info_path) {
                let mut original_path = None;
                let mut deletion_date = None;
                for line in content.lines() {
                    if let Some(p) = line.strip_prefix("Path=") {
                        original_path = Some(trash_url_decode(p));
                    } else if let Some(d) = line.strip_prefix("DeletionDate=") {
                        deletion_date = Some(d.to_string());
                    }
                }
                return TrashItemInfo { original_path, deletion_date };
            }
        }
    }
    TrashItemInfo { original_path: None, deletion_date: None }
}

pub fn restore_from_trash(file_name: &str) -> Result<(), String> {
    if file_name.is_empty() || file_name.contains('/') || file_name.contains('\\') || file_name.contains("..") {
        return Err("Invalid trash file name".into());
    }

    let trash_path = get_trash_path();
    let trash_files_dir = trash_path.join("files");
    let file_path = trash_files_dir.join(file_name);
    let info_path = trash_path.join("info").join(format!("{}.trashinfo", file_name));

    // Verify the resolved path stays within trash/files/
    if let Ok(canonical) = file_path.canonicalize() {
        if let Ok(canonical_base) = trash_files_dir.canonicalize() {
            if !canonical.starts_with(&canonical_base) {
                return Err("Path escape detected in trash file".into());
            }
        }
    }

    if !file_path.exists() {
        return Err(format!("File '{}' not found in trash", file_name));
    }

    let original_path = if info_path.exists() {
        fs::read_to_string(&info_path)
            .ok()
            .and_then(|content| {
                content.lines()
                    .find(|l| l.starts_with("Path="))
                    .map(|l| {
                        let raw = l.trim_start_matches("Path=");
                        trash_url_decode(raw)
                    })
            })
    } else {
        None
    };

    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    let dest = if let Some(ref orig) = original_path {
        PathBuf::from(orig)
    } else {
        home.join(file_name)
    };

    // Validate destination: must be within home, /tmp, or /media (external drives)
    let dest_str = dest.to_string_lossy();
    let home_str = home.to_string_lossy();
    let allowed = dest_str.starts_with(home_str.as_ref())
        || dest_str.starts_with("/tmp")
        || dest_str.starts_with("/media")
        || dest_str.starts_with("/mnt");
    if !allowed {
        return Err(format!("Restore destination {} is outside allowed directories", dest.display()));
    }

    // Reject sensitive system directories
    for blocked in &["/proc", "/sys", "/dev", "/etc", "/boot", "/usr/bin", "/usr/sbin"] {
        if dest_str.starts_with(blocked) {
            return Err(format!("Cannot restore to {}", blocked));
        }
    }

    if let Some(parent) = dest.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    fs::rename(&file_path, &dest).map_err(|e| e.to_string())?;

    if info_path.exists() {
        let _ = fs::remove_file(&info_path);
    }

    Ok(())
}

fn trash_url_decode(s: &str) -> String {
    use percent_encoding::percent_decode_str;
    percent_decode_str(s).decode_utf8_lossy().to_string()
}

// ── Permissions modification ──

pub fn set_permissions(path: &Path, mode: u32) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let perms = fs::Permissions::from_mode(mode);
    fs::set_permissions(path, perms).map_err(|e| e.to_string())
}

pub fn elevated_set_permissions(path: &Path, mode: u32) -> Result<(), String> {
    let mode_str = format!("{:o}", mode);
    let path_s = path.to_string_lossy();
    pkexec_run("chmod", &[&mode_str, "--", &path_s]).map(|_| ())
}

// ── Symlink creation ──

pub fn create_symlink(target: &Path, link_path: &Path) -> Result<(), String> {
    std::os::unix::fs::symlink(target, link_path).map_err(|e| e.to_string())
}

// ── Quick Access ──

fn quick_access_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("luzumi")
        .join("quick_access.json")
}

pub fn get_quick_access() -> Vec<BookmarkEntry> {
    let path = quick_access_path();
    if !path.exists() {
        return Vec::new();
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn add_quick_access(name: &str, dir_path: &str, icon: &str) -> Result<(), String> {
    let mut items = get_quick_access();
    if items.iter().any(|i| i.path == dir_path) {
        return Ok(());
    }
    items.push(BookmarkEntry {
        name: name.to_string(),
        path: dir_path.to_string(),
        icon: icon.to_string(),
    });
    save_quick_access(&items)
}

pub fn remove_quick_access(dir_path: &str) -> Result<(), String> {
    let mut items = get_quick_access();
    items.retain(|i| i.path != dir_path);
    save_quick_access(&items)
}

fn save_quick_access(items: &[BookmarkEntry]) -> Result<(), String> {
    let path = quick_access_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(items).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

#[derive(Clone, Serialize)]
pub struct DuplicateGroup {
    pub size: u64,
    pub paths: Vec<String>,
}

pub fn find_duplicates(dir: &Path, recursive: bool) -> Result<Vec<DuplicateGroup>, String> {
    use std::collections::HashMap;

    let mut size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    fn collect(dir: &Path, recursive: bool, map: &mut HashMap<u64, Vec<PathBuf>>, depth: u32) {
        if depth > 100 { return; }
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let ft = match entry.file_type() {
                    Ok(ft) => ft,
                    Err(_) => continue,
                };
                let path = entry.path();
                if ft.is_dir() {
                    if recursive {
                        collect(&path, true, map, depth + 1);
                    }
                } else if ft.is_file() {
                    if let Ok(meta) = fs::metadata(&path) {
                        let size = meta.len();
                        if size > 0 {
                            map.entry(size).or_default().push(path);
                        }
                    }
                }
            }
        }
    }

    collect(dir, recursive, &mut size_map, 0);

    size_map.retain(|_, v| v.len() > 1);

    let mut groups: Vec<DuplicateGroup> = Vec::new();

    for (size, paths) in &size_map {
        let mut content_map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

        for path in paths {
            if let Ok(fingerprint) = file_fingerprint(path, *size) {
                content_map.entry(fingerprint).or_default().push(path.to_string_lossy().to_string());
            }
        }

        for (_, dup_paths) in content_map {
            if dup_paths.len() > 1 {
                groups.push(DuplicateGroup {
                    size: *size,
                    paths: dup_paths,
                });
            }
        }
    }

    groups.sort_by(|a, b| b.size.cmp(&a.size));
    Ok(groups)
}

fn file_fingerprint(path: &Path, size: u64) -> Result<Vec<u8>, std::io::Error> {
    use std::io::{Read, Seek, SeekFrom};

    const MAX_FINGERPRINT_SIZE: u64 = 100 * 1024 * 1024; // 100MB limit
    if size > MAX_FINGERPRINT_SIZE {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "File too large for fingerprinting"));
    }

    // Only fingerprint regular files (not symlinks, FIFOs, /dev/zero, etc.)
    let meta = fs::symlink_metadata(path)?;
    if !meta.file_type().is_file() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not a regular file"));
    }

    let mut f = fs::File::open(path)?;
    let chunk = 4096u64;

    if size <= chunk * 2 {
        let mut buf = Vec::with_capacity(size as usize);
        f.read_to_end(&mut buf)?;
        return Ok(buf);
    }

    let mut buf = vec![0u8; chunk as usize * 2];
    f.read_exact(&mut buf[..chunk as usize])?;
    f.seek(SeekFrom::End(-(chunk as i64)))?;
    f.read_exact(&mut buf[chunk as usize..])?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_device_path_valid() {
        assert!(validate_device_path("/dev/sda1").is_ok());
        assert!(validate_device_path("/dev/nvme0n1p1").is_ok());
        assert!(validate_device_path("/dev/dm-0").is_ok());
    }

    #[test]
    fn test_validate_device_path_rejects_traversal() {
        assert!(validate_device_path("/dev/../etc/shadow").is_err());
        assert!(validate_device_path("/dev/sda1\0/etc/passwd").is_err());
    }

    #[test]
    fn test_validate_device_path_rejects_non_dev() {
        assert!(validate_device_path("/etc/passwd").is_err());
        assert!(validate_device_path("/tmp/fake").is_err());
        assert!(validate_device_path("sda1").is_err());
        assert!(validate_device_path("").is_err());
    }

    #[test]
    fn test_copy_dir_all_basic() {
        let tmp = std::env::temp_dir().join("luzumi_test_copy");
        let src = tmp.join("src");
        let dst = tmp.join("dst");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("file.txt"), "hello").unwrap();
        fs::create_dir_all(src.join("sub")).unwrap();
        fs::write(src.join("sub/nested.txt"), "world").unwrap();

        copy_dir_all(&src, &dst).unwrap();

        assert_eq!(fs::read_to_string(dst.join("file.txt")).unwrap(), "hello");
        assert_eq!(fs::read_to_string(dst.join("sub/nested.txt")).unwrap(), "world");

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_copy_dir_all_rejects_dangerous_symlinks() {
        let tmp = std::env::temp_dir().join("luzumi_test_symlink");
        let src = tmp.join("src");
        let dst = tmp.join("dst");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&src).unwrap();

        // Create a symlink pointing to an absolute path
        std::os::unix::fs::symlink("/etc/passwd", src.join("evil")).unwrap();
        // Create a relative symlink with ..
        std::os::unix::fs::symlink("../../etc/shadow", src.join("sneaky")).unwrap();

        // copy_dir_all should copy the target content, not recreate dangerous symlinks
        let result = copy_dir_all(&src, &dst);
        // It may succeed (copies target) or fail (target doesn't exist as regular file)
        // but it should NOT create symlinks pointing outside
        if result.is_ok() {
            let evil_meta = fs::symlink_metadata(dst.join("evil"));
            if let Ok(m) = evil_meta {
                assert!(!m.is_symlink(), "Dangerous absolute symlink should not be recreated");
            }
        }

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_trash_filename_validation() {
        // These should all fail validation in restore_from_trash
        assert!(restore_from_trash("").is_err());
        assert!(restore_from_trash("../escape").is_err());
        assert!(restore_from_trash("foo/bar").is_err());
        assert!(restore_from_trash("foo\\bar").is_err());
        assert!(restore_from_trash("..").is_err());
    }

    #[test]
    fn test_file_fingerprint_nonexistent() {
        let result = file_fingerprint(Path::new("/nonexistent/path/12345"), 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_file_fingerprint_regular_file() {
        let tmp = std::env::temp_dir().join("luzumi_test_fp");
        fs::write(&tmp, "test content").unwrap();
        let size = fs::metadata(&tmp).unwrap().len();
        let result = file_fingerprint(&tmp, size);
        assert!(result.is_ok());
        let fp = result.unwrap();
        assert!(!fp.is_empty());
        let _ = fs::remove_file(&tmp);
    }
}
