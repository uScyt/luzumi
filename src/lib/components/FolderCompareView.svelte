<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { formatSize, formatDate } from "../types";

  interface CompareEntry {
    name: string;
    path: string;
    size: number;
    modified: number;
  }

  interface ComparisonResult {
    only_left: CompareEntry[];
    only_right: CompareEntry[];
    modified: Array<{ left: CompareEntry; right: CompareEntry }>;
    identical: CompareEntry[];
  }

  let leftPath = $state(fm.currentPath);
  let rightPath = $state("");
  let result = $state<ComparisonResult | null>(null);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let filter = $state<"all" | "different" | "left_only" | "right_only" | "identical">("all");

  async function compare() {
    if (!leftPath || !rightPath) return;
    loading = true;
    error = null;
    result = null;
    try {
      result = await invoke<ComparisonResult>("cmd_compare_directories", { left: leftPath, right: rightPath });
    } catch (e) {
      error = String(e);
    }
    loading = false;
  }

  function close() {
    fm.ui.showFolderCompare = false;
  }

  function swap() {
    const tmp = leftPath;
    leftPath = rightPath;
    rightPath = tmp;
    if (result) compare();
  }

  const stats = $derived.by(() => {
    if (!result) return null;
    return {
      leftOnly: result.only_left.length,
      rightOnly: result.only_right.length,
      modified: result.modified.length,
      identical: result.identical.length,
      total: result.only_left.length + result.only_right.length + result.modified.length + result.identical.length,
    };
  });
</script>

