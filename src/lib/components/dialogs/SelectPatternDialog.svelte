<script lang="ts">
  import { fm } from "../../fileManager.svelte";
  import { t } from "../../i18n";

  let pattern = $state("");

  function apply() {
    fm.selectByPattern(pattern);
    fm.showSelectPattern = false;
  }

  function close() {
    fm.showSelectPattern = false;
  }
</script>

<div class="overlay" role="dialog" aria-modal="true" tabindex="-1" onclick={close} onkeydown={(e) => { if (e.key === "Escape") close(); }}>
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div class="dialog" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
    <div class="dialog-header">
      <div class="icon-wrap">
        <svg width="17" height="17" viewBox="0 0 16 16" fill="none">
          <circle cx="7" cy="7" r="4.5" stroke="currentColor" stroke-width="1.5" fill="none"/>
          <path d="M10.5 10.5L14 14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </div>
      <h2>{t.selectByPattern}</h2>
    </div>

    <div class="dialog-sep"></div>

    <form onsubmit={(e) => { e.preventDefault(); apply(); }}>
      <!-- svelte-ignore a11y_autofocus -->
      <input
        class="field-input"
        bind:value={pattern}
        placeholder={t.selectPattern}
        autofocus
      />
      <div class="actions">
        <button type="button" class="btn-cancel" onclick={close}>{t.cancel}</button>
        <button type="submit" class="btn-apply" disabled={!pattern}>{t.selectMatching}</button>
      </div>
    </form>
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
    width: 380px;
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
    background: var(--success-bg);
    border: 1px solid var(--success-bg);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--green);
    flex-shrink: 0;
  }

  h2 {
    font-size: 15px;
    font-weight: 700;
    color: var(--text);
  }

  .dialog-sep {
    height: 1px;
    background: var(--border-light);
    margin: 0 0 14px;
  }

  .field-input {
    width: 100%;
    background: var(--input-bg-strong);
    border: 1.5px solid var(--border-medium);
    border-radius: 8px;
    padding: 9px 12px;
    color: var(--text);
    font-size: 14px;
    outline: none;
    transition: border-color 0.15s;
    box-sizing: border-box;
  }

  .field-input:focus { border-color: var(--success-border); }

  .actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 14px;
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
    background: var(--green);
    font-weight: 600;
    transition: opacity 0.1s;
  }

  .btn-apply:hover:not(:disabled) { opacity: 0.9; }
  .btn-apply:disabled { opacity: 0.4; cursor: default; }
</style>
