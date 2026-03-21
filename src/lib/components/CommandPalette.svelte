<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { commandRegistry, type Command } from "../commandRegistry";
  import { parseNaturalCommand, getSuggestions, type NaturalAction } from "../naturalCommand";
  import { t } from "../i18n";

  let query = $state("");
  let selectedIndex = $state(0);
  let inputEl: HTMLInputElement | undefined = $state();

  const results = $derived.by(() => {
    if (!query.trim()) {
      const recent = commandRegistry.getRecent();
      if (recent.length > 0) return { recent: true, commands: recent, naturalAction: null, suggestions: [] as string[] };
      return { recent: false, commands: commandRegistry.getAll(), naturalAction: null, suggestions: [] as string[] };
    }
    const commands = commandRegistry.search(query);
    const naturalAction = commands.length === 0 ? parseNaturalCommand(query) : null;
    const suggestions = commands.length === 0 && !naturalAction ? getSuggestions(query) : [];
    return { recent: false, commands, naturalAction, suggestions };
  });

  function close() {
    fm.ui.showCommandPalette = false;
    query = "";
    selectedIndex = 0;
  }

  async function execute(cmd: Command) {
    close();
    await commandRegistry.execute(cmd.id);
  }

  async function executeNatural(action: NaturalAction) {
    close();
    switch (action.action) {
      case "sort":
        if (action.sortBy === "name" || action.sortBy === "size" || action.sortBy === "date" || action.sortBy === "type") {
          fm.setSort(action.sortBy);
        }
        break;
      case "find":
        fm.showDuplicateFinder = true;
        break;
      case "filter":
        if (action.filter) {
          fm.searchQuery = action.filter;
        } else if (action.dateAfter) {
          fm.search.showAdvancedPanel = true;
        }
        break;
      case "delete":
        fm.search.showAdvancedPanel = true;
        break;
      default:
        fm.setStatus(`Action: ${action.description}`);
        break;
    }
  }

  function applySuggestion(suggestion: string) {
    query = suggestion;
    selectedIndex = 0;
  }

  function onKeydown(e: KeyboardEvent) {
    const cmds = results.commands;
    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, cmds.length - 1);
        break;
      case "ArrowUp":
        e.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        break;
      case "Enter":
        e.preventDefault();
        if (cmds[selectedIndex]) {
          execute(cmds[selectedIndex]);
        } else if (results.naturalAction) {
          executeNatural(results.naturalAction);
        }
        break;
      case "Escape":
        e.preventDefault();
        close();
        break;
    }
  }

  $effect(() => {
    if (fm.ui.showCommandPalette && inputEl) {
      inputEl.focus();
    }
  });

  $effect(() => {
    // Reset selection when results change
    query;
    selectedIndex = 0;
  });

  const categoryLabels: Record<string, string> = {
    navigation: "Navigation",
    file: "File",
    edit: "Edit",
    view: "View",
    tools: "Tools",
    settings: "Settings",
    help: "Help",
  };
</script>

{#if fm.ui.showCommandPalette}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="command-palette-overlay" onclick={close}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="command-palette" onclick={(e) => e.stopPropagation()}>
      <div class="palette-input-wrapper">
        <svg class="palette-icon" viewBox="0 0 24 24" width="16" height="16">
          <path fill="currentColor" d="M15.5 14h-.79l-.28-.27A6.47 6.47 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
        </svg>
        <input
          bind:this={inputEl}
          bind:value={query}
          type="text"
          placeholder="Type a command..."
          spellcheck="false"
          autocomplete="off"
          onkeydown={onKeydown}
        />
      </div>

      <div class="palette-results">
        {#if results.recent && results.commands.length > 0}
          <div class="palette-section-label">Recent</div>
        {/if}

        {#each results.commands as cmd, i (cmd.id)}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="palette-item"
            class:selected={i === selectedIndex}
            onclick={() => execute(cmd)}
            onmouseenter={() => selectedIndex = i}
          >
            <span class="palette-item-category">{categoryLabels[cmd.category] ?? cmd.category}</span>
            <span class="palette-item-label">{cmd.label}</span>
            {#if cmd.shortcut}
              <span class="palette-item-shortcut">{cmd.shortcut}</span>
            {/if}
          </div>
        {/each}

        {#if results.commands.length === 0 && results.naturalAction}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div class="palette-item selected" onclick={() => results.naturalAction && executeNatural(results.naturalAction)}>
            <span class="palette-item-category">Action</span>
            <span class="palette-item-label">{results.naturalAction.description}</span>
            <span class="palette-item-shortcut">↵</span>
          </div>
        {:else if results.commands.length === 0 && results.suggestions.length > 0}
          <div class="palette-section-label">Try saying...</div>
          {#each results.suggestions as suggestion}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="palette-item" onclick={() => applySuggestion(suggestion)}>
              <span class="palette-item-category">Hint</span>
              <span class="palette-item-label suggestion">{suggestion}</span>
            </div>
          {/each}
        {:else if results.commands.length === 0}
          <div class="palette-empty">No commands found</div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .command-palette-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 9999;
    display: flex;
    justify-content: center;
    padding-top: 15vh;
  }

  .command-palette {
    width: 560px;
    max-height: 420px;
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 10px;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .palette-input-wrapper {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color, #45475a);
    gap: 10px;
  }

  .palette-icon {
    color: var(--text-secondary, #a6adc8);
    flex-shrink: 0;
  }

  .palette-input-wrapper input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary, #cdd6f4);
    font-size: 15px;
    font-family: inherit;
  }

  .palette-input-wrapper input::placeholder {
    color: var(--text-secondary, #a6adc8);
  }

  .palette-results {
    overflow-y: auto;
    flex: 1;
    padding: 4px 0;
  }

  .palette-section-label {
    padding: 6px 16px 4px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-secondary, #a6adc8);
  }

  .palette-item {
    display: flex;
    align-items: center;
    padding: 8px 16px;
    cursor: pointer;
    gap: 10px;
  }

  .palette-item.selected {
    background: var(--bg-hover, #313244);
  }

  .palette-item-category {
    font-size: 11px;
    color: var(--accent, #89b4fa);
    min-width: 70px;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    font-weight: 500;
  }

  .palette-item-label {
    flex: 1;
    color: var(--text-primary, #cdd6f4);
    font-size: 13px;
  }

  .palette-item-shortcut {
    font-size: 11px;
    color: var(--text-secondary, #a6adc8);
    background: var(--bg-tertiary, #45475a);
    padding: 2px 6px;
    border-radius: 4px;
    font-family: monospace;
  }

  .suggestion {
    font-style: italic;
    opacity: 0.7;
  }

  .palette-empty {
    padding: 24px 16px;
    text-align: center;
    color: var(--text-secondary, #a6adc8);
    font-size: 13px;
  }
</style>
