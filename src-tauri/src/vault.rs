use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::Argon2;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::{Archive, Builder};
use zeroize::Zeroize;

const VAULT_MAGIC: &[u8; 8] = b"LUZVAULT";
const VAULT_VERSION: u8 = 1;
const SALT_LEN: usize = 32;
const NONCE_LEN: usize = 12;
const VAULT_EXT: &str = "luzumi-vault";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultInfo {
    pub path: String,
    pub name: String,
    pub has_dummy: bool,
    pub created: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultUnlockResult {
    pub mount_path: String,
    pub is_dummy: bool,
}

/// Derive a 256-bit key from password + salt using Argon2id.
fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32], String> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| format!("Key derivation failed: {}", e))?;
    Ok(key)
}

/// Encrypt data with AES-256-GCM. Returns salt + nonce + ciphertext.
fn encrypt_data(data: &[u8], password: &str) -> Result<Vec<u8>, String> {
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut salt);

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let mut key = derive_key(password, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| format!("Cipher init failed: {}", e))?;
    key.zeroize();

    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|_| "Encryption failed".to_string())?;

    let mut result = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// Decrypt data. Input = salt + nonce + ciphertext.
fn decrypt_data(blob: &[u8], password: &str) -> Result<Vec<u8>, String> {
    if blob.len() < SALT_LEN + NONCE_LEN + 16 {
        return Err("Vault data too short".into());
    }
    let salt = &blob[..SALT_LEN];
    let nonce_bytes = &blob[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &blob[SALT_LEN + NONCE_LEN..];

    let mut key = derive_key(password, salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| format!("Cipher init failed: {}", e))?;
    key.zeroize();

    let nonce = Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Wrong password".to_string())
}

/// Create a tar archive of a directory in memory.
fn tar_directory(dir: &Path) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    {
        let mut builder = Builder::new(&mut buf);
        add_dir_to_tar(&mut builder, dir, Path::new(""))?;
        builder.finish().map_err(|e| format!("Tar finish failed: {}", e))?;
    }
    Ok(buf)
}

fn add_dir_to_tar(builder: &mut Builder<&mut Vec<u8>>, base: &Path, prefix: &Path) -> Result<(), String> {
    let entries = fs::read_dir(base).map_err(|e| format!("Read dir failed: {}", e))?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = prefix.join(entry.file_name());

        if path.is_symlink() {
            let target = fs::read_link(&path).map_err(|e| e.to_string())?;
            let mut header = tar::Header::new_gnu();
            header.set_entry_type(tar::EntryType::Symlink);
            header.set_size(0);
            let meta = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
            header.set_mode(meta_mode(&meta));
            header.set_mtime(meta.modified().ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0));
            builder.append_link(&mut header, &name, &target)
                .map_err(|e| format!("Tar symlink failed: {}", e))?;
        } else if path.is_dir() {
            let meta = fs::metadata(&path).map_err(|e| e.to_string())?;
            let mut header = tar::Header::new_gnu();
            header.set_entry_type(tar::EntryType::Directory);
            header.set_size(0);
            header.set_mode(meta_mode_std(&meta));
            header.set_mtime(meta.modified().ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0));
            let dir_name = format!("{}/", name.to_string_lossy());
            builder.append_data(&mut header, &dir_name, &[][..])
                .map_err(|e| format!("Tar dir failed: {}", e))?;
            add_dir_to_tar(builder, &path, &name)?;
        } else if path.is_file() {
            builder.append_path_with_name(&path, &name)
                .map_err(|e| format!("Tar file failed: {}", e))?;
        }
    }
    Ok(())
}

#[cfg(unix)]
fn meta_mode(meta: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::MetadataExt;
    meta.mode()
}

#[cfg(not(unix))]
fn meta_mode(_meta: &std::fs::Metadata) -> u32 {
    0o755
}

#[cfg(unix)]
fn meta_mode_std(meta: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::MetadataExt;
    meta.mode()
}

