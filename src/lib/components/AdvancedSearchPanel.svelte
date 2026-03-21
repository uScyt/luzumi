<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { formatSize, formatDate } from "../types";
  import LoadingSpinner from "./LoadingSpinner.svelte";

  let searchQuery = $state("");
  let contentQuery = $state("");
  let searchMode = $state<"filename" | "content">("filename");
  let recursive = $state(true);

  // Filters
  let filterType = $state<string>("");
  let filterSizeMin = $state("");
  let filterSizeMax = $state("");
  let filterDateAfter = $state("");
  let filterDateBefore = $state("");

  let results = $state<any[]>([]);
  let contentResults = $state<any[]>([]);
  let isSearching = $state(false);
  let searchError = $state<string | null>(null);

  async function doSearch() {
    if (!searchQuery.trim() && searchMode === "filename") return;
    if (!contentQuery.trim() && searchMode === "content") return;

    isSearching = true;
    searchError = null;
    results = [];
    contentResults = [];

    try {
      if (searchMode === "filename") {
        await fm.search.searchAdvanced(
          searchQuery,
          {
            types: filterType ? [filterType] : [],
            sizeMin: filterSizeMin ? parseInt(filterSizeMin) * 1024 : undefined,
            sizeMax: filterSizeMax ? parseInt(filterSizeMax) * 1024 * 1024 : undefined,
            dateAfter: filterDateAfter ? Math.floor(new Date(filterDateAfter).getTime() / 1000) : undefined,
            dateBefore: filterDateBefore ? Math.floor(new Date(filterDateBefore).getTime() / 1000) : undefined,
          },
          fm.currentPath,
          recursive,
        );
        results = fm.search.results;
      } else {
        await fm.search.searchContent(contentQuery, fm.currentPath, recursive);
        contentResults = fm.search.contentResults;
      }
    } catch (e) {
      searchError = String(e);
    } finally {
      isSearching = false;
    }
  }

  function cancelSearch() {
    fm.search.cancelSearch();
  }

  function close() {
    fm.search.showAdvancedPanel = false;
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") doSearch();
    if (e.key === "Escape") close();
  }

  function navigateToResult(path: string) {
    const parent = path.substring(0, path.lastIndexOf("/")) || "/";
    fm.navigate(parent);
    close();
  }

  const typeOptions = [
    { value: "", label: "All types" },
    { value: "image", label: "Images" },
    { value: "video", label: "Videos" },
    { value: "audio", label: "Audio" },
    { value: "document", label: "Documents" },
    { value: "code", label: "Code" },
    { value: "archive", label: "Archives" },
    { value: "directory", label: "Folders" },
  ];
</script>

