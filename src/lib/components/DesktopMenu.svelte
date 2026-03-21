<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import { t } from "../i18n";

  interface ExtensionItem {
    label: string;
    icon: string | null;
    exec: string;
    separator_before: boolean | null;
  }

  let extensions = $state<ExtensionItem[]>([]);
  let ready = $state(false);

  const win = getCurrentWindow();

  // Inline SVG icon fragments (16x16 viewBox)
  const IC = {
    Folder:   `<path d="M2 4.5C2 3.67 2.67 3 3.5 3H6.17C6.7 3 7.2 3.21 7.59 3.59L8.41 4.41C8.8 4.79 9.3 5 9.83 5H12.5C13.33 5 14 5.67 14 6.5V11.5C14 12.33 13.33 13 12.5 13H3.5C2.67 13 2 12.33 2 11.5V4.5Z" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/>`,
    File:     `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>`,
    Paste:    `<rect x="4" y="1" width="8" height="2" rx="0.5" stroke="currentColor" stroke-width="1.2" fill="none"/><rect x="3" y="3" width="10" height="11" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/>`,
    Terminal: `<rect x="1.5" y="2.5" width="13" height="11" rx="2" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/><path d="M4.5 6L7 8.5L4.5 11" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" fill="none"/><path d="M8.5 11H11.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>`,
    App:      `<rect x="2" y="2" width="12" height="12" rx="3" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><circle cx="8" cy="8" r="2.5" stroke="currentColor" stroke-width="1.2" fill="none"/>`,
    Lock:     `<rect x="3" y="7" width="10" height="7" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M5 7V5a3 3 0 016 0v2" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>`,
    Logout:   `<path d="M9 2H12.5C13.33 2 14 2.67 14 3.5V12.5C14 13.33 13.33 14 12.5 14H9" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/><path d="M6 8H1.5M1.5 8L3.5 6M1.5 8L3.5 10" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" fill="none"/>`,
    Wallpaper:`<rect x="2" y="3" width="12" height="10" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/><circle cx="5.5" cy="6.5" r="1.5" stroke="currentColor" stroke-width="1" fill="currentColor" opacity=".2"/><path d="M2 11L5.5 8L8 10L10.5 7.5L14 11" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>`,
    Display:  `<rect x="2" y="2" width="12" height="9" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/><path d="M6 13H10" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><path d="M8 11V13" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>`,
    Extension:`<circle cx="8" cy="8" r="5.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/><path d="M8 5.5V10.5M5.5 8H10.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>`,
  };

  function ic(name: keyof typeof IC) {
    return `<svg width="14" height="14" viewBox="0 0 16 16" fill="none">${IC[name]}</svg>`;
  }

  async function closeMenu() {
    await win.close();
  }

  function action(fn: () => void | Promise<void>) {
    fn();
    closeMenu();
  }

  // ── Luzumi actions ──

  async function newFolder() {
    const home = await invoke<string>("get_home_dir");
    const desktop = `${home}/Desktop`;
    let name = "New Folder";
    let i = 1;
    // Try to find a non-conflicting name
    while (true) {
      try {
        await invoke("cmd_create_directory", { path: `${desktop}/${name}` });
        break;
      } catch {
        i++;
        name = `New Folder (${i})`;
        if (i > 50) break;
      }
    }
  }

  async function newFile() {
    const home = await invoke<string>("get_home_dir");
    const desktop = `${home}/Desktop`;
    let name = "untitled.txt";
    let i = 1;
    while (true) {
      try {
        await invoke("cmd_create_file", { path: `${desktop}/${name}` });
        break;
      } catch {
        i++;
        name = `untitled (${i}).txt`;
        if (i > 50) break;
      }
    }
  }

  async function paste() {
    // Read clipboard text — may contain file paths
    try {
      const text = await navigator.clipboard.readText();
      if (text) {
        // TODO: integrate with system clipboard for file paste
      }
    } catch { /* clipboard not available */ }
  }

  async function openTerminal() {
    const home = await invoke<string>("get_home_dir");
    await invoke("cmd_open_terminal", { path: `${home}/Desktop` });
  }

  async function openLuzumi() {
    const home = await invoke<string>("get_home_dir");
    // Open luzumi at Desktop folder
    await invoke("open_file", { path: `${home}/Desktop`, isDir: true, command: null });
  }

  // ── KDE actions ──

  function lockScreen() {
    invoke("cmd_desktop_action", { action: "lock-screen" });
  }

  function logout() {
    invoke("cmd_desktop_action", { action: "logout" });
  }

  function wallpaperSettings() {
    invoke("cmd_desktop_action", { action: "wallpaper-settings" });
  }

  function displayConfig() {
    invoke("cmd_desktop_action", { action: "display-config" });
  }

  function execExtension(exec: string) {
    invoke("cmd_exec_desktop_extension", { exec });
  }

  onMount(() => {
    let unlisten: (() => void) | undefined;

    (async () => {
      // Load extensions
      try {
        extensions = await invoke<ExtensionItem[]>("cmd_get_desktop_menu_extensions");
      } catch { /* no extensions */ }
      ready = true;

      // Close on focus loss
      unlisten = await win.onFocusChanged(({ payload: focused }) => {
        if (!focused) closeMenu();
      });
    })();

    return () => { unlisten?.(); };
  });

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") closeMenu();
  }
