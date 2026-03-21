<script lang="ts">
  import { fm } from "../../fileManager.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import TagBadge from "../TagBadge.svelte";

  interface Tag {
    id: number;
    name: string;
    color: string;
  }

  let tags = $state<Tag[]>([]);
  let fileTags = $state<Tag[]>([]);
  let newTagName = $state("");
  let newTagColor = $state("#89b4fa");
  let editingTag = $state<Tag | null>(null);
  let loading = $state(true);

  const selectedPaths = $derived([...fm.selected]);
  const singleFile = $derived(selectedPaths.length === 1 ? selectedPaths[0] : null);

  $effect(() => {
    if (fm.ui.showTagManager) {
      loadTags();
      if (singleFile) loadFileTags(singleFile);
    }
  });

  async function loadTags() {
    loading = true;
    try {
      tags = await invoke<Tag[]>("cmd_list_tags");
    } catch {}
    loading = false;
  }

  async function loadFileTags(path: string) {
    try {
      fileTags = await invoke<Tag[]>("cmd_get_file_tags", { path });
    } catch {
      fileTags = [];
    }
  }

  async function createTag() {
    if (!newTagName.trim()) return;
    try {
      await invoke("cmd_create_tag", { name: newTagName.trim(), color: newTagColor });
      newTagName = "";
      await loadTags();
    } catch (e) {
      fm.ui.error = `Failed to create tag: ${e}`;
    }
  }

  async function deleteTag(id: number) {
    try {
      await invoke("cmd_delete_tag", { id });
      await loadTags();
      if (singleFile) await loadFileTags(singleFile);
    } catch (e) {
      fm.ui.error = `Failed to delete tag: ${e}`;
    }
  }

  async function toggleFileTag(tag: Tag) {
    if (!singleFile) return;
    const hasTag = fileTags.some(t => t.id === tag.id);
    try {
      if (hasTag) {
        await invoke("cmd_untag_file", { path: singleFile, tagId: tag.id });
      } else {
        await invoke("cmd_tag_file", { path: singleFile, tagId: tag.id });
      }
      await loadFileTags(singleFile);
    } catch (e) {
      fm.ui.error = `Failed to update tag: ${e}`;
    }
  }

  async function tagMultipleFiles(tag: Tag) {
    for (const path of selectedPaths) {
      try {
        await invoke("cmd_tag_file", { path, tagId: tag.id });
      } catch {}
    }
    fm.ui.setStatus(`Tagged ${selectedPaths.length} files with "${tag.name}"`);
  }

  function close() {
    fm.ui.showTagManager = false;
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") createTag();
    if (e.key === "Escape") close();
  }

  const presetColors = ["#f38ba8", "#fab387", "#f9e2af", "#a6e3a1", "#89b4fa", "#cba6f7", "#f5c2e7", "#94e2d5"];
</script>

