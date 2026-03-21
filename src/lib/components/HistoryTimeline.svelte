<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import type { Snippet } from "svelte";

  let { direction, children }: { direction: "back" | "forward"; children: Snippet } = $props();
  let showDropdown = $state(false);

  const items = $derived.by(() => {
    const hist = fm.history;
    const pos = fm.historyPos;
    if (direction === "back") {
      return hist.slice(0, pos).reverse().slice(0, 15);
    } else {
      return hist.slice(pos + 1).slice(0, 15);
    }
  });

  function navigate(path: string) {
    showDropdown = false;
    fm.navigate(path);
  }

  function baseName(path: string): string {
    if (path === "/") return "/";
    const parts = path.split("/");
    return parts[parts.length - 1] || parts[parts.length - 2] || path;
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="history-timeline" onmouseleave={() => showDropdown = false}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="history-trigger"
    oncontextmenu={(e) => { e.preventDefault(); showDropdown = !showDropdown; }}
    onclick={() => { if (items.length > 0) showDropdown = !showDropdown; }}
  >
    {@render children()}
  </div>

  {#if showDropdown && items.length > 0}
    <div class="history-dropdown">
      {#each items as path, i (i)}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="history-item" onclick={() => navigate(path)}>
          <span class="history-name">{baseName(path)}</span>
          <span class="history-path">{path}</span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .history-timeline {
    position: relative;
    display: inline-flex;
  }

  .history-trigger {
    display: inline-flex;
  }

  .history-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    margin-top: 4px;
    min-width: 240px;
    max-width: 400px;
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 8px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
    z-index: 9001;
    overflow: hidden;
    max-height: 320px;
    overflow-y: auto;
  }

  .history-item {
    display: flex;
    flex-direction: column;
    padding: 6px 12px;
    cursor: pointer;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .history-item:last-child {
    border-bottom: none;
  }

  .history-item:hover {
    background: var(--bg-hover, #313244);
  }

  .history-name {
    font-size: 13px;
    color: var(--text-primary, #cdd6f4);
    font-weight: 500;
  }

  .history-path {
    font-size: 10px;
    color: var(--text-secondary, #a6adc8);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
