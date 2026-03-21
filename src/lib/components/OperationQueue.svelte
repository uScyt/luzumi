<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  interface Operation {
    id: string;
    op_type: string;
    status: string;
    progress: number;
    total: number;
    current_file: string;
    paused: boolean;
  }

  let operations = $state<Operation[]>([]);
  let expanded = $state(false);

  let pollInterval: ReturnType<typeof setInterval> | null = null;

  function startPolling() {
    if (pollInterval) return;
    pollInterval = setInterval(refreshOps, 2000);
  }

  function stopPolling() {
    if (pollInterval) { clearInterval(pollInterval); pollInterval = null; }
  }

  onMount(() => {
    const unsubs: Array<() => void> = [];

    (async () => {
      unsubs.push(await listen<any>("operation-progress", (ev) => {
        const p = ev.payload;
        const idx = operations.findIndex(o => o.id === p.id);
        if (idx >= 0) {
          operations[idx] = { ...operations[idx], progress: p.progress, total: p.total, current_file: p.current_file ?? "" };
        }
        startPolling();
      }));
      unsubs.push(await listen<any>("operation-complete", () => {
        refreshOps().then(() => {
          if (operations.filter(o => o.status === "running" || o.status === "paused").length === 0) {
            stopPolling();
          }
        });
      }));
    })();

    // Initial check
    refreshOps().then(() => {
      if (operations.filter(o => o.status === "running" || o.status === "paused").length > 0) {
        startPolling();
      }
    });

    return () => {
      stopPolling();
      unsubs.forEach(u => u());
    };
  });

  async function refreshOps() {
    try {
      operations = await invoke<Operation[]>("cmd_list_operations");
    } catch (e) { console.error("Failed to refresh operations:", e); }
  }

  async function pauseOp(id: string) {
    try { await invoke("cmd_pause_operation", { id }); refreshOps(); } catch (e) { console.error("Failed to pause operation:", e); }
  }

  async function resumeOp(id: string) {
    try { await invoke("cmd_resume_operation", { id }); refreshOps(); } catch (e) { console.error("Failed to resume operation:", e); }
  }

  async function cancelOp(id: string) {
    try { await invoke("cmd_cancel_operation_by_id", { id }); refreshOps(); } catch (e) { console.error("Failed to cancel operation:", e); }
  }

  function formatProgress(op: Operation): string {
    if (op.total <= 0) return "Preparing...";
    const pct = Math.round((op.progress / op.total) * 100);
    return `${pct}% (${op.progress}/${op.total})`;
  }

  const activeOps = $derived(operations.filter(o => o.status === "running" || o.status === "paused"));
  const hasActive = $derived(activeOps.length > 0);
</script>

{#if hasActive}
  <div class="op-queue" class:expanded>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="op-queue-header" onclick={() => expanded = !expanded}>
      <span class="op-badge">{activeOps.length}</span>
      <span class="op-label">Operation{activeOps.length > 1 ? "s" : ""} in progress</span>
      <svg class="op-chevron" viewBox="0 0 24 24" width="14" height="14" style="transform: rotate({expanded ? 180 : 0}deg)">
        <path fill="currentColor" d="M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6z"/>
      </svg>
    </div>

    {#if expanded}
      <div class="op-list">
        {#each activeOps as op (op.id)}
          <div class="op-item">
            <div class="op-item-header">
              <span class="op-type">{op.op_type}</span>
              <span class="op-progress-text">{formatProgress(op)}</span>
            </div>
            {#if op.current_file}
              <div class="op-current-file">{op.current_file.split("/").pop()}</div>
            {/if}
            <div class="op-progress-bar">
              <div class="op-progress-fill" class:paused={op.paused} style="width: {op.total > 0 ? (op.progress / op.total * 100) : 0}%"></div>
            </div>
            <div class="op-actions">
              {#if op.paused}
                <button onclick={() => resumeOp(op.id)} title="Resume">
                  <svg viewBox="0 0 24 24" width="12" height="12"><path fill="currentColor" d="M8 5v14l11-7z"/></svg>
                </button>
              {:else}
                <button onclick={() => pauseOp(op.id)} title="Pause">
                  <svg viewBox="0 0 24 24" width="12" height="12"><path fill="currentColor" d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>
                </button>
              {/if}
              <button onclick={() => cancelOp(op.id)} title="Cancel" class="cancel-btn">
                <svg viewBox="0 0 24 24" width="12" height="12"><path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style>
  .op-queue {
    position: fixed;
    bottom: 28px;
    left: 50%;
    transform: translateX(-50%);
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--border-color, #45475a);
    border-bottom: none;
    border-radius: 10px 10px 0 0;
    box-shadow: 0 -4px 20px rgba(0, 0, 0, 0.3);
    z-index: 8000;
    min-width: 320px;
    max-width: 480px;
  }

  .op-queue-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 14px;
    cursor: pointer;
    user-select: none;
  }

  .op-queue-header:hover {
    background: var(--bg-hover, #313244);
  }

  .op-badge {
    background: var(--accent, #89b4fa);
    color: var(--bg-primary, #1e1e2e);
    width: 18px;
    height: 18px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    font-weight: 700;
  }

  .op-label {
    flex: 1;
    font-size: 12px;
    color: var(--text-primary, #cdd6f4);
  }

  .op-chevron {
    transition: transform 0.2s;
    color: var(--text-secondary, #a6adc8);
  }

  .op-list {
    border-top: 1px solid var(--border-color, #45475a);
    max-height: 300px;
    overflow-y: auto;
  }

  .op-item {
    padding: 10px 14px;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .op-item-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 4px;
  }

  .op-type {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary, #cdd6f4);
    text-transform: capitalize;
  }

  .op-progress-text {
    font-size: 11px;
    color: var(--text-secondary, #a6adc8);
  }

  .op-current-file {
    font-size: 10px;
    color: var(--text-secondary, #a6adc8);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 4px;
  }

  .op-progress-bar {
    height: 4px;
    background: var(--bg-tertiary, #45475a);
    border-radius: 2px;
    overflow: hidden;
    margin-bottom: 6px;
  }

  .op-progress-fill {
    height: 100%;
    background: var(--accent, #89b4fa);
    border-radius: 2px;
    transition: width 0.3s ease;
  }

  .op-progress-fill.paused {
    background: var(--warning, #fab387);
  }

  .op-actions {
    display: flex;
    gap: 6px;
    justify-content: flex-end;
  }

  .op-actions button {
    background: var(--bg-tertiary, #45475a);
    border: none;
    color: var(--text-secondary, #a6adc8);
    border-radius: 4px;
    padding: 3px 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
  }

  .op-actions button:hover {
    background: var(--bg-hover, #313244);
    color: var(--text-primary, #cdd6f4);
  }

  .op-actions .cancel-btn:hover {
    color: var(--danger, #f38ba8);
  }
</style>
