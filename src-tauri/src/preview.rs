use std::fs;
use std::path::Path;
use std::process::Command;
use serde::Serialize;
use base64::Engine;

#[derive(Debug, Serialize)]
pub struct HexData {
    pub offset: usize,
    pub bytes: Vec<u8>,
    pub total_size: u64,
}

/// Read a PDF page as a base64-encoded PNG using pdftoppm (poppler-utils)
pub fn read_pdf_page(path: &str, page: u32) -> Result<String, String> {
    let output = Command::new("pdftoppm")
        .args([
            "-png",
            "-f", &page.to_string(),
            "-l", &page.to_string(),
            "-r", "150",
            "-singlefile",
            path,
        ])
        .output()
        .map_err(|e| format!("pdftoppm not found. Install poppler-utils: {}", e))?;

    if !output.status.success() {
        return Err(format!("pdftoppm failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let b64 = base64::engine::general_purpose::STANDARD.encode(&output.stdout);
    Ok(format!("data:image/png;base64,{}", b64))
}

/// Get total page count of a PDF using pdfinfo
pub fn get_pdf_page_count(path: &str) -> Result<u32, String> {
    let output = Command::new("pdfinfo")
        .arg(path)
        .output()
        .map_err(|e| format!("pdfinfo not found: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.starts_with("Pages:") {
            let count = line.trim_start_matches("Pages:").trim();
            return count.parse::<u32>().map_err(|e| e.to_string());
        }
    }
    Err("Could not determine page count".into())
}

/// Render markdown to HTML
pub fn render_markdown(content: &str) -> String {
    use pulldown_cmark::{Parser, Options, html};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Read raw bytes for hex viewer
pub fn read_hex(path: &str, offset: usize, length: usize) -> Result<HexData, String> {
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    let total_size = metadata.len();

    let data = fs::read(path).map_err(|e| e.to_string())?;
    let end = std::cmp::min(offset + length, data.len());
    let bytes = if offset < data.len() {
        data[offset..end].to_vec()
    } else {
        Vec::new()
    };

    Ok(HexData { offset, bytes, total_size })
}

/// Read file content with syntax detection
pub fn read_file_preview(path: &str, max_lines: usize) -> Result<(String, String), String> {
    let p = Path::new(path);
    let ext = p.extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    let data = fs::read(path).map_err(|e| e.to_string())?;

    // Check if binary
    if data.iter().take(512).any(|&b| b == 0) {
        return Err("Binary file".into());
    }

    let content = String::from_utf8(data).map_err(|_| "Not valid UTF-8".to_string())?;
    let lines: Vec<&str> = content.lines().take(max_lines).collect();
    let truncated = lines.join("\n");

    // Map extension to highlight.js language name
    let language = match ext.as_str() {
        "rs" => "rust",
        "py" | "pyw" => "python",
        "js" | "mjs" | "cjs" => "javascript",
        "ts" | "mts" | "cts" => "typescript",
        "jsx" => "javascript",
        "tsx" => "typescript",
        "svelte" => "xml",
        "vue" => "xml",
        "html" | "htm" => "html",
        "css" | "scss" | "sass" | "less" => "css",
        "json" | "jsonc" | "json5" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "xml" | "xsl" | "xsd" => "xml",
        "md" | "markdown" => "markdown",
        "sh" | "bash" | "zsh" => "bash",
        "c" | "h" => "c",
        "cpp" | "hpp" | "cc" | "cxx" => "cpp",
        "java" => "java",
        "go" => "go",
        "rb" => "ruby",
        "php" => "php",
        "swift" => "swift",
        "kt" | "kts" => "kotlin",
        "cs" => "csharp",
        "sql" => "sql",
        "r" => "r",
        "lua" => "lua",
        "perl" | "pl" | "pm" => "perl",
        "scala" | "sc" => "scala",
        "ex" | "exs" => "elixir",
        "hs" => "haskell",
        "clj" | "cljs" => "clojure",
        "dart" => "dart",
        "dockerfile" => "dockerfile",
        "makefile" => "makefile",
        "cmake" => "cmake",
        "graphql" | "gql" => "graphql",
        "proto" => "protobuf",
        "tf" | "hcl" => "hcl",
        "nix" => "nix",
        "zig" => "zig",
        "nim" => "nim",
        "txt" | "text" | "log" => "plaintext",
        "ini" | "cfg" | "conf" => "ini",
        "diff" | "patch" => "diff",
        _ => "plaintext",
    };

    Ok((truncated, language.to_string()))
}
