<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fm } from "../fileManager.svelte";
  import { formatSize, formatDate, getFileIcon, getFileColor } from "../types";
  import { t } from "../i18n";

  interface TrashItemInfo {
    originalPath: string | null;
    deletionDate: string | null;
  }

  let trashInfo = $state<Map<string, TrashItemInfo>>(new Map());

  $effect(() => {
    const entries = fm.trashEntries;
    if (entries.length === 0) { trashInfo = new Map(); return; }
    const newMap = new Map<string, TrashItemInfo>();
    for (const entry of entries) {
      invoke<TrashItemInfo>("cmd_get_trash_item_info", { fileName: entry.name })
        .then((info) => {
          newMap.set(entry.name, info);
          trashInfo = new Map(newMap);
        })
        .catch(() => {});
    }
  });

  const svgPaths: Record<string, string> = {
    Folder:        `<path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" fill="currentColor" opacity=".2" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round"/>`,
    File:          `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>`,
    FileCode:      `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><path d="M6 10L8.5 12.5L6 15M10 10L7.5 12.5L10 15" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" stroke-linejoin="round"/>`,
    FileText:      `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><path d="M6 10H10M6 12.5H9" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,
    FileImage:     `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><circle cx="6.5" cy="10" r="1" fill="currentColor"/><path d="M5 14L8 11L10 13L11.5 11.5L13 13" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" stroke-linejoin="round"/>`,
    FilmStrip:     `<rect x="3" y="3" width="10" height="11" rx="1" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M3 6H13M3 11H13" stroke="currentColor" stroke-width="1.1"/>`,
    MusicNote:     `<path d="M9 12V4L13 3" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/><circle cx="7" cy="12" r="2" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".15"/>`,
    FilePdf:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><text x="4.5" y="13.5" font-size="5" font-weight="700" fill="currentColor" font-family="monospace">PDF</text>`,
    FileArchive:   `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><path d="M7 8H9M7 10H9M7 12H9" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,
    Link:          `<path d="M6.5 10.5L9.5 7.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><path d="M9 11L9.5 11.5C10.88 12.88 13.12 12.88 14.5 11.5s1.38-3.62 0-5L13 5C11.62 3.62 9.38 3.62 8 5L7.5 5.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>`,
    Gear:          `<circle cx="8" cy="8" r="2.5" stroke="currentColor" stroke-width="1.2" fill="none"/><path d="M8 1v2M8 13v2M1 8h2M13 8h2" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>`,
    TerminalWindow:`<rect x="2" y="3" width="12" height="10" rx="1.5" stroke="currentColor" stroke-width="1.2" fill="none"/><path d="M5 7L7 9L5 11M8 11H11" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>`,
    Package:       `<path d="M8 2L14 5.5V10.5L8 14L2 10.5V5.5L8 2Z" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round" fill="currentColor" opacity=".1"/>`,
    Disc:          `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><circle cx="8" cy="8" r="2" stroke="currentColor" stroke-width="1.2" fill="none"/>`,
    Database:      `<ellipse cx="8" cy="4.5" rx="5" ry="2" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".15"/><path d="M3 4.5V8C3 9.1 5.24 10 8 10S13 9.1 13 8V4.5" stroke="currentColor" stroke-width="1.2" fill="none"/>`,
  };

  function renderIcon(iconName: string) {
    return svgPaths[iconName] ?? svgPaths["File"];
  }
</script>

