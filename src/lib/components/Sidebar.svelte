<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { formatSize } from "../types";
  import type { DriveInfo } from "../types";
  import { t } from "../i18n";

  const icons: Record<string, string> = {
    House: `<path d="M3 9.5L8 5L13 9.5V14H10V11H6V14H3V9.5Z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" fill="none"/><path d="M1 11L8 4.5L15 11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>`,
    Monitor: `<rect x="2" y="3" width="12" height="9" rx="1.5" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M6 15H10M8 12V15" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>`,
    Files: `<rect x="4" y="5" width="9" height="11" rx="1.5" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M4 5V4C4 3.45 4.45 3 5 3H9L12 6V5" stroke="currentColor" stroke-width="1.5" fill="none"/>`,
    DownloadSimple: `<path d="M8 3V11M8 11L5 8M8 11L11 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><path d="M3 14H13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>`,
    MusicNotes: `<path d="M9 13V4L14 3V12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/><circle cx="7" cy="13" r="2" stroke="currentColor" stroke-width="1.5" fill="none"/><circle cx="12" cy="12" r="2" stroke="currentColor" stroke-width="1.5" fill="none"/>`,
    Image: `<rect x="2" y="3" width="12" height="10" rx="1.5" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M2 10L5.5 7L8 9.5L11 6.5L14 10" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" fill="none"/><circle cx="5.5" cy="6" r="1.5" stroke="currentColor" stroke-width="1.3" fill="none"/>`,
    VideoCamera: `<path d="M2 5H10V11H2V5Z" rx="1" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" fill="none"/><path d="M10 7L14 5V11L10 9" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" fill="none"/>`,
    Users: `<circle cx="6" cy="6" r="2.5" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M1 14C1 11.24 3.24 9 6 9s5 2.24 5 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" fill="none"/><circle cx="11.5" cy="5.5" r="2" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M13.5 9c1.38 0 2.5 1.12 2.5 2.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" fill="none"/>`,
    HardDrives: `<rect x="2" y="3" width="12" height="5" rx="1.5" stroke="currentColor" stroke-width="1.5" fill="none"/><rect x="2" y="9" width="12" height="5" rx="1.5" stroke="currentColor" stroke-width="1.5" fill="none"/><circle cx="12" cy="5.5" r="1" fill="currentColor"/><circle cx="12" cy="11.5" r="1" fill="currentColor"/>`,
    Usb: `<path d="M8 2V10M8 2L6 4M8 2L10 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><circle cx="8" cy="12" r="2" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M5 7L8 10L11 7" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>`,
    Lock: `<rect x="4" y="7" width="8" height="7" rx="1.5" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M6 7V5C6 3.34 6.9 2 8 2s2 1.34 2 3v2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" fill="none"/><circle cx="8" cy="10.5" r="1" fill="currentColor"/>`,
    Eject: `<path d="M4 10L8 4L12 10H4Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="none"/><path d="M4 13H12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>`,
    Trash: `<path d="M3 5H13M5.5 5V3.5C5.5 3.22 5.72 3 6 3H10C10.28 3 10.5 3.22 10.5 3.5V5M12 5V13C12 13.55 11.55 14 11 14H5C4.45 14 4 13.55 4 13V5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" fill="none"/><path d="M6.5 7.5V11.5M9.5 7.5V11.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>`,
    Folder: `<path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="none"/>`,
    Pin: `<path d="M5 9L3 14L8 11M11 2L7 6V9L11 10L14 6L11 2Z" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>`,
    Plus: `<path d="M8 3V13M3 8H13" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>`,
    Star: `<path d="M8 2L9.8 5.6L13.8 6.2L10.9 9L11.6 13L8 11.1L4.4 13L5.1 9L2.2 6.2L6.2 5.6L8 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="none"/>`,
  };

  function renderIcon(name: string) {
    return icons[name] ?? icons["Folder"];
  }

  function isActive(path: string) {
    return fm.currentPath === path;
  }

  function onDriveClick(drive: DriveInfo) {
    if (drive.isMounted && drive.path) {
      fm.navigate(drive.path);
    } else {
      fm.mountDrive(drive);
    }
  }

  function addCurrentAsQuickAccess() {
    const name = fm.currentPath.split("/").filter(Boolean).pop() || fm.currentPath;
    fm.addQuickAccess(name, fm.currentPath);
  }
</script>

