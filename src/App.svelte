<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { listen } from "@tauri-apps/api/event";
  import { fm } from "./lib/fileManager.svelte";
  import { t } from "./lib/i18n";
  import { getFileIcon, getFileColor } from "./lib/types";
  import { renderIcon } from "./lib/icons";
  import TitleBar from "./lib/components/TitleBar.svelte";
  import Toolbar from "./lib/components/Toolbar.svelte";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import FileList from "./lib/components/FileList.svelte";
  import FileGrid from "./lib/components/FileGrid.svelte";
  import TabBar from "./lib/components/TabBar.svelte";
  import TrashView from "./lib/components/TrashView.svelte";
  import StatusBar from "./lib/components/StatusBar.svelte";
  import ContextMenu from "./lib/components/ContextMenu.svelte";
  import NewFolderDialog from "./lib/components/dialogs/NewFolderDialog.svelte";
  import PropertiesDialog from "./lib/components/dialogs/PropertiesDialog.svelte";
  import SecureDeleteDialog from "./lib/components/dialogs/SecureDeleteDialog.svelte";
  import EmptyTrashDialog from "./lib/components/dialogs/EmptyTrashDialog.svelte";
  import UnlockDriveDialog from "./lib/components/dialogs/UnlockDriveDialog.svelte";
  import DragDropDialog from "./lib/components/dialogs/DragDropDialog.svelte";
  import SaveAsDialog from "./lib/components/dialogs/SaveAsDialog.svelte";
  import NewFileDialog from "./lib/components/dialogs/NewFileDialog.svelte";
  import DeleteProgressBar from "./lib/components/DeleteProgressBar.svelte";
  import SettingsPanel from "./lib/components/SettingsPanel.svelte";
  import DefaultBanner from "./lib/components/DefaultBanner.svelte";
  import BulkRenameDialog from "./lib/components/dialogs/BulkRenameDialog.svelte";
  import DuplicateFinderDialog from "./lib/components/dialogs/DuplicateFinderDialog.svelte";
  import KeyboardShortcutsDialog from "./lib/components/dialogs/KeyboardShortcutsDialog.svelte";
  import SelectPatternDialog from "./lib/components/dialogs/SelectPatternDialog.svelte";
  import PreviewPane from "./lib/components/PreviewPane.svelte";
  import SplitPane from "./lib/components/SplitPane.svelte";
  import PickerFooter from "./lib/components/PickerFooter.svelte";

  const win = getCurrentWindow();

  onMount(() => {
    let unlisten1: (() => void) | undefined;
    let unlisten2: (() => void) | undefined;

    (async () => {
      await fm.init();
      unlisten1 = await listen<{ current: string; done: number; total: number }>("delete-progress", (ev) => {
        if (fm.deleteProgress) {
          fm.deleteProgress = ev.payload;
        }
      });
      unlisten2 = await listen<{ current: string; done: number; total: number }>("copy-move-progress", (ev) => {
        const p = ev.payload;
        if (p.done >= p.total) {
          fm.copyMoveProgress = null;
        } else {
          fm.copyMoveProgress = p;
        }
      });
    })();

    return () => { unlisten1?.(); unlisten2?.(); fm.destroy(); };
  });

  function onKeydown(e: KeyboardEvent) {
    // Picker mode: Escape cancels, Enter submits (unless in an input)
    if (fm.isPickerMode) {
      if (e.key === "Escape") { e.preventDefault(); fm.cancelPicker(); return; }
      if (e.key === "Enter" && !(e.target instanceof HTMLInputElement)) {
        e.preventDefault(); fm.submitPicker(); return;
      }
      // In picker mode, only allow navigation keys
      if (e.target instanceof HTMLInputElement) return;
      if (e.altKey && e.key === "ArrowLeft") { e.preventDefault(); fm.goBack(); return; }
      if (e.altKey && e.key === "ArrowRight") { e.preventDefault(); fm.goForward(); return; }
      if (e.altKey && e.key === "ArrowUp") { e.preventDefault(); fm.goUp(); return; }
      if (e.key === "F5") { e.preventDefault(); fm.reload(); return; }
      if (e.ctrlKey && e.key === "a") { e.preventDefault(); fm.selectAll(); return; }
      if (e.ctrlKey && e.key === "l") { e.preventDefault(); fm.focusAddressBar = true; return; }
      if (e.ctrlKey && e.key === "f") { e.preventDefault(); fm.focusSearch = true; return; }
      if (e.ctrlKey && e.key === "1") { e.preventDefault(); fm.viewMode = "list"; return; }
      if (e.ctrlKey && e.key === "2") { e.preventDefault(); fm.viewMode = "grid"; return; }
      if ((e.key === "ArrowDown" || e.key === "ArrowUp") && !e.altKey) {
        const entries = fm.filteredEntries();
        if (entries.length === 0) return;
        e.preventDefault();
        const sel = [...fm.selected];
        const last = sel.at(-1);
        const idx = last ? entries.findIndex(e2 => e2.path === last) : -1;
        const next = e.key === "ArrowDown"
          ? Math.min(idx + 1, entries.length - 1)
          : Math.max(idx - 1, 0);
        fm.selected = new Set([entries[next].path]);
      }
      return;
    }

    if (e.target instanceof HTMLInputElement) return;

    if (e.key === "Delete" && e.shiftKey) { e.preventDefault(); fm.triggerSecureDelete(); }
    else if (e.key === "Delete") { e.preventDefault(); fm.deleteSelected(); }
    if (e.key === "F2") { e.preventDefault(); const first = [...fm.selected][0]; const entry = fm.entries.find(x => x.path === first); if (entry) fm.startRename(entry); }
    if (e.ctrlKey && e.key === ",") { e.preventDefault(); fm.showSettings = !fm.showSettings; return; }
    if (e.key === "Escape") {
      if (fm.showSettings) { fm.showSettings = false; return; }
      fm.selected = new Set(); fm.renameTarget = null; fm.contextMenu = null; fm.showTrashView = false;
    }
    if (e.ctrlKey && !e.shiftKey && e.key === "z") { e.preventDefault(); fm.undo(); }
    if (e.ctrlKey && !e.shiftKey && e.key === "y") { e.preventDefault(); fm.redo(); return; }
    if (e.ctrlKey && e.shiftKey && e.key === "Z") { e.preventDefault(); fm.redo(); return; }
    if (e.ctrlKey && e.shiftKey && e.key === "A") { e.preventDefault(); fm.invertSelection(); return; }
    if (e.ctrlKey && e.key === "a") { e.preventDefault(); fm.selectAll(); }
    if (e.ctrlKey && e.key === "g") { e.preventDefault(); fm.showSelectPattern = true; return; }
    if (e.ctrlKey && e.key === "p") { e.preventDefault(); fm.showPreview = !fm.showPreview; return; }
    if (e.key === "?" && !e.ctrlKey && !e.altKey) { e.preventDefault(); fm.showKeyboardShortcuts = true; return; }
    if (e.ctrlKey && e.key === "c") { e.preventDefault(); fm.copySelected(); }
    if (e.ctrlKey && e.key === "x") { e.preventDefault(); fm.cutSelected(); }
    if (e.ctrlKey && e.key === "v") { e.preventDefault(); fm.paste(); }
    if (e.ctrlKey && e.key === "d") { e.preventDefault(); fm.duplicateSelected(); }
    if (e.ctrlKey && e.key === "h") { e.preventDefault(); fm.toggleHidden(); }
    if (e.ctrlKey && e.key === "l") { e.preventDefault(); fm.focusAddressBar = true; }
    if (e.ctrlKey && e.key === "f") { e.preventDefault(); fm.focusSearch = true; }
    if (e.ctrlKey && e.key === "t") { e.preventDefault(); fm.openTerminal(); }
    if (e.ctrlKey && e.key === "1") { e.preventDefault(); fm.viewMode = "list"; }
    if (e.ctrlKey && e.key === "2") { e.preventDefault(); fm.viewMode = "grid"; }
    if (e.ctrlKey && e.shiftKey && e.key === "N") { e.preventDefault(); fm.showNewFolder = true; fm.newFolderName = "New Folder"; }
    if (e.key === "F5") { e.preventDefault(); fm.reload(); }
    if (e.key === "F3") { e.preventDefault(); fm.toggleSplitView(); }
    if (e.altKey && e.key === "ArrowLeft") { e.preventDefault(); fm.goBack(); }
    if (e.altKey && e.key === "ArrowRight") { e.preventDefault(); fm.goForward(); }
    if (e.altKey && e.key === "ArrowUp") { e.preventDefault(); fm.goUp(); }
    if (e.ctrlKey && e.key === "t") { e.preventDefault(); fm.addTab(); return; }
    if (e.ctrlKey && e.key === "w") { e.preventDefault(); fm.closeTab(fm.activeTabIndex); return; }
    if (e.key === "Enter" && fm.selected.size === 1) {
      const entry = fm.entries.find(x => fm.selected.has(x.path));
      if (entry) fm.open(entry);
    }
    if ((e.key === "ArrowDown" || e.key === "ArrowUp") && !e.altKey) {
      const entries = fm.filteredEntries();
      if (entries.length === 0) return;
      e.preventDefault();
      const sel = [...fm.selected];
      const last = sel.at(-1);
      const idx = last ? entries.findIndex(e2 => e2.path === last) : -1;
      const next = e.key === "ArrowDown"
        ? Math.min(idx + 1, entries.length - 1)
        : Math.max(idx - 1, 0);
      fm.selected = new Set([entries[next].path]);
    }
  }

  function onWindowClick() {
    if (fm.contextMenu) fm.contextMenu = null;
  }
