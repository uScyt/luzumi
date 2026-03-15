<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { t } from "../i18n";

  function close() {
    fm.showSettings = false;
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
<div class="overlay" onclick={close} onkeydown={() => {}}></div>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<aside class="panel" onkeydown={(e) => e.stopPropagation()}>
  <div class="panel-header">
    <div class="panel-title-wrap">
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none" class="panel-icon">
        <path d="M6.3 2L6.7 3.8C6.2 4 5.7 4.3 5.3 4.7L3.6 4.1L2.3 6.4L3.7 7.5C3.6 7.7 3.6 7.8 3.6 8C3.6 8.2 3.6 8.3 3.7 8.5L2.3 9.6L3.6 11.9L5.3 11.3C5.7 11.7 6.2 12 6.7 12.2L6.3 14H9.7L10.1 12.2C10.6 12 11.1 11.7 11.5 11.3L13.2 11.9L14.5 9.6L13.1 8.5C13.2 8.3 13.2 8.2 13.2 8C13.2 7.8 13.2 7.7 13.1 7.5L14.5 6.4L13.2 4.1L11.5 4.7C11.1 4.3 10.6 4 10.1 3.8L9.7 2Z" stroke="currentColor" stroke-width="1.35" stroke-linejoin="round"/>
        <circle cx="8" cy="8" r="2" stroke="currentColor" stroke-width="1.35" fill="none"/>
      </svg>
      <span class="panel-title">{t.settings}</span>
    </div>
    <button class="close-btn" title={t.closeEsc} onclick={close}>
      <svg width="11" height="11" viewBox="0 0 16 16" fill="none">
        <path d="M2 2L14 14M14 2L2 14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
      </svg>
    </button>
  </div>

  <div class="panel-body">
    <div class="section-label">{t.theme}</div>

    <div class="theme-list">
      <button
        class="theme-item"
        class:active={fm.currentTheme === "Default"}
        onclick={() => fm.applyTheme("Default")}
      >
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
          <circle cx="8" cy="8" r="5.5" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".12"/>
          <path d="M8 4V8L10.5 10.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>
        </svg>
        {t.themeDefault}
      </button>
      {#each fm.availableThemes as theme}
        <button
          class="theme-item"
          class:active={fm.currentTheme === theme}
          onclick={() => fm.applyTheme(theme)}
        >
          <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
            <rect x="2" y="2" width="12" height="12" rx="3" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".1"/>
            {#if fm.currentTheme === theme}
              <path d="M5 8L7 10L11 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            {/if}
          </svg>
          {theme}
        </button>
      {/each}
    </div>

    <div class="theme-actions">
      <button class="theme-action-btn" onclick={() => fm.loadThemeList()} title={t.reloadThemes}>
        <svg width="11" height="11" viewBox="0 0 16 16" fill="none">
          <path d="M2 8C2 4.69 4.69 2 8 2C10.22 2 12.16 3.22 13.2 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          <path d="M14 8C14 11.31 11.31 14 8 14C5.78 14 3.84 12.78 2.8 11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          <path d="M13 2.5V5.5H10" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M3 13.5V10.5H6" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        {t.reloadThemes}
      </button>
      <button class="theme-action-btn" onclick={() => fm.openThemesFolder()} title={t.openThemesFolder}>
        <svg width="11" height="11" viewBox="0 0 16 16" fill="none">
          <path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="none"/>
        </svg>
        {t.openThemesFolder}
      </button>
    </div>

    <div class="divider"></div>

    <div class="section-label">{t.display}</div>

    <div class="setting-row">
      <span class="setting-label">{t.view}</span>
      <div class="seg-control">
        <button class="seg-btn" class:active={fm.viewMode === "list"} onclick={() => fm.viewMode = "list"}>
          <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
            <path d="M3 4H13M3 8H13M3 12H13" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/>
          </svg>
          {t.list}
        </button>
        <button class="seg-btn" class:active={fm.viewMode === "grid"} onclick={() => fm.viewMode = "grid"}>
          <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
            <rect x="2" y="2" width="5" height="5" rx="1.2" stroke="currentColor" stroke-width="1.5"/>
            <rect x="9" y="2" width="5" height="5" rx="1.2" stroke="currentColor" stroke-width="1.5"/>
            <rect x="2" y="9" width="5" height="5" rx="1.2" stroke="currentColor" stroke-width="1.5"/>
            <rect x="9" y="9" width="5" height="5" rx="1.2" stroke="currentColor" stroke-width="1.5"/>
          </svg>
          {t.grid}
        </button>
      </div>
    </div>

    <div class="setting-row">
      <span class="setting-label">{t.hiddenFiles}</span>
      <button class="toggle" class:on={fm.showHidden} onclick={() => fm.toggleHidden()} aria-pressed={fm.showHidden} title={t.hiddenFiles}>
        <span class="toggle-thumb"></span>
      </button>
    </div>

    <div class="divider"></div>

    <div class="section-label">{t.icons}</div>

    <div class="setting-row setting-col">
      <div class="setting-label-row">
        <span class="setting-label">{t.size}</span>
        <span class="setting-value">{fm.gridIconSize}px</span>
      </div>
      <div class="slider-row">
        <svg width="11" height="11" viewBox="0 0 16 16" fill="none" class="slider-icon">
          <rect x="3" y="3" width="4" height="4" rx="1" stroke="currentColor" stroke-width="1.5" fill="none"/>
          <rect x="9" y="9" width="4" height="4" rx="1" stroke="currentColor" stroke-width="1.5" fill="none"/>
        </svg>
        <input
          type="range"
          min="48"
          max="128"
          step="8"
          value={fm.gridIconSize}
          oninput={(e) => fm.setGridIconSize(parseInt((e.target as HTMLInputElement).value))}
          class="slider"
          aria-label={t.iconSize}
        />
        <svg width="15" height="15" viewBox="0 0 16 16" fill="none" class="slider-icon">
          <rect x="1" y="1" width="6" height="6" rx="1.2" stroke="currentColor" stroke-width="1.5" fill="none"/>
          <rect x="9" y="9" width="6" height="6" rx="1.2" stroke="currentColor" stroke-width="1.5" fill="none"/>
          <rect x="1" y="9" width="6" height="6" rx="1.2" stroke="currentColor" stroke-width="1.5" fill="none"/>
          <rect x="9" y="1" width="6" height="6" rx="1.2" stroke="currentColor" stroke-width="1.5" fill="none"/>
        </svg>
      </div>
    </div>

    <div class="setting-row">
      <div class="setting-label-col">
        <span class="setting-label">{t.hoverHighlight}</span>
        <span class="setting-hint">{t.gridOnly}</span>
      </div>
      <button class="toggle" class:on={fm.showHoverBox} onclick={() => fm.toggleHoverBox()} aria-pressed={fm.showHoverBox} title={t.hoverHighlight}>
        <span class="toggle-thumb"></span>
      </button>
    </div>

    <div class="divider"></div>

    <div class="section-label">{t.integration}</div>

    <div class="setting-row">
      <div class="setting-label-col">
        <span class="setting-label">{t.defaultManager}</span>
        {#if fm.isDefaultFileManager}
          <span class="setting-hint setting-hint-ok">{t.active}</span>
        {:else}
          <span class="setting-hint">{t.notDefault}</span>
        {/if}
      </div>
      <button
        class="set-default-btn"
        class:is-active={fm.isDefaultFileManager}
        disabled={fm.isDefaultFileManager}
        onclick={() => fm.setAsDefaultFileManager()}
        title={t.setDefault}
      >
        {fm.isDefaultFileManager ? t.active : t.setDefault}
      </button>
    </div>

    <div class="setting-row">
      <div class="setting-label-col">
        <span class="setting-label">{t.installPortal}</span>
        {#if fm.portalInstalled}
          <span class="setting-hint setting-hint-ok">{t.portalInstalled}</span>
        {:else}
          <span class="setting-hint">{t.portalNotInstalled}</span>
        {/if}
      </div>
      <button
        class="set-default-btn"
        class:is-active={fm.portalInstalled}
        onclick={() => fm.portalInstalled ? fm.uninstallPortal() : fm.installPortal()}
        title={fm.portalInstalled ? t.uninstallPortal : t.installPortal}
      >
        {fm.portalInstalled ? t.uninstallPortal : t.installPortal}
      </button>
    </div>
  </div>

  <div class="panel-footer">
    <kbd>Ctrl</kbd><span>+</span><kbd>,</kbd>
  </div>
</aside>

<style>
  .overlay {
    position: fixed;
    top: 72px;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 400;
  }

  .panel {
    position: fixed;
    top: 72px;
    right: 0;
    bottom: 0;
    width: 264px;
    z-index: 401;
    background: var(--glass-bg-strong);
    backdrop-filter: var(--glass-blur-strong);
    border-left: 1px solid var(--border-light);
    border-top: 1px solid rgba(255, 255, 255, 0.05);
    display: flex;
    flex-direction: column;
    animation: slide-in 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes slide-in {
    from { transform: translateX(100%); opacity: 0; }
    to   { transform: translateX(0); opacity: 1; }
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 14px;
    height: 48px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    box-shadow: 0 1px 0 rgba(255, 255, 255, 0.03) inset;
    flex-shrink: 0;
  }

  .panel-title-wrap {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .panel-icon {
    color: var(--overlay1);
  }

  .panel-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--subtext1);
  }

  .close-btn {
    width: 26px;
    height: 26px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay0);
    transition: background 0.1s, color 0.1s;
  }

  .close-btn:hover {
    background: var(--hover-bg);
    color: var(--subtext0);
  }

  .panel-body {
    flex: 1;
    overflow-y: auto;
    padding: 16px 14px 8px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .section-label {
    font-size: 9.5px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: var(--overlay0);
    margin-bottom: 8px;
    margin-top: 4px;
  }

  .divider {
    height: 1px;
    background: rgba(255, 255, 255, 0.05);
    margin: 10px 0 14px;
  }

  .theme-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
    margin-bottom: 6px;
  }

  .theme-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 10px;
    border-radius: var(--radius-sm);
    font-size: 12.5px;
    color: var(--subtext0);
    transition: background 0.12s, color 0.12s;
    text-align: left;
  }

  .theme-item:hover {
    background: var(--hover-bg);
    color: var(--text);
  }

  .theme-item.active {
    background: var(--accent-muted);
    color: var(--accent);
  }

  .theme-actions {
    display: flex;
    gap: 6px;
  }

  .theme-action-btn {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 4px 8px;
    border-radius: var(--radius-sm);
    font-size: 10.5px;
    color: var(--overlay1);
    transition: background 0.1s, color 0.1s;
  }

  .theme-action-btn:hover {
    background: var(--hover-bg);
    color: var(--subtext0);
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-height: 38px;
    gap: 12px;
  }

  .setting-col {
    flex-direction: column;
    align-items: stretch;
    gap: 8px;
    margin-bottom: 2px;
  }

  .setting-label-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .setting-label-col {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .setting-label {
    font-size: 13px;
    color: var(--subtext0);
  }

  .setting-hint {
    font-size: 10.5px;
    color: var(--overlay0);
  }

  .setting-hint-ok {
    color: var(--green);
  }

  .setting-value {
    font-size: 11px;
    color: var(--overlay1);
    font-variant-numeric: tabular-nums;
  }

  .seg-control {
    display: flex;
    gap: 2px;
    background: var(--border-subtle);
    border-radius: 8px;
    padding: 3px;
    border: 1px solid rgba(255, 255, 255, 0.05);
  }

  .seg-btn {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 4px 10px;
    border-radius: 5px;
    font-size: 12px;
    color: var(--overlay1);
    transition: background 0.12s, color 0.12s;
    white-space: nowrap;
  }

  .seg-btn:hover {
    color: var(--subtext0);
  }

  .seg-btn.active {
    background: var(--accent-muted);
    color: var(--accent);
  }

  .toggle {
    position: relative;
    width: 34px;
    height: 19px;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.09);
    border: 1px solid var(--border-medium);
    transition: background 0.18s, border-color 0.18s;
    flex-shrink: 0;
    cursor: pointer;
  }

  .toggle.on {
    background: var(--accent-bg);
    border-color: rgba(198, 160, 246, 0.3);
  }

  .toggle-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 13px;
    height: 13px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.8);
    transition: left 0.18s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .toggle.on .toggle-thumb {
    left: 17px;
    background: white;
  }

  .slider-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .slider-icon {
    color: var(--overlay0);
    flex-shrink: 0;
  }

  .slider {
    flex: 1;
    height: 3px;
    accent-color: var(--accent);
    cursor: pointer;
    -webkit-appearance: none;
    appearance: none;
    background: var(--hover-bg-strong);
    border-radius: 2px;
    outline: none;
  }

  .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 13px;
    height: 13px;
    border-radius: 50%;
    background: var(--accent);
    cursor: pointer;
  }

  .set-default-btn {
    font-size: 11px;
    padding: 4px 10px;
    border-radius: 6px;
    background: rgba(198, 160, 246, 0.15);
    color: var(--accent);
    transition: background 0.1s;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .set-default-btn:hover:not(:disabled) {
    background: rgba(198, 160, 246, 0.25);
  }

  .set-default-btn.is-active,
  .set-default-btn:disabled {
    opacity: 0.5;
    cursor: default;
    background: var(--accent-subtle);
  }

  .panel-footer {
    padding: 10px 14px;
    border-top: 1px solid var(--border-subtle);
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 3px;
    flex-shrink: 0;
  }

  kbd {
    font-size: 10px;
    color: var(--overlay0);
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border-medium);
    border-radius: 4px;
    padding: 1px 5px;
    font-family: inherit;
  }

  .panel-footer span {
    font-size: 10px;
    color: var(--overlay0);
  }
</style>
