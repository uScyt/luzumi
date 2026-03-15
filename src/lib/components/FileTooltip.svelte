<script lang="ts">
  import type { FileEntry } from "../types";
  import { formatSize, getFileIcon, getFileColor } from "../types";
  import { renderIcon } from "../icons";
  import { t } from "../i18n";

  let { entry, x, y }: { entry: FileEntry; x: number; y: number } = $props();

  function formatDate(ts: number | null): string {
    if (!ts) return "—";
    return new Date(ts * 1000).toLocaleString(undefined, {
      year: "numeric", month: "short", day: "numeric",
      hour: "2-digit", minute: "2-digit",
    });
  }

  function permStr(entry: FileEntry): string {
    const parts: string[] = [];
    if (!entry.isWritable) parts.push(t.readOnly);
    if (entry.kind === "symlink") parts.push("Symlink");
    if (entry.isHidden) parts.push(t.hiddenFiles);
    return parts.join(", ") || "—";
  }

  const clampedX = $derived(Math.min(x, window.innerWidth - 260));
  const clampedY = $derived(Math.min(y + 18, window.innerHeight - 140));
</script>

<div
  class="file-tooltip"
  style="left: {clampedX}px; top: {clampedY}px;"
>
  <div class="tooltip-header">
    <span class="tooltip-icon" style="color: {getFileColor(entry)}">
      <svg width="20" height="20" viewBox="0 0 16 16">{@html renderIcon(getFileIcon(entry))}</svg>
    </span>
    <span class="tooltip-name">{entry.name}</span>
  </div>
  <div class="tooltip-rows">
    {#if entry.kind === "file"}
      <div class="tooltip-row">
        <span class="tooltip-label">{t.size}</span>
        <span class="tooltip-value">{formatSize(entry.size ?? 0)}</span>
      </div>
    {/if}
    {#if entry.extension}
      <div class="tooltip-row">
        <span class="tooltip-label">{t.type}</span>
        <span class="tooltip-value">.{entry.extension}</span>
      </div>
    {:else if entry.kind === "directory"}
      <div class="tooltip-row">
        <span class="tooltip-label">{t.type}</span>
        <span class="tooltip-value">{t.directory}</span>
      </div>
    {/if}
    <div class="tooltip-row">
      <span class="tooltip-label">{t.modified}</span>
      <span class="tooltip-value">{formatDate(entry.modified)}</span>
    </div>
    {#if permStr(entry) !== "—"}
      <div class="tooltip-row">
        <span class="tooltip-label">Info</span>
        <span class="tooltip-value">{permStr(entry)}</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .file-tooltip {
    position: fixed;
    z-index: 800;
    min-width: 180px;
    max-width: 260px;
    padding: 10px 12px;
    border-radius: 10px;
    background: var(--surface0);
    border: 1px solid var(--border-medium);
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.45), 0 0 0 0.5px var(--border-light);
    backdrop-filter: blur(16px);
    pointer-events: none;
    animation: tooltip-in 0.15s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes tooltip-in {
    from { opacity: 0; transform: translateY(4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .tooltip-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
    padding-bottom: 7px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .tooltip-icon {
    flex-shrink: 0;
    display: flex;
  }

  .tooltip-name {
    font-size: 12px;
    font-weight: 600;
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .tooltip-rows {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .tooltip-row {
    display: flex;
    justify-content: space-between;
    gap: 12px;
    font-size: 11px;
  }

  .tooltip-label {
    color: var(--overlay1);
    flex-shrink: 0;
  }

  .tooltip-value {
    color: var(--subtext0);
    text-align: right;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