{#if fm.ui.showFolderCompare}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="compare-overlay" onclick={close}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="compare-panel" onclick={(e) => e.stopPropagation()}>
      <div class="compare-header">
        <h3>Folder Comparison</h3>
        <button class="close-btn" onclick={close} aria-label="Close">
          <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
        </button>
      </div>

      <div class="compare-paths">
        <div class="path-input">
          <label for="compare-left">Left</label>
          <input id="compare-left" bind:value={leftPath} type="text" placeholder="Left folder path" />
        </div>
        <button class="swap-btn" onclick={swap} title="Swap">
          <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M6.99 11L3 15l3.99 4v-3H14v-2H6.99v-3zM21 9l-3.99-4v3H10v2h7.01v3L21 9z"/></svg>
        </button>
        <div class="path-input">
          <label for="compare-right">Right</label>
          <input id="compare-right" bind:value={rightPath} type="text" placeholder="Right folder path" />
        </div>
      </div>

      <div class="compare-actions">
        <button class="compare-btn" onclick={compare} disabled={loading || !leftPath || !rightPath}>
          {loading ? "Comparing..." : "Compare"}
        </button>
      </div>

      {#if error}
        <div class="compare-error">{error}</div>
      {/if}

      {#if result && stats}
        <div class="compare-stats">
          <button class:active={filter === "all"} onclick={() => filter = "all"}>All ({stats.total})</button>
          <button class:active={filter === "left_only"} onclick={() => filter = "left_only"} class="left-only">Left only ({stats.leftOnly})</button>
          <button class:active={filter === "right_only"} onclick={() => filter = "right_only"} class="right-only">Right only ({stats.rightOnly})</button>
          <button class:active={filter === "different"} onclick={() => filter = "different"} class="modified">Modified ({stats.modified})</button>
          <button class:active={filter === "identical"} onclick={() => filter = "identical"} class="identical">Identical ({stats.identical})</button>
        </div>

        <div class="compare-results">
          {#if filter === "all" || filter === "left_only"}
            {#each result.only_left as entry (entry.path)}
              <div class="result-row left-only-row">
                <span class="status-badge left">L</span>
                <span class="entry-name">{entry.name}</span>
                <span class="entry-size">{formatSize(entry.size)}</span>
              </div>
            {/each}
          {/if}
          {#if filter === "all" || filter === "right_only"}
            {#each result.only_right as entry (entry.path)}
              <div class="result-row right-only-row">
                <span class="status-badge right">R</span>
                <span class="entry-name">{entry.name}</span>
                <span class="entry-size">{formatSize(entry.size)}</span>
              </div>
            {/each}
          {/if}
          {#if filter === "all" || filter === "different"}
            {#each result.modified as pair (pair.left.path)}
              <div class="result-row modified-row">
                <span class="status-badge modified">M</span>
                <span class="entry-name">{pair.left.name}</span>
                <span class="entry-size">{formatSize(pair.left.size)} / {formatSize(pair.right.size)}</span>
              </div>
            {/each}
          {/if}
          {#if filter === "all" || filter === "identical"}
            {#each result.identical as entry (entry.path)}
              <div class="result-row identical-row">
                <span class="status-badge identical">=</span>
                <span class="entry-name">{entry.name}</span>
                <span class="entry-size">{formatSize(entry.size)}</span>
              </div>
            {/each}
          {/if}
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .compare-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 10000;
    display: flex;
    justify-content: center;
    padding-top: 5vh;
  }

  .compare-panel {
    width: 720px;
    max-height: 85vh;
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 12px;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .compare-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .compare-header h3 { margin: 0; font-size: 16px; color: var(--text-primary, #cdd6f4); }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
  }

  .close-btn:hover { background: var(--bg-hover, #313244); }

  .compare-paths {
    display: flex;
    align-items: flex-end;
    gap: 8px;
    padding: 12px 20px;
  }

  .path-input {
    flex: 1;
  }

  .path-input label {
    display: block;
    font-size: 11px;
    color: var(--text-secondary, #a6adc8);
    margin-bottom: 4px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .path-input input {
    width: 100%;
    background: var(--bg-primary, #181825);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 6px;
    color: var(--text-primary, #cdd6f4);
    padding: 6px 10px;
    font-size: 12px;
    outline: none;
    box-sizing: border-box;
  }

  .swap-btn {
    background: var(--bg-tertiary, #45475a);
    border: none;
    color: var(--text-secondary, #a6adc8);
    border-radius: 6px;
    padding: 6px 8px;
    cursor: pointer;
    display: flex;
    margin-bottom: 1px;
  }

  .swap-btn:hover { color: var(--text-primary, #cdd6f4); }

  .compare-actions {
    padding: 0 20px 12px;
  }

  .compare-btn {
    background: var(--accent, #89b4fa);
    color: var(--bg-primary, #1e1e2e);
    border: none;
    border-radius: 6px;
    padding: 8px 24px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
  }

  .compare-btn:disabled { opacity: 0.4; cursor: default; }

  .compare-error {
    padding: 8px 20px;
    color: var(--danger, #f38ba8);
    font-size: 12px;
  }

  .compare-stats {
    display: flex;
    gap: 4px;
    padding: 8px 20px;
    border-top: 1px solid var(--border-color, #45475a);
    border-bottom: 1px solid var(--border-color, #45475a);
    flex-wrap: wrap;
  }

  .compare-stats button {
    background: none;
    border: 1px solid var(--border-color, #45475a);
    color: var(--text-secondary, #a6adc8);
    border-radius: 14px;
    padding: 3px 10px;
    font-size: 11px;
    cursor: pointer;
  }

  .compare-stats button.active {
    background: var(--bg-tertiary, #45475a);
    color: var(--text-primary, #cdd6f4);
  }

  .compare-results {
    flex: 1;
    overflow-y: auto;
    padding: 4px 0;
  }

  .result-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px 20px;
    font-size: 13px;
  }

  .result-row:hover { background: var(--bg-hover, #313244); }

  .status-badge {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 700;
    flex-shrink: 0;
  }

  .status-badge.left { background: color-mix(in srgb, #a6e3a1 20%, transparent); color: #a6e3a1; }
  .status-badge.right { background: color-mix(in srgb, #f38ba8 20%, transparent); color: #f38ba8; }
  .status-badge.modified { background: color-mix(in srgb, #fab387 20%, transparent); color: #fab387; }
  .status-badge.identical { background: color-mix(in srgb, #a6adc8 15%, transparent); color: #a6adc8; }

  .entry-name {
    flex: 1;
    color: var(--text-primary, #cdd6f4);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .entry-size {
    font-size: 11px;
    color: var(--text-secondary, #a6adc8);
    flex-shrink: 0;
  }
</style>
