<script lang="ts">
  import { fm } from "../../fileManager.svelte";
  import { t } from "../../i18n";
  import { invoke } from "@tauri-apps/api/core";

  type Mode = "findReplace" | "numbering" | "case";

  let mode = $state<Mode>("findReplace");
  let findText = $state("");
  let replaceText = $state("");
  let startAt = $state(1);
  let caseMode = $state<"lower" | "upper" | "title">("lower");

  const selectedEntries = $derived(
    fm.entries.filter(e => fm.selected.has(e.path))
  );

  const previews = $derived(computePreviews());

  function computePreviews(): Array<{ original: string; renamed: string; path: string }> {
    return selectedEntries.map((entry, i) => {
      const name = entry.name;
      let renamed = name;

      if (mode === "findReplace" && findText) {
        renamed = name.replaceAll(findText, replaceText);
      } else if (mode === "numbering") {
        const dot = name.lastIndexOf(".");
        const base = dot > 0 ? name.slice(0, dot) : name;
        const ext = dot > 0 ? name.slice(dot) : "";
        const num = String(startAt + i).padStart(3, "0");
        renamed = `${base}_${num}${ext}`;
      } else if (mode === "case") {
        if (caseMode === "lower") renamed = name.toLowerCase();
        else if (caseMode === "upper") renamed = name.toUpperCase();
        else renamed = name.replace(/\b\w/g, c => c.toUpperCase()).replace(/\B\w/g, c => c.toLowerCase());
      }

      return { original: name, renamed, path: entry.path };
    });
  }

  const hasChanges = $derived(previews.some(p => p.original !== p.renamed));

  async function apply() {
    const toRename = previews.filter(p => p.original !== p.renamed);
    if (toRename.length === 0) return;

    for (const item of toRename) {
      try {
        await invoke("cmd_rename_entry", { from: item.path, newName: item.renamed });
      } catch (e) {
        fm.error = String(e);
        break;
      }
    }

    fm.showBulkRename = false;
    fm.selected = new Set();
    await fm.reload();
    fm.setStatus(t.renamedItems(toRename.length));
  }

  function cancel() {
    fm.showBulkRename = false;
  }
</script>

