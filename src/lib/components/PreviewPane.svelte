<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fm } from "../fileManager.svelte";
  import { t } from "../i18n";
  import { formatSize } from "../types";

  const IMAGE_EXTS = new Set(["jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "ico", "avif"]);
  const TEXT_EXTS = new Set([
    "txt", "md", "json", "js", "ts", "tsx", "jsx", "svelte", "html", "css", "scss",
    "rs", "toml", "yaml", "yml", "xml", "csv", "log", "sh", "bash", "zsh",
    "py", "rb", "go", "java", "c", "cpp", "h", "hpp", "sql", "env", "gitignore",
    "conf", "cfg", "ini", "lock", "dockerfile",
  ]);

  const MAX_PREVIEW_LINES = 100;

  let thumbnail = $state<string | null>(null);
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
  const isText = $derived(TEXT_EXTS.has(ext));

  $effect(() => {
    const entry = selectedEntry;
    if (!entry || entry.path === lastPath) return;
    lastPath = entry.path;
    thumbnail = null;
    textContent = null;

    if (entry.kind === "file" && isImage) {
      loadingPreview = true;
      invoke<string>("cmd_read_thumbnail", { path: entry.path })
        .then(d => { thumbnail = d; })
        .catch(() => {})
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

    <div class="preview-body">
      {#if loadingPreview}
        <div class="loading">
          <svg class="spin" width="20" height="20" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5" opacity="0.2"/>
            <path d="M14 8A6 6 0 0 0 8 2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </div>
      {:else if thumbnail}
        <div class="image-preview">
          <img src="data:image/{ext};base64,{thumbnail}" alt={selectedEntry.name} />
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
