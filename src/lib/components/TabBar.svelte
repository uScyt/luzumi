<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { t } from "../i18n";

  function tabLabel(path: string): string {
    if (!path) return t.newFolder;
    const parts = path.split("/").filter(Boolean);
    return parts.length === 0 ? "/" : parts[parts.length - 1];
  }
</script>

<div class="tabbar">
  <div class="tabs-scroll">
    {#each fm.tabs as tab, i (tab.id)}
      {@const isActive = i === fm.activeTabIndex}
      {@const displayPath = isActive ? fm.currentPath : tab.path}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="tab"
        class:active={isActive}
        class:drop-target={fm.tabDropTarget === i}
        onclick={() => fm.switchTab(i)}
        onkeydown={(e) => { if (e.key === "Enter" || e.key === " ") fm.switchTab(i); }}
        ondragover={(e) => { e.preventDefault(); if (fm.dragPaths.length > 0) { if (e.dataTransfer) e.dataTransfer.dropEffect = "move"; fm.tabDropTarget = i; } }}
        ondragleave={() => { if (fm.tabDropTarget === i) fm.tabDropTarget = null; }}
        ondrop={(e) => { e.preventDefault(); fm.dropOnTab(i); }}
        role="tab"
        tabindex="0"
        aria-selected={isActive}
        title={tab.path}
      >
        <svg width="11" height="11" viewBox="0 0 16 16" fill="none" class="tab-icon">
          <path d="M2 4.5C2 3.67 2.67 3 3.5 3H6.38C6.76 3 7.12 3.16 7.38 3.44L8.62 4.78C8.88 5.06 9.24 5.22 9.62 5.22H12.5C13.33 5.22 14 5.89 14 6.72V11.5C14 12.33 13.33 13 12.5 13H3.5C2.67 13 2 12.33 2 11.5V4.5Z" stroke="currentColor" stroke-width="1.4" fill="none"/>
        </svg>
        <span class="tab-label">{tabLabel(displayPath)}</span>
        {#if fm.tabs.length > 1}
          <button
            class="tab-close"
            onclick={(e) => { e.stopPropagation(); fm.closeTab(i); }}
            title={t.closeTab}
            aria-label={t.closeTab}
          >
            <svg width="8" height="8" viewBox="0 0 10 10" fill="none">
              <path d="M1 1L9 9M9 1L1 9" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/>
            </svg>
          </button>
        {/if}
      </div>
    {/each}
  </div>
  {#if fm.tabs.length < 5}
    <button class="new-tab-btn" onclick={() => fm.addTab()} title={t.newTabTitle}>
      <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
        <path d="M8 3V13M3 8H13" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
      </svg>
    </button>
  {/if}
</div>

<style>
  .tabbar {
    display: flex;
    align-items: stretch;
    background: var(--app-bg);
    border-bottom: 1px solid var(--border-subtle);
    height: 34px;
    overflow: hidden;
    user-select: none;
    -webkit-user-select: none;
  }

  .tabs-scroll {
    display: flex;
    align-items: stretch;
    flex: 1;
    overflow: hidden;
  }

  .tab {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 0 26px 0 9px;
    flex: 1;
    min-width: 0;
    height: 34px;
    border-right: 1px solid var(--border-subtle);
    color: var(--overlay1);
    font-size: 13px;
    cursor: default;
    position: relative;
    flex-shrink: 1;
    transition: background 0.1s, color 0.1s, border-top-color 0.1s;
    white-space: nowrap;
    overflow: hidden;
  }

  .tab:hover {
    background: var(--hover-bg-subtle);
    color: var(--subtext0);
  }

  .tab.active {
    background: var(--content-bg);
    color: var(--subtext1);
    border-top: 2px solid var(--accent-bg);
    /* visually merge with the content area below */
    border-bottom: 1px solid var(--content-bg);
  }

  .tab.drop-target {
    background: var(--blue-muted);
    color: var(--blue);
    border-top: 2px solid var(--blue-border-strong);
  }

  .tab-icon {
    flex-shrink: 0;
    color: inherit;
    opacity: 0.8;
  }

  .tab.active .tab-icon {
    color: var(--mauve);
    opacity: 1;
  }

  .tab.drop-target .tab-icon {
    color: var(--blue);
    opacity: 1;
  }

  .tab-label {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tab-close {
    position: absolute;
    right: 6px;
    width: 16px;
    height: 16px;
    border-radius: 3px;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    color: var(--overlay1);
    transition: opacity 0.1s, background 0.1s, color 0.1s;
    cursor: pointer;
  }

  .tab:hover .tab-close {
    opacity: 0.6;
  }

  .tab-close:hover {
    opacity: 1 !important;
    background: var(--danger-bg);
    color: var(--red);
  }

  .new-tab-btn {
    flex-shrink: 0;
    width: 36px;
    height: 34px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay0);
    border-left: 1px solid var(--border-subtle);
    transition: background 0.1s, color 0.1s;
  }

  .new-tab-btn:hover {
    background: var(--hover-bg-subtle);
    color: var(--subtext0);
  }
</style>