<div class="overlay" role="dialog" aria-modal="true" tabindex="-1" onclick={cancel} onkeydown={(e) => { if (e.key === "Escape") cancel(); }}>
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div class="dialog" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
    <div class="dialog-header">
      <div class="icon-wrap">
        <svg width="17" height="17" viewBox="0 0 16 16" fill="none">
          <path d="M3 12L10 5L13 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M10 5L11.5 3.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </div>
      <h2>{t.bulkRename}</h2>
      <span class="count-badge">{selectedEntries.length}</span>
    </div>

    <div class="dialog-sep"></div>

    <div class="tabs">
      <button class="tab" class:active={mode === "findReplace"} onclick={() => mode = "findReplace"}>{t.findReplace}</button>
      <button class="tab" class:active={mode === "numbering"} onclick={() => mode = "numbering"}>{t.addNumbering}</button>
      <button class="tab" class:active={mode === "case"} onclick={() => mode = "case"}>{t.changeCase}</button>
    </div>

    <div class="mode-body">
      {#if mode === "findReplace"}
        <label class="field-label" for="bulk-find">{t.find}</label>
        <input id="bulk-find" class="field-input" bind:value={findText} placeholder={t.pattern} />
        <label class="field-label" for="bulk-replace">{t.replaceWith}</label>
        <input id="bulk-replace" class="field-input" bind:value={replaceText} />
      {:else if mode === "numbering"}
        <label class="field-label" for="bulk-start">{t.startAt}</label>
        <input id="bulk-start" class="field-input" type="number" min="0" bind:value={startAt} />
      {:else}
        <div class="case-options">
          <button class="case-btn" class:active={caseMode === "lower"} onclick={() => caseMode = "lower"}>{t.lowercase}</button>
          <button class="case-btn" class:active={caseMode === "upper"} onclick={() => caseMode = "upper"}>{t.uppercase}</button>
          <button class="case-btn" class:active={caseMode === "title"} onclick={() => caseMode = "title"}>{t.titlecase}</button>
        </div>
      {/if}
    </div>

    <div class="dialog-sep"></div>

    <div class="preview-label">{t.preview}</div>
    <div class="preview-list">
      {#each previews as p}
        <div class="preview-row" class:changed={p.original !== p.renamed}>
          <span class="preview-old">{p.original}</span>
          <svg width="10" height="10" viewBox="0 0 16 16" fill="none" class="arrow">
            <path d="M3 8H13M13 8L9 4M13 8L9 12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <span class="preview-new">{p.renamed}</span>
        </div>
      {/each}
    </div>

    <div class="actions">
      <button class="btn-cancel" onclick={cancel}>{t.cancel}</button>
      <button class="btn-apply" disabled={!hasChanges} onclick={apply}>
        {t.apply}
      </button>
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 600;
    animation: fade 0.12s ease;
  }

  @keyframes fade { from { opacity: 0; } to { opacity: 1; } }

  .dialog {
    background: var(--glass-bg-strong);
    backdrop-filter: blur(30px) saturate(1.5);
    border: 1px solid var(--border-medium);
    border-radius: 16px;
    padding: 20px 22px 18px;
    width: 480px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 20px 56px rgba(0,0,0,0.6), 0 0 0 0.5px var(--border-subtle);
    animation: pop 0.18s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes pop {
    from { transform: scale(0.94) translateY(-6px); opacity: 0; }
    to   { transform: scale(1) translateY(0); opacity: 1; }
  }

  .dialog-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 16px;
  }

  .icon-wrap {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    background: var(--accent-subtle);
    border: 1px solid var(--accent-muted);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--mauve);
    flex-shrink: 0;
  }

  h2 {
    font-size: 15px;
    font-weight: 700;
    color: var(--text);
    line-height: 1.2;
  }

  .count-badge {
    font-size: 10px;
    font-weight: 700;
    padding: 1px 7px;
    border-radius: 4px;
    background: var(--hover-bg-strong);
    color: var(--mauve);
  }

  .dialog-sep {
    height: 1px;
    background: var(--border-light);
    margin: 0 0 14px;
  }

  .tabs {
    display: flex;
    gap: 2px;
    background: var(--border-subtle);
    border-radius: 8px;
    padding: 3px;
    border: 1px solid var(--border-subtle);
    margin-bottom: 14px;
  }

  .tab {
    flex: 1;
    padding: 5px 8px;
    border-radius: 5px;
    font-size: 12px;
    color: var(--overlay1);
    transition: background 0.12s, color 0.12s;
    white-space: nowrap;
  }

  .tab:hover { color: var(--subtext0); }
  .tab.active { background: var(--accent-muted); color: var(--mauve); }

  .mode-body {
    margin-bottom: 14px;
  }

  .field-label {
    display: block;
    font-size: 11px;
    color: var(--overlay1);
    margin-bottom: 4px;
    margin-top: 8px;
  }

  .field-label:first-child { margin-top: 0; }

  .field-input {
    width: 100%;
    background: rgba(54, 58, 79, 0.4);
    border: 1.5px solid var(--border-medium);
    border-radius: 8px;
    padding: 7px 10px;
    color: var(--text);
    font-size: 13px;
    outline: none;
    transition: border-color 0.15s;
    box-sizing: border-box;
  }

  .field-input:focus { border-color: var(--accent-bg); }

  .case-options {
    display: flex;
    gap: 6px;
  }

  .case-btn {
    flex: 1;
    padding: 6px 10px;
    border-radius: 7px;
    font-size: 12px;
    color: var(--overlay1);
    background: var(--border-subtle);
    border: 1px solid var(--border-light);
    transition: background 0.12s, color 0.12s;
  }

  .case-btn:hover { color: var(--subtext0); }
  .case-btn.active { background: var(--accent-muted); color: var(--mauve); border-color: var(--accent-muted); }

  .preview-label {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--overlay0);
    margin-bottom: 8px;
  }

  .preview-list {
    max-height: 200px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 14px;
  }

  .preview-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    padding: 4px 8px;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.02);
  }

  .preview-row.changed {
    background: var(--accent-subtle);
  }

  .preview-old {
    color: var(--overlay1);
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }

  .arrow {
    flex-shrink: 0;
    color: var(--overlay0);
  }

  .preview-new {
    color: var(--text);
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }

  .preview-row.changed .preview-new {
    color: var(--mauve);
  }

  .actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .btn-cancel {
    padding: 7px 16px;
    border-radius: 8px;
    font-size: 13px;
    color: var(--subtext1);
    background: var(--hover-bg);
    border: 1px solid var(--border-light);
    transition: background 0.1s, color 0.1s;
  }

  .btn-cancel:hover { background: var(--hover-bg-strong); color: var(--text); }

  .btn-apply {
    padding: 7px 16px;
    border-radius: 8px;
    font-size: 13px;
    color: var(--base);
    background: var(--mauve);
    font-weight: 600;
    transition: opacity 0.1s;
  }

  .btn-apply:hover:not(:disabled) { opacity: 0.9; }
  .btn-apply:disabled { opacity: 0.4; cursor: default; }
</style>
