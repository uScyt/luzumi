<script lang="ts">
  import { fm } from "../../fileManager.svelte";
  import { t } from "../../i18n";

  $effect(() => {
    if (fm.showDragDropDialog) {
      requestAnimationFrame(() => moveBtn?.focus());
    }
  });

  let moveBtn: HTMLButtonElement | null = null;

  function getDestName() {
    if (!fm.pendingDrop) return "";
    const parts = fm.pendingDrop.dest.split("/").filter(Boolean);
    return parts[parts.length - 1] || "/";
  }

  function getCount() {
    return fm.pendingDrop?.paths.length ?? 0;
  }
</script>

<svelte:window
  onkeydown={(e) => {
    if (!fm.showDragDropDialog) return;
    if (e.key === "Escape") { e.preventDefault(); fm.cancelDrop(); }
    if (e.key === "Enter") { e.preventDefault(); fm.confirmDropMove(); }
  }}
/>

<div class="overlay" role="dialog" aria-modal="true" tabindex="-1" onclick={() => fm.cancelDrop()} onkeydown={(e) => { if (e.key === "Escape") fm.cancelDrop(); }}>
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div class="dialog" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
    <div class="dialog-header">
      <div class="folders-preview">
        <div class="folder-chip blue">
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
            <path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".15" stroke-linejoin="round"/>
          </svg>
        </div>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none" class="arrow">
          <path d="M3 8H13M9 4L13 8L9 12" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <div class="folder-chip mauve">
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
            <path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".15" stroke-linejoin="round"/>
          </svg>
        </div>
      </div>
      <div class="header-text">
        <h2>{t.moveOrCopy}</h2>
        <p class="subtitle">
          {t.elements(getCount())} {t.to}
          <span class="dest-name">{getDestName()}</span>
        </p>
      </div>
    </div>

    <div class="dialog-sep"></div>

    <div class="actions">
      <button class="btn-cancel" onclick={() => fm.cancelDrop()}>{t.cancel}</button>
      <button class="btn-copy" onclick={() => fm.confirmDropCopy()}>
        <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
          <rect x="5" y="5" width="8" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/>
          <path d="M11 5V3C11 2.45 10.55 2 10 2H3C2.45 2 2 2.45 2 3V10C2 10.55 2.45 11 3 11H5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
        </svg>
        {t.copy}
      </button>
      <button class="btn-move" bind:this={moveBtn} onclick={() => fm.confirmDropMove()}>
        <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
          <path d="M3 8H13M9 4L13 8L9 12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        {t.move}
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
    gap: 14px;
    margin-bottom: 16px;
  }

  .folders-preview {
    display: flex;
    align-items: center;
    gap: 5px;
    flex-shrink: 0;
  }

  .folder-chip {
    width: 32px;
    height: 32px;
    border-radius: 9px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .folder-chip.blue {
    background: var(--blue-muted);
    border: 1px solid var(--blue-bg);
    color: var(--blue);
  }

  .folder-chip.mauve {
    background: var(--hover-bg-strong);
    border: 1px solid var(--accent-muted);
    color: var(--mauve);
  }

  .arrow {
    color: var(--overlay0);
  }

  .header-text {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  h2 {
    font-size: 15px;
    font-weight: 700;
    color: var(--text);
    line-height: 1.2;
  }

  .subtitle {
    font-size: 12px;
    color: var(--subtext0);
    line-height: 1.4;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .dest-name {
    color: var(--blue);
    font-weight: 500;
  }

  .dialog-sep {
    height: 1px;
    background: var(--border-light);
    margin: 0 0 16px;
  }

  .actions {
    display: flex;
    gap: 7px;
    justify-content: flex-end;
  }

  .btn-cancel {
    padding: 7px 14px;
    border-radius: 8px;
    font-size: 13px;
    color: var(--subtext1);
    background: var(--hover-bg);
    border: 1px solid var(--border-light);
    transition: background 0.1s, color 0.1s;
  }

  .btn-cancel:hover { background: var(--hover-bg-strong); color: var(--text); }
  .btn-cancel:active { transform: scale(0.97); }

  .btn-copy {
    padding: 7px 14px;
    border-radius: 8px;
    font-size: 13px;
    color: var(--subtext1);
    background: var(--hover-bg);
    border: 1px solid var(--border-light);
    display: flex;
    align-items: center;
    gap: 6px;
    transition: background 0.1s, color 0.1s;
  }

  .btn-copy:hover { background: var(--hover-bg-strong); color: var(--text); }
  .btn-copy:active { transform: scale(0.97); }

  .btn-move {
    padding: 7px 16px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 600;
    color: var(--base);
    background: var(--mauve);
    display: flex;
    align-items: center;
    gap: 6px;
    transition: opacity 0.1s;

  }

  .btn-move:hover { opacity: 0.9;  }
  .btn-move:active { transform: scale(0.97); }
  .btn-move:focus-visible { outline: 2px solid var(--accent-bg); outline-offset: 2px; }
</style>
