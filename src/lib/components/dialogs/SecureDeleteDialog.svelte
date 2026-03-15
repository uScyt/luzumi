<script lang="ts">
  import { fm } from "../../fileManager.svelte";
  import { t } from "../../i18n";

  const count = $derived(fm.selected.size);
  let skipNext = $state(false);

  function cancel() {
    fm.showSecureDeleteConfirm = false;
  }

  function confirm() {
    if (skipNext) {
      try { localStorage.setItem("luzumi_skip_secure_confirm", "1"); } catch (_) {}
    }
    fm.secureDeleteSelected();
  }
</script>

<div class="overlay" role="dialog" aria-modal="true" tabindex="-1" onclick={cancel} onkeydown={(e) => { if (e.key === "Escape") cancel(); }}>
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div class="dialog" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
    <div class="dialog-header">
      <div class="icon-wrap">
        <svg width="18" height="18" viewBox="0 0 16 16" fill="none">
          <path d="M8 1.5L14.5 13H1.5L8 1.5Z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" fill="currentColor" opacity=".12"/>
          <path d="M8 6.5V9.5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
          <circle cx="8" cy="11.5" r="0.9" fill="currentColor"/>
        </svg>
      </div>
      <div class="header-text">
        <h2>{t.permanentDeletion}</h2>
        <p class="header-sub">{t.actionIrreversible}</p>
      </div>
    </div>

    <div class="dialog-sep"></div>

    <p class="body-text">
      {@html t.secureDeleteBody(count)}
    </p>

    <label class="skip-label">
      <input type="checkbox" bind:checked={skipNext} />
      <span>{t.dontAskAgainSecureDelete}</span>
    </label>

    <div class="dialog-sep"></div>

    <div class="actions">
      <button class="btn-cancel" onclick={cancel}>{t.cancel}</button>
      <button class="btn-destroy" onclick={confirm}>
        <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
          <path d="M3 5H13M5.5 5V3.5C5.5 3.22 5.72 3 6 3H10C10.28 3 10.5 3.22 10.5 3.5V5M12 5V13C12 13.55 11.55 14 11 14H5C4.45 14 4 13.55 4 13V5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        {t.deletePermanentlyBtn}
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
    z-index: 700;
    animation: fade 0.12s ease;
  }

  @keyframes fade { from { opacity: 0; } to { opacity: 1; } }

  .dialog {
    background: var(--glass-bg-strong);
    backdrop-filter: blur(30px) saturate(1.5);
    border: 1px solid var(--danger-bg);
    border-radius: 16px;
    padding: 22px 24px 20px;
    width: 390px;
    box-shadow: 0 24px 64px rgba(0,0,0,0.65), 0 0 0 0.5px var(--border-subtle);
    animation: pop 0.18s cubic-bezier(0.16, 1, 0.3, 1);
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  @keyframes pop {
    from { transform: scale(0.94) translateY(-6px); opacity: 0; }
    to   { transform: scale(1) translateY(0); opacity: 1; }
  }

  .dialog-header {
    display: flex;
    align-items: center;
    gap: 14px;
    margin-bottom: 16px;
  }

  .icon-wrap {
    width: 42px;
    height: 42px;
    border-radius: 12px;
    background: var(--danger-bg);
    border: 1px solid var(--danger-bg);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--red);
    flex-shrink: 0;
  }

  .header-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  h2 {
    font-size: 16px;
    font-weight: 700;
    color: var(--text);
    line-height: 1.2;
  }

  .header-sub {
    font-size: 11.5px;
    color: var(--red);
    opacity: 0.75;
  }

  .dialog-sep {
    height: 1px;
    background: var(--border-light);
    margin: 0 0 16px;
  }

  .body-text {
    font-size: 13.5px;
    color: var(--subtext0);
    line-height: 1.6;
    margin-bottom: 14px;
  }

  .body-text :global(strong) {
    color: var(--red);
    font-weight: 600;
  }

  .skip-label {
    display: flex;
    align-items: center;
    gap: 9px;
    font-size: 12px;
    color: var(--overlay1);
    cursor: pointer;
    margin-bottom: 16px;
    padding: 8px 10px;
    border-radius: 8px;
    background: var(--hover-bg-subtle);
    border: 1px solid var(--border-subtle);
    transition: background 0.1s;
    user-select: none;
  }

  .skip-label:hover {
    background: var(--hover-bg);
    color: var(--subtext1);
  }

  .skip-label input[type="checkbox"] {
    accent-color: var(--mauve);
    width: 14px;
    height: 14px;
    flex-shrink: 0;
    cursor: pointer;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .btn-cancel {
    padding: 8px 18px;
    border-radius: 9px;
    font-size: 13px;
    color: var(--subtext1);
    background: var(--hover-bg);
    font-weight: 500;
    transition: background 0.1s, color 0.1s;
    border: 1px solid var(--border-light);
  }

  .btn-cancel:hover { background: var(--hover-bg-strong); color: var(--text); }
  .btn-cancel:active { transform: scale(0.97); }

  .btn-destroy {
    display: flex;
    align-items: center;
    gap: 7px;
    padding: 8px 18px;
    border-radius: 9px;
    font-size: 13px;
    color: #fff;
    background: var(--red);
    font-weight: 600;
    transition: opacity 0.1s;

  }

  .btn-destroy:hover { opacity: 0.9;  }
  .btn-destroy:active { transform: scale(0.97); }
</style>