#[cfg(not(unix))]
fn meta_mode_std(_meta: &std::fs::Metadata) -> u32 {
    0o755
}

/// Extract a tar archive to a directory.
fn untar_to_directory(tar_data: &[u8], dest: &Path) -> Result<(), String> {
    fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    let mut archive = Archive::new(tar_data);
    archive.set_preserve_permissions(true);
    archive.unpack(dest).map_err(|e| format!("Extract failed: {}", e))?;
    Ok(())
}

/// Vault file format:
///   8 bytes: magic "LUZVAULT"
///   1 byte:  version
///   8 bytes: created timestamp (u64 LE)
///   4 bytes: main payload length (u32 LE)
///   N bytes: main encrypted payload (salt + nonce + ciphertext)
///   1 byte:  has_dummy flag (0 or 1)
///   if has_dummy:
///     4 bytes: dummy payload length (u32 LE)
///     M bytes: dummy encrypted payload
fn write_vault_file(
    path: &Path,
    main_payload: &[u8],
    dummy_payload: Option<&[u8]>,
    created: u64,
) -> Result<(), String> {
    let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
    file.write_all(VAULT_MAGIC).map_err(|e| e.to_string())?;
    file.write_all(&[VAULT_VERSION]).map_err(|e| e.to_string())?;
    file.write_all(&created.to_le_bytes()).map_err(|e| e.to_string())?;

    let main_len = main_payload.len() as u32;
    file.write_all(&main_len.to_le_bytes()).map_err(|e| e.to_string())?;
    file.write_all(main_payload).map_err(|e| e.to_string())?;

    if let Some(dummy) = dummy_payload {
        file.write_all(&[1u8]).map_err(|e| e.to_string())?;
        let dummy_len = dummy.len() as u32;
        file.write_all(&dummy_len.to_le_bytes()).map_err(|e| e.to_string())?;
        file.write_all(dummy).map_err(|e| e.to_string())?;
    } else {
        file.write_all(&[0u8]).map_err(|e| e.to_string())?;
    }

    Ok(())
}

struct VaultFile {
    created: u64,
    main_payload: Vec<u8>,
    dummy_payload: Option<Vec<u8>>,
}

fn read_vault_file(path: &Path) -> Result<VaultFile, String> {
    let data = fs::read(path).map_err(|e| e.to_string())?;
    if data.len() < 8 + 1 + 8 + 4 {
        return Err("Invalid vault file".into());
    }
    if &data[..8] != VAULT_MAGIC {
        return Err("Not a vault file".into());
    }
    if data[8] != VAULT_VERSION {
        return Err(format!("Unsupported vault version: {}", data[8]));
    }

    let mut pos = 9;
    if pos + 8 > data.len() {
        return Err("Vault file truncated (timestamp)".into());
    }
    let created = u64::from_le_bytes(
        data[pos..pos + 8].try_into().map_err(|_| "Malformed vault data (timestamp)")?,
    );
    pos += 8;

    if pos + 4 > data.len() {
        return Err("Vault file truncated (main length)".into());
    }
    let main_len = u32::from_le_bytes(
        data[pos..pos + 4].try_into().map_err(|_| "Malformed vault data (main length)")?,
    ) as usize;
    pos += 4;
    if pos + main_len > data.len() {
        return Err("Vault file truncated (main payload)".into());
    }
    let main_payload = data[pos..pos + main_len].to_vec();
    pos += main_len;

    if pos >= data.len() {
        return Err("Vault file truncated (dummy flag)".into());
    }
    let has_dummy = data[pos] == 1;
    pos += 1;

    let dummy_payload = if has_dummy {
        if pos + 4 > data.len() {
            return Err("Vault file truncated (dummy length)".into());
        }
        let dummy_len = u32::from_le_bytes(
            data[pos..pos + 4].try_into().map_err(|_| "Malformed vault data (dummy length)")?,
        ) as usize;
        pos += 4;
        if pos + dummy_len > data.len() {
            return Err("Vault file truncated (dummy payload)".into());
        }
        Some(data[pos..pos + dummy_len].to_vec())
    } else {
        None
    };

    Ok(VaultFile {
        created,
        main_payload,
        dummy_payload,
    })
}

