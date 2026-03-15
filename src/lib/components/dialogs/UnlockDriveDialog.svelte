<script lang="ts">
  import { fm } from "../../fileManager.svelte";
  import type { DriveInfo } from "../../types";
  import { t } from "../../i18n";

  let { drive }: { drive: DriveInfo } = $props();
  let password = $state("");
  let unlocking = $state(false);

  function cancel() {
    fm.showUnlockDialog = null;
  }

  async function unlock() {
    if (!password || unlocking) return;
    unlocking = true;
    await fm.unlockDrive(drive.device, password);
    unlocking = false;
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") { e.preventDefault(); unlock(); }
    if (e.key === "Escape") cancel();
  }
</script>

<div class="overlay" role="dialog" aria-modal="true">
  <div class="dialog">
    <div class="dialog-header">
      <div class="icon-lock">
        <svg width="24" height="24" viewBox="0 0 16 16" fill="none">
          <rect x="4" y="7" width="8" height="7" rx="1.5" stroke="currentColor" stroke-width="1.5" fill="none"/>
          <path d="M6 7V5C6 3.34 6.9 2 8 2s2 1.34 2 3v2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" fill="none"/>
          <circle cx="8" cy="10.5" r="1" fill="currentColor"/>
        </svg>
      </div>
      <div>
        <h2>{t.unlockEncryptedDrive}</h2>
        <span class="device-name">{drive.name} ({drive.device})</span>
      </div>
    </div>

    <div class="field">
      <label for="pw">{t.password}</label>
      <input
        id="pw"
        type="password"
        class="pw-input"
        bind:value={password}
        onkeydown={onKeydown}
        placeholder={t.enterPassword}
        disabled={unlocking}
      />
    </div>

    <div class="actions">
      <button class="btn-cancel" onclick={cancel} disabled={unlocking}>{t.cancel}</button>
      <button class="btn-unlock" onclick={unlock} disabled={!password || unlocking}>
        {unlocking ? t.unlocking : t.unlockMount}
      </button>
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.55);
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
    border: 1px solid var(--border-medium);
    border-radius: 14px;
    padding: 28px;
    width: 420px;
    box-shadow: 0 20px 60px rgba(0,0,0,0.6);
    animation: pop 0.15s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes pop {
    from { transform: scale(0.95); opacity: 0; }
    to   { transform: scale(1); opacity: 1; }
  }

  .dialog-header {
    display: flex;
    align-items: center;
    gap: 14px;
    margin-bottom: 20px;
  }

  .icon-lock {
    color: var(--mauve);
    flex-shrink: 0;
  }

  h2 {
    font-size: 16px;
    font-weight: 700;
    color: var(--text);
  }

  .device-name {
    font-size: 12px;
    color: var(--overlay1);
  }

  .field {
    margin-bottom: 24px;
  }

  label {
    display: block;
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--overlay0);
    margin-bottom: 6px;
  }

  .pw-input {
    width: 100%;
    height: 38px;
    background: var(--surface0);
    border: 1.5px solid var(--border-medium);
    border-radius: 8px;
    padding: 0 12px;
    color: var(--text);
    font-size: 14px;
    outline: none;
    transition: border-color 0.15s;
  }

  .pw-input:focus {
    border-color: var(--mauve);
  }

  .pw-input:disabled {
    opacity: 0.5;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
  }

  .btn-cancel {
    padding: 8px 18px;
    border-radius: 8px;
    font-size: 13.5px;
    color: var(--text);
    background: var(--surface0);
    font-weight: 500;
    transition: background 0.1s;
  }

  .btn-cancel:hover:not(:disabled) { background: var(--surface1); }

  .btn-unlock {
    padding: 8px 18px;
    border-radius: 8px;
    font-size: 13.5px;
    color: var(--crust);
    background: var(--mauve);
    font-weight: 700;
    transition: opacity 0.1s;
  }

  .btn-unlock:hover:not(:disabled) { opacity: 0.88; }
  .btn-unlock:disabled { opacity: 0.5; cursor: default; }
  .btn-cancel:disabled { opacity: 0.5; cursor: default; }
</style>
