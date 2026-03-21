<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { getFileIcon, getFileColor, formatSize } from "../types";
  import type { FileEntry } from "../types";
  import { renderIcon } from "../icons";
  import { t } from "../i18n";

  function parentDir(path: string): string {
    const idx = path.lastIndexOf("/");
    if (idx <= 0) return "/";
    return path.substring(0, idx);
  }

  function onDblClick(entry: FileEntry) {
    if (entry.kind === "directory") {
      fm.navigateSplit(entry.path);
    } else {
      fm.open(entry);
    }
  }

  function goUp() {
    if (fm.splitPath !== "/") {
      fm.navigateSplit(parentDir(fm.splitPath));
    }
  }

  function onDragStart(e: DragEvent, entry: FileEntry) {
    if (!e.dataTransfer) return;
    e.dataTransfer.effectAllowed = "move";
    e.dataTransfer.setData("text/plain", entry.path);
    fm.setDragPaths(entry);
  }

  function onDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) e.dataTransfer.dropEffect = "move";
  }

  function onDrop(e: DragEvent) {
    e.preventDefault();
    fm.requestDrop(fm.splitPath);
  }

  function formatDate(ts: number | null): string {
    if (!ts) return "";
    return new Date(ts * 1000).toLocaleDateString(undefined, {
      month: "short", day: "numeric", hour: "2-digit", minute: "2-digit",
    });
  }

  const displayed = $derived(
    fm.splitEntries
      .filter(e => fm.showHidden || !e.name.startsWith("."))
      .sort((a, b) => {
        if (a.kind !== b.kind) return a.kind === "directory" ? -1 : 1;
        return a.name.localeCompare(b.name);
      })
  );
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  class="split-pane"
  class:focused={fm.splitFocused}
  role="complementary"
  aria-label="Split view"
  tabindex="-1"
  onclick={() => { fm.splitFocused = true; }}
  onkeydown={() => {}}
>
  <div class="split-header">
    <button class="split-up-btn" onclick={goUp} disabled={fm.splitPath === "/"} title="Up">
      <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
        <path d="M8 12V4M4 8L8 4L12 8" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
    <span class="split-path" title={fm.splitPath}>{fm.splitPath}</span>
    <button class="split-close-btn" onclick={() => { fm.splitView = false; }} title={t.close}>
      <svg width="10" height="10" viewBox="0 0 16 16" fill="none">
        <path d="M3 3L13 13M13 3L3 13" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
      </svg>
    </button>
  </div>

  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="split-list" role="list" ondragover={onDragOver} ondrop={onDrop}>
    {#if displayed.length === 0}
      <div class="split-empty">{t.emptyDirectory}</div>
    {:else}
      {#each displayed as entry (entry.path)}
        {@const iconName = getFileIcon(entry)}
        {@const iconColor = getFileColor(entry)}
        <button
          class="split-row"
          ondblclick={() => onDblClick(entry)}
          draggable="true"
          ondragstart={(e) => onDragStart(e, entry)}
          title={entry.path}
        >
          <span class="split-icon" style="color: {iconColor}">
            <svg width="16" height="16" viewBox="0 0 16 16">{@html renderIcon(iconName)}</svg>
          </span>
          <span class="split-name">{entry.name}</span>
          <span class="split-size">{entry.kind === "file" ? formatSize(entry.size ?? 0) : ""}</span>
          <span class="split-date">{formatDate(entry.modified)}</span>
        </button>
      {/each}
    {/if}
  </div>
</div>

<style>
  .split-pane {
    display: flex;
    flex-direction: column;
    overflow: hidden;
    border-left: 1px solid var(--border-medium);
    background: var(--content-bg);
  }

  .split-pane.focused {
    border-left-color: var(--mauve);
  }

  .split-header {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    background: var(--panel-bg);
    backdrop-filter: blur(12px);
    border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }

  .split-up-btn {
    width: 24px;
    height: 24px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    transition: background 0.1s, color 0.1s;
    flex-shrink: 0;
  }

  .split-up-btn:hover:not(:disabled) {
    background: var(--hover-bg);
    color: var(--text);
  }

  .split-up-btn:disabled {
    opacity: 0.3;
  }

  .split-path {
    flex: 1;
    font-size: 11px;
    color: var(--subtext0);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    direction: rtl;
    text-align: left;
  }

  .split-close-btn {
    width: 22px;
    height: 22px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    transition: background 0.1s, color 0.1s;
    flex-shrink: 0;
  }

  .split-close-btn:hover {
    background: var(--danger-bg);
    color: var(--red);
  }

  .split-list {
    flex: 1;
    overflow-y: auto;
    padding: 4px;
  }

  .split-empty {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 120px;
    color: var(--overlay1);
    font-size: 13px;
  }

  .split-row {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 4px 8px;
    border-radius: 6px;
    font-size: 12px;
    color: var(--text);
    text-align: left;
    cursor: default;
    transition: background 0.08s;
  }

  .split-row:hover {
    background: var(--hover-bg);
  }

  .split-icon {
    flex-shrink: 0;
    display: flex;
    align-items: center;
  }

  .split-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .split-size {
    flex-shrink: 0;
    font-size: 10px;
    color: var(--overlay1);
    min-width: 50px;
    text-align: right;
  }

  .split-date {
    flex-shrink: 0;
    font-size: 10px;
    color: var(--overlay0);
    min-width: 90px;
    text-align: right;
  }
</style>