{#if fm.search.showAdvancedPanel}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="search-overlay" onclick={close}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="search-panel" onclick={(e) => e.stopPropagation()}>
      <div class="search-header">
        <h3>Advanced Search</h3>
        <button class="close-btn" aria-label="Close" onclick={close}>
          <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
        </button>
      </div>

      <div class="search-mode-tabs">
        <button class:active={searchMode === "filename"} onclick={() => searchMode = "filename"}>Filename</button>
        <button class:active={searchMode === "content"} onclick={() => searchMode = "content"}>Content</button>
      </div>

      <div class="search-inputs">
        {#if searchMode === "filename"}
          <input
            bind:value={searchQuery}
            type="text"
            placeholder="Search by filename..."
            onkeydown={onKeydown}
          />
        {:else}
          <input
            bind:value={contentQuery}
            type="text"
            placeholder="Search inside files..."
            onkeydown={onKeydown}
          />
        {/if}
      </div>

      <div class="search-filters">
        <select bind:value={filterType}>
          {#each typeOptions as opt}
            <option value={opt.value}>{opt.label}</option>
          {/each}
        </select>

        <label class="filter-check">
          <input type="checkbox" bind:checked={recursive} />
          Recursive
        </label>
      </div>

      <div class="search-filters-row">
        <input bind:value={filterSizeMin} type="number" placeholder="Min size (KB)" />
        <input bind:value={filterSizeMax} type="number" placeholder="Max size (MB)" />
        <input bind:value={filterDateAfter} type="date" title="Modified after" />
        <input bind:value={filterDateBefore} type="date" title="Modified before" />
      </div>

      <div class="search-actions">
        {#if isSearching}
          <button class="cancel-btn" onclick={cancelSearch}>Cancel</button>
          <LoadingSpinner size={18} />
        {:else}
          <button class="search-btn" onclick={doSearch}>Search</button>
        {/if}
      </div>

      {#if searchError}
        <div class="search-error">{searchError}</div>
      {/if}

      <div class="search-results">
        {#if searchMode === "filename"}
          {#if results.length > 0}
            <div class="result-count">{results.length} result{results.length === 1 ? "" : "s"}</div>
            {#each results as result}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div class="result-item" onclick={() => navigateToResult(result.entry?.path ?? result.path)}>
                <div class="result-name">{result.entry?.name ?? result.name}</div>
                <div class="result-path">{result.entry?.path ?? result.path}</div>
                {#if result.entry?.size != null}
                  <div class="result-meta">{formatSize(result.entry.size)}</div>
                {/if}
              </div>
            {/each}
          {:else if !isSearching && searchQuery}
            <div class="no-results">No results found</div>
          {/if}
        {:else}
          {#if contentResults.length > 0}
            <div class="result-count">{contentResults.length} match{contentResults.length === 1 ? "" : "es"}</div>
            {#each contentResults as match}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div class="result-item content-result" onclick={() => navigateToResult(match.path)}>
                <div class="result-name">{match.path.split("/").pop()}</div>
                <div class="result-line">Line {match.line_number}</div>
                <pre class="result-content">{match.line_text.trim()}</pre>
              </div>
            {/each}
          {:else if !isSearching && contentQuery}
            <div class="no-results">No matches found</div>
          {/if}
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .search-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 9998;
    display: flex;
    justify-content: center;
    padding-top: 8vh;
  }

  .search-panel {
    width: 640px;
    max-height: 75vh;
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 10px;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .search-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .search-header h3 {
    margin: 0;
    font-size: 15px;
    color: var(--text-primary, #cdd6f4);
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
  }

  .close-btn:hover { background: var(--bg-hover, #313244); }

  .search-mode-tabs {
    display: flex;
    padding: 8px 16px 0;
    gap: 4px;
  }

  .search-mode-tabs button {
    background: none;
    border: none;
    color: var(--text-secondary, #a6adc8);
    padding: 6px 16px;
    cursor: pointer;
    border-radius: 6px 6px 0 0;
    font-size: 13px;
  }

  .search-mode-tabs button.active {
    background: var(--bg-tertiary, #45475a);
    color: var(--text-primary, #cdd6f4);
  }

  .search-inputs {
    padding: 10px 16px;
  }

  .search-inputs input {
    width: 100%;
    padding: 8px 12px;
    background: var(--bg-primary, #181825);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 6px;
    color: var(--text-primary, #cdd6f4);
    font-size: 14px;
    outline: none;
    box-sizing: border-box;
  }

  .search-inputs input:focus {
    border-color: var(--accent, #89b4fa);
  }

  .search-filters {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 0 16px 6px;
  }

  .search-filters select {
    background: var(--bg-primary, #181825);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 4px;
    color: var(--text-primary, #cdd6f4);
    padding: 4px 8px;
    font-size: 12px;
  }

  .filter-check {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
  }

  .search-filters-row {
    display: flex;
    gap: 6px;
    padding: 0 16px 10px;
  }

  .search-filters-row input {
    flex: 1;
    padding: 4px 8px;
    background: var(--bg-primary, #181825);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 4px;
    color: var(--text-primary, #cdd6f4);
    font-size: 11px;
    min-width: 0;
  }

  .search-actions {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 0 16px 10px;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .search-btn, .cancel-btn {
    padding: 6px 20px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
  }

  .search-btn {
    background: var(--accent, #89b4fa);
    color: var(--bg-primary, #1e1e2e);
  }

  .cancel-btn {
    background: var(--bg-tertiary, #45475a);
    color: var(--text-primary, #cdd6f4);
  }

  .search-error {
    padding: 8px 16px;
    color: var(--danger, #f38ba8);
    font-size: 12px;
  }

  .search-results {
    flex: 1;
    overflow-y: auto;
    padding: 4px 0;
  }

  .result-count {
    padding: 6px 16px;
    font-size: 11px;
    color: var(--text-secondary, #a6adc8);
  }

  .result-item {
    padding: 8px 16px;
    cursor: pointer;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .result-item:hover {
    background: var(--bg-hover, #313244);
  }

  .result-name {
    font-size: 13px;
    color: var(--text-primary, #cdd6f4);
    font-weight: 500;
  }

  .result-path {
    font-size: 11px;
    color: var(--text-secondary, #a6adc8);
    margin-top: 2px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .result-meta {
    font-size: 11px;
    color: var(--text-secondary, #a6adc8);
    margin-top: 2px;
  }

  .result-line {
    font-size: 11px;
    color: var(--accent, #89b4fa);
    margin-top: 2px;
  }

  .result-content {
    font-size: 11px;
    color: var(--text-primary, #cdd6f4);
    margin: 4px 0 0;
    padding: 4px 8px;
    background: var(--bg-primary, #181825);
    border-radius: 4px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: monospace;
  }

  .no-results {
    padding: 24px 16px;
    text-align: center;
    color: var(--text-secondary, #a6adc8);
    font-size: 13px;
  }
</style>
