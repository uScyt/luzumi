<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { t } from "../i18n";

  const config = $derived(fm.pickerConfig);
  const isSave = $derived(config?.mode === "save");
  const filters = $derived(config?.filters ?? []);
  const canSubmit = $derived(
    isSave ? fm.pickerFileName.trim().length > 0 : fm.selected.size > 0
  );
  const buttonLabel = $derived(isSave ? t.pickerSave : t.pickerOpen);

  function onSubmit(e: Event) {
    e.preventDefault();
    fm.submitPicker();
  }
</script>

<form class="picker-footer" onsubmit={onSubmit}>
  <div class="footer-fields">
    {#if isSave}
      <div class="field">
        <label class="field-label" for="picker-filename">{t.fileName}</label>
        <!-- svelte-ignore a11y_autofocus -->
        <input
          id="picker-filename"
          class="field-input"
          bind:value={fm.pickerFileName}
          placeholder={t.fileName}
          autofocus
        />
      </div>
    {:else}
      <div class="field hint">
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none" class="hint-icon">
          <circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3"/>
          <path d="M8 5V9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          <circle cx="8" cy="11.5" r="0.7" fill="currentColor"/>
        </svg>
        <span>{config?.directory ? t.selectFolder : t.selectFiles}</span>
      </div>
    {/if}

    {#if filters.length > 0}
      <div class="field filter-field">
        <label class="field-label" for="picker-filter">{t.fileType}</label>
        <select id="picker-filter" class="field-select" bind:value={fm.pickerFilterIndex}>
          {#each filters as f, i}
            <option value={i}>{f.name} ({f.patterns.join(", ")})</option>
          {/each}
          <option value={filters.length}>{t.allFiles}</option>
        </select>
      </div>
    {/if}
  </div>

  <div class="actions">
    <button type="button" class="btn-cancel" onclick={() => fm.cancelPicker()}>{t.cancel}</button>
    <button type="submit" class="btn-submit" disabled={!canSubmit}>
      {buttonLabel}
    </button>
  </div>
</form>

<style>
  .picker-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 10px 18px;
    background: var(--glass-bg-strong);
    border-top: 1px solid rgba(255, 255, 255, 0.07);
    backdrop-filter: blur(12px);
  }

  .footer-fields {
    display: flex;
    align-items: center;
    gap: 14px;
    flex: 1;
    min-width: 0;
  }

  .field {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    min-width: 0;
  }

  .hint {
    font-size: 13px;
    color: var(--overlay1);
    gap: 8px;
  }

  .hint-icon {
    flex-shrink: 0;
    opacity: 0.6;
  }

  .field-label {
    font-size: 12px;
    font-weight: 500;
    color: var(--subtext0);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .field-input {
    flex: 1;
    min-width: 0;
    background: rgba(54, 58, 79, 0.5);
    border: 1.5px solid var(--border-medium);
    border-radius: 8px;
    padding: 7px 12px;
    color: var(--text);
    font-size: 13px;
    outline: none;
    transition: border-color 0.15s, box-shadow 0.15s;
    box-sizing: border-box;
  }

  .field-input:focus {
    border-color: rgba(138, 173, 244, 0.5);
    box-shadow: 0 0 0 2px rgba(138, 173, 244, 0.12);
  }

  .filter-field {
    flex: 0 0 auto;
    max-width: 280px;
  }

  .field-select {
    background: rgba(54, 58, 79, 0.5);
    border: 1.5px solid var(--border-medium);
    border-radius: 8px;
    padding: 6px 10px;
    color: var(--text);
    font-size: 12px;
    font-family: inherit;
    outline: none;
    max-width: 220px;
    -webkit-appearance: none;
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg width='10' height='10' viewBox='0 0 16 16' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4 6L8 10L12 6' stroke='%23939ab7' stroke-width='1.8' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 8px center;
    padding-right: 26px;
    cursor: pointer;
    transition: border-color 0.15s;
  }

  .field-select:focus {
    border-color: rgba(138, 173, 244, 0.5);
  }

  .field-select option {
    background: var(--content-bg);
    color: var(--text);
  }

  .actions {
    display: flex;
    gap: 8px;
    flex-shrink: 0;
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

  .btn-submit {
    padding: 7px 20px;
    border-radius: 8px;
    font-size: 13px;
    color: var(--base);
    background: var(--blue);
    font-weight: 600;
    transition: opacity 0.1s;
    min-width: 70px;
    text-align: center;
  }

  .btn-submit:hover:not(:disabled) { opacity: 0.9; }
  .btn-submit:disabled { opacity: 0.4; cursor: default; }
</style>