fn vault_mount_dir() -> PathBuf {
    let base = dirs::runtime_dir()
        .or_else(dirs::cache_dir)
        .unwrap_or_else(|| {
            let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
            home.join(".cache")
        });
    let mount_dir = base.join("luzumi").join("vaults");
    if let Err(e) = fs::create_dir_all(&mount_dir) {
        eprintln!("Failed to create vault mount dir: {}", e);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&mount_dir, std::fs::Permissions::from_mode(0o700));
    }
    mount_dir
}

fn validate_vault_password(password: &str) -> Result<(), String> {
    let mut issues = Vec::new();
    if password.len() < 8 {
        issues.push("at least 8 characters");
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        issues.push("at least one uppercase letter");
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        issues.push("at least one lowercase letter");
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        issues.push("at least one digit");
    }
    if !issues.is_empty() {
        return Err(format!("Password must have: {}", issues.join(", ")));
    }
    Ok(())
}

/// Create a vault from a directory.
/// Encrypts the folder contents, writes a .luzumi-vault file, removes the original directory.
pub fn create_vault(
    dir_path: &str,
    password: &str,
    dummy_password: Option<&str>,
) -> Result<String, String> {
    validate_vault_password(password)?;
    if let Some(dp) = dummy_password {
        validate_vault_password(dp)?;
        if dp == password {
            return Err("Dummy password must be different from main password".into());
        }
    }

    let source = Path::new(dir_path);
    if !source.is_dir() {
        return Err("Source path is not a directory".into());
    }

    // Archive the directory
    let tar_data = tar_directory(source)?;

    // Encrypt with main password
    let main_encrypted = encrypt_data(&tar_data, password)?;

    // Create dummy payload if requested (empty folder by default)
    let dummy_encrypted = if let Some(dp) = dummy_password {
        // Create a minimal tar with just an empty directory
        let mut dummy_tar = Vec::new();
        {
            let mut builder = Builder::new(&mut dummy_tar);
            builder.finish().map_err(|e| e.to_string())?;
        }
        Some(encrypt_data(&dummy_tar, dp)?)
    } else {
        None
    };

    let created = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let vault_path = PathBuf::from(format!("{}.{}", dir_path, VAULT_EXT));
    write_vault_file(&vault_path, &main_encrypted, dummy_encrypted.as_deref(), created)?;

    // Remove the original directory
    fs::remove_dir_all(source).map_err(|e| format!("Failed to remove original directory: {}", e))?;

    Ok(vault_path.to_string_lossy().to_string())
}

/// Unlock a vault — decrypt and extract to a temporary mount point.
/// Returns the mount path and whether dummy content was accessed.
pub fn unlock_vault(vault_path: &str, password: &str) -> Result<VaultUnlockResult, String> {
    let path = Path::new(vault_path);
    if !path.exists() {
        return Err("Vault file not found".into());
    }

    let vault = read_vault_file(path)?;

    // Try main password first
    if let Ok(tar_data) = decrypt_data(&vault.main_payload, password) {
        let mount_dir = vault_mount_dir();
        let vault_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            // strip the extra .luzumi-vault from "folder.luzumi-vault"
            .map(|s| s.trim_end_matches(&format!(".{}", VAULT_EXT)))
            .unwrap_or("vault");
        let mount_path = mount_dir.join(vault_name);
        if mount_path.exists() {
            fs::remove_dir_all(&mount_path).ok();
        }
        untar_to_directory(&tar_data, &mount_path)?;
        return Ok(VaultUnlockResult {
            mount_path: mount_path.to_string_lossy().to_string(),
            is_dummy: false,
        });
    }

    // Try dummy password
    if let Some(dummy_payload) = &vault.dummy_payload {
        if let Ok(tar_data) = decrypt_data(dummy_payload, password) {
            let mount_dir = vault_mount_dir();
            let vault_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("vault");
            let mount_path = mount_dir.join(vault_name);
            if mount_path.exists() {
                fs::remove_dir_all(&mount_path).ok();
            }
            untar_to_directory(&tar_data, &mount_path)?;
            return Ok(VaultUnlockResult {
                mount_path: mount_path.to_string_lossy().to_string(),
                is_dummy: true,
            });
        }
    }

    Err("Wrong password".into())
}

