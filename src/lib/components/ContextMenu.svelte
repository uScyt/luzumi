<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fm } from "../fileManager.svelte";
  import { t } from "../i18n";

  interface AppInfo {
    desktop_id: string;
    name: string;
    icon: string;
    is_default: boolean;
  }

  const menu = $derived(fm.contextMenu);

  let openWithMode = $state(false);
  let openWithApps = $state<AppInfo[]>([]);
  let openWithLoading = $state(false);

  function close() {
    fm.contextMenu = null;
    openWithMode = false;
    openWithApps = [];
  }

  function action(fn: () => void) {
    close();
    fn();
  }

  async function showOpenWith(path: string) {
    openWithLoading = true;
    openWithMode = true;
    try {
      openWithApps = await invoke<AppInfo[]>("cmd_list_open_with_apps", { path });
    } catch {
      openWithApps = [];
    }
    openWithLoading = false;
  }

  function goBack() {
    openWithMode = false;
    openWithApps = [];
  }

  const hasClipboard = $derived(!!fm.clipboard);
  const singleSelected = $derived(
    fm.selected.size === 1
      ? fm.entries.find((e) => fm.selected.has(e.path)) ?? null
      : null
  );
  const isPinned = $derived(
    menu?.target?.kind === "directory"
      ? fm.quickAccess.some(qa => qa.path === menu?.target?.path)
      : false
  );
</script>

