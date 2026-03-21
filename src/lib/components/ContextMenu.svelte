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
  let submenuOpen = $state<string | null>(null);
  let submenuTimer = $state<ReturnType<typeof setTimeout> | null>(null);
  let submenuCloseTimer = $state<ReturnType<typeof setTimeout> | null>(null);

  function close() {
    fm.contextMenu = null;
    openWithMode = false;
    openWithApps = [];
    submenuOpen = null;
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

  function enterSubmenu(id: string) {
    if (submenuCloseTimer) { clearTimeout(submenuCloseTimer); submenuCloseTimer = null; }
    if (submenuOpen === id) return;
    if (submenuTimer) clearTimeout(submenuTimer);
    submenuTimer = setTimeout(() => { submenuOpen = id; }, 80);
  }

  function leaveSubmenu() {
    if (submenuTimer) { clearTimeout(submenuTimer); submenuTimer = null; }
    submenuCloseTimer = setTimeout(() => { submenuOpen = null; }, 120);
  }

  function keepSubmenu() {
    if (submenuCloseTimer) { clearTimeout(submenuCloseTimer); submenuCloseTimer = null; }
  }

  // Position submenu panel
  function positionSubmenu(node: HTMLElement) {
    requestAnimationFrame(() => {
      const rect = node.getBoundingClientRect();
      const pad = 8;
      // Flip horizontally if overflows right
      if (rect.right > window.innerWidth - pad) {
        node.classList.add("flip-left");
      }
      // Clamp vertically if overflows bottom
      if (rect.bottom > window.innerHeight - pad) {
        const shift = rect.bottom - window.innerHeight + pad;
        node.style.top = `${-shift}px`;
      }
    });
  }

  let menuEl = $state<HTMLElement | null>(null);
  let positioned = $state(false);
  let adjustedX = $state(0);
  let adjustedY = $state(0);

  $effect(() => {
    if (menu) {
      positioned = false;
      adjustedX = menu.x;
      adjustedY = menu.y;
      submenuOpen = null;
    }
  });

  $effect(() => {
    if (menu && menuEl && !positioned) {
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          if (!menuEl || !menu) return;
          const rect = menuEl.getBoundingClientRect();
          const pad = 8;
          let x = menu.x;
          let y = menu.y;
          if (x + rect.width > window.innerWidth - pad) {
            x = Math.max(pad, window.innerWidth - rect.width - pad);
          }
          if (y + rect.height > window.innerHeight - pad) {
            y = Math.max(pad, window.innerHeight - rect.height - pad);
          }
          adjustedX = x;
          adjustedY = y;
          positioned = true;
        });
      });
    }
  });

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
  const isArchive = $derived(
    singleSelected?.extension
      ? ["zip", "tar", "gz", "bz2", "xz", "7z", "rar"].includes(singleSelected.extension.toLowerCase())
      : false
  );

  /* ---- SVG icon fragments (14×14 viewBox="0 0 16 16") ---- */
  const IC = {
    open:      `<path d="M6 3L11 8L6 13" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/>`,
    openWith:  `<path d="M6 3L11 8L6 13" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/><path d="M2 8H5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>`,
    tab:       `<rect x="1" y="4" width="14" height="10" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="none"/><path d="M1 6.5H6.5V4" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/>`,
    admin:     `<rect x="4" y="7" width="8" height="7" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M6 7V5C6 3.34 6.9 2 8 2s2 1.34 2 3v2" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" fill="none"/><circle cx="8" cy="10.5" r="1" fill="currentColor"/>`,
    pin:       `<path d="M5 9L3 14L8 11M11 2L7 6V9L11 10L14 6L11 2Z" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>`,
    pinFill:   `<path d="M5 9L3 14L8 11M11 2L7 6V9L11 10L14 6L11 2Z" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" fill="currentColor" opacity=".2"/>`,
    cut:       `<circle cx="4.5" cy="12.5" r="2" stroke="currentColor" stroke-width="1.4"/><circle cx="11.5" cy="12.5" r="2" stroke="currentColor" stroke-width="1.4"/><path d="M8 8L4.5 12.5M8 8L11.5 12.5M8 8V2" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>`,
    copy:      `<rect x="5.5" y="5.5" width="8" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4"/><path d="M5.5 11H4C3.45 11 3 10.55 3 10V3C3 2.45 3.45 2 4 2H9C9.55 2 10 2.45 10 3V5.5" stroke="currentColor" stroke-width="1.4" fill="none"/>`,
    paste:     `<rect x="3" y="5" width="10" height="10" rx="1.5" stroke="currentColor" stroke-width="1.4"/><path d="M5.5 5V3.5C5.5 2.95 5.95 2.5 6.5 2.5H9.5C10.05 2.5 10.5 2.95 10.5 3.5V5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" fill="none"/>`,
    rename:    `<path d="M10.5 2.5L13.5 5.5L5.5 13.5H2.5V10.5L10.5 2.5Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/>`,
    bulkRen:   `<path d="M3 12L10 5L13 8" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/><path d="M10 5L11.5 3.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>`,
    trash:     `<path d="M3 5H13M5.5 5V3.5C5.5 3.22 5.72 3 6 3H10C10.28 3 10.5 3.22 10.5 3.5V5M12 5V13C12 13.55 11.55 14 11 14H5C4.45 14 4 13.55 4 13V5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>`,
    delPerm:   `<path d="M2 2L14 14M14 2L2 14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>`,
    chevron:   `<path d="M6 4L10 8L6 12" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>`,
    // More Actions sub-items
    duplicate: `<rect x="2" y="2" width="8" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><rect x="6" y="5" width="8" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/>`,
    save:      `<path d="M3 10V13H13V10" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/><path d="M8 2V10M5 7L8 10L11 7" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>`,
    copyPath:  `<path d="M5 2H11C11.55 2 12 2.45 12 3V4M4 4H10C10.55 4 11 4.45 11 5V13C11 13.55 10.55 14 10 14H4C3.45 14 3 13.55 3 13V5C3 4.45 3.45 4 4 4Z" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>`,
    symlink:   `<path d="M6.5 10.5L9.5 7.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><path d="M9 11L9.5 11.5C10.88 12.88 13.12 12.88 14.5 11.5s1.38-3.62 0-5L13 5C11.62 3.62 9.38 3.62 8 5L7.5 5.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>`,
    compress:  `<rect x="3" y="2" width="10" height="12" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M7 4H9M7 6H9M7 8H9M6 10H10" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>`,
    extract:   `<rect x="3" y="2" width="10" height="12" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M8 6V11M6 9L8 11L10 9" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>`,
    terminal:  `<rect x="2" y="3" width="12" height="10" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M5 6L7.5 8.5L5 11M8.5 11H11" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>`,
    calcSize:  `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M8 5V8.5H11" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>`,
    vault:     `<rect x="2" y="7" width="12" height="8" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="none"/><path d="M5 7V5a3 3 0 016 0v2" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><circle cx="8" cy="11" r="1" fill="currentColor"/>`,
    vaultOpen: `<rect x="2" y="7" width="12" height="8" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="none"/><path d="M5 7V5a3 3 0 015.5-1.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><circle cx="8" cy="11" r="1" fill="currentColor"/>`,
    props:     `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4"/><path d="M8 7V11M8 5V5.5" stroke="currentColor" stroke-width="1.7" stroke-linecap="round"/>`,
    // Background menu
    newFolder: `<path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M8 8.5V11.5M6.5 10H9.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>`,
    newFile:   `<path d="M4 2H9L13 6V14C13 14.55 12.55 15 12 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M9 2V6H13" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M8 9.5V12.5M6.5 11H9.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>`,
    selectAll: `<rect x="2" y="2" width="12" height="12" rx="2" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M5 8L7.5 10.5L11 5.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>`,
    invert:    `<rect x="2" y="2" width="5" height="5" rx="1" fill="currentColor" opacity="0.3" stroke="currentColor" stroke-width="1.2"/><rect x="9" y="2" width="5" height="5" rx="1" stroke="currentColor" stroke-width="1.2" fill="none"/><rect x="2" y="9" width="5" height="5" rx="1" stroke="currentColor" stroke-width="1.2" fill="none"/><rect x="9" y="9" width="5" height="5" rx="1" fill="currentColor" opacity="0.3" stroke="currentColor" stroke-width="1.2"/>`,
    pattern:   `<circle cx="7" cy="7" r="4.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M10.5 10.5L14 14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>`,
    sort:      `<path d="M3 4H10M3 8H8M3 12H6" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M12 6V14M10 12L12 14L14 12" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>`,
    check:     `<path d="M4 8L7 11L12 5" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/>`,
  } as const;

  function ic(name: keyof typeof IC) {
    return `<svg width="14" height="14" viewBox="0 0 16 16" fill="none">${IC[name]}</svg>`;
  }
