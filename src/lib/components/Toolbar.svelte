<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { t } from "../i18n";

  let editingPath = $state(false);
  let pathBuffer = $state("");
  let searchInputEl: HTMLInputElement | null = null;

  function startEditPath() {
    pathBuffer = fm.currentPath;
    editingPath = true;
  }

  $effect(() => {
    if (fm.focusAddressBar) {
      fm.focusAddressBar = false;
      startEditPath();
    }
  });

  $effect(() => {
    if (fm.focusSearch) {
      fm.focusSearch = false;
      requestAnimationFrame(() => {
        searchInputEl?.focus();
        searchInputEl?.select();
      });
    }
  });

  function commitPath(e: Event) {
    e.preventDefault();
    editingPath = false;
    if (pathBuffer && pathBuffer !== fm.currentPath) {
      fm.navigate(pathBuffer);
    }
  }

  function cancelEditPath() {
    editingPath = false;
  }

  const crumbs = $derived(() => {
    const parts = fm.currentPath.split("/").filter(Boolean);
    return [
      { label: "/", path: "/" },
      ...parts.map((part, i) => ({
        label: part,
        path: "/" + parts.slice(0, i + 1).join("/"),
      })),
    ];
  });
</script>

<nav class="toolbar">
  <!-- Nav buttons -->
  <div class="nav-buttons">
    <button
      class="tool-btn"
      disabled={!fm.canGoBack}
      title="{t.back} (Alt+Left)"
      onclick={() => fm.goBack()}
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M10 3L5 8L10 13" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
    <button
      class="tool-btn"
      disabled={!fm.canGoForward}
      title="{t.forward} (Alt+Right)"
      onclick={() => fm.goForward()}
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M6 3L11 8L6 13" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
    <button
      class="tool-btn"
      disabled={fm.currentPath === "/"}
      title="{t.up} (Alt+Up)"
      onclick={() => fm.goUp()}
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M8 12V4M8 4L4 8M8 4L12 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
    <button class="tool-btn" title="{t.reload} (F5)" onclick={() => fm.reload()}>
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M2.5 8A5.5 5.5 0 1 0 4 4.5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
        <path d="M2.5 2V4.5H5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
  </div>

  <!-- Path bar -->
  <div class="path-area">
    {#if editingPath}
      <form onsubmit={commitPath}>
        <input
          class="path-input"
          bind:value={pathBuffer}
          onblur={cancelEditPath}
          onkeydown={(e) => { if (e.key === "Escape") cancelEditPath(); }}
          use:focusAndSelect
        />
      </form>
    {:else}
      <div
        class="breadcrumb"
        role="button"
        tabindex="0"
        ondblclick={startEditPath}
        onkeydown={(e) => { if (e.key === "Enter") startEditPath(); }}
        ondragover={(e) => { e.preventDefault(); if (e.dataTransfer) e.dataTransfer.dropEffect = "move"; }}
        ondrop={(e) => { e.preventDefault(); fm.requestDrop(fm.currentPath); }}
      >
        {#each crumbs() as crumb, i}
          {#if i > 0}<span class="sep">/</span>{/if}
          <button
            class="crumb"
            class:active={crumb.path === fm.currentPath}
            onclick={() => fm.navigate(crumb.path)}
            ondragover={(e) => {
              e.preventDefault();
              e.stopPropagation();
              if (e.dataTransfer) e.dataTransfer.dropEffect = "move";
              fm.dropTarget = crumb.path;
            }}
            ondragleave={() => { if (fm.dropTarget === crumb.path) fm.dropTarget = null; }}
            ondrop={(e) => {
              e.preventDefault();
              e.stopPropagation();
              fm.requestDrop(crumb.path);
            }}
            class:drop-target={fm.dropTarget === crumb.path}
          >{crumb.label}</button>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Search + hidden toggle -->
  <div class="right-actions">
    <div class="search-wrap">
      <svg class="search-icon" width="13" height="13" viewBox="0 0 16 16" fill="none">
        <circle cx="7" cy="7" r="5" stroke="currentColor" stroke-width="1.8"/>
        <path d="M11 11L14 14" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
      </svg>
      <input
        class="search-input"
        type="search"
        placeholder="Search..."
        bind:value={fm.searchQuery}
        bind:this={searchInputEl}
        oninput={() => fm.triggerGlobalSearch(fm.searchQuery)}
      />
      {#if fm.searchQuery}
        {@const found = fm.filteredEntries().length}
        {@const total = fm.entries.length}
        {@const global = fm.globalSearchResults.length}
        <span class="search-count">{found}/{total}{#if global > 0} +{global}{/if}</span>
      {/if}
    </div>
    <select
      class="type-filter"
      value={fm.typeFilter ?? ""}
      onchange={(e) => { fm.typeFilter = (e.target as HTMLSelectElement).value || null; }}
      title="Filter by type"
    >
      <option value="">All</option>
      <option value="directory">Folders</option>
      <option value="image">Images</option>
      <option value="video">Videos</option>
      <option value="audio">Audio</option>
      <option value="document">Documents</option>
      <option value="code">Code</option>
      <option value="archive">Archives</option>
    </select>
    <button
      class="tool-btn"
      class:active={fm.showPreview}
      title="{t.previewPane} (Ctrl+P)"
      onclick={() => fm.showPreview = !fm.showPreview}
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <rect x="2" y="2" width="12" height="12" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/>
        <path d="M10 2V14" stroke="currentColor" stroke-width="1.4"/>
      </svg>
    </button>
    <button
      class="tool-btn"
      class:active={fm.showSettings}
      title={t.settingsShortcut}
      onclick={() => fm.showSettings = !fm.showSettings}
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M6.3 2L6.7 3.8C6.2 4 5.7 4.3 5.3 4.7L3.6 4.1L2.3 6.4L3.7 7.5C3.6 7.7 3.6 7.8 3.6 8C3.6 8.2 3.6 8.3 3.7 8.5L2.3 9.6L3.6 11.9L5.3 11.3C5.7 11.7 6.2 12 6.7 12.2L6.3 14H9.7L10.1 12.2C10.6 12 11.1 11.7 11.5 11.3L13.2 11.9L14.5 9.6L13.1 8.5C13.2 8.3 13.2 8.2 13.2 8C13.2 7.8 13.2 7.7 13.1 7.5L14.5 6.4L13.2 4.1L11.5 4.7C11.1 4.3 10.6 4 10.1 3.8L9.7 2Z" stroke="currentColor" stroke-width="1.35" stroke-linejoin="round"/>
        <circle cx="8" cy="8" r="2" stroke="currentColor" stroke-width="1.35" fill="none"/>
      </svg>
    </button>
  </div>
</nav>

<script lang="ts" module>
  function focusAndSelect(node: HTMLInputElement) {
    node.focus();
    node.select();
  }
</script>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 12px;
    background: var(--app-bg);
    height: var(--toolbar-h);
  }

  .nav-buttons {
    display: flex;
    gap: 2px;
    flex-shrink: 0;
  }

  .tool-btn {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--subtext0);
    transition: background 0.1s, color 0.1s;
    flex-shrink: 0;
  }

  .tool-btn:hover:not(:disabled) {
    background: var(--hover-bg);
    color: var(--text);
  }

  .tool-btn:active:not(:disabled) {
    background: var(--hover-bg-strong);
  }

  .tool-btn.active {
    color: var(--mauve);
  }

  .tool-btn:disabled {
    color: var(--overlay0);
    cursor: default;
    opacity: 0.45;
  }

  .path-area {
    flex: 1;
    min-width: 0;
    height: 34px;
  }

  .breadcrumb {
    display: flex;
    align-items: center;
    gap: 1px;
    height: 100%;
    padding: 0 12px;
    border-radius: 8px;
    background: var(--input-bg);
    border: 1px solid var(--border-subtle);
    backdrop-filter: blur(6px);
    overflow: hidden;
    cursor: text;
    transition: background 0.12s, border-color 0.12s;
  }

  .breadcrumb:hover {
    background: var(--input-bg-focus);
    border-color: var(--border-medium);
  }

  .sep {
    color: var(--overlay0);
    font-size: 12px;
    padding: 0 2px;
    opacity: 0.6;
  }

  .crumb {
    font-size: 13px;
    color: var(--subtext0);
    padding: 3px 6px;
    border-radius: 5px;
    white-space: nowrap;
    max-width: 140px;
    overflow: hidden;
    text-overflow: ellipsis;
    transition: color 0.1s, background 0.1s;
  }

  .crumb:hover {
    color: var(--text);
    background: var(--border-light);
  }

  .crumb.active {
    color: var(--text);
    font-weight: 500;
  }

  .crumb.drop-target {
    background: var(--blue-bg-strong);
    color: var(--blue);
    box-shadow: 0 0 0 1px var(--blue-border-strong);
  }

  .path-input {
    width: 100%;
    height: 100%;
    background: var(--input-bg-focus);
    border: 1.5px solid var(--mauve);
    border-radius: 8px;
    padding: 0 12px;
    color: var(--text);
    font-size: 13px;
    outline: none;
  }

  form {
    height: 100%;
  }

  .right-actions {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
  }

  .search-wrap {
    position: relative;
    display: flex;
    align-items: center;
  }

  .type-filter {
    appearance: none;
    -webkit-appearance: none;
    background: var(--input-bg);
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    color: var(--subtext0);
    font-size: 12px;
    padding: 6px 24px 6px 8px;
    cursor: pointer;
    outline: none;
    height: 32px;
    background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M1 1L5 5L9 1' stroke='%238087a2' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 8px center;
    transition: border-color 0.15s, background 0.15s, color 0.15s;
  }

  .type-filter:hover {
    background: var(--input-bg-focus);
    border-color: var(--border-medium);
    color: var(--text);
  }

  .type-filter:focus {
    border-color: var(--accent-bg);
    background: var(--input-bg-focus);
  }

  .type-filter option {
    background: var(--surface0, #363a4f);
    color: var(--text);
    padding: 6px 8px;
  }

  .search-icon {
    position: absolute;
    left: 10px;
    z-index: 1;
    color: var(--overlay1);
    pointer-events: none;
  }

  .search-input {
    width: 180px;
    height: 32px;
    background: var(--input-bg);
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    padding: 0 10px 0 32px;
    color: var(--text);
    font-size: 13px;
    outline: none;
    backdrop-filter: blur(6px);
    transition: border-color 0.15s, background 0.15s, width 0.2s;
  }

  .search-input::placeholder {
    color: var(--overlay0);
  }

  .search-input:focus {
    border-color: var(--accent-bg);
    background: var(--input-bg-focus);
    width: 220px;
  }

  .search-input::-webkit-search-cancel-button {
    display: none;
  }

  .search-count {
    position: absolute;
    right: 8px;
    font-size: 10px;
    color: var(--overlay1);
    pointer-events: none;
    font-variant-numeric: tabular-nums;
  }

</style>