{#if fm.ui.showTagManager}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="tag-overlay" onclick={close}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="tag-dialog" onclick={(e) => e.stopPropagation()}>
      <div class="tag-header">
        <h3>Tags</h3>
        <button class="close-btn" onclick={close} aria-label="Close">
          <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
        </button>
      </div>

      {#if singleFile}
        <div class="file-tags-section">
          <div class="section-label">Tags for: {singleFile.split("/").pop()}</div>
          <div class="file-tags">
            {#if fileTags.length > 0}
              {#each fileTags as tag (tag.id)}
                <TagBadge name={tag.name} color={tag.color} removable onremove={() => toggleFileTag(tag)} />
              {/each}
            {:else}
              <span class="no-tags">No tags assigned</span>
            {/if}
          </div>
        </div>
      {/if}

      <div class="section-label">All Tags</div>
      <div class="tag-list">
        {#if loading}
          <div class="tag-loading">Loading...</div>
        {:else if tags.length === 0}
          <div class="tag-empty">No tags created yet</div>
        {:else}
          {#each tags as tag (tag.id)}
            <div class="tag-row">
              <div class="tag-row-left" onclick={() => singleFile ? toggleFileTag(tag) : tagMultipleFiles(tag)} role="button" tabindex="0" onkeydown={(e) => e.key === "Enter" && (singleFile ? toggleFileTag(tag) : tagMultipleFiles(tag))}>
                <span class="tag-color-dot" style="background: {tag.color}"></span>
                <span class="tag-label">{tag.name}</span>
                {#if fileTags.some(t => t.id === tag.id)}
                  <svg viewBox="0 0 24 24" width="14" height="14" class="tag-check"><path fill="currentColor" d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/></svg>
                {/if}
              </div>
              <button class="tag-delete" onclick={() => deleteTag(tag.id)} aria-label="Delete tag">
                <svg viewBox="0 0 24 24" width="12" height="12"><path fill="currentColor" d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/></svg>
              </button>
            </div>
          {/each}
        {/if}
      </div>

      <div class="create-section">
        <div class="create-row">
          <input
            bind:value={newTagName}
            type="text"
            placeholder="New tag name..."
            onkeydown={onKeydown}
          />
          <input type="color" bind:value={newTagColor} class="color-picker" title="Tag color" />
          <button class="create-btn" onclick={createTag} disabled={!newTagName.trim()}>Add</button>
        </div>
        <div class="preset-colors">
          {#each presetColors as color}
            <button
              class="preset-color"
              class:active={newTagColor === color}
              style="background: {color}"
              onclick={() => newTagColor = color}
              aria-label="Color {color}"
            ></button>
          {/each}
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .tag-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 10000;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .tag-dialog {
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 12px;
    width: 400px;
    max-height: 70vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.5);
  }

  .tag-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .tag-header h3 { margin: 0; font-size: 16px; color: var(--text-primary, #cdd6f4); }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
  }

  .close-btn:hover { background: var(--bg-hover, #313244); }

  .file-tags-section {
    padding: 12px 20px;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .file-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 6px;
  }

  .no-tags {
    font-size: 12px;
    color: var(--text-secondary, #a6adc8);
  }

  .section-label {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-secondary, #a6adc8);
    padding: 10px 20px 4px;
    font-weight: 600;
  }

  .tag-list {
    flex: 1;
    overflow-y: auto;
    padding: 4px 12px;
  }

  .tag-row {
    display: flex;
    align-items: center;
    padding: 6px 8px;
    border-radius: 6px;
  }

  .tag-row:hover { background: var(--bg-hover, #313244); }

  .tag-row-left {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
  }

  .tag-color-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .tag-label {
    font-size: 13px;
    color: var(--text-primary, #cdd6f4);
    flex: 1;
  }

  .tag-check { color: var(--accent, #89b4fa); flex-shrink: 0; }

  .tag-delete {
    background: none;
    border: none;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    opacity: 0;
  }

  .tag-row:hover .tag-delete { opacity: 1; }
  .tag-delete:hover { color: var(--danger, #f38ba8); }

  .tag-loading, .tag-empty {
    padding: 20px;
    text-align: center;
    font-size: 13px;
    color: var(--text-secondary, #a6adc8);
  }

  .create-section {
    padding: 12px 20px 16px;
    border-top: 1px solid var(--border-color, #45475a);
  }

  .create-row {
    display: flex;
    gap: 6px;
  }

  .create-row input[type="text"] {
    flex: 1;
    background: var(--bg-primary, #181825);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 6px;
    color: var(--text-primary, #cdd6f4);
    padding: 6px 10px;
    font-size: 13px;
    outline: none;
  }

  .create-row input[type="text"]:focus {
    border-color: var(--accent, #89b4fa);
  }

  .color-picker {
    width: 32px;
    height: 32px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    padding: 0;
  }

  .create-btn {
    background: var(--accent, #89b4fa);
    color: var(--bg-primary, #1e1e2e);
    border: none;
    border-radius: 6px;
    padding: 6px 16px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
  }

  .create-btn:disabled { opacity: 0.4; cursor: default; }

  .preset-colors {
    display: flex;
    gap: 4px;
    margin-top: 8px;
  }

  .preset-color {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: 2px solid transparent;
    cursor: pointer;
    padding: 0;
  }

  .preset-color.active {
    border-color: var(--text-primary, #cdd6f4);
  }

  .preset-color:hover {
    transform: scale(1.15);
  }
</style>
