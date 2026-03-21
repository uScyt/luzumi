<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fm } from "../fileManager.svelte";
  import { t } from "../i18n";
  import { formatSize, getFileIcon, getFileColor } from "../types";
  import { renderIcon } from "../icons";
  import CodePreview from "./preview/CodePreview.svelte";
  import MarkdownPreview from "./preview/MarkdownPreview.svelte";
  import PdfPreview from "./preview/PdfPreview.svelte";
  import HexViewer from "./preview/HexViewer.svelte";

  const IMAGE_EXTS = new Set(["jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "ico", "avif", "tiff", "tif", "heic", "heif", "jxl"]);
  const VIDEO_EXTS = new Set(["mp4", "m4v", "mkv", "avi", "mov", "webm", "flv", "wmv", "mpg", "mpeg", "ogv", "ogg", "3gp", "ts", "mts", "m2ts", "vob"]);
  const CODE_EXTS = new Set([
    "js", "ts", "tsx", "jsx", "svelte", "html", "css", "scss", "less",
    "rs", "toml", "yaml", "yml", "xml", "json",
    "py", "rb", "go", "java", "c", "cpp", "h", "hpp", "sql",
    "sh", "bash", "zsh", "dockerfile",
  ]);
  const TEXT_EXTS = new Set([
    "txt", "md", "json", "js", "ts", "tsx", "jsx", "svelte", "html", "css", "scss",
    "rs", "toml", "yaml", "yml", "xml", "csv", "log", "sh", "bash", "zsh",
    "py", "rb", "go", "java", "c", "cpp", "h", "hpp", "sql", "env", "gitignore",
    "conf", "cfg", "ini", "lock", "dockerfile",
  ]);
  const PDF_EXTS = new Set(["pdf"]);
  const MD_EXTS = new Set(["md", "markdown"]);

  const MAX_PREVIEW_LINES = 100;

  let thumbnail = $state<string | null>(null);
  let thumbFailed = $state(false);
  let textContent = $state<string | null>(null);
  let loadingPreview = $state(false);
  let lastPath = $state("");

  const selectedEntry = $derived(
    fm.selected.size === 1
      ? fm.entries.find(e => fm.selected.has(e.path)) ?? null
      : null
  );

  const ext = $derived(selectedEntry?.extension?.toLowerCase() ?? "");
  const isImage = $derived(IMAGE_EXTS.has(ext));
  const isVideo = $derived(VIDEO_EXTS.has(ext));
  const isText = $derived(TEXT_EXTS.has(ext));
  const isCode = $derived(CODE_EXTS.has(ext));
  const isPdf = $derived(PDF_EXTS.has(ext));
  const isMarkdown = $derived(MD_EXTS.has(ext));

  let previewMode = $state<"auto" | "hex">("auto");

  $effect(() => {
    const entry = selectedEntry;
    if (!entry || entry.path === lastPath) return;
    lastPath = entry.path;
    thumbnail = null;
    thumbFailed = false;
    textContent = null;

    if (entry.kind === "file" && (isImage || isVideo)) {
      loadingPreview = true;
      invoke<string>("cmd_read_thumbnail", { path: entry.path })
        .then(d => { thumbnail = d; })
        .catch(() => { thumbFailed = true; })
        .finally(() => { loadingPreview = false; });
    } else if (entry.kind === "file" && isText) {
      loadingPreview = true;
      invoke<string>("cmd_read_text_preview", { path: entry.path, maxLines: MAX_PREVIEW_LINES })
        .then(d => { textContent = d; })
        .catch(() => {})
        .finally(() => { loadingPreview = false; });
    }
  });
</script>

<aside class="preview-pane">
  {#if !selectedEntry}
    <div class="empty">
      <svg width="32" height="32" viewBox="0 0 16 16" fill="none" opacity="0.2">
        <path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1" fill="currentColor" opacity=".2"/>
      </svg>
      <span>{t.noPreview}</span>
    </div>
  {:else}
    <div class="preview-header">
      <span class="file-name" title={selectedEntry.name}>{selectedEntry.name}</span>
      <span class="file-meta">
        {#if selectedEntry.kind === "file"}{formatSize(selectedEntry.size ?? 0)}{:else}{t.folderSize}{/if}
      </span>
    </div>

    <div class="preview-mode-toggle">
      <button class:active={previewMode === "auto"} onclick={() => previewMode = "auto"}>Preview</button>
      {#if selectedEntry.kind === "file"}
        <button class:active={previewMode === "hex"} onclick={() => previewMode = "hex"}>Hex</button>
      {/if}
    </div>

    <div class="preview-body">
      {#if previewMode === "hex" && selectedEntry.kind === "file"}
        <HexViewer filePath={selectedEntry.path} />
      {:else if loadingPreview}
        <div class="loading">
          <svg class="spin" width="20" height="20" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5" opacity="0.2"/>
            <path d="M14 8A6 6 0 0 0 8 2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </div>
      {:else if isPdf}
        <PdfPreview filePath={selectedEntry.path} />
      {:else if isMarkdown}
        <MarkdownPreview filePath={selectedEntry.path} />
      {:else if isCode}
        <CodePreview filePath={selectedEntry.path} />
      {:else if thumbnail}
        <div class="image-preview">
          <img src={thumbnail} alt={selectedEntry.name} onerror={() => { thumbnail = null; thumbFailed = true; }} />
        </div>
      {:else if thumbFailed && selectedEntry}
        <div class="fallback-icon">
          <svg width="48" height="48" viewBox="0 0 16 16" style="color: {getFileColor(selectedEntry)}">
            {@html renderIcon(getFileIcon(selectedEntry))}
          </svg>
          <span class="fallback-label">{selectedEntry.extension?.toUpperCase()}</span>
        </div>
      {:else if textContent !== null}
        <pre class="text-preview">{textContent}</pre>
        {#if textContent.split('\n').length >= MAX_PREVIEW_LINES}
          <div class="truncated">{MAX_PREVIEW_LINES} {t.lines}...</div>
        {/if}
      {:else if selectedEntry.kind === "directory"}
        <div class="info-preview">
          <div class="info-row"><span class="info-label">{t.type}</span><span>{t.directory}</span></div>
          {#if selectedEntry.modified}
            <div class="info-row"><span class="info-label">{t.modified}</span><span>{new Date(selectedEntry.modified * 1000).toLocaleString()}</span></div>
          {/if}
        </div>
      {:else}
        <div class="empty">
          <span>{t.noPreview}</span>
        </div>
      {/if}
    </div>
  {/if}
</aside>

<style>
  .preview-pane {
    background: var(--content-bg);
    border-left: 1px solid var(--border-light);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .fallback-icon {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
  }

  .fallback-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--overlay1);
    letter-spacing: 0.04em;
  }

  .empty {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    color: var(--overlay0);
    font-size: 12px;
  }

  .preview-header {
    padding: 12px 14px;
    border-bottom: 1px solid var(--border-light);
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .file-name {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-meta {
    font-size: 11px;
    color: var(--overlay1);
  }

  .preview-mode-toggle {
    display: flex;
    gap: 2px;
    padding: 4px 10px;
    border-bottom: 1px solid var(--border-light);
  }

  .preview-mode-toggle button {
    background: none;
    border: none;
    color: var(--overlay1);
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 10px;
    cursor: pointer;
  }

  .preview-mode-toggle button.active {
    background: var(--accent-muted, rgba(137, 180, 250, 0.15));
    color: var(--accent, #89b4fa);
  }

  .preview-body {
    flex: 1;
    overflow: auto;
    display: flex;
    flex-direction: column;
  }

  .loading {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .spin {
    animation: spin 1s linear infinite;
  }

  @keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }

  .image-preview {
    padding: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
  }

  .image-preview img {
    max-width: 100%;
    max-height: 100%;
    border-radius: 6px;
    object-fit: contain;
  }

  .text-preview {
    margin: 0;
    padding: 12px;
    font-size: 11px;
    font-family: "JetBrains Mono", "Fira Code", monospace;
    color: var(--subtext1);
    line-height: 1.5;
    white-space: pre-wrap;
    word-break: break-all;
    tab-size: 2;
  }

  .truncated {
    padding: 4px 12px 12px;
    font-size: 10px;
    color: var(--overlay0);
    font-style: italic;
  }

  .info-preview {
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    font-size: 12px;
  }

  .info-label {
    color: var(--overlay1);
  }

  .info-row span:last-child {
    color: var(--text);
  }
</style>