{#if menu}
  <div
    class="menu"
    style="left: {menu.x}px; top: {menu.y}px"
    role="menu"
    tabindex="0"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => { if (e.key === "Escape") close(); e.stopPropagation(); }}
  >
    {#if menu.target && openWithMode}
      {@const owTarget = menu.target}
      <!-- Open With submenu -->
      <button class="menu-item" onclick={goBack}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M10 3L5 8L10 13" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/></svg>
        {t.back}
      </button>
      <div class="sep"></div>
      {#if openWithLoading}
        <div class="menu-item loading">
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none" class="spinner"><circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" stroke-dasharray="28" stroke-dashoffset="8" fill="none"/></svg>
          ...
        </div>
      {:else if openWithApps.length === 0}
        <div class="menu-item disabled">{t.noAppsFound}</div>
      {:else}
        {#each openWithApps as app}
          <button class="menu-item" onclick={() => { const p = owTarget.path; const id = app.desktop_id; close(); setTimeout(() => invoke("cmd_open_with_app", { path: p, desktopId: id }).catch(() => {}), 0); }}>
            <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="2" y="2" width="12" height="12" rx="3" stroke="currentColor" stroke-width="1.2" fill="none"/><path d="M5 8H11M8 5V11" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/></svg>
            <span class="app-name">{app.name}</span>
            {#if app.is_default}<span class="hint">{t.open}</span>{/if}
          </button>
        {/each}
      {/if}
    {:else if menu.target}
      {@const tgt = menu.target}
      {@const targetLocked = !tgt.isWritable && !fm.elevated}
      <button class="menu-item" onclick={() => action(() => fm.open(tgt))}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M6 3L11 8L6 13" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/></svg>
        {t.open}
      </button>
      <button class="menu-item" onclick={() => showOpenWith(tgt.path)}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M6 3L11 8L6 13" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/><path d="M2 8H5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/></svg>
        {tgt.kind === "directory" ? t.openFolderWith : t.openWith}
      </button>
      {#if targetLocked}
        <button class="menu-item admin-hint" onclick={() => { close(); invoke("cmd_authenticate_admin").then(() => { fm.elevated = true; }).catch(() => {}); }}>
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="4" y="7" width="8" height="7" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M6 7V5C6 3.34 6.9 2 8 2s2 1.34 2 3v2" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" fill="none"/><circle cx="8" cy="10.5" r="1" fill="currentColor"/></svg>
          {t.enableAdmin}
        </button>
      {/if}

      {#if tgt.kind === "directory"}
        <button class="menu-item" onclick={() => action(() => {
          if (isPinned) {
            fm.removeQuickAccess(tgt.path);
          } else {
            fm.addQuickAccess(tgt.name, tgt.path);
          }
        })}>
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
            {#if isPinned}
              <path d="M5 9L3 14L8 11M11 2L7 6V9L11 10L14 6L11 2Z" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" fill="currentColor" opacity=".2"/>
            {:else}
              <path d="M5 9L3 14L8 11M11 2L7 6V9L11 10L14 6L11 2Z" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
            {/if}
          </svg>
          {isPinned ? t.unpinQuickAccess : t.pinQuickAccess}
        </button>
      {/if}

      <div class="sep"></div>
      {#if singleSelected}
        <button class="menu-item" onclick={() => action(() => fm.startRename(singleSelected!))}>
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M10.5 2.5L13.5 5.5L5.5 13.5H2.5V10.5L10.5 2.5Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/></svg>
          {t.rename}
          <span class="hint">F2</span>
        </button>
      {/if}
      {#if fm.selected.size >= 2}
        <button class="menu-item" onclick={() => action(() => fm.bulkRename())}>
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M3 12L10 5L13 8" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/><path d="M10 5L11.5 3.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/></svg>
          {t.bulkRename}
        </button>
      {/if}
      {#if singleSelected && singleSelected.kind === "directory"}
        <button class="menu-item" onclick={() => action(() => fm.calculateFolderSize(singleSelected!.path))}>
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M8 5V8.5H11" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/></svg>
          {t.calculateSize}
        </button>
      {/if}
      <div class="sep"></div>
      <button class="menu-item" onclick={() => action(() => fm.copySelected())}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="5.5" y="5.5" width="8" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4"/><path d="M5.5 11H4C3.45 11 3 10.55 3 10V3C3 2.45 3.45 2 4 2H9C9.55 2 10 2.45 10 3V5.5" stroke="currentColor" stroke-width="1.4" fill="none"/></svg>
        {t.copy}
        <span class="hint">Ctrl+C</span>
      </button>
      <button class="menu-item" onclick={() => action(() => fm.duplicateSelected())}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="2" y="2" width="8" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><rect x="6" y="5" width="8" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/></svg>
        {t.duplicate}
        <span class="hint">Ctrl+D</span>
      </button>
      {#if tgt.kind !== "directory"}
        <button class="menu-item" onclick={() => action(() => fm.openSaveAs(tgt.path))}>
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M3 10V13H13V10" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/><path d="M8 2V10M5 7L8 10L11 7" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/></svg>
          {t.saveCopyAs}
        </button>
      {/if}
      <button class="menu-item" onclick={() => action(() => fm.cutSelected())}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><circle cx="4.5" cy="12.5" r="2" stroke="currentColor" stroke-width="1.4"/><circle cx="11.5" cy="12.5" r="2" stroke="currentColor" stroke-width="1.4"/><path d="M8 8L4.5 12.5M8 8L11.5 12.5M8 8V2" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/></svg>
        {t.cut}
        <span class="hint">Ctrl+X</span>
      </button>
    {/if}

    {#if hasClipboard}
      <button class="menu-item" onclick={() => action(() => fm.paste())}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="3" y="5" width="10" height="10" rx="1.5" stroke="currentColor" stroke-width="1.4"/><path d="M5.5 5V3.5C5.5 2.95 5.95 2.5 6.5 2.5H9.5C10.05 2.5 10.5 2.95 10.5 3.5V5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" fill="none"/></svg>
        {t.paste}
        <span class="hint">Ctrl+V</span>
      </button>
    {/if}

    {#if menu.target}
      {@const tgt2 = menu.target}
      <div class="sep"></div>
      <button class="menu-item" onclick={() => action(() => navigator.clipboard.writeText(tgt2.path))}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M5 2H11C11.55 2 12 2.45 12 3V4M4 4H10C10.55 4 11 4.45 11 5V13C11 13.55 10.55 14 10 14H4C3.45 14 3 13.55 3 13V5C3 4.45 3.45 4 4 4Z" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/></svg>
        {t.copyPath}
      </button>
      {#if tgt2.kind === "directory"}
        <button class="menu-item" onclick={() => action(() => fm.openTerminal(tgt2.path))}>
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="2" y="3" width="12" height="10" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M5 6L7.5 8.5L5 11M8.5 11H11" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/></svg>
          {t.openTerminal}
        </button>
      {/if}
      <button class="menu-item" onclick={() => action(() => fm.compressSelected())}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="3" y="2" width="10" height="12" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M7 4H9M7 6H9M7 8H9M6 10H10" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/></svg>
        {t.compress}
      </button>
      {#if singleSelected && singleSelected.extension?.toLowerCase() === "zip"}
        <button class="menu-item" onclick={() => action(() => fm.extractArchive(singleSelected!.path))}>
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="3" y="2" width="10" height="12" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M8 6V11M6 9L8 11L10 9" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/></svg>
          {t.extractHere}
        </button>
      {/if}
      <div class="sep"></div>
      <button class="menu-item danger" onclick={() => action(() => fm.deleteSelected())}>

        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M3 5H13M5.5 5V3.5C5.5 3.22 5.72 3 6 3H10C10.28 3 10.5 3.22 10.5 3.5V5M12 5V13C12 13.55 11.55 14 11 14H5C4.45 14 4 13.55 4 13V5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/></svg>
        {t.moveToTrash}
        <span class="hint">Del</span>
      </button>
      <button class="menu-item danger-severe" onclick={() => action(() => { fm.triggerSecureDelete(); })}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M2 2L14 14M14 2L2 14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>
        {t.deletePermanently}
        <span class="hint">Shift+Del</span>
      </button>
      <div class="sep"></div>
      <button class="menu-item" onclick={() => action(() => { fm.showProperties = tgt2; })}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4"/><path d="M8 7V11M8 5V5.5" stroke="currentColor" stroke-width="1.7" stroke-linecap="round"/></svg>
        {t.properties}
      </button>
    {:else}
      <button class="menu-item" onclick={() => action(() => fm.openTerminal())}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="2" y="3" width="12" height="10" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M5 6L7.5 8.5L5 11M8.5 11H11" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/></svg>
        {t.openTerminal}
      </button>
      <button class="menu-item" onclick={() => action(() => { fm.showNewFolder = true; fm.newFolderName = "New Folder"; })}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M8 8.5V11.5M6.5 10H9.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
        {t.newFolder}
        <span class="hint">⌃⇧N</span>
      </button>
      <button class="menu-item" onclick={() => action(() => { fm.showNewFile = true; fm.newFileName = "untitled.txt"; })}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M4 2H9L13 6V14C13 14.55 12.55 15 12 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M9 2V6H13" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M8 9.5V12.5M6.5 11H9.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
        {t.newFile}
      </button>
      <button class="menu-item" onclick={() => action(() => {
        const name = fm.currentPath.split("/").filter(Boolean).pop() || fm.currentPath;
        fm.addQuickAccess(name, fm.currentPath);
      })}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M5 9L3 14L8 11M11 2L7 6V9L11 10L14 6L11 2Z" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>
        {t.pinQuickAccess}
      </button>
      <button class="menu-item" onclick={() => action(() => fm.invertSelection())}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="2" y="2" width="5" height="5" rx="1" fill="currentColor" opacity="0.3" stroke="currentColor" stroke-width="1.2"/><rect x="9" y="2" width="5" height="5" rx="1" stroke="currentColor" stroke-width="1.2" fill="none"/><rect x="2" y="9" width="5" height="5" rx="1" stroke="currentColor" stroke-width="1.2" fill="none"/><rect x="9" y="9" width="5" height="5" rx="1" fill="currentColor" opacity="0.3" stroke="currentColor" stroke-width="1.2"/></svg>
        {t.invertSelection}
        <span class="hint">⌃⇧A</span>
      </button>
      <button class="menu-item" onclick={() => action(() => { fm.showSelectPattern = true; })}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><circle cx="7" cy="7" r="4.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M10.5 10.5L14 14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/></svg>
        {t.selectByPattern}
        <span class="hint">⌃G</span>
      </button>
      <div class="sep"></div>
      <button class="menu-item" onclick={() => action(() => fm.findDuplicates())}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="2" y="2" width="8" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><rect x="6" y="5" width="8" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/></svg>
        {t.findDuplicates}
      </button>
    {/if}
  </div>
{/if}

<style>
  .menu {
    position: fixed;
    background: var(--glass-bg-strong);
    backdrop-filter: var(--glass-blur);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 12px;
    padding: 5px;
    min-width: 230px;
    z-index: 500;
    box-shadow: 0 12px 40px rgba(0,0,0,0.55), 0 0 0 0.5px rgba(255,255,255,0.05);
    animation: pop 0.12s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes pop {
    from { opacity: 0; transform: scale(0.95) translateY(-4px); }
    to   { opacity: 1; transform: scale(1) translateY(0); }
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 7px 12px;
    border-radius: 8px;
    font-size: 13px;
    color: var(--text);
    transition: background 0.08s;
    text-align: left;
    white-space: nowrap;
  }

  .menu-item:hover {
    background: var(--hover-bg);
  }

  .menu-item.admin-hint {
    color: var(--peach);
  }

  .menu-item.admin-hint:hover {
    background: rgba(245, 169, 127, 0.12);
  }

  .menu-item.danger {
    color: var(--red);
  }

  .menu-item.danger:hover {
    background: var(--danger-bg);
  }

  .menu-item.danger-severe {
    color: var(--maroon);
  }

  .menu-item.danger-severe:hover {
    background: var(--danger-bg);
  }

  .hint {
    margin-left: auto;
    font-size: 11px;
    color: var(--overlay0);
    padding-left: 16px;
  }

  .sep {
    height: 1px;
    background: var(--border-light);
    margin: 3px 8px;
  }

  .menu-item.loading,
  .menu-item.disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  .app-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .spinner {
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
