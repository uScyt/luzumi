<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let { filePath }: { filePath: string } = $props();

  let pageImage = $state("");
  let currentPage = $state(1);
  let totalPages = $state(0);
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    if (filePath) {
      currentPage = 1;
      loadPageCount(filePath);
    }
  });

  $effect(() => {
    if (filePath && totalPages > 0) {
      loadPage(filePath, currentPage);
    }
  });

  async function loadPageCount(path: string) {
    try {
      totalPages = await invoke<number>("cmd_get_pdf_page_count", { path });
      if (totalPages > 0) await loadPage(path, 1);
    } catch (e) {
      error = `PDF preview requires poppler-utils: ${e}`;
      loading = false;
    }
  }

  async function loadPage(path: string, page: number) {
    loading = true;
    error = null;
    try {
      pageImage = await invoke<string>("cmd_read_pdf_page", { path, page });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function prevPage() {
    if (currentPage > 1) currentPage--;
  }

  function nextPage() {
    if (currentPage < totalPages) currentPage++;
  }
</script>

<div class="pdf-preview">
  {#if error}
    <div class="pdf-error">{error}</div>
  {:else}
    {#if totalPages > 0}
      <div class="pdf-toolbar">
        <button aria-label="Previous page" onclick={prevPage} disabled={currentPage <= 1}>
          <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z"/></svg>
        </button>
        <span class="pdf-page-info">{currentPage} / {totalPages}</span>
        <button aria-label="Next page" onclick={nextPage} disabled={currentPage >= totalPages}>
          <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/></svg>
        </button>
      </div>
    {/if}
    <div class="pdf-content">
      {#if loading}
        <div class="pdf-loading">Loading page...</div>
      {:else if pageImage}
        <img src={pageImage} alt="PDF page {currentPage}" class="pdf-page" />
      {/if}
    </div>
  {/if}
</div>

<style>
  .pdf-preview {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .pdf-toolbar {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 8px;
    border-bottom: 1px solid var(--border-color, #45475a);
    flex-shrink: 0;
  }

  .pdf-toolbar button {
    background: var(--bg-tertiary, #45475a);
    border: none;
    color: var(--text-primary, #cdd6f4);
    border-radius: 4px;
    padding: 4px 8px;
    cursor: pointer;
    display: flex;
    align-items: center;
  }

  .pdf-toolbar button:disabled {
    opacity: 0.3;
    cursor: default;
  }

  .pdf-toolbar button:not(:disabled):hover {
    background: var(--bg-hover, #313244);
  }

  .pdf-page-info {
    font-size: 12px;
    color: var(--text-secondary, #a6adc8);
    min-width: 60px;
    text-align: center;
  }

  .pdf-content {
    flex: 1;
    overflow: auto;
    display: flex;
    justify-content: center;
    padding: 12px;
  }

  .pdf-page {
    max-width: 100%;
    height: auto;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
    border-radius: 2px;
  }

  .pdf-loading, .pdf-error {
    padding: 24px;
    text-align: center;
    color: var(--text-secondary, #a6adc8);
    font-size: 13px;
  }

  .pdf-error { color: var(--danger, #f38ba8); }
</style>
