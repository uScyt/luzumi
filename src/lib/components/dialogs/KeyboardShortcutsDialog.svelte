<script lang="ts">
  import { fm } from "../../fileManager.svelte";
  import { t } from "../../i18n";

  function close() {
    fm.showKeyboardShortcuts = false;
  }

  const sections = [
    {
      title: t.shortcutGeneral,
      shortcuts: [
        ["Ctrl+Z", "Undo"],
        ["Ctrl+Y", "Redo"],
        ["Ctrl+T", "New tab"],
        ["Ctrl+W", "Close tab"],
        ["Ctrl+L", "Address bar"],
        ["Ctrl+F", "Search"],
        ["Ctrl+,", t.settings],
        ["?", t.keyboardShortcuts],
        ["F5", t.reload],
      ],
    },
    {
      title: t.shortcutNavigation,
      shortcuts: [
        ["Alt+←", t.back],
        ["Alt+→", t.forward],
        ["Alt+↑", t.up],
        ["Enter", t.open],
      ],
    },
    {
      title: t.shortcutFileOps,
      shortcuts: [
        ["Ctrl+C", "Copy"],
        ["Ctrl+X", "Cut"],
        ["Ctrl+V", "Paste"],
        ["Ctrl+D", "Duplicate"],
        ["F2", t.rename],
        ["Delete", t.moveToTrash],
        ["Shift+Delete", t.deletePermanently],
        ["Ctrl+N", t.newFolder],
      ],
    },
    {
      title: t.shortcutSelection,
      shortcuts: [
        ["Ctrl+A", "Select all"],
        ["Ctrl+Shift+A", t.invertSelection],
        ["Ctrl+G", t.selectByPattern],
        ["Escape", "Deselect"],
      ],
    },
    {
      title: t.shortcutView,
      shortcuts: [
        ["Ctrl+1", t.list],
        ["Ctrl+2", t.grid],
        ["Ctrl+H", t.hiddenFiles],
        ["Ctrl+P", t.previewPane],
      ],
    },
  ];
</script>

<div class="overlay" role="dialog" aria-modal="true" tabindex="-1" onclick={close} onkeydown={(e) => { if (e.key === "Escape") close(); }}>
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div class="dialog" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
    <div class="dialog-header">
      <div class="icon-wrap">
        <svg width="17" height="17" viewBox="0 0 16 16" fill="none">
          <rect x="1" y="5" width="14" height="9" rx="2" stroke="currentColor" stroke-width="1.4" fill="none"/>
          <rect x="3" y="7" width="2" height="1.5" rx="0.3" fill="currentColor" opacity="0.6"/>
          <rect x="6" y="7" width="2" height="1.5" rx="0.3" fill="currentColor" opacity="0.6"/>
          <rect x="9" y="7" width="2" height="1.5" rx="0.3" fill="currentColor" opacity="0.6"/>
          <rect x="12" y="7" width="1.5" height="1.5" rx="0.3" fill="currentColor" opacity="0.6"/>
          <rect x="4" y="9.5" width="2" height="1.5" rx="0.3" fill="currentColor" opacity="0.6"/>
          <rect x="7" y="9.5" width="2" height="1.5" rx="0.3" fill="currentColor" opacity="0.6"/>
          <rect x="10" y="9.5" width="2" height="1.5" rx="0.3" fill="currentColor" opacity="0.6"/>
          <rect x="5" y="12" width="6" height="1.2" rx="0.3" fill="currentColor" opacity="0.4"/>
        </svg>
      </div>
      <h2>{t.keyboardShortcuts}</h2>
    </div>

    <div class="dialog-sep"></div>

    <div class="shortcuts-grid">
      {#each sections as section}
        <div class="section">
          <h3 class="section-title">{section.title}</h3>
          {#each section.shortcuts as [key, label]}
            <div class="shortcut-row">
              <kbd class="key">{key}</kbd>
              <span class="label">{label}</span>
            </div>
          {/each}
        </div>
      {/each}
    </div>

    <div class="actions">
      <button class="btn-close" onclick={close}>{t.closeEsc}</button>
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
    width: 560px;
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
    background: var(--blue-muted);
    border: 1px solid var(--blue-bg);
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
    margin: 0 0 14px;
  }

  .shortcuts-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px 24px;
    overflow-y: auto;
    max-height: 55vh;
    padding-right: 4px;
  }

  .section-title {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--overlay0);
    margin-bottom: 8px;
  }

  .shortcut-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 3px 0;
    gap: 8px;
  }

  kbd.key {
    font-family: inherit;
    font-size: 11px;
    font-weight: 600;
    padding: 2px 7px;
    border-radius: 5px;
    background: var(--border-light);
    border: 1px solid var(--border-medium);
    color: var(--subtext1);
    white-space: nowrap;
    min-width: 28px;
    text-align: center;
  }

  .label {
    font-size: 12px;
    color: var(--overlay1);
    text-align: right;
    flex: 1;
  }

  .actions {
    margin-top: 14px;
    display: flex;
    justify-content: flex-end;
  }

  .btn-close {
    padding: 7px 16px;
    border-radius: 8px;
    font-size: 13px;
    color: var(--subtext1);
    background: var(--hover-bg);
    border: 1px solid var(--border-light);
    transition: background 0.1s, color 0.1s;
  }

  .btn-close:hover { background: var(--hover-bg-strong); color: var(--text); }
</style>