<div class="trash-view">
  <div class="trash-header">
    <div class="trash-title">
      <div class="trash-icon-wrap">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M3 5H13M5.5 5V3.5C5.5 3.22 5.72 3 6 3H10C10.28 3 10.5 3.22 10.5 3.5V5M12 5V13C12 13.55 11.55 14 11 14H5C4.45 14 4 13.55 4 13V5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
          <path d="M6.5 7.5V11.5M9.5 7.5V11.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
        </svg>
      </div>
      <div class="trash-label">
        <span class="trash-name-text">{t.trash}</span>
        {#if fm.trashSize > 0}
          <span class="trash-sub">{formatSize(fm.trashSize)}</span>
        {/if}
      </div>
      <span class="trash-count">
        {t.elements(fm.trashEntries.length)}
      </span>
    </div>
    <div class="trash-actions">
      <button class="btn-empty" disabled={fm.trashEntries.length === 0} onclick={() => { fm.showEmptyTrashConfirm = true; }}>
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
          <path d="M3 5H13M5.5 5V3.5C5.5 3.22 5.72 3 6 3H10C10.28 3 10.5 3.22 10.5 3.5V5M12 5V13C12 13.55 11.55 14 11 14H5C4.45 14 4 13.55 4 13V5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        {t.emptyTrashBtn}
      </button>
    </div>
  </div>

  <div class="trash-body">
    {#if fm.trashEntries.length === 0}
      <div class="empty-state">
        <svg width="48" height="48" viewBox="0 0 16 16" fill="none" style="color: var(--overlay0); opacity: 0.4">
          <path d="M3 5H13M5.5 5V3.5C5.5 3.22 5.72 3 6 3H10C10.28 3 10.5 3.22 10.5 3.5V5M12 5V13C12 13.55 11.55 14 11 14H5C4.45 14 4 13.55 4 13V5" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
          <path d="M6.5 7.5V11.5M9.5 7.5V11.5" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>
        </svg>
        <span>{t.trashEmpty}</span>
      </div>
    {:else}
      {#each fm.trashEntries as entry (entry.path)}
        {@const iconName = getFileIcon(entry)}
        {@const iconColor = getFileColor(entry)}
        {@const info = trashInfo.get(entry.name)}
        <div class="trash-row">
          <div class="trash-icon">
            <svg width="18" height="18" viewBox="0 0 16 16" style="color: {iconColor}">
              {@html renderIcon(iconName)}
            </svg>
          </div>
          <div class="trash-info">
            <span class="trash-name" title={info?.originalPath ?? entry.name}>{entry.name}</span>
            <span class="trash-meta">
              {entry.size != null ? formatSize(entry.size) : ""}
              {#if info?.deletionDate}
                <span class="trash-date">{info.deletionDate.replace("T", " ").slice(0, 16)}</span>
              {:else if entry.modified}
                {formatDate(entry.modified)}
              {/if}
            </span>
          </div>
          <button class="restore-btn" title={t.restore} onclick={() => fm.restoreFromTrash(entry.name)}>
            <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
              <path d="M2.5 6.5h4v-4" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M2.5 6.5A6 6 0 1 1 6.5 13.5" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/>
            </svg>
          </button>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .trash-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--surface0);
  }

  .trash-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 16px;
    border-bottom: 1px solid rgba(237, 135, 150, 0.12);
    background: linear-gradient(
      to right,
      rgba(237, 135, 150, 0.07) 0%,
      rgba(24, 25, 38, 0.5) 70%
    );
    backdrop-filter: blur(12px);
    flex-shrink: 0;
    gap: 12px;
  }

  .trash-title {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }

  .trash-icon-wrap {
    width: 30px;
    height: 30px;
    border-radius: 9px;
    background: rgba(237, 135, 150, 0.12);
    border: 1px solid rgba(237, 135, 150, 0.18);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--maroon);
    flex-shrink: 0;
  }

  .trash-label {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .trash-name-text {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    line-height: 1.2;
  }

  .trash-sub {
    font-size: 10.5px;
    color: var(--overlay1);
    line-height: 1;
  }

  .trash-count {
    font-size: 11px;
    font-weight: 500;
    color: var(--maroon);
    background: rgba(237, 135, 150, 0.1);
    border: 1px solid rgba(237, 135, 150, 0.14);
    padding: 2px 9px;
    border-radius: 10px;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .trash-actions {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .btn-empty {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 14px;
    border-radius: 9px;
    font-size: 12px;
    font-weight: 600;
    color: var(--red);
    background: rgba(237, 135, 150, 0.1);
    border: 1px solid rgba(237, 135, 150, 0.18);
    transition: background 0.12s, border-color 0.12s, box-shadow 0.12s;
    white-space: nowrap;
  }

  .btn-empty:hover:not(:disabled) {
    background: rgba(237, 135, 150, 0.18);
    border-color: rgba(237, 135, 150, 0.3);
    box-shadow: 0 0 0 3px rgba(237, 135, 150, 0.06);
  }

  .btn-empty:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .trash-body {
    flex: 1;
    overflow-y: auto;
    padding: 6px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    height: 260px;
    color: var(--overlay1);
    font-size: 13px;
  }

  .trash-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 7px 10px;
    border-radius: 8px;
    transition: background 0.08s;
  }

  .trash-row:hover {
    background: var(--hover-bg-subtle);
  }

  .trash-icon {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    opacity: 0.7;
  }

  .trash-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .trash-name {
    font-size: 13px;
    color: var(--subtext1);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .trash-meta {
    font-size: 10.5px;
    color: var(--overlay1);
    display: flex;
    gap: 6px;
  }

  .restore-btn {
    width: 28px;
    height: 28px;
    border-radius: 7px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    flex-shrink: 0;
    opacity: 0;
    transition: background 0.1s, color 0.1s, opacity 0.15s;
  }

  .trash-row:hover .restore-btn {
    opacity: 1;
  }

  .restore-btn:hover {
    background: var(--success-bg);
    color: var(--green);
  }

  .trash-date {
    color: var(--overlay0);
    font-size: 10px;
  }
</style>
