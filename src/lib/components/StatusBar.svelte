<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { formatSize } from "../types";
  import { t } from "../i18n";

  const clipInfo = $derived(
    fm.clipboard
      ? t.clipboardInfo(fm.clipboard.mode, fm.clipboard.paths.length)
      : null
  );
</script>

<footer class="statusbar">
  <div class="left">
    {#if fm.showTrashView}
      <span class="status-text">
        <svg class="status-icon" width="12" height="12" viewBox="0 0 16 16" fill="none" style="color: var(--maroon)">
          <path d="M3 5H13M5.5 5V3.5C5.5 3.22 5.72 3 6 3H10C10.28 3 10.5 3.22 10.5 3.5V5M12 5V13C12 13.55 11.55 14 11 14H5C4.45 14 4 13.55 4 13V5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        {t.trash} &middot; {t.elements(fm.trashEntries.length)}
        {#if fm.trashSize > 0}
          &middot; {formatSize(fm.trashSize)}
        {/if}
      </span>
      {#if fm.deleteProgress}
        {@const pct = fm.deleteProgress.total > 0 ? fm.deleteProgress.done / fm.deleteProgress.total : 0}
        <svg width="14" height="14" viewBox="0 0 14 14" class="del-ring">
          <circle cx="7" cy="7" r="5" stroke="rgba(255,255,255,0.1)" stroke-width="2" fill="none"/>
          <circle cx="7" cy="7" r="5" stroke="var(--mauve)" stroke-width="2" fill="none"
            stroke-dasharray="31.4"
            stroke-dashoffset={31.4 * (1 - pct)}
            stroke-linecap="round"
            transform="rotate(-90 7 7)"
            style="transition: stroke-dashoffset 0.3s ease"/>
        </svg>
      {/if}
    {:else if fm.selected.size > 0}
      <span class="status-text">{fm.selected.size} {t.selected} &middot; {t.elements(fm.entries.length)}</span>
    {:else}
      <span class="status-text">{t.elements(fm.entries.length)}</span>
    {/if}
    {#if fm.isLoading}
      <span class="loading">
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none" class="spin">
          <path d="M8 2A6 6 0 1 0 14 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
        </svg>
      </span>
    {/if}
    {#if fm.copyMoveProgress}
      {@const pct = fm.copyMoveProgress.total > 0 ? fm.copyMoveProgress.done / fm.copyMoveProgress.total : 0}
      <span class="progress-badge">
        <svg width="14" height="14" viewBox="0 0 14 14" class="progress-ring">
          <circle cx="7" cy="7" r="5" stroke="rgba(255,255,255,0.1)" stroke-width="2" fill="none"/>
          <circle cx="7" cy="7" r="5" stroke="var(--blue)" stroke-width="2" fill="none"
            stroke-dasharray="31.4"
            stroke-dashoffset={31.4 * (1 - pct)}
            stroke-linecap="round"
            transform="rotate(-90 7 7)"
            style="transition: stroke-dashoffset 0.3s ease"/>
        </svg>
        <span class="progress-text">{fm.copyMoveProgress.current}</span>
        <span class="progress-count">{fm.copyMoveProgress.done}/{fm.copyMoveProgress.total}</span>
      </span>
    {/if}
    {#if clipInfo}
      <span class="clip-badge">
        <svg width="9" height="9" viewBox="0 0 16 16" fill="none">
          <rect x="3" y="5" width="10" height="10" rx="1.5" stroke="currentColor" stroke-width="1.7"/>
          <path d="M5.5 5V3.5C5.5 2.95 5.95 2.5 6.5 2.5H9.5C10.05 2.5 10.5 2.95 10.5 3.5V5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" fill="none"/>
        </svg>
        {clipInfo}
      </span>
    {/if}
    {#if fm.undoStack.length > 0}
      <span class="undo-hint">Ctrl+Z</span>
    {/if}
    {#if fm.elevated}
      <span class="elevated-badge">
        <svg width="9" height="9" viewBox="0 0 16 16" fill="none">
          <rect x="4" y="7" width="8" height="7" rx="1.5" stroke="currentColor" stroke-width="1.8" fill="none"/>
          <path d="M11 7V5C11 3.34 9.66 2 8 2S5 3.34 5 5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" fill="none"/>
        </svg>
        {t.admin}
      </span>
    {/if}
  </div>

  <div class="center">
    {#if fm.gitStatus?.is_repo}
      <span class="git-badge">
        <svg width="10" height="10" viewBox="0 0 16 16" fill="none">
          <circle cx="5" cy="5" r="2" stroke="currentColor" stroke-width="1.5" fill="none"/>
          <circle cx="11" cy="11" r="2" stroke="currentColor" stroke-width="1.5" fill="none"/>
          <path d="M5 7V9C5 10.1 5.9 11 7 11H9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
        {fm.gitStatus.branch}
        {#if fm.gitStatus.modified.length > 0}<span class="git-mod">+{fm.gitStatus.modified.length}</span>{/if}
        {#if fm.gitStatus.staged.length > 0}<span class="git-staged">~{fm.gitStatus.staged.length}</span>{/if}
        {#if fm.gitStatus.untracked.length > 0}<span class="git-untracked">?{fm.gitStatus.untracked.length}</span>{/if}
      </span>
    {/if}
    {#if fm.diskSpace}
      <span class="disk-badge">{formatSize(fm.diskSpace.available)} {t.free} {t.of} {formatSize(fm.diskSpace.total)}</span>
    {/if}
  </div>

  <div class="right">
    {#if fm.viewMode === "grid"}
      <input
        type="range"
        class="zoom-slider"
        min="48"
        max="128"
        step="8"
        value={fm.gridIconSize}
        oninput={(e) => { fm.gridIconSize = parseInt((e.target as HTMLInputElement).value); try { localStorage.setItem("luzumi_icon_size", String(fm.gridIconSize)); } catch {} }}
        title="Icon size: {fm.gridIconSize}px"
      />
    {/if}
    <button
      class="view-btn"
      class:active={fm.viewMode === "list"}
      title={t.listView}
      onclick={() => (fm.viewMode = "list")}
    >
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
        <path d="M3 4H13M3 8H13M3 12H13" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/>
      </svg>
    </button>
    <button
      class="view-btn"
      class:active={fm.viewMode === "grid"}
      title={t.gridView}
      onclick={() => (fm.viewMode = "grid")}
    >
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
        <rect x="2" y="2" width="5" height="5" rx="1.2" stroke="currentColor" stroke-width="1.5"/>
        <rect x="9" y="2" width="5" height="5" rx="1.2" stroke="currentColor" stroke-width="1.5"/>
        <rect x="2" y="9" width="5" height="5" rx="1.2" stroke="currentColor" stroke-width="1.5"/>
        <rect x="9" y="9" width="5" height="5" rx="1.2" stroke="currentColor" stroke-width="1.5"/>
      </svg>
    </button>
  </div>
</footer>

<style>
  .statusbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 14px;
    background: var(--app-bg);
    border-top: 1px solid var(--border-subtle);
    height: 28px;
    flex-shrink: 0;
  }

  .left {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .status-text {
    font-size: 11.5px;
    color: var(--subtext0);
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .status-icon {
    flex-shrink: 0;
  }

  .loading {
    color: var(--overlay1);
    display: flex;
    align-items: center;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .spin {
    animation: spin 0.8s linear infinite;
  }

  .del-ring {
    flex-shrink: 0;
  }

  .progress-badge {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 10px;
    color: var(--blue);
    padding: 1px 7px 1px 3px;
    border-radius: 4px;
    background: rgba(138, 173, 244, 0.08);
  }

  .progress-ring {
    flex-shrink: 0;
  }

  .progress-text {
    max-width: 100px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .progress-count {
    opacity: 0.7;
    font-size: 9px;
  }

  .clip-badge {
    font-size: 9px;
    font-weight: 600;
    letter-spacing: 0.04em;
    padding: 1px 7px 1px 5px;
    border-radius: 4px;
    background: rgba(138, 173, 244, 0.1);
    color: var(--blue);
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .undo-hint {
    font-size: 9px;
    font-weight: 500;
    letter-spacing: 0.04em;
    padding: 1px 6px;
    border-radius: 4px;
    background: var(--accent-subtle);
    color: var(--mauve);
    opacity: 0.7;
  }

  .elevated-badge {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.06em;
    padding: 1px 7px 1px 5px;
    border-radius: 4px;
    background: rgba(245, 169, 127, 0.12);
    color: var(--peach);
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .center {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .git-badge {
    font-size: 10px;
    font-weight: 600;
    padding: 1px 7px 1px 5px;
    border-radius: 4px;
    background: rgba(166, 218, 149, 0.1);
    color: var(--green);
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .git-mod {
    color: var(--yellow);
    font-weight: 700;
  }

  .git-staged {
    color: var(--green);
    font-weight: 700;
  }

  .git-untracked {
    color: var(--overlay1);
    font-weight: 700;
  }

  .disk-badge {
    font-size: 10px;
    color: var(--overlay1);
  }

  .zoom-slider {
    width: 64px;
    height: 4px;
    appearance: none;
    background: var(--surface1);
    border-radius: 2px;
    outline: none;
    cursor: pointer;
    margin-right: 6px;
  }

  .zoom-slider::-webkit-slider-thumb {
    appearance: none;
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--accent);
    cursor: pointer;
  }

  .right {
    display: flex;
    gap: 2px;
    align-items: center;
  }

  .view-btn {
    width: 26px;
    height: 24px;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    transition: background 0.1s, color 0.1s;
  }

  .view-btn:hover {
    background: var(--hover-bg);
    color: var(--text);
  }

  .view-btn.active {
    color: var(--mauve);
  }
</style>
