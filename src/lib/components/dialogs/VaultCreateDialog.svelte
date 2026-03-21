<script lang="ts">
  import { fm } from "../../fileManager.svelte";
  import { t } from "../../i18n";

  let password = $state("");
  let confirmPassword = $state("");
  let dummyPassword = $state("");
  let showDummy = $state(false);
  let creating = $state(false);
  let error = $state("");

  const path = $derived(fm.showVaultCreate);

  const folderName = $derived(
    path ? path.split("/").filter(Boolean).pop() || "vault" : ""
  );

  async function create() {
    error = "";
    if (password.length < 4) {
      error = t.vaultPasswordTooShort;
      return;
    }
    if (password !== confirmPassword) {
      error = t.vaultPasswordMismatch;
      return;
    }
    if (showDummy && dummyPassword && dummyPassword === password) {
      error = "Dummy password must be different";
      return;
    }
    creating = true;
    const dp = showDummy && dummyPassword.length >= 4 ? dummyPassword : undefined;
    await fm.createVault(path!, password, dp);
    creating = false;
    close();
  }

  function close() {
    fm.showVaultCreate = null;
    password = "";
    confirmPassword = "";
    dummyPassword = "";
    showDummy = false;
    error = "";
  }
</script>

{#if path}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="overlay" onclick={close}>
    <div class="dialog" onclick={(e) => e.stopPropagation()} role="dialog" aria-label={t.createVault} tabindex="-1">
      <div class="dialog-header">
        <div class="icon-wrap">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
            <rect x="3" y="10" width="18" height="12" rx="2" stroke="currentColor" stroke-width="1.8" fill="currentColor" opacity=".1"/>
            <path d="M7 10V7a5 5 0 0110 0v3" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
            <circle cx="12" cy="16" r="1.5" fill="currentColor"/>
            <path d="M12 17.5V19" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </div>
        <div>
          <h2>{t.createVault}</h2>
          <span class="subtitle">{folderName}</span>
        </div>
      </div>

      <div class="dialog-sep"></div>

      <form onsubmit={(e) => { e.preventDefault(); create(); }}>
        <div class="field">
          <label for="vault-pw">{t.vaultPassword}</label>
          <input id="vault-pw" type="password" bind:value={password} placeholder="••••••••" disabled={creating} autocomplete="new-password" />
        </div>

        <div class="field">
          <label for="vault-pw-confirm">{t.vaultConfirmPassword}</label>
          <input id="vault-pw-confirm" type="password" bind:value={confirmPassword} placeholder="••••••••" disabled={creating} autocomplete="new-password" />
        </div>

        <div class="dummy-toggle">
          <label class="toggle-row">
            <input type="checkbox" bind:checked={showDummy} disabled={creating} />
            <span>{t.vaultDummyPassword}</span>
          </label>
          <span class="hint">{t.vaultDummyHint}</span>
        </div>

        {#if showDummy}
          <div class="field">
            <label for="vault-dummy">{t.vaultDummyPassword}</label>
            <input id="vault-dummy" type="password" bind:value={dummyPassword} placeholder="••••••••" disabled={creating} autocomplete="new-password" />
          </div>
        {/if}

        {#if error}
          <div class="error">{error}</div>
        {/if}

        <div class="actions">
          <button type="button" class="btn-cancel" onclick={close} disabled={creating}>{t.cancel}</button>
          <button type="submit" class="btn-create" disabled={creating || !password || !confirmPassword}>
            {#if creating}
              <span class="spinner"></span>
            {/if}
            {t.createVault}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 700;
  }
  .dialog {
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 20px;
    width: 360px;
    max-width: calc(100vw - 40px);
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
  }
  .dialog-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 12px;
  }
  .icon-wrap {
    width: 44px;
    height: 44px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 180, 50, 0.15);
    color: #ffb432;
  }
  h2 {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
    color: var(--text-primary);
  }
  .subtitle {
    font-size: 12px;
    color: var(--text-secondary);
    opacity: 0.7;
  }
  .dialog-sep {
    height: 1px;
    background: var(--border);
    margin: 0 -20px 16px;
  }
  form {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .field label {
    font-size: 11px;
    color: var(--text-secondary);
    font-weight: 500;
  }
  .field input {
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 8px 10px;
    color: var(--text-primary);
    font-size: 13px;
    outline: none;
  }
  .field input:focus {
    border-color: var(--accent);
  }
  .dummy-toggle {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .toggle-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: var(--text-primary);
    cursor: pointer;
  }
  .toggle-row input[type="checkbox"] {
    accent-color: var(--accent);
  }
  .hint {
    font-size: 10px;
    color: var(--text-secondary);
    opacity: 0.7;
    padding-left: 24px;
  }
  .error {
    font-size: 12px;
    color: #ff6b6b;
    background: rgba(255, 107, 107, 0.1);
    padding: 6px 10px;
    border-radius: 6px;
  }
  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 4px;
  }
  .actions button {
    padding: 7px 16px;
    border-radius: 6px;
    border: none;
    font-size: 13px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .btn-cancel {
    background: var(--bg-tertiary);
    color: var(--text-secondary);
  }
  .btn-cancel:hover {
    background: var(--bg-hover);
  }
  .btn-create {
    background: #ffb432;
    color: #1a1a1a;
    font-weight: 600;
  }
  .btn-create:hover:not(:disabled) {
    background: #ffc04d;
  }
  .btn-create:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(0, 0, 0, 0.2);
    border-top-color: #1a1a1a;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
