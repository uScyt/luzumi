<script lang="ts">
  import { fm } from "../../fileManager.svelte";
  import { t } from "../../i18n";

  let input: HTMLInputElement | null = null;

  $effect(() => {
    if (fm.showNewFolder) {
      requestAnimationFrame(() => {
        input?.focus();
        input?.select();
      });
    }
  });

  async function submit(e: Event) {
    e.preventDefault();
    const name = fm.newFolderName.trim();
    if (name) {
      fm.showNewFolder = false;
      await fm.createFolder(name);
    }
  }

  function cancel() {
    fm.showNewFolder = false;
  }
</script>

<div class="overlay" role="dialog" aria-modal="true" tabindex="-1" onclick={cancel} onkeydown={(e) => { if (e.key === "Escape") cancel(); }}>
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div class="dialog" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
    <div class="dialog-header">
      <div class="icon-wrap">
        <svg width="17" height="17" viewBox="0 0 16 16" fill="none">
          <path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1.4" fill="none" stroke-linejoin="round"/>
          <path d="M8 8.5V11.5M6.5 10H9.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </div>
      <h2>{t.newFolder}</h2>
    </div>
    <div class="dialog-sep"></div>
    <form onsubmit={submit}>
      <input
        bind:this={input}
        bind:value={fm.newFolderName}
        class="name-input"
        placeholder={t.folderNamePlaceholder}
        onkeydown={(e) => { if (e.key === "Escape") cancel(); }}
      />
      <div class="actions">
        <button type="button" class="btn-cancel" onclick={cancel}>{t.cancel}</button>
        <button type="submit" class="btn-create">
          <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
            <path d="M8 3V13M3 8H13" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
          </svg>
          {t.create}
        </button>
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
    width: 350px;
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
    background: rgba(138, 173, 244, 0.1);
    border: 1px solid rgba(138, 173, 244, 0.18);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--blue);
    flex-shrink: 0;
  }

  h2 {
    font-size: 15px;
    font-weight: 700;
    color: var(--text);
    line-height: 1.2;
  }

  .dialog-sep {
    height: 1px;
    background: var(--border-light);
    margin: 0 0 16px;
  }

  .name-input {
    width: 100%;
    background: rgba(54, 58, 79, 0.4);
    border: 1.5px solid var(--border-medium);
    border-radius: 9px;
    padding: 9px 12px;
    color: var(--text);
    font-size: 14px;
    outline: none;
    transition: border-color 0.15s, background 0.15s;
    box-sizing: border-box;
  }

  .name-input:focus {
    border-color: var(--accent-bg);
    background: rgba(54, 58, 79, 0.6);
  }

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
  .btn-cancel:active { transform: scale(0.97); }

  .btn-create {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 7px 16px;
    border-radius: 8px;
    font-size: 13px;
    color: var(--base);
    background: var(--mauve);
    font-weight: 600;
    transition: opacity 0.1s;

  }

  .btn-create:hover { opacity: 0.9;  }
  .btn-create:active { transform: scale(0.97); }
</style>
