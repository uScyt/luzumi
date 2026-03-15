<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fm } from "../../fileManager.svelte";
  import { t } from "../../i18n";
  import { formatSize } from "../../types";

  let checked = $state<Set<string>>(new Set());

  const isScanning = $derived(fm.showDuplicateFinder && fm.duplicateGroups.length === 0 && !fm.error);

  function toggleCheck(path: string) {
    const next = new Set(checked);
    if (next.has(path)) next.delete(path);
    else next.add(path);
    checked = next;
  }

  function keepNewest() {
    const next = new Set<string>();
    for (const group of fm.duplicateGroups) {
      // Sort by modification time, keep the first (newest stays unchecked)
      const sorted = [...group.paths].sort();
      for (let i = 1; i < sorted.length; i++) {
        next.add(sorted[i]);
      }
    }
    checked = next;
  }

  async function deleteChecked() {
    if (checked.size === 0) return;
    const paths = [...checked];
    try {
      await invoke("cmd_delete_entries", { paths });
      fm.showDuplicateFinder = false;
      fm.duplicateGroups = [];
      checked = new Set();
      await fm.reload();
      fm.setStatus(t.deletedItems(paths.length));
    } catch (e) {
      fm.error = String(e);
    }
  }

  function close() {
    fm.showDuplicateFinder = false;
    fm.duplicateGroups = [];
    checked = new Set();
  }
</script>

<div class="overlay" role="dialog" aria-modal="true" tabindex="-1" onclick={close} onkeydown={(e) => { if (e.key === "Escape") close(); }}>
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div class="dialog" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
    <div class="dialog-header">
      <div class="icon-wrap">
        <svg width="17" height="17" viewBox="0 0 16 16" fill="none">
          <rect x="2" y="2" width="8" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/>
          <rect x="6" y="5" width="8" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/>
        </svg>
      </div>
      <h2>{t.findDuplicates}</h2>
      <button class="close-btn" onclick={close} aria-label={t.dismissClose}>
        <svg width="11" height="11" viewBox="0 0 16 16" fill="none">
          <path d="M2 2L14 14M14 2L2 14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
      </button>
    </div>

    <div class="dialog-sep"></div>

    {#if isScanning}
      <div class="scanning">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" class="spin-icon">
          <path d="M8 2A6 6 0 1 0 14 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
        </svg>
        {t.scanning}
      </div>
    {:else if fm.duplicateGroups.length === 0}
      <div class="no-results">{t.noDuplicates}</div>
    {:else}
      <div class="summary">
        {t.duplicatesFound(fm.duplicateGroups.length)}
      </div>

      <div class="groups-list">
        {#each fm.duplicateGroups as group, gi}
          <div class="group">
            <div class="group-header">
              <span class="group-size">{formatSize(group.size)}</span>
              <span class="group-count">{group.paths.length} files</span>
            </div>
            {#each group.paths as path}
              <label class="dup-row">
                <input type="checkbox" checked={checked.has(path)} onchange={() => toggleCheck(path)} />
                <span class="dup-path" title={path}>{path.split("/").pop()}</span>
                <span class="dup-dir">{path.split("/").slice(0, -1).join("/")}</span>
              </label>
            {/each}
          </div>
        {/each}
      </div>

      <div class="actions">
        <button class="btn-secondary" onclick={keepNewest}>{t.keepNewest}</button>
        <button class="btn-danger" disabled={checked.size === 0} onclick={deleteChecked}>
          {t.deleteSelected} ({checked.size})
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 600;
    animation: fade 0.12s ease;
  }

  @keyframes fade { from { opacity: 0; } to { opacity: 1; } }

  .dialog {
    background: var(--glass-bg-strong);
    backdrop-filter: blur(30px) saturate(1.5);
    border: 1px solid var(--border-medium);
    border-radius: 16px;
    padding: 20px 22px 18px;
    width: 520px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 20px 56px rgba(0,0,0,0.6), 0 0 0 0.5px var(--border-subtle);
    animation: pop 0.18s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes pop {
    from { transform: scale(0.94) translateY(-6px); opacity: 0; }
    to   { transform: scale(1) translateY(0); opacity: 1; }
  }

  .dialog-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 16px;
  }

  .icon-wrap {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    background: rgba(245, 169, 127, 0.1);
    border: 1px solid rgba(245, 169, 127, 0.18);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--peach);
    flex-shrink: 0;
  }

  h2 {
    font-size: 15px;
    font-weight: 700;
    color: var(--text);
    line-height: 1.2;
    flex: 1;
  }

  .close-btn {
    width: 26px;
    height: 26px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay0);
    transition: background 0.1s, color 0.1s;
  }

  .close-btn:hover { background: var(--hover-bg); color: var(--subtext0); }

  .dialog-sep {
    height: 1px;
    background: var(--border-light);
    margin: 0 0 14px;
  }

  .scanning, .no-results {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    padding: 40px 0;
    color: var(--overlay1);
    font-size: 14px;
  }

  @keyframes spin { to { transform: rotate(360deg); } }
  .spin-icon { animation: spin 0.8s linear infinite; }

  .summary {
    font-size: 12px;
    color: var(--subtext0);
    margin-bottom: 12px;
  }

  .groups-list {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 10px;
    max-height: 400px;
    margin-bottom: 14px;
  }

  .group {
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid var(--border-subtle);
    border-radius: 10px;
    padding: 8px 10px;
  }

  .group-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 6px;
  }

  .group-size {
    font-size: 11px;
    font-weight: 700;
    color: var(--peach);
  }

  .group-count {
    font-size: 10px;
    color: var(--overlay0);
  }

  .dup-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 3px 4px;
    border-radius: 5px;
    cursor: pointer;
    transition: background 0.08s;
  }

  .dup-row:hover { background: var(--hover-bg-subtle); }

  .dup-row input[type="checkbox"] {
    accent-color: var(--mauve);
    flex-shrink: 0;
  }

  .dup-path {
    font-size: 12px;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex-shrink: 0;
  }

  .dup-dir {
    font-size: 10px;
    color: var(--overlay0);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    min-width: 0;
    flex: 1;
  }

  .actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .btn-secondary {
    padding: 7px 14px;
    border-radius: 8px;
    font-size: 13px;
    color: var(--subtext1);
    background: var(--hover-bg);
    border: 1px solid var(--border-light);
    transition: background 0.1s;
  }

  .btn-secondary:hover { background: var(--hover-bg-strong); }

  .btn-danger {
    padding: 7px 14px;
    border-radius: 8px;
    font-size: 13px;
    color: white;
    background: var(--red);
    font-weight: 600;
    transition: opacity 0.1s;
  }

  .btn-danger:hover:not(:disabled) { opacity: 0.9; }
  .btn-danger:disabled { opacity: 0.4; cursor: default; }
</style>
