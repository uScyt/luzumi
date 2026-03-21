use rusqlite::{Connection, params};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct FileTag {
    pub path: String,
    pub tag: Tag,
}

fn get_db_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("luzumi")
        .join("tags.db")
}

pub fn init_db() -> Result<(), String> {
    open_db().map(|_| ())
}

fn open_db() -> Result<Connection, String> {
    let db_path = get_db_path();
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            color TEXT NOT NULL DEFAULT '#888888'
        );
        CREATE TABLE IF NOT EXISTS file_tags (
            path TEXT NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (path, tag_id),
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        );
        CREATE INDEX IF NOT EXISTS idx_file_tags_path ON file_tags(path);
        CREATE INDEX IF NOT EXISTS idx_file_tags_tag ON file_tags(tag_id);"
    ).map_err(|e| e.to_string())?;
    Ok(conn)
}

pub fn create_tag(name: &str, color: &str) -> Result<Tag, String> {
    let conn = open_db()?;
    conn.execute(
        "INSERT INTO tags (name, color) VALUES (?1, ?2)",
        params![name, color],
    ).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(Tag { id, name: name.to_string(), color: color.to_string() })
}

pub fn delete_tag(id: i64) -> Result<(), String> {
    let conn = open_db()?;
    conn.execute("DELETE FROM file_tags WHERE tag_id = ?1", params![id]).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tags WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn rename_tag(id: i64, name: &str) -> Result<(), String> {
    let conn = open_db()?;
    conn.execute("UPDATE tags SET name = ?1 WHERE id = ?2", params![name, id]).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn recolor_tag(id: i64, color: &str) -> Result<(), String> {
    let conn = open_db()?;
    conn.execute("UPDATE tags SET color = ?1 WHERE id = ?2", params![color, id]).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn list_tags() -> Result<Vec<Tag>, String> {
    let conn = open_db()?;
    let mut stmt = conn.prepare("SELECT id, name, color FROM tags ORDER BY name")
        .map_err(|e| e.to_string())?;
    let tags = stmt.query_map([], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    Ok(tags)
}

pub fn tag_file(path: &str, tag_id: i64) -> Result<(), String> {
    let conn = open_db()?;
    conn.execute(
        "INSERT OR IGNORE INTO file_tags (path, tag_id) VALUES (?1, ?2)",
        params![path, tag_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn untag_file(path: &str, tag_id: i64) -> Result<(), String> {
    let conn = open_db()?;
    conn.execute(
        "DELETE FROM file_tags WHERE path = ?1 AND tag_id = ?2",
        params![path, tag_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_file_tags(path: &str) -> Result<Vec<Tag>, String> {
    let conn = open_db()?;
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, t.color FROM tags t
         JOIN file_tags ft ON t.id = ft.tag_id
         WHERE ft.path = ?1 ORDER BY t.name"
    ).map_err(|e| e.to_string())?;
    let tags = stmt.query_map(params![path], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    Ok(tags)
}

pub fn get_files_by_tag(tag_id: i64) -> Result<Vec<String>, String> {
    let conn = open_db()?;
    let mut stmt = conn.prepare(
        "SELECT path FROM file_tags WHERE tag_id = ?1 ORDER BY path"
    ).map_err(|e| e.to_string())?;
    let paths = stmt.query_map(params![tag_id], |row| {
        row.get::<_, String>(0)
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    Ok(paths)
}

pub fn get_all_file_tags(paths: &[String]) -> Result<Vec<FileTag>, String> {
    if paths.is_empty() { return Ok(Vec::new()); }
    let conn = open_db()?;
    let placeholders: String = paths.iter().enumerate().map(|(i, _)| format!("?{}", i + 1)).collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT ft.path, t.id, t.name, t.color FROM file_tags ft
         JOIN tags t ON t.id = ft.tag_id
         WHERE ft.path IN ({}) ORDER BY ft.path, t.name", placeholders
    );
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let params: Vec<&dyn rusqlite::types::ToSql> = paths.iter().map(|p| p as &dyn rusqlite::types::ToSql).collect();
    let file_tags = stmt.query_map(params.as_slice(), |row| {
        Ok(FileTag {
            path: row.get(0)?,
            tag: Tag {
                id: row.get(1)?,
                name: row.get(2)?,
                color: row.get(3)?,
            },
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    Ok(file_tags)
}
