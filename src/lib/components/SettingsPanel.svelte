<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fm } from "../fileManager.svelte";
  import { t } from "../i18n";

  let activeTab = $state<"general" | "appearance" | "system">("general");

  interface IntegrityItem {
    name: string;
    path: string;
    status: "ok" | "missing" | "repaired" | "error";
    detail: string;
  }

  let integrityResults = $state<IntegrityItem[] | null>(null);
  let integrityLoading = $state(false);

  async function checkIntegrity(repair: boolean) {
    integrityLoading = true;
    try {
      integrityResults = await invoke<IntegrityItem[]>("cmd_check_integrity", { repair });
    } catch (e) {
      console.error("Integrity check failed:", e);
      integrityResults = [{
        name: "Check failed",
        path: "",
        status: "error",
        detail: String(e),
      }];
    }
    integrityLoading = false;
  }

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

  <div class="tab-bar">
    <button class="tab" class:active={activeTab === "general"} onclick={() => activeTab = "general"}>
      <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><path d="M3 4H13M3 8H13M3 12H13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
      {t.display}
    </button>
    <button class="tab" class:active={activeTab === "appearance"} onclick={() => activeTab = "appearance"}>
      <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="5.5" stroke="currentColor" stroke-width="1.4"/><path d="M8 4V8L10.5 10.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/></svg>
      {t.appearance}
    </button>
    <button class="tab" class:active={activeTab === "system"} onclick={() => activeTab = "system"}>
      <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="none"/></svg>
      {t.integration}
    </button>
  </div>

  <div class="panel-body">
    {#if activeTab === "general"}
      <!-- Theme -->
      <div class="section-label">{t.theme}</div>

      <div class="theme-list">
        <button
          class="theme-item"
          class:active={fm.currentTheme === "Default"}
          onclick={() => fm.applyTheme("Default")}
        >
          <span class="theme-dot" style="background: var(--mauve)"></span>
          {t.themeDefault}
        </button>
        {#each fm.availableThemes as theme}
          <button
            class="theme-item"
            class:active={fm.currentTheme === theme}
            onclick={() => fm.applyTheme(theme)}
          >
            <span class="theme-dot" style="background: var(--accent)"></span>
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

      <!-- View mode -->
      <div class="section-label">{t.display}</div>

      <div class="setting-row">
        <span class="setting-label">{t.view}</span>
        <div class="seg-control">
          <button class="seg-btn" class:active={fm.viewMode === "list"} onclick={() => fm.viewMode = "list"}>
            <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
              <path d="M3 4H13M3 8H13M3 12H13" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/>
            </svg>
            {t.list}
          </button>
          <button class="seg-btn" class:active={fm.viewMode === "grid"} onclick={() => fm.viewMode = "grid"}>
            <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
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

      <!-- Icons -->
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

      <!-- Sidebar -->
      <div class="section-label">Sidebar</div>

      <div class="setting-row">
        <div class="setting-label-col">
          <span class="setting-label">Recent locations</span>
          <span class="setting-hint">Show recently visited folders</span>
        </div>
        <button class="toggle" class:on={fm.ui.showRecents} onclick={() => fm.ui.toggleRecents()} aria-pressed={fm.ui.showRecents} title="Toggle recent locations">
          <span class="toggle-thumb"></span>
        </button>
      </div>

    {:else if activeTab === "appearance"}
      <!-- Density -->
      <div class="section-label">Density</div>

      <div class="density-grid">
        <button class="density-btn" class:active={fm.ui.density === "comfortable"} onclick={() => fm.ui.setDensity("comfortable")}>
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none"><path d="M3 3H13M3 8H13M3 13H13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
          <span>Normal</span>
        </button>
        <button class="density-btn" class:active={fm.ui.density === "compact"} onclick={() => fm.ui.setDensity("compact")}>
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none"><path d="M3 4H13M3 7H13M3 10H13M3 13H13" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/></svg>
          <span>Compact</span>
        </button>
        <button class="density-btn" class:active={fm.ui.density === "dense"} onclick={() => fm.ui.setDensity("dense")}>
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none"><path d="M3 3H13M3 5.5H13M3 8H13M3 10.5H13M3 13H13" stroke="currentColor" stroke-width="1" stroke-linecap="round"/></svg>
          <span>Dense</span>
        </button>
      </div>

      <div class="divider"></div>

      <!-- Font Size -->
      <div class="section-label">Font</div>

      <div class="setting-row setting-col">
        <div class="setting-label-row">
          <span class="setting-label">Font Size</span>
          <span class="setting-value">{fm.ui.fontSize}px</span>
        </div>
        <div class="slider-row">
          <span class="slider-label-sm">A</span>
          <input
            type="range"
            min="10"
            max="22"
            step="1"
            value={fm.ui.fontSize}
            oninput={(e) => fm.ui.setFontSize(parseInt((e.target as HTMLInputElement).value))}
            class="slider"
            aria-label="Font size"
          />
          <span class="slider-label-lg">A</span>
        </div>
      </div>

      <div class="setting-row setting-col">
        <div class="setting-label-row">
          <span class="setting-label">{t.lineHeight}</span>
          <span class="setting-value">{fm.ui.lineHeight.toFixed(1)}</span>
        </div>
        <div class="slider-row">
          <input
            type="range"
            min="1.0"
            max="2.5"
            step="0.1"
            value={fm.ui.lineHeight}
            oninput={(e) => fm.ui.setLineHeight(parseFloat((e.target as HTMLInputElement).value))}
            class="slider"
            aria-label="Line height"
          />
        </div>
      </div>

      <div class="divider"></div>

      <!-- Toggles -->
      <div class="section-label">{t.options}</div>

      <div class="setting-row">
        <div class="setting-label-col">
          <span class="setting-label">{t.animations}</span>
          <span class="setting-hint">Transitions and motion effects</span>
        </div>
        <button class="toggle" class:on={fm.ui.animationsEnabled} onclick={() => fm.ui.toggleAnimations()} aria-pressed={fm.ui.animationsEnabled} title="Toggle animations">
          <span class="toggle-thumb"></span>
        </button>
      </div>

      <div class="setting-row">
        <div class="setting-label-col">
          <span class="setting-label">{t.highContrast}</span>
          <span class="setting-hint">Stronger borders and focus rings</span>
        </div>
        <button class="toggle" class:on={fm.ui.highContrast} onclick={() => fm.ui.toggleHighContrast()} aria-pressed={fm.ui.highContrast} title="Toggle high contrast">
          <span class="toggle-thumb"></span>
        </button>
      </div>

    {:else if activeTab === "system"}
      <!-- Default file manager -->
      <div class="section-label">{t.integration}</div>

      <div class="setting-card">
        <div class="setting-card-header">
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="none"/></svg>
          <span class="setting-label">{t.defaultManager}</span>
        </div>
        {#if fm.isDefaultFileManager}
          <span class="setting-status ok">{t.active}</span>
        {:else}
          <span class="setting-status">{t.notDefault}</span>
        {/if}
        <button
          class="action-btn"
          class:is-active={fm.isDefaultFileManager}
          disabled={fm.isDefaultFileManager}
          onclick={() => fm.setAsDefaultFileManager()}
        >
          {fm.isDefaultFileManager ? t.active : t.setDefault}
        </button>
      </div>

      <div class="setting-card">
        <div class="setting-card-header">
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="2" y="3" width="12" height="10" rx="2" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M5 7H11M5 10H8" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/></svg>
          <span class="setting-label">{t.installPortal}</span>
        </div>
        {#if fm.portalInstalled}
          <span class="setting-status ok">{t.portalInstalled}</span>
        {:else}
          <span class="setting-status">{t.portalNotInstalled}</span>
        {/if}
        <button
          class="action-btn"
          class:is-active={fm.portalInstalled}
          onclick={() => fm.portalInstalled ? fm.uninstallPortal() : fm.installPortal()}
        >
          {fm.portalInstalled ? t.uninstallPortal : t.installPortal}
        </button>
      </div>

      <div class="divider"></div>
      <div class="section-label">{t.integrity}</div>

      <div class="setting-card integrity-card">
        <div class="setting-card-header">
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
            <path d="M8 1L2 4V7.5C2 11.1 4.5 14.3 8 15C11.5 14.3 14 11.1 14 7.5V4L8 1Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="none"/>
            <path d="M5.5 8L7.5 10L11 6" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
          </svg>
          <span class="setting-label">{t.integrityCheck}</span>
        </div>

        <div class="integrity-actions">
          <button class="action-btn" disabled={integrityLoading} onclick={() => checkIntegrity(true)}>
            {integrityLoading ? t.integrityChecking : t.integrityCheckBtn}
          </button>
        </div>

        {#if integrityResults}
          <div class="integrity-results">
            {#each integrityResults as item}
              <div class="integrity-row" class:ok={item.status === "ok"} class:repaired={item.status === "repaired"} class:missing={item.status === "missing"} class:error={item.status === "error"}>
                <span class="integrity-icon">
                  {#if item.status === "ok"}
                    <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6" stroke="var(--green)" stroke-width="1.5" fill="none"/><path d="M5 8L7 10L11 6" stroke="var(--green)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
                  {:else if item.status === "repaired"}
                    <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6" stroke="var(--blue)" stroke-width="1.5" fill="none"/><path d="M5 8L7 10L11 6" stroke="var(--blue)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
                  {:else if item.status === "missing"}
                    <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6" stroke="var(--yellow)" stroke-width="1.5" fill="none"/><path d="M8 5V9M8 11V11.5" stroke="var(--yellow)" stroke-width="1.5" stroke-linecap="round"/></svg>
                  {:else}
                    <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6" stroke="var(--red)" stroke-width="1.5" fill="none"/><path d="M6 6L10 10M10 6L6 10" stroke="var(--red)" stroke-width="1.5" stroke-linecap="round"/></svg>
                  {/if}
                </span>
                <span class="integrity-name">{item.name}</span>
                {#if item.detail}
                  <span class="integrity-detail">{item.detail}</span>
                {/if}
              </div>
            {/each}
            <div class="integrity-summary" class:all-ok={integrityResults.every(r => r.status === "ok" || r.status === "repaired")}>
              {#if integrityResults.every(r => r.status === "ok" || r.status === "repaired")}
                {t.integrityAllOk}
              {:else}
                {t.integrityIssues(integrityResults.filter(r => r.status === "missing" || r.status === "error").length)}
              {/if}
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <div class="panel-footer">
    <kbd>Ctrl</kbd><span>+</span><kbd>,</kbd>
  </div>
</aside>

<style>
  .overlay {
    position: fixed;
    top: calc(var(--titlebar-h) + var(--tabbar-h));
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 400;
  }

  .panel {
    position: fixed;
    top: calc(var(--titlebar-h) + var(--tabbar-h));
    right: 0;
    bottom: 0;
    width: 300px;
    max-width: calc(100vw - 40px);
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
    height: 42px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    flex-shrink: 0;
  }

  .panel-title-wrap {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .panel-icon { color: var(--overlay1); }

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

  /* Tab bar */
  .tab-bar {
    display: flex;
    padding: 6px 10px 0;
    gap: 2px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    flex-shrink: 0;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 6px 10px;
    font-size: 11.5px;
    color: var(--overlay1);
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
    transition: color 0.12s, border-color 0.12s;
    white-space: nowrap;
  }

  .tab:hover { color: var(--subtext0); }

  .tab.active {
    color: var(--accent);
    border-bottom-color: var(--accent);
  }

  .panel-body {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 14px;
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

  /* Theme list */
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
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .theme-item:hover {
    background: var(--hover-bg);
    color: var(--text);
  }

  .theme-item.active {
    background: var(--accent-muted);
    color: var(--accent);
  }

  .theme-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .theme-actions {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
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
    white-space: nowrap;
  }

  .theme-action-btn:hover {
    background: var(--hover-bg);
    color: var(--subtext0);
  }

  /* Setting rows */
  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-height: 36px;
    gap: 10px;
  }

  .setting-col {
    flex-direction: column;
    align-items: stretch;
    gap: 6px;
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
    min-width: 0;
  }

  .setting-label {
    font-size: 12.5px;
    color: var(--subtext0);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .setting-hint {
    font-size: 10px;
    color: var(--overlay0);
  }

  .setting-value {
    font-size: 11px;
    color: var(--overlay1);
    font-variant-numeric: tabular-nums;
    flex-shrink: 0;
  }

  /* Segmented control */
  .seg-control {
    display: flex;
    gap: 2px;
    background: var(--border-subtle);
    border-radius: 8px;
    padding: 3px;
    border: 1px solid rgba(255, 255, 255, 0.05);
    flex-shrink: 0;
  }

  .seg-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 8px;
    border-radius: 5px;
    font-size: 11.5px;
    color: var(--overlay1);
    transition: background 0.12s, color 0.12s;
    white-space: nowrap;
  }

  .seg-btn:hover { color: var(--subtext0); }

  .seg-btn.active {
    background: var(--accent-muted);
    color: var(--accent);
  }

  /* Density grid */
  .density-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 6px;
    margin-bottom: 4px;
  }

  .density-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 10px 4px;
    border-radius: 8px;
    background: var(--border-subtle);
    border: 1px solid transparent;
    color: var(--overlay1);
    font-size: 10.5px;
    transition: background 0.12s, color 0.12s, border-color 0.12s;
  }

  .density-btn:hover {
    background: var(--hover-bg);
    color: var(--subtext0);
  }

  .density-btn.active {
    background: var(--accent-muted);
    color: var(--accent);
    border-color: var(--accent-border);
  }

  /* Toggle */
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
    border-color: var(--accent-border);
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

  /* Sliders */
  .slider-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .slider-icon {
    color: var(--overlay0);
    flex-shrink: 0;
  }

  .slider-label-sm {
    font-size: 10px;
    color: var(--overlay0);
    flex-shrink: 0;
    width: 12px;
    text-align: center;
  }

  .slider-label-lg {
    font-size: 15px;
    color: var(--overlay0);
    flex-shrink: 0;
    width: 12px;
    text-align: center;
  }

  .slider {
    flex: 1;
    min-width: 0;
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

  /* System tab cards */
  .setting-card {
    background: var(--border-subtle);
    border: 1px solid rgba(255, 255, 255, 0.04);
    border-radius: 10px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 8px;
  }

  .setting-card-header {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--overlay1);
  }

  .setting-card-header .setting-label {
    font-weight: 500;
    color: var(--subtext0);
  }

  .setting-status {
    font-size: 10.5px;
    color: var(--overlay0);
  }

  .setting-status.ok {
    color: var(--green);
  }

  .action-btn {
    font-size: 11px;
    padding: 6px 12px;
    border-radius: 6px;
    background: var(--accent-glow);
    color: var(--accent);
    transition: background 0.1s;
    align-self: flex-start;
  }

  .action-btn:hover:not(:disabled) {
    background: var(--accent-btn-hover);
  }

  .action-btn.is-active,
  .action-btn:disabled {
    opacity: 0.5;
    cursor: default;
    background: var(--accent-subtle);
  }

  /* Integrity check */
  .integrity-actions {
    display: flex;
    gap: 8px;
  }

.integrity-results {
    display: flex;
    flex-direction: column;
    gap: 3px;
    margin-top: 4px;
  }

  .integrity-row {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    padding: 3px 0;
  }

  .integrity-icon {
    flex-shrink: 0;
    display: flex;
    align-items: center;
  }

  .integrity-name {
    color: var(--subtext0);
    font-weight: 500;
  }

  .integrity-detail {
    color: var(--overlay0);
    font-size: 10px;
    margin-left: auto;
  }

  .integrity-row.repaired .integrity-name {
    color: var(--blue);
  }

  .integrity-row.missing .integrity-name {
    color: var(--yellow);
  }

  .integrity-row.error .integrity-name {
    color: var(--red);
  }

  .integrity-summary {
    margin-top: 6px;
    padding: 6px 8px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 500;
    background: rgba(237, 135, 150, 0.1);
    color: var(--red);
    text-align: center;
  }

  .integrity-summary.all-ok {
    background: rgba(166, 218, 149, 0.1);
    color: var(--green);
  }

  /* Footer */
  .panel-footer {
    padding: 8px 14px;
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
