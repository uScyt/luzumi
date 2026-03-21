<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface WorkspaceInfo {
    name: string;
    tabCount: number;
  }

  let workspaces = $state<WorkspaceInfo[]>([]);
  let showDropdown = $state(false);
  let showSaveInput = $state(false);
  let newName = $state("");
  let currentWorkspace = $state<string | null>(null);

  async function loadWorkspaces() {
    try {
      const names = await invoke<string[]>("cmd_list_workspaces");
      // Load each workspace to get tab count
      const infos: WorkspaceInfo[] = [];
      for (const name of names) {
        try {
          const raw = await invoke<string | null>("cmd_load_workspace", { name });
          const parsed = raw ? JSON.parse(raw) : null;
          infos.push({ name, tabCount: parsed?.tabs?.length ?? 0 });
        } catch {
          infos.push({ name, tabCount: 0 });
        }
      }
      workspaces = infos;
    } catch {
      workspaces = [];
    }
  }

  async function saveWorkspace() {
    if (!newName.trim()) return;
    try {
      const data = JSON.stringify({
        tabs: fm.tabs.map(tab => ({
          path: tab.path,
          history: tab.history,
          historyPos: tab.historyPos,
          searchQuery: tab.searchQuery,
        })),
        activeTabIndex: fm.activeTabIndex,
        viewMode: fm.viewMode,
        showHidden: fm.showHidden,
        sortBy: fm.ui.sortBy,
        sortDir: fm.ui.sortDir,
      });
      await invoke("cmd_save_workspace", { name: newName.trim(), data });
      currentWorkspace = newName.trim();
      newName = "";
      showSaveInput = false;
      await loadWorkspaces();
      fm.ui.setStatus(`Workspace "${currentWorkspace}" saved`);
    } catch (e) {
      fm.ui.error = `Failed to save workspace: ${e}`;
    }
  }

  async function loadWorkspace(name: string) {
    try {
      const raw = await invoke<string | null>("cmd_load_workspace", { name });
      if (raw) {
        const ws = JSON.parse(raw);
        if (ws.tabs && ws.tabs.length > 0) {
          await fm.navigate(ws.tabs[0].path);
        }
      }
      currentWorkspace = name;
      showDropdown = false;
      fm.ui.setStatus(`Workspace "${name}" loaded`);
    } catch (e) {
      fm.ui.error = `Failed to load workspace: ${e}`;
    }
  }

  async function deleteWorkspace(name: string) {
    try {
      await invoke("cmd_delete_workspace", { name });
      if (currentWorkspace === name) currentWorkspace = null;
      await loadWorkspaces();
    } catch {}
  }

  function toggleDropdown() {
    showDropdown = !showDropdown;
    if (showDropdown) loadWorkspaces();
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && showSaveInput) saveWorkspace();
    if (e.key === "Escape") { showDropdown = false; showSaveInput = false; }
  }
</script>

<div class="workspace-switcher">
  <button class="workspace-btn" onclick={toggleDropdown} title="Workspaces">
    <svg viewBox="0 0 24 24" width="14" height="14">
      <path fill="currentColor" d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10z"/>
    </svg>
    {#if currentWorkspace}
      <span class="workspace-name">{currentWorkspace}</span>
    {/if}
  </button>

  {#if showDropdown}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="workspace-backdrop" onclick={() => showDropdown = false}></div>
    <div class="workspace-dropdown">
      <div class="workspace-dropdown-header">Workspaces</div>

      {#if workspaces.length > 0}
        {#each workspaces as ws (ws.name)}
          <div class="workspace-item">
            <button class="workspace-item-btn" onclick={() => loadWorkspace(ws.name)}>
              <span class="ws-name">{ws.name}</span>
              <span class="ws-tabs">{ws.tabCount} tab{ws.tabCount !== 1 ? "s" : ""}</span>
            </button>
            <button class="workspace-delete" onclick={() => deleteWorkspace(ws.name)} aria-label="Delete workspace">
              <svg viewBox="0 0 24 24" width="12" height="12"><path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
            </button>
          </div>
        {/each}
      {:else}
        <div class="workspace-empty">No saved workspaces</div>
      {/if}

      <div class="workspace-save-section">
        {#if showSaveInput}
          <input
            bind:value={newName}
            type="text"
            placeholder="Workspace name..."
            onkeydown={onKeydown}
          />
          <button class="workspace-save-btn" onclick={saveWorkspace}>Save</button>
        {:else}
          <button class="workspace-new-btn" onclick={() => showSaveInput = true}>
            + Save Current
          </button>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .workspace-switcher {
    position: relative;
  }

  .workspace-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    background: none;
    border: 1px solid var(--border-color, #45475a);
    color: var(--text-secondary, #a6adc8);
    border-radius: 6px;
    padding: 4px 10px;
    cursor: pointer;
    font-size: 12px;
  }

  .workspace-btn:hover {
    background: var(--bg-hover, #313244);
    color: var(--text-primary, #cdd6f4);
  }

  .workspace-name {
    max-width: 100px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .workspace-backdrop {
    position: fixed;
    inset: 0;
    z-index: 9000;
  }

  .workspace-dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 4px;
    width: 260px;
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 8px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
    z-index: 9001;
    overflow: hidden;
  }

  .workspace-dropdown-header {
    padding: 8px 12px;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-secondary, #a6adc8);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .workspace-item {
    display: flex;
    align-items: center;
  }

  .workspace-item-btn {
    flex: 1;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    background: none;
    border: none;
    color: var(--text-primary, #cdd6f4);
    cursor: pointer;
    font-size: 13px;
    text-align: left;
  }

  .workspace-item-btn:hover {
    background: var(--bg-hover, #313244);
  }

  .ws-tabs {
    font-size: 11px;
    color: var(--text-secondary, #a6adc8);
  }

  .workspace-delete {
    background: none;
    border: none;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
    padding: 4px 8px;
    display: flex;
  }

  .workspace-delete:hover {
    color: var(--danger, #f38ba8);
  }

  .workspace-empty {
    padding: 16px 12px;
    text-align: center;
    font-size: 12px;
    color: var(--text-secondary, #a6adc8);
  }

  .workspace-save-section {
    padding: 8px;
    border-top: 1px solid var(--border-color, #45475a);
    display: flex;
    gap: 6px;
  }

  .workspace-save-section input {
    flex: 1;
    background: var(--bg-primary, #181825);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 4px;
    color: var(--text-primary, #cdd6f4);
    padding: 4px 8px;
    font-size: 12px;
    outline: none;
  }

  .workspace-save-btn {
    background: var(--accent, #89b4fa);
    color: var(--bg-primary, #1e1e2e);
    border: none;
    border-radius: 4px;
    padding: 4px 12px;
    cursor: pointer;
    font-size: 12px;
    font-weight: 500;
  }

  .workspace-new-btn {
    width: 100%;
    background: none;
    border: 1px dashed var(--border-color, #45475a);
    color: var(--text-secondary, #a6adc8);
    border-radius: 4px;
    padding: 6px;
    cursor: pointer;
    font-size: 12px;
  }

  .workspace-new-btn:hover {
    background: var(--bg-hover, #313244);
    color: var(--text-primary, #cdd6f4);
  }
</style>
