<script lang="ts">
  import { fm } from "../../fileManager.svelte";
  import { t } from "../../i18n";

  let fileNameInput: HTMLInputElement | null = null;
  let destPath = $state("");
  let fileName = $state("");

  $effect(() => {
    if (fm.showSaveAsDialog && fm.saveAsSource) {
      const name = fm.saveAsSource.split("/").pop() ?? "";
      fileName = name;
      destPath = fm.currentPath;
      requestAnimationFrame(() => {
        fileNameInput?.focus();
        fileNameInput?.select();
      });
    }
  });

  function cancel() {
    fm.showSaveAsDialog = false;
    fm.saveAsSource = null;
  }

  async function submit(e: Event) {
    e.preventDefault();
    if (fileName.trim()) {
      await fm.executeSaveAs(destPath, fileName.trim());
    }
  }
</script>

<svelte:window
  onkeydown={(e) => {
    if (!fm.showSaveAsDialog) return;
    if (e.key === "Escape") { e.preventDefault(); cancel(); }
  }}
/>

<div class="overlay" role="dialog" aria-modal="true">
  <div class="dialog">
    <div class="dialog-header">
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none" class="dialog-icon">
        <path d="M13 13V5.5L9.5 2H3C2.45 2 2 2.45 2 3V13C2 13.55 2.45 14 3 14H12C12.55 14 13 13.55 13 13Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".1"/>
        <path d="M9 2V6H13" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
        <path d="M5 9H11M5 11.5H9" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>
      </svg>
      <h2>{t.saveCopyAs}</h2>
    </div>
    <form onsubmit={submit}>
      <label class="field-label" for="saveas-dest">{t.destination}</label>
      <div class="dest-row">
        <input
          id="saveas-dest"
          class="name-input dest-input"
          bind:value={destPath}
          placeholder="/path/to/folder"
        />
      </div>

      {#if fm.bookmarks.length > 0 || fm.quickAccess.length > 0}
        <div class="shortcuts">
          {#each fm.bookmarks as bm}
            <button type="button" class="shortcut" onclick={() => destPath = bm.path} title={bm.path}>
              <svg width="11" height="11" viewBox="0 0 16 16" fill="none">
                <path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z"
                  fill="var(--blue)" opacity=".25" stroke="var(--blue)" stroke-width="1.3" stroke-linejoin="round"/>
              </svg>
              {bm.name}
            </button>
          {/each}
          {#each fm.quickAccess as qa}
            <button type="button" class="shortcut shortcut-qa" onclick={() => destPath = qa.path} title={qa.path}>
              <svg width="11" height="11" viewBox="0 0 16 16" fill="none">
                <path d="M5 9L3 14L8 11M11 2L7 6V9L11 10L14 6L11 2Z" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              {qa.name}
            </button>
          {/each}
        </div>
      {/if}

      <label class="field-label" for="saveas-filename" style="margin-top: 12px;">{t.fileName}</label>
      <input
        id="saveas-filename"
        bind:this={fileNameInput}
        bind:value={fileName}
        class="name-input"
        placeholder="filename.txt"
      />

      <div class="actions">
        <button type="button" class="btn-cancel" onclick={cancel}>{t.cancel}</button>
        <button type="submit" class="btn-save" disabled={!fileName.trim()}>{t.save}</button>
      </div>
    </form>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 600;
    animation: fade 0.15s ease;
  }

  @keyframes fade { from { opacity: 0; } to { opacity: 1; } }

  .dialog {
    background: var(--glass-bg);
    backdrop-filter: blur(30px) saturate(1.5);
    border: 1px solid var(--border-medium);
    border-radius: var(--radius-lg);
    padding: 24px;
    width: 380px;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.6);
    animation: pop 0.15s cubic-bezier(0.16, 1, 0.3, 1);
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  @keyframes pop {
    from { transform: scale(0.95) translateY(-8px); opacity: 0; }
    to   { transform: scale(1) translateY(0); opacity: 1; }
  }

  .dialog-header {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 16px;
  }

  .dialog-icon {
    color: var(--overlay1);
    flex-shrink: 0;
  }

  h2 {
    font-size: 15px;
    font-weight: 600;
    color: var(--text);
    margin: 0;
  }

  .field-label {
    font-size: 11px;
    font-weight: 500;
    color: var(--overlay1);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    display: block;
    margin-bottom: 5px;
  }

  .dest-row {
    display: flex;
    gap: 6px;
  }

  .name-input {
    width: 100%;
    background: var(--surface0);
    border: 1.5px solid var(--surface1);
    border-radius: var(--radius-sm);
    padding: 8px 10px;
    color: var(--text);
    font-size: 13px;
    outline: none;
    transition: border-color 0.15s;
    box-sizing: border-box;
  }

  .name-input:focus {
    border-color: var(--accent-bg);
  }

  .shortcuts {
    display: flex;
    flex-wrap: wrap;
    gap: 5px;
    margin-top: 8px;
  }

  .shortcut {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 4px 9px;
    background: var(--surface0);
    border-radius: var(--radius-sm);
    font-size: 12px;
    color: var(--subtext1);
    transition: background 0.1s;
    max-width: 120px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .shortcut:hover {
    background: var(--surface1);
    color: var(--text);
  }

  .shortcut-qa {
    color: var(--mauve);
  }

  .shortcut-qa:hover {
    background: var(--accent-subtle);
    color: var(--mauve);
  }

  .actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 18px;
  }

  .btn-cancel {
    padding: 7px 14px;
    border-radius: var(--radius-sm);
    font-size: 13px;
    color: var(--subtext1);
    background: var(--surface0);
    transition: background 0.1s;
  }

  .btn-cancel:hover { background: var(--surface1); }

  .btn-save {
    padding: 7px 16px;
    border-radius: var(--radius-sm);
    font-size: 13px;
    font-weight: 600;
    color: var(--base);
    background: var(--mauve);
    transition: opacity 0.1s;
  }

  .btn-save:hover:not(:disabled) { opacity: 0.88; }
  .btn-save:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