</script>

<svelte:window onkeydown={onKeydown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="desktop-menu-backdrop" onclick={closeMenu}>
  {#if ready}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div class="menu" onclick={(e) => e.stopPropagation()}>
      <!-- ── Section: Luzumi file actions ── -->
      <div class="section-label">{t.appName ?? "Luzumi"}</div>

      <button class="menu-item" onclick={() => action(newFolder)}>
        {@html ic("Folder")}
        <span>{t.newFolder}</span>
        <span class="hint">Ctrl+Shift+N</span>
      </button>

      <button class="menu-item" onclick={() => action(newFile)}>
        {@html ic("File")}
        <span>{t.newFile}</span>
      </button>

      <button class="menu-item" onclick={() => action(paste)}>
        {@html ic("Paste")}
        <span>{t.paste}</span>
        <span class="hint">Ctrl+V</span>
      </button>

      <div class="sep"></div>

      <button class="menu-item" onclick={() => action(openTerminal)}>
        {@html ic("Terminal")}
        <span>{t.openTerminal}</span>
      </button>

      <button class="menu-item" onclick={() => action(openLuzumi)}>
        {@html ic("App")}
        <span>{t.openLuzumi}</span>
      </button>

      <div class="sep"></div>

      <!-- ── Section: KDE system actions ── -->
      <div class="section-label">{t.desktopActions}</div>

      <button class="menu-item" onclick={() => action(lockScreen)}>
        {@html ic("Lock")}
        <span>{t.lockScreen}</span>
      </button>

      <button class="menu-item" onclick={() => action(logout)}>
        {@html ic("Logout")}
        <span>{t.showLogoutScreen}</span>
      </button>

      <button class="menu-item" onclick={() => action(wallpaperSettings)}>
        {@html ic("Wallpaper")}
        <span>{t.wallpaperSettings}</span>
      </button>

      <button class="menu-item" onclick={() => action(displayConfig)}>
        {@html ic("Display")}
        <span>{t.displayConfiguration}</span>
      </button>

      {#if extensions.length > 0}
        <div class="sep"></div>

        <!-- ── Section: Third-party extensions ── -->
        <div class="section-label">{t.extensions}</div>

        {#each extensions as ext}
          {#if ext.separator_before}
            <div class="sep"></div>
          {/if}
          <button class="menu-item" onclick={() => action(() => execExtension(ext.exec))}>
            {@html ic("Extension")}
            <span>{ext.label}</span>
          </button>
        {/each}
      {/if}
    </div>
  {/if}
</div>

<style>
  .desktop-menu-backdrop {
    position: fixed;
    inset: 0;
    z-index: 9999;
    /* transparent to allow clicking outside to close */
  }

  .menu {
    position: absolute;
    top: 0;
    left: 0;
    background: var(--glass-bg-strong);
    backdrop-filter: var(--glass-blur);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 12px;
    padding: 5px;
    min-width: 240px;
    max-width: 320px;
    box-shadow: 0 12px 40px rgba(0,0,0,0.55), 0 0 0 0.5px rgba(255,255,255,0.05);
    animation: pop 0.12s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes pop {
    from { opacity: 0; transform: scale(0.95) translateY(-4px); }
    to   { opacity: 1; transform: scale(1) translateY(0); }
  }

  .section-label {
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.6px;
    color: var(--overlay0);
    padding: 4px 12px 2px;
    pointer-events: none;
    user-select: none;
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
    background: none;
    border: none;
    cursor: pointer;
    font-family: inherit;
  }

  .menu-item:hover {
    background: var(--hover-bg);
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
</style>
