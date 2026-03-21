<script lang="ts">
  import { fm } from "../../fileManager.svelte";
  import { formatSize, formatDate } from "../../types";

  interface ConflictInfo {
    sourcePath: string;
    destPath: string;
    sourceName: string;
    sourceSize: number;
    sourceModified: number;
    destSize: number;
    destModified: number;
  }

  let {
    conflicts,
    onResolve,
    onCancel,
  }: {
    conflicts: ConflictInfo[];
    onResolve: (strategy: "overwrite" | "rename" | "skip", applyAll: boolean) => void;
    onCancel: () => void;
  } = $props();

  let currentIndex = $state(0);
  let applyToAll = $state(false);

  const current = $derived(conflicts[currentIndex]);
  const remaining = $derived(conflicts.length - currentIndex);

  function resolve(strategy: "overwrite" | "rename" | "skip") {
    onResolve(strategy, applyToAll);
    if (!applyToAll && currentIndex < conflicts.length - 1) {
      currentIndex++;
    }
  }
</script>

{#if conflicts.length > 0 && current}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="conflict-overlay" onclick={onCancel}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="conflict-dialog" onclick={(e) => e.stopPropagation()}>
      <div class="conflict-header">
        <h3>File Conflict</h3>
        {#if conflicts.length > 1}
          <span class="conflict-count">{remaining} remaining</span>
        {/if}
      </div>

      <p class="conflict-message">
        A file named <strong>{current.sourceName}</strong> already exists at the destination.
      </p>

      <div class="comparison">
        <div class="compare-col">
          <div class="compare-label">Source</div>
          <div class="compare-name">{current.sourceName}</div>
          <div class="compare-detail">Size: {formatSize(current.sourceSize)}</div>
          <div class="compare-detail">Modified: {formatDate(current.sourceModified)}</div>
        </div>
        <div class="compare-arrow">
          <svg viewBox="0 0 24 24" width="20" height="20"><path fill="currentColor" d="M12 4l-1.41 1.41L16.17 11H4v2h12.17l-5.58 5.59L12 20l8-8z"/></svg>
        </div>
        <div class="compare-col">
          <div class="compare-label">Destination</div>
          <div class="compare-name">{current.sourceName}</div>
          <div class="compare-detail">Size: {formatSize(current.destSize)}</div>
          <div class="compare-detail">Modified: {formatDate(current.destModified)}</div>
        </div>
      </div>

      {#if conflicts.length > 1}
        <label class="apply-all">
          <input type="checkbox" bind:checked={applyToAll} />
          Apply to all {conflicts.length} conflicts
        </label>
      {/if}

      <div class="conflict-actions">
        <button class="action-btn overwrite" onclick={() => resolve("overwrite")}>
          Overwrite
        </button>
        <button class="action-btn rename" onclick={() => resolve("rename")}>
          Auto-Rename
        </button>
        <button class="action-btn skip" onclick={() => resolve("skip")}>
          Skip
        </button>
        <button class="action-btn cancel" onclick={onCancel}>
          Cancel All
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .conflict-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 10000;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .conflict-dialog {
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 12px;
    padding: 24px;
    width: 480px;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.5);
  }

  .conflict-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .conflict-header h3 {
    margin: 0;
    font-size: 16px;
    color: var(--text-primary, #cdd6f4);
  }

  .conflict-count {
    font-size: 12px;
    color: var(--warning, #fab387);
    background: color-mix(in srgb, var(--warning, #fab387) 15%, transparent);
    padding: 2px 8px;
    border-radius: 10px;
  }

  .conflict-message {
    font-size: 13px;
    color: var(--text-secondary, #a6adc8);
    margin: 0 0 16px;
    line-height: 1.5;
  }

  .conflict-message strong {
    color: var(--text-primary, #cdd6f4);
  }

  .comparison {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 16px;
  }

  .compare-col {
    flex: 1;
    background: var(--bg-primary, #181825);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 8px;
    padding: 12px;
  }

  .compare-label {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-secondary, #a6adc8);
    margin-bottom: 6px;
  }

  .compare-name {
    font-size: 13px;
    color: var(--text-primary, #cdd6f4);
    font-weight: 500;
    margin-bottom: 4px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .compare-detail {
    font-size: 11px;
    color: var(--text-secondary, #a6adc8);
  }

  .compare-arrow {
    color: var(--text-secondary, #a6adc8);
    flex-shrink: 0;
  }

  .apply-all {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
    margin-bottom: 16px;
  }

  .conflict-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .action-btn {
    padding: 7px 16px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
  }

  .action-btn.overwrite {
    background: var(--accent, #89b4fa);
    color: var(--bg-primary, #1e1e2e);
  }

  .action-btn.rename {
    background: var(--success, #a6e3a1);
    color: var(--bg-primary, #1e1e2e);
  }

  .action-btn.skip {
    background: var(--bg-tertiary, #45475a);
    color: var(--text-primary, #cdd6f4);
  }

  .action-btn.cancel {
    background: none;
    border: 1px solid var(--border-color, #45475a);
    color: var(--text-secondary, #a6adc8);
  }

  .action-btn:hover { opacity: 0.9; }
</style>