/// Lock a vault — re-encrypt contents from mount point back into vault file.
pub fn lock_vault(vault_path: &str, mount_path: &str, password: &str) -> Result<(), String> {
    let vpath = Path::new(vault_path);
    let mpath = Path::new(mount_path);

    if !mpath.exists() {
        return Err("Mount path does not exist".into());
    }

    // Read existing vault to preserve dummy payload and metadata
    let old_vault = read_vault_file(vpath)?;

    // Re-archive the (potentially modified) contents
    let tar_data = tar_directory(mpath)?;
    let main_encrypted = encrypt_data(&tar_data, password)?;

    write_vault_file(
        vpath,
        &main_encrypted,
        old_vault.dummy_payload.as_deref(),
        old_vault.created,
    )?;

    // Clean up mount directory
    fs::remove_dir_all(mpath).ok();

    Ok(())
}

/// Update dummy vault content. Must provide main password for verification + dummy password.
pub fn set_dummy_content(
    vault_path: &str,
    main_password: &str,
    dummy_password: &str,
    dummy_dir_path: &str,
) -> Result<(), String> {
    validate_vault_password(dummy_password)?;
    if main_password == dummy_password {
        return Err("Dummy password must be different from main password".into());
    }

    let vpath = Path::new(vault_path);
    let vault = read_vault_file(vpath)?;

    // Verify main password
    decrypt_data(&vault.main_payload, main_password)
        .map_err(|_| "Main password is incorrect".to_string())?;

    // Archive dummy content
    let dummy_dir = Path::new(dummy_dir_path);
    if !dummy_dir.is_dir() {
        return Err("Dummy content path is not a directory".into());
    }
    let tar_data = tar_directory(dummy_dir)?;
    let dummy_encrypted = encrypt_data(&tar_data, dummy_password)?;

    write_vault_file(vpath, &vault.main_payload, Some(&dummy_encrypted), vault.created)?;
    Ok(())
}

/// Change the main password of a vault.
pub fn change_vault_password(
    vault_path: &str,
    old_password: &str,
    new_password: &str,
) -> Result<(), String> {
    validate_vault_password(new_password)?;

    let vpath = Path::new(vault_path);
    let vault = read_vault_file(vpath)?;

    // Decrypt with old password
    let tar_data = decrypt_data(&vault.main_payload, old_password)?;

    // Re-encrypt with new password
    let new_encrypted = encrypt_data(&tar_data, new_password)?;

    write_vault_file(vpath, &new_encrypted, vault.dummy_payload.as_deref(), vault.created)?;
    Ok(())
}

/// Check if a file is a vault.
pub fn is_vault(path: &str) -> bool {
    let p = Path::new(path);
    p.is_file()
        && p.extension()
            .map(|e| e == VAULT_EXT)
            .unwrap_or(false)
}

/// Get vault info without decrypting.
pub fn get_vault_info(path: &str) -> Result<VaultInfo, String> {
    let p = Path::new(path);
    let vault = read_vault_file(p)?;
    let name = p
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("vault")
        .to_string();

    Ok(VaultInfo {
        path: path.to_string(),
        name,
        has_dummy: vault.dummy_payload.is_some(),
        created: vault.created as i64,
    })
}

/// List all unlocked vault mount points.
pub fn list_unlocked_vaults() -> Vec<String> {
    let mount_dir = vault_mount_dir();
    if !mount_dir.exists() {
        return Vec::new();
    }
    fs::read_dir(&mount_dir)
        .ok()
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .map(|e| e.path().to_string_lossy().to_string())
                .collect()
        })
        .unwrap_or_default()
}
