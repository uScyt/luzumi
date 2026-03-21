<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let logs = $state<string[]>([]);
  let metrics = $state<Record<string, string>>({});
  let activeTab = $state<"logs" | "metrics" | "performance">("logs");
  let loading = $state(false);
  let autoRefresh = $state(false);
  let refreshTimer: ReturnType<typeof setInterval> | null = null;

  $effect(() => {
    if (fm.ui.showDebugPanel) {
      fetchLogs();
      fetchMetrics();
    }
  });

  $effect(() => {
    if (autoRefresh && fm.ui.showDebugPanel) {
      refreshTimer = setInterval(() => {
        fetchLogs();
        fetchMetrics();
      }, 3000);
    }
    return () => { if (refreshTimer) { clearInterval(refreshTimer); refreshTimer = null; } };
  });

  async function fetchLogs() {
    loading = true;
    try {
      logs = await invoke<string[]>("cmd_get_logs");
    } catch (e) {
      logs = [`Error fetching logs: ${e}`];
    } finally {
      loading = false;
    }
  }

  async function fetchMetrics() {
    try {
      metrics = await invoke<Record<string, string>>("cmd_get_performance_metrics");
    } catch {}
  }

  async function exportLogs() {
    try {
      const path = await invoke<string>("cmd_export_logs");
      fm.ui.setStatus(`Logs exported to ${path}`);
    } catch (e) {
      fm.ui.error = `Export failed: ${e}`;
    }
  }

  function close() {
    fm.ui.showDebugPanel = false;
  }

  const frontendMetrics = $derived.by(() => {
    const m: Record<string, string> = {};
    m["IPC Calls"] = String(fm.debug.ipcCallCount);
    m["Last IPC Duration"] = `${fm.debug.lastIpcDuration}ms`;
    m["Avg IPC Duration"] = `${fm.debug.getAverageIpcDuration().toFixed(1)}ms`;
    m["Frontend Logs"] = String(fm.debug.logs.length);
    m["Entries Loaded"] = String(fm.entries.length);
    m["Selected"] = String(fm.selected.size);
    m["Undo Stack"] = String(fm.undoStack.length);
    return m;
  });
</script>

{#if fm.ui.showDebugPanel}
  <div class="debug-panel">
    <div class="debug-header">
      <h3>Debug Panel</h3>
      <div class="debug-header-actions">
        <label class="auto-refresh">
          <input type="checkbox" bind:checked={autoRefresh} />
          Auto
        </label>
        <button class="debug-btn" onclick={fetchLogs} title="Refresh">
          <svg viewBox="0 0 24 24" width="14" height="14"><path fill="currentColor" d="M17.65 6.35A7.96 7.96 0 0012 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0112 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg>
        </button>
        <button class="debug-btn" onclick={exportLogs} title="Export">
          <svg viewBox="0 0 24 24" width="14" height="14"><path fill="currentColor" d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/></svg>
        </button>
        <button class="debug-btn" onclick={close} title="Close">
          <svg viewBox="0 0 24 24" width="14" height="14"><path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
        </button>
      </div>
    </div>

    <div class="debug-tabs">
      <button class:active={activeTab === "logs"} onclick={() => activeTab = "logs"}>Logs</button>
      <button class:active={activeTab === "metrics"} onclick={() => activeTab = "metrics"}>Backend</button>
      <button class:active={activeTab === "performance"} onclick={() => activeTab = "performance"}>Frontend</button>
    </div>

    <div class="debug-content">
      {#if activeTab === "logs"}
        <div class="log-list">
          {#if loading}
            <div class="debug-empty">Loading...</div>
          {:else if logs.length === 0}
            <div class="debug-empty">No logs available</div>
          {:else}
            {#each logs as line}
              <div class="log-line" class:error={line.includes("ERROR")} class:warn={line.includes("WARN")} class:info={line.includes("INFO")}>{line}</div>
            {/each}
          {/if}
        </div>
      {:else if activeTab === "metrics"}
        <div class="metrics-grid">
          {#each Object.entries(metrics) as [key, value]}
            <div class="metric-item">
              <span class="metric-key">{key}</span>
              <span class="metric-value">{value}</span>
            </div>
          {/each}
          {#if Object.keys(metrics).length === 0}
            <div class="debug-empty">No backend metrics available</div>
          {/if}
        </div>
      {:else}
        <div class="metrics-grid">
          {#each Object.entries(frontendMetrics) as [key, value]}
            <div class="metric-item">
              <span class="metric-key">{key}</span>
              <span class="metric-value">{value}</span>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .debug-panel {
    position: fixed;
    bottom: 28px;
    right: 0;
    width: 420px;
    max-height: 50vh;
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--border-color, #45475a);
    border-right: none;
    border-bottom: none;
    border-top-left-radius: 10px;
    box-shadow: -4px -4px 20px rgba(0, 0, 0, 0.3);
    display: flex;
    flex-direction: column;
    z-index: 9000;
    font-size: 12px;
  }

  .debug-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .debug-header h3 {
    margin: 0;
    font-size: 13px;
    color: var(--text-primary, #cdd6f4);
  }

  .debug-header-actions {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .auto-refresh {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 11px;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
  }

  .debug-btn {
    background: none;
    border: none;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
    padding: 3px;
    border-radius: 4px;
    display: flex;
    align-items: center;
  }

  .debug-btn:hover {
    background: var(--bg-hover, #313244);
    color: var(--text-primary, #cdd6f4);
  }

  .debug-tabs {
    display: flex;
    gap: 2px;
    padding: 4px 8px;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .debug-tabs button {
    background: none;
    border: none;
    color: var(--text-secondary, #a6adc8);
    padding: 4px 12px;
    cursor: pointer;
    border-radius: 4px;
    font-size: 11px;
  }

  .debug-tabs button.active {
    background: var(--bg-tertiary, #45475a);
    color: var(--text-primary, #cdd6f4);
  }

  .debug-content {
    flex: 1;
    overflow-y: auto;
    padding: 4px;
  }

  .log-list {
    font-family: "JetBrains Mono", "Fira Code", monospace;
    font-size: 10px;
  }

  .log-line {
    padding: 1px 8px;
    color: var(--text-primary, #cdd6f4);
    white-space: pre-wrap;
    word-break: break-all;
  }

  .log-line.error { color: var(--danger, #f38ba8); }
  .log-line.warn { color: var(--warning, #fab387); }
  .log-line.info { color: var(--accent, #89b4fa); }

  .metrics-grid {
    padding: 8px;
  }

  .metric-item {
    display: flex;
    justify-content: space-between;
    padding: 4px 8px;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .metric-key {
    color: var(--text-secondary, #a6adc8);
  }

  .metric-value {
    color: var(--text-primary, #cdd6f4);
    font-family: monospace;
  }

  .debug-empty {
    padding: 24px;
    text-align: center;
    color: var(--text-secondary, #a6adc8);
  }
</style>