</script>

{#if menu}
  <div
    class="menu"
    bind:this={menuEl}
    style="left: {adjustedX}px; top: {adjustedY}px; visibility: {positioned ? 'visible' : 'hidden'}"
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

      <!-- === PRIMARY ACTIONS === -->
      <button class="menu-item" onclick={() => action(() => fm.open(tgt))}>
        {@html ic("open")}
        {t.open}
      </button>
      <button class="menu-item" onclick={() => showOpenWith(tgt.path)}>
        {@html ic("openWith")}
        {tgt.kind === "directory" ? t.openFolderWith : t.openWith}
      </button>
      {#if tgt.kind === "directory"}
        <button class="menu-item" onclick={() => action(() => fm.addTab(tgt.path))}>
          {@html ic("tab")}
          {t.openInNewTab}
        </button>
      {/if}
      {#if targetLocked}
        <button class="menu-item admin-hint" onclick={() => { close(); invoke("cmd_authenticate_admin").then(() => { fm.elevated = true; }).catch(() => {}); }}>
          {@html ic("admin")}
          {t.enableAdmin}
        </button>
      {/if}
      {#if tgt.kind === "directory"}
        <button class="menu-item" onclick={() => action(() => {
          if (isPinned) fm.removeQuickAccess(tgt.path);
          else fm.addQuickAccess(tgt.name, tgt.path);
        })}>
          {@html isPinned ? ic("pinFill") : ic("pin")}
          {isPinned ? t.unpinQuickAccess : t.pinQuickAccess}
        </button>
      {/if}

      <!-- === CLIPBOARD & EDIT === -->
      <div class="sep"></div>
      <button class="menu-item" onclick={() => action(() => fm.cutSelected())}>
        {@html ic("cut")}
        {t.cut}
        <span class="hint">Ctrl+X</span>
      </button>
      <button class="menu-item" onclick={() => action(() => fm.copySelected())}>
        {@html ic("copy")}
        {t.copy}
        <span class="hint">Ctrl+C</span>
      </button>
      {#if hasClipboard}
        <button class="menu-item" onclick={() => action(() => fm.paste())}>
          {@html ic("paste")}
          {t.paste}
          <span class="hint">Ctrl+V</span>
        </button>
      {/if}
      {#if singleSelected}
        <button class="menu-item" onclick={() => action(() => fm.startRename(singleSelected!))}>
          {@html ic("rename")}
          {t.rename}
          <span class="hint">F2</span>
        </button>
      {/if}
      {#if fm.selected.size >= 2}
        <button class="menu-item" onclick={() => action(() => fm.bulkRename())}>
          {@html ic("bulkRen")}
          {t.bulkRename}
        </button>
      {/if}
      <button class="menu-item danger" onclick={() => action(() => fm.deleteSelected())}>
        {@html ic("trash")}
        {t.moveToTrash}
        <span class="hint">Del</span>
      </button>

      <!-- === MORE ACTIONS SUBMENU === -->
      <div class="sep"></div>
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="submenu-trigger"
        onmouseenter={() => enterSubmenu("more")}
        onmouseleave={leaveSubmenu}
      >
        <button class="menu-item" onclick={() => { submenuOpen = submenuOpen === "more" ? null : "more"; }}>
          {@html ic("chevron")}
          {t.moreActions}
          <span class="hint chevron-hint">{@html ic("chevron")}</span>
        </button>
        {#if submenuOpen === "more"}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div class="submenu-panel" use:positionSubmenu onmouseenter={keepSubmenu} onmouseleave={leaveSubmenu}>
            <button class="menu-item" onclick={() => action(() => fm.duplicateSelected())}>
              {@html ic("duplicate")}
              {t.duplicate}
              <span class="hint">Ctrl+D</span>
            </button>
            {#if tgt.kind !== "directory"}
              <button class="menu-item" onclick={() => action(() => fm.openSaveAs(tgt.path))}>
                {@html ic("save")}
                {t.saveCopyAs}
              </button>
            {/if}
            <button class="menu-item" onclick={() => action(() => navigator.clipboard.writeText(tgt.path))}>
              {@html ic("copyPath")}
              {t.copyPath}
            </button>
            <button class="menu-item" onclick={() => action(async () => {
              const name = tgt.name + ".link";
              const parent = tgt.path.substring(0, tgt.path.lastIndexOf("/")) || "/";
              const linkPath = parent + "/" + name;
              try {
                await invoke("cmd_create_symlink", { target: tgt.path, linkPath });
                fm.setStatus(`Created symlink: ${name}`);
                await fm.reload();
              } catch (e) { fm.error = String(e); }
            })}>
              {@html ic("symlink")}
              {t.createSymlink}
            </button>
            <div class="sep"></div>
            <button class="menu-item" onclick={() => action(() => fm.compressSelected())}>
              {@html ic("compress")}
              {t.compress}
            </button>
            {#if isArchive}
              <button class="menu-item" onclick={() => action(() => fm.extractArchive(singleSelected!.path))}>
                {@html ic("extract")}
                {t.extractHere}
              </button>
            {/if}
            {#if tgt.kind === "directory"}
              <div class="sep"></div>
              <button class="menu-item" onclick={() => action(() => fm.openTerminal(tgt.path))}>
                {@html ic("terminal")}
                {t.openTerminal}
              </button>
              <button class="menu-item" onclick={() => action(() => fm.calculateFolderSize(tgt.path))}>
                {@html ic("calcSize")}
                {t.calculateSize}
              </button>
            {/if}
            {#if singleSelected && singleSelected.kind === "directory" && !singleSelected.isVault}
              <div class="sep"></div>
              <button class="menu-item" onclick={() => action(() => { fm.showVaultCreate = singleSelected!.path; })}>
                {@html ic("vault")}
                {t.createVault}
              </button>
            {/if}
            {#if singleSelected && singleSelected.isVault}
              <div class="sep"></div>
              <button class="menu-item" onclick={() => action(() => { fm.showVaultUnlock = singleSelected!; })}>
                {@html ic("vaultOpen")}
                {t.unlockVault}
              </button>
            {/if}
            <div class="sep"></div>
            <button class="menu-item danger-severe" onclick={() => action(() => { fm.triggerSecureDelete(); })}>
              {@html ic("delPerm")}
              {t.deletePermanently}
              <span class="hint">Shift+Del</span>
            </button>
          </div>
        {/if}
      </div>

      <!-- === PROPERTIES (always last) === -->
      <div class="sep"></div>
      <button class="menu-item" onclick={() => { const entry = tgt; close(); setTimeout(() => { fm.showProperties = entry; }, 0); }}>
        {@html ic("props")}
        {t.properties}
      </button>

    {:else}
      <!-- ========== BACKGROUND MENU ========== -->
      <button class="menu-item" onclick={() => action(() => fm.createFolder())}>
        {@html ic("newFolder")}
        {t.newFolder}
        <span class="hint">⌃⇧N</span>
      </button>
      <button class="menu-item" onclick={() => action(() => fm.createFile())}>
        {@html ic("newFile")}
        {t.newFile}
      </button>

      <div class="sep"></div>
      {#if hasClipboard}
        <button class="menu-item" onclick={() => action(() => fm.paste())}>
          {@html ic("paste")}
          {t.paste}
          <span class="hint">Ctrl+V</span>
        </button>
        <button class="menu-item" onclick={() => action(() => fm.pasteAsSymlink())}>
          {@html ic("symlink")}
          {t.pasteAsSymlink}
        </button>
      {/if}

      <div class="sep"></div>
      <button class="menu-item" onclick={() => action(() => fm.selectAll())}>
        {@html ic("selectAll")}
        {t.selectAll}
        <span class="hint">Ctrl+A</span>
      </button>
      <button class="menu-item" onclick={() => action(() => fm.invertSelection())}>
        {@html ic("invert")}
        {t.invertSelection}
        <span class="hint">⌃⇧A</span>
      </button>
      <button class="menu-item" onclick={() => action(() => { fm.showSelectPattern = true; })}>
        {@html ic("pattern")}
        {t.selectByPattern}
        <span class="hint">⌃G</span>
      </button>

      <!-- Sort By submenu -->
      <div class="sep"></div>
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="submenu-trigger"
        onmouseenter={() => enterSubmenu("sort")}
        onmouseleave={leaveSubmenu}
      >
        <button class="menu-item" onclick={() => { submenuOpen = submenuOpen === "sort" ? null : "sort"; }}>
          {@html ic("sort")}
          {t.sortByLabel}
          <span class="hint chevron-hint">{@html ic("chevron")}</span>
        </button>
        {#if submenuOpen === "sort"}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div class="submenu-panel" use:positionSubmenu onmouseenter={keepSubmenu} onmouseleave={leaveSubmenu}>
            <button class="menu-item" onclick={() => { fm.setSort("name"); }}>
              {#if fm.sortBy === "name"}{@html ic("check")}{:else}<span class="icon-spacer"></span>{/if}
              {t.name}
            </button>
            <button class="menu-item" onclick={() => { fm.setSort("size"); }}>
              {#if fm.sortBy === "size"}{@html ic("check")}{:else}<span class="icon-spacer"></span>{/if}
              {t.size}
            </button>
            <button class="menu-item" onclick={() => { fm.setSort("date"); }}>
              {#if fm.sortBy === "date"}{@html ic("check")}{:else}<span class="icon-spacer"></span>{/if}
              {t.date}
            </button>
            <button class="menu-item" onclick={() => { fm.setSort("type"); }}>
              {#if fm.sortBy === "type"}{@html ic("check")}{:else}<span class="icon-spacer"></span>{/if}
              {t.type}
            </button>
            <div class="sep"></div>
            <button class="menu-item" onclick={() => { fm.sortDir = "asc"; }}>
              {#if fm.sortDir === "asc"}{@html ic("check")}{:else}<span class="icon-spacer"></span>{/if}
              {t.ascending}
            </button>
            <button class="menu-item" onclick={() => { fm.sortDir = "desc"; }}>
              {#if fm.sortDir === "desc"}{@html ic("check")}{:else}<span class="icon-spacer"></span>{/if}
              {t.descending}
            </button>
          </div>
        {/if}
      </div>

      <div class="sep"></div>
      <button class="menu-item" onclick={() => action(() => fm.openTerminal())}>
        {@html ic("terminal")}
        {t.openTerminal}
      </button>
      <button class="menu-item" onclick={() => action(() => {
        const name = fm.currentPath.split("/").filter(Boolean).pop() || fm.currentPath;
        fm.addQuickAccess(name, fm.currentPath);
      })}>
        {@html ic("pin")}
        {t.pinQuickAccess}
      </button>
      <button class="menu-item" onclick={() => action(() => fm.findDuplicates())}>
        {@html ic("duplicate")}
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
    min-width: 220px;
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
    padding: 6px 12px;
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

  .menu-item.admin-hint { color: var(--peach); }
  .menu-item.admin-hint:hover { background: rgba(245, 169, 127, 0.12); }
  .menu-item.danger { color: var(--red); }
  .menu-item.danger:hover { background: var(--danger-bg); }
  .menu-item.danger-severe { color: var(--maroon); }
  .menu-item.danger-severe:hover { background: var(--danger-bg); }

  .hint {
    margin-left: auto;
    font-size: 11px;
    color: var(--overlay0);
    padding-left: 16px;
  }

  .chevron-hint {
    display: flex;
    align-items: center;
    padding-left: 8px;
    opacity: 0.5;
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

  .icon-spacer {
    display: inline-block;
    width: 14px;
    height: 14px;
    flex-shrink: 0;
  }

  .spinner {
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* ── Submenu system ── */
  .submenu-trigger {
    position: relative;
  }

  .submenu-panel {
    position: absolute;
    left: calc(100% - 4px);
    top: -5px;
    background: var(--glass-bg-strong);
    backdrop-filter: var(--glass-blur);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 10px;
    padding: 5px;
    min-width: 200px;
    z-index: 510;
    box-shadow: 0 10px 36px rgba(0,0,0,0.5), 0 0 0 0.5px rgba(255,255,255,0.05);
    animation: submenuIn 0.1s ease-out;
  }

  .submenu-panel:global(.flip-left) {
    left: auto;
    right: calc(100% - 4px);
  }

  @keyframes submenuIn {
    from { opacity: 0; transform: translateX(-4px); }
    to   { opacity: 1; transform: translateX(0); }
  }
</style>