<aside class="sidebar">
  <div class="sidebar-scroll">
    <!-- Favorites / Places -->
    <section>
      <div class="section-label">
        <svg class="section-icon" width="11" height="11" viewBox="0 0 16 16" fill="none">
          {@html renderIcon("Star")}
        </svg>
        {t.favorites}
      </div>
      {#each fm.bookmarks as bm}
        <button
          class="sidebar-item"
          class:active={isActive(bm.path)}
          onclick={() => fm.navigate(bm.path)}
          title={bm.path}
          ondragover={(e) => { e.preventDefault(); if (e.dataTransfer) e.dataTransfer.dropEffect = "move"; fm.dropTarget = bm.path; }}
          ondragleave={() => { if (fm.dropTarget === bm.path) fm.dropTarget = null; }}
          ondrop={(e) => { e.preventDefault(); fm.dropOnto(bm.path); }}
          class:drop-target={fm.dropTarget === bm.path}
        >
          <svg class="sidebar-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
            {@html renderIcon(bm.icon)}
          </svg>
          <span>{bm.name}</span>
        </button>
      {/each}
    </section>

    <!-- Quick Access -->
    <section>
      <div class="section-label">
        <svg class="section-icon" width="11" height="11" viewBox="0 0 16 16" fill="none">
          {@html renderIcon("Pin")}
        </svg>
        {t.quickAccess}
        <button class="section-action" title={t.pinFolder} onclick={addCurrentAsQuickAccess}>
          <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
            {@html renderIcon("Plus")}
          </svg>
        </button>
      </div>
      {#if fm.quickAccess.length === 0}
        <div class="empty-hint">{t.pinFolderHint}</div>
      {:else}
        {#each fm.quickAccess as qa}
          <div class="qa-row" class:active={isActive(qa.path)}>
            <button
              class="sidebar-item qa-item"
              onclick={() => fm.navigate(qa.path)}
              title={qa.path}
              ondragover={(e) => { e.preventDefault(); if (e.dataTransfer) e.dataTransfer.dropEffect = "move"; fm.dropTarget = qa.path; }}
              ondragleave={() => { if (fm.dropTarget === qa.path) fm.dropTarget = null; }}
              ondrop={(e) => { e.preventDefault(); fm.dropOnto(qa.path); }}
              class:drop-target={fm.dropTarget === qa.path}
            >
              <svg class="sidebar-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
                {@html renderIcon(qa.icon)}
              </svg>
              <span>{qa.name}</span>
            </button>
            <button
              class="unpin-btn"
              title={t.unpin}
              onclick={(e) => { e.stopPropagation(); fm.removeQuickAccess(qa.path); }}
            >
              <svg width="10" height="10" viewBox="0 0 16 16" fill="none">
                <path d="M3 3L13 13M13 3L3 13" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              </svg>
            </button>
          </div>
        {/each}
      {/if}
    </section>

    <!-- Recent -->
    {#if fm.recentLocations.length > 0}
      <section>
        <div class="section-label">
          <svg class="section-icon" width="11" height="11" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5" fill="none"/>
            <path d="M8 5V8.5L10.5 10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          {t.recent}
          <button class="section-action" title={t.clearRecent} onclick={() => fm.clearRecentLocations()}>
            <svg width="10" height="10" viewBox="0 0 16 16" fill="none">
              <path d="M3 3L13 13M13 3L3 13" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </button>
        </div>
        {#each fm.recentLocations.slice(0, 10) as loc}
          <button
            class="sidebar-item"
            class:active={isActive(loc)}
            onclick={() => fm.navigate(loc)}
            title={loc}
          >
            <svg class="sidebar-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
              {@html renderIcon("Folder")}
            </svg>
            <span>{loc.split("/").filter(Boolean).pop() || loc}</span>
          </button>
        {/each}
      </section>
    {/if}

    <!-- Devices -->
    <section>
      <div class="section-label">
        <svg class="section-icon" width="11" height="11" viewBox="0 0 16 16" fill="none">
          {@html renderIcon("HardDrives")}
        </svg>
        {t.devices}
        <button class="section-action" title={t.refreshDevices} onclick={() => fm.refreshDrives()}>
          <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
            <path d="M13.5 2.5v4h-4M2.5 13.5v-4h4" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M13.5 6.5A6 6 0 1 1 9.5 2.5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
          </svg>
        </button>
      </div>
      {#each fm.drives as drive}
        <div class="drive-row" class:active={drive.isMounted && isActive(drive.path)}>
          <button
            class="sidebar-item drive-item"
            class:unmounted={!drive.isMounted}
            onclick={() => onDriveClick(drive)}
            title={drive.isMounted ? drive.path : `${drive.device} (not mounted)`}
          >
            <svg class="sidebar-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
              {@html renderIcon(drive.icon)}
            </svg>
            <span>{drive.name}</span>
            {#if !drive.isMounted}
              <span class="badge">{drive.isEncrypted ? t.locked : t.mount}</span>
            {/if}
          </button>
          {#if drive.isMounted && drive.isRemovable}
            <button
              class="eject-btn"
              title={t.eject}
              onclick={(e) => { e.stopPropagation(); fm.ejectDrive(drive); }}
            >
              <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
                {@html renderIcon("Eject")}
              </svg>
            </button>
          {/if}
        </div>
      {/each}
    </section>

    <!-- Spacer -->
    <div class="spacer"></div>

    <!-- Trash -->
    <section class="trash-section">
      <button
        class="sidebar-item trash-item"
        class:active={fm.showTrashView}
        onclick={() => fm.openTrash()}
      >
        <svg class="sidebar-icon trash-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
          {@html renderIcon("Trash")}
        </svg>
        <span>{t.trash}</span>
        {#if fm.trashSize > 0}
          <span class="trash-size">{formatSize(fm.trashSize)}</span>
        {/if}
      </button>
    </section>
  </div>
</aside>

<style>
  .sidebar {
    background: var(--app-bg);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    box-shadow: 6px 0 18px rgba(0,0,0,0.18);
  }

  .sidebar-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 6px 6px 8px;
    display: flex;
    flex-direction: column;
  }

  section {
    margin-bottom: 2px;
  }

  .spacer {
    flex: 1;
    min-height: 8px;
  }

  .trash-section {
    border-top: 1px solid var(--border-light);
    padding-top: 6px;
    margin-bottom: 0;
  }

  .section-label {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.1em;
    color: var(--overlay0);
    text-transform: uppercase;
    padding: 10px 10px 5px;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .section-icon {
    color: var(--overlay0);
    flex-shrink: 0;
  }

  .section-action {
    margin-left: auto;
    width: 20px;
    height: 20px;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay0);
    transition: background 0.1s, color 0.1s;
  }

  .section-action:hover {
    background: var(--surface0);
    color: var(--text);
  }

  .drive-row {
    display: flex;
    align-items: center;
    border-radius: 8px;
  }

  .drive-row.active {
    background: var(--hover-bg-strong);
  }

  .drive-row.active .sidebar-item {
    color: var(--mauve);
  }

  .drive-row.active .sidebar-icon {
    color: var(--mauve);
  }

  .qa-row {
    display: flex;
    align-items: center;
    border-radius: 8px;
  }

  .qa-row.active {
    background: var(--hover-bg-strong);
  }

  .qa-row.active .sidebar-item {
    color: var(--mauve);
  }

  .qa-row.active .sidebar-icon {
    color: var(--mauve);
  }

  .qa-row .unpin-btn {
    opacity: 0;
    transition: opacity 0.15s;
  }

  .qa-row:hover .unpin-btn {
    opacity: 1;
  }

  .sidebar-item {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 6px 10px;
    border-radius: 8px;
    color: var(--subtext1);
    font-size: 13px;
    font-weight: 450;
    text-align: left;
    transition: background 0.1s, color 0.1s;
    user-select: none;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sidebar-item:hover {
    background: var(--hover-bg);
    color: var(--text);
  }

  .sidebar-item.active {
    background: var(--hover-bg-strong);
    color: var(--mauve);
  }

  .sidebar-item.active .sidebar-icon {
    color: var(--mauve);
  }

  .sidebar-item.unmounted {
    opacity: 0.6;
  }

  .sidebar-item.drop-target {
    background: rgba(138, 173, 244, 0.2);
    color: var(--blue);
  }

  .sidebar-icon {
    flex-shrink: 0;
    color: var(--overlay1);
    transition: color 0.1s;
  }

  .sidebar-item:hover .sidebar-icon {
    color: var(--subtext0);
  }

  .drive-item, .qa-item {
    flex: 1;
    min-width: 0;
  }

  .badge {
    margin-left: auto;
    font-size: 9px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    padding: 1px 6px;
    border-radius: 4px;
    background: var(--hover-bg-strong);
    color: var(--mauve);
    flex-shrink: 0;
  }

  .eject-btn {
    width: 24px;
    height: 24px;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }

  .eject-btn:hover {
    background: var(--danger-bg);
    color: var(--red);
  }

  .unpin-btn {
    width: 22px;
    height: 22px;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay0);
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
    margin-right: 4px;
  }

  .unpin-btn:hover {
    background: var(--danger-bg);
    color: var(--red);
  }

  .empty-hint {
    font-size: 11px;
    color: var(--overlay0);
    padding: 4px 12px 8px;
    font-style: italic;
  }

  .trash-item {
    color: var(--subtext0);
  }

  .trash-item:hover {
    color: var(--text);
  }

  .trash-item.active {
    background: rgba(238, 153, 160, 0.12);
    color: var(--maroon);
  }

  .trash-item.active .trash-icon {
    color: var(--maroon);
  }

  .trash-icon {
    color: var(--overlay1);
  }

  .trash-item:hover .trash-icon {
    color: var(--subtext0);
  }

  .trash-size {
    margin-left: auto;
    font-size: 10px;
    font-weight: 600;
    color: var(--overlay0);
    flex-shrink: 0;
  }

  span {
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