</script>

<svelte:window onkeydown={onKeydown} onclick={onWindowClick} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="app" class:picker-mode={fm.isPickerMode} role="application" oncontextmenu={(e) => e.preventDefault()}>
  {#if fm.isPickerMode}
    <!-- Picker mode: compact titlebar for dragging + title -->
    <div class="picker-titlebar" onmousedown={(e) => { if (e.target === e.currentTarget || (e.target as HTMLElement).classList.contains('picker-title')) { e.preventDefault(); win.startDragging(); }}}>
      <span class="picker-title">{fm.pickerConfig?.title ?? ''}</span>
      <div class="picker-titlebar-btns">
        <button class="picker-win-btn" onclick={() => win.minimize()} aria-label="Minimize">
          <svg width="10" height="10" viewBox="0 0 16 16" fill="none"><path d="M3 8H13" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></svg>
        </button>
        <button class="picker-win-btn picker-close" onclick={() => fm.cancelPicker()} aria-label="Close">
          <svg width="10" height="10" viewBox="0 0 16 16" fill="none"><path d="M3 3L13 13M13 3L3 13" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></svg>
        </button>
      </div>
    </div>
  {:else}
    <TitleBar />
    <TabBar />
  {/if}
  <Toolbar />
  <div class="main-area" class:with-preview={fm.showPreview && !fm.isPickerMode} class:with-split={fm.splitView && !fm.isPickerMode}>
    <Sidebar />
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <main class="content" onclick={() => { fm.splitFocused = false; }} onkeydown={() => {}}>
      {#if fm.showDefaultBanner && !fm.showTrashView && !fm.isPickerMode}
        <DefaultBanner />
      {/if}
      {#if fm.showTrashView && !fm.isPickerMode}
        <TrashView />
      {:else if fm.viewMode === "list"}
        <FileList />
      {:else}
        <FileGrid />
      {/if}
      {#if fm.searchQuery && (fm.globalSearchResults.length > 0 || fm.globalSearchLoading)}
        <div class="global-search-section">
          <div class="global-search-header">
            <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><circle cx="7" cy="7" r="5" stroke="currentColor" stroke-width="1.6"/><path d="M11 11L14 14" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/></svg>
            {t.otherLocations ?? "Other locations"}
            {#if fm.globalSearchLoading}
              <span class="global-search-loading">...</span>
            {/if}
          </div>
          <div class="global-search-results">
            {#each fm.globalSearchResults as entry (entry.path)}
              <button
                class="global-result"
                ondblclick={() => fm.open(entry)}
                onclick={() => fm.navigate(entry.path.substring(0, entry.path.lastIndexOf("/")) || "/")}
                title={entry.path}
              >
                <span class="global-result-icon" style="color: {getFileColor(entry)}">
                  {@html renderIcon(getFileIcon(entry))}
                </span>
                <span class="global-result-name">{entry.name}</span>
                <span class="global-result-path">{entry.path.substring(0, entry.path.lastIndexOf("/"))}</span>
              </button>
            {/each}
          </div>
        </div>
      {/if}
    </main>
    {#if fm.splitView && !fm.isPickerMode}
      <SplitPane />
    {/if}
    {#if fm.showPreview && !fm.isPickerMode}
      <PreviewPane />
    {/if}
  </div>
  {#if fm.isPickerMode}
    <PickerFooter />
  {:else}
    <StatusBar />
  {/if}

  {#if fm.contextMenu}
    <ContextMenu />
  {/if}

  {#if fm.showNewFolder}
    <NewFolderDialog />
  {/if}

  {#if fm.showProperties}
    <PropertiesDialog entry={fm.showProperties} />
  {/if}

  {#if fm.showSecureDeleteConfirm}
    <SecureDeleteDialog />
  {/if}

  {#if fm.showEmptyTrashConfirm}
    <EmptyTrashDialog />
  {/if}

  {#if fm.showUnlockDialog}
    <UnlockDriveDialog drive={fm.showUnlockDialog} />
  {/if}

  {#if fm.showDragDropDialog}
    <DragDropDialog />
  {/if}

  {#if fm.showSaveAsDialog}
    <SaveAsDialog />
  {/if}

  {#if fm.showNewFile}
    <NewFileDialog />
  {/if}

  {#if fm.showBulkRename}
    <BulkRenameDialog />
  {/if}

  {#if fm.showDuplicateFinder}
    <DuplicateFinderDialog />
  {/if}

  {#if fm.showKeyboardShortcuts}
    <KeyboardShortcutsDialog />
  {/if}

  {#if fm.showSelectPattern}
    <SelectPatternDialog />
  {/if}

  <DeleteProgressBar />

  {#if fm.showSettings}
    <SettingsPanel />
  {/if}

  {#if fm.error}
    <div class="toast toast-error" role="alert">
      <svg class="toast-icon" width="14" height="14" viewBox="0 0 16 16" fill="none">
        <circle cx="8" cy="8" r="6.5" stroke="currentColor" stroke-width="1.4"/>
        <path d="M8 5V9M8 11V11.5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
      </svg>
      <span>{fm.error}</span>
      <button aria-label={t.dismissError} onclick={() => fm.clearError()}>
        <svg width="10" height="10" viewBox="0 0 16 16" fill="none">
          <path d="M3 3L13 13M13 3L3 13" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
      </button>
    </div>
  {/if}

  {#if fm.statusMessage}
    <div class="toast toast-info" role="status">
      <svg class="toast-icon" width="14" height="14" viewBox="0 0 16 16" fill="none">
        <path d="M5 8L7 10L11 6" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
        <circle cx="8" cy="8" r="6.5" stroke="currentColor" stroke-width="1.4"/>
      </svg>
      <span>{fm.statusMessage}</span>
      <button aria-label={t.dismissNotification} onclick={() => fm.clearStatus()}>
        <svg width="10" height="10" viewBox="0 0 16 16" fill="none">
          <path d="M3 3L13 13M13 3L3 13" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
      </button>
    </div>
  {/if}
</div>

<div class="resize-n"  aria-hidden="true" onmousedown={(e) => { e.preventDefault(); win.startResizeDragging('North'); }}></div>
<div class="resize-s"  aria-hidden="true" onmousedown={(e) => { e.preventDefault(); win.startResizeDragging('South'); }}></div>
<div class="resize-e"  aria-hidden="true" onmousedown={(e) => { e.preventDefault(); win.startResizeDragging('East'); }}></div>
<div class="resize-w"  aria-hidden="true" onmousedown={(e) => { e.preventDefault(); win.startResizeDragging('West'); }}></div>
<div class="resize-nw" aria-hidden="true" onmousedown={(e) => { e.preventDefault(); win.startResizeDragging('NorthWest'); }}></div>
<div class="resize-ne" aria-hidden="true" onmousedown={(e) => { e.preventDefault(); win.startResizeDragging('NorthEast'); }}></div>
<div class="resize-sw" aria-hidden="true" onmousedown={(e) => { e.preventDefault(); win.startResizeDragging('SouthWest'); }}></div>
<div class="resize-se" aria-hidden="true" onmousedown={(e) => { e.preventDefault(); win.startResizeDragging('SouthEast'); }}></div>

<style>
  .app {
    display: grid;
    grid-template-rows: 38px 34px 50px 1fr 28px;
    height: 100vh;
    background: var(--app-bg);
    border-radius: 14px;
    overflow: hidden;
    clip-path: inset(0 round 14px);
    border: 1px solid rgba(255,255,255,0.1);
    box-shadow:
      0 30px 100px rgba(0,0,0,0.5),
      0 0 0 0.5px var(--border-light),
      inset 0 1px 0 var(--border-light);
  }

  .app.picker-mode {
    grid-template-rows: 34px 50px 1fr 52px;
  }

  .picker-titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 8px 0 16px;
    background: var(--app-bg);
    -webkit-app-region: drag;
    user-select: none;
    border-bottom: 1px solid var(--border-subtle);
  }

  .picker-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--subtext0);
    letter-spacing: 0.02em;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .picker-titlebar-btns {
    display: flex;
    gap: 2px;
    -webkit-app-region: no-drag;
  }

  .picker-win-btn {
    width: 28px;
    height: 28px;
    border-radius: 7px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    transition: background 0.1s, color 0.1s;
  }

  .picker-win-btn:hover {
    background: var(--hover-bg);
    color: var(--subtext0);
  }

  .picker-close:hover {
    background: var(--danger-bg);
    color: var(--red);
  }

  .main-area {
    display: grid;
    grid-template-columns: 220px 1fr;
    overflow: hidden;
    background: var(--app-bg);
  }

  .main-area.with-preview {
    grid-template-columns: 220px 1fr 280px;
  }

  .main-area.with-split {
    grid-template-columns: 220px 1fr 1fr;
  }

  .main-area.with-split.with-preview {
    grid-template-columns: 220px 1fr 1fr 280px;
  }

  .content {
    overflow: hidden;
    display: flex;
    flex-direction: column;
    background: var(--content-bg);
    border-top-left-radius: 20px;
  }

  .toast {
    position: fixed;
    bottom: 44px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 18px;
    border-radius: 12px;
    font-size: 13px;
    z-index: 1000;
    backdrop-filter: blur(20px) saturate(1.5);
    box-shadow: 0 8px 32px rgba(0,0,0,0.5), 0 0 0 0.5px var(--border-light);
    animation: slide-up 0.22s cubic-bezier(0.16, 1, 0.3, 1);
    max-width: 500px;
  }

  .toast-icon {
    flex-shrink: 0;
  }

  .toast-error {
    background: var(--danger-bg);
    border: 1px solid var(--danger-bg);
    color: var(--red);
  }

  .toast-info {
    background: var(--success-bg);
    border: 1px solid var(--success-bg);
    color: var(--green);
  }

  .toast button {
    color: inherit;
    opacity: 0.6;
    font-size: 12px;
    padding: 4px;
    border-radius: 5px;
    transition: opacity 0.1s, background 0.1s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .toast button:hover { opacity: 1; background: var(--border-light); }

  .toast span {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  @keyframes slide-up {
    from { opacity: 0; transform: translateX(-50%) translateY(10px); }
    to   { opacity: 1; transform: translateX(-50%) translateY(0); }
  }

  /* Resize handles for frameless window */
  :global(.resize-n), :global(.resize-s),
  :global(.resize-e), :global(.resize-w),
  :global(.resize-nw), :global(.resize-ne),
  :global(.resize-sw), :global(.resize-se) {
    position: fixed;
    z-index: 9999;
  }
  :global(.resize-n)  { top: 0; left: 8px; right: 8px; height: 5px; cursor: n-resize; }
  :global(.resize-s)  { bottom: 0; left: 8px; right: 8px; height: 5px; cursor: s-resize; }
  :global(.resize-e)  { right: 0; top: 8px; bottom: 8px; width: 5px; cursor: e-resize; }
  :global(.resize-w)  { left: 0; top: 8px; bottom: 8px; width: 5px; cursor: w-resize; }
  :global(.resize-nw) { top: 0; left: 0; width: 10px; height: 10px; cursor: nw-resize; }
  :global(.resize-ne) { top: 0; right: 0; width: 10px; height: 10px; cursor: ne-resize; }
  :global(.resize-sw) { bottom: 0; left: 0; width: 10px; height: 10px; cursor: sw-resize; }
  :global(.resize-se) { bottom: 0; right: 0; width: 10px; height: 10px; cursor: se-resize; }

  .global-search-section {
    border-top: 1px solid var(--border-medium);
    padding: 8px 12px;
    flex-shrink: 0;
    max-height: 240px;
    overflow-y: auto;
  }

  .global-search-header {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    font-weight: 600;
    color: var(--subtext0);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 6px;
  }

  .global-search-loading {
    animation: pulse 1s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 0.3; }
    50% { opacity: 1; }
  }

  .global-search-results {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .global-result {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px 8px;
    border-radius: 6px;
    font-size: 13px;
    color: var(--text);
    text-align: left;
    transition: background 0.08s;
    cursor: pointer;
  }

  .global-result:hover {
    background: var(--hover-bg);
  }

  .global-result-icon {
    flex-shrink: 0;
    display: flex;
  }

  .global-result-name {
    flex-shrink: 0;
    font-weight: 500;
  }

  .global-result-path {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 11px;
    color: var(--overlay0);
    direction: rtl;
    text-align: left;
  }
</style>
