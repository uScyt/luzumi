<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface Suggestion {
    id: string;
    label: string;
    description: string;
    action: () => void;
  }

  let suggestions = $state<Suggestion[]>([]);
  let dismissed = $state<Set<string>>(new Set());
  let lastAnalyzedPath = $state("");

  $effect(() => {
    const path = fm.currentPath;
    const entriesLen = fm.entries.length;
    if (path && path !== lastAnalyzedPath && entriesLen > 0) {
      lastAnalyzedPath = path;
      analyzeSuggestions(path);
    }
  });

  async function analyzeSuggestions(path: string) {
    try {
      const analysis = await invoke<{ suggested_actions: Array<{ action_type: string; description: string; count: number }> }>("cmd_analyze_directory", { path });
      const newSuggestions: Suggestion[] = [];

      for (const a of analysis.suggested_actions) {
        const id = `${path}:${a.action_type}`;
        if (dismissed.has(id)) continue;

        if (a.action_type === "organize" && a.count > 0) {
          newSuggestions.push({
            id,
            label: "Organize",
            description: a.description,
            action: () => dismiss(id),
          });
        } else if (a.action_type === "duplicates" && a.count > 0) {
          newSuggestions.push({
            id,
            label: "Find Duplicates",
            description: a.description,
            action: () => { fm.showDuplicateFinder = true; dismiss(id); },
          });
        } else if (a.action_type === "large_files" && a.count > 0) {
          newSuggestions.push({
            id,
            label: "Review",
            description: a.description,
            action: () => dismiss(id),
          });
        } else if (a.action_type === "bulk_rename" && a.count > 0) {
          newSuggestions.push({
            id,
            label: "Bulk Rename",
            description: a.description,
            action: () => { fm.showBulkRename = true; dismiss(id); },
          });
        }
      }

      suggestions = newSuggestions.slice(0, 2);
    } catch {
      suggestions = [];
    }
  }

  function dismiss(id: string) {
    dismissed = new Set([...dismissed, id]);
    suggestions = suggestions.filter(s => s.id !== id);
  }

  const visible = $derived(suggestions.length > 0);
</script>

{#if visible}
  <div class="suggestion-bar">
    <svg class="suggestion-icon" viewBox="0 0 24 24" width="14" height="14">
      <path fill="currentColor" d="M9 21c0 .55.45 1 1 1h4c.55 0 1-.45 1-1v-1H9v1zm3-19C8.14 2 5 5.14 5 9c0 2.38 1.19 4.47 3 5.74V17c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2.26c1.81-1.27 3-3.36 3-5.74 0-3.86-3.14-7-7-7z"/>
    </svg>
    {#each suggestions as suggestion (suggestion.id)}
      <div class="suggestion-item">
        <span class="suggestion-text">{suggestion.description}</span>
        <button class="suggestion-action" onclick={suggestion.action}>{suggestion.label}</button>
        <button class="suggestion-dismiss" onclick={() => dismiss(suggestion.id)} aria-label="Dismiss">
          <svg viewBox="0 0 24 24" width="10" height="10"><path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
        </button>
      </div>
    {/each}
  </div>
{/if}

<style>
  .suggestion-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 12px;
    background: color-mix(in srgb, var(--accent, #89b4fa) 8%, transparent);
    border-bottom: 1px solid color-mix(in srgb, var(--accent, #89b4fa) 20%, transparent);
    font-size: 12px;
    flex-shrink: 0;
  }

  .suggestion-icon {
    color: var(--accent, #89b4fa);
    flex-shrink: 0;
  }

  .suggestion-item {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .suggestion-text {
    color: var(--text-secondary, #a6adc8);
  }

  .suggestion-action {
    background: var(--accent, #89b4fa);
    color: var(--bg-primary, #1e1e2e);
    border: none;
    border-radius: 4px;
    padding: 2px 10px;
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
  }

  .suggestion-action:hover {
    opacity: 0.9;
  }

  .suggestion-dismiss {
    background: none;
    border: none;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
    padding: 2px;
    display: flex;
    border-radius: 3px;
  }

  .suggestion-dismiss:hover {
    color: var(--text-primary, #cdd6f4);
    background: var(--bg-hover, #313244);
  }
</style>
