<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";
  import { fm } from "../fileManager.svelte";

  const win = getCurrentWindow();

  let isMaximized = $state(false);
  let authenticating = $state(false);

  $effect(() => {
    win.isMaximized().then((v) => (isMaximized = v));
    const unlisten = win.onResized(() => {
      win.isMaximized().then((v) => (isMaximized = v));
    });
    return () => { unlisten.then((f) => f()); };
  });

  async function toggleElevated() {
    if (fm.elevated) {
      // Stop the watcher FIRST to prevent any more elevated reads
      fm.stopWatch();
      // Check if we need to navigate away before dropping privileges
      let needsNav = false;
      try {
        needsNav = !(await invoke<boolean>("cmd_check_path_readable", { path: fm.currentPath }));
      } catch {
        needsNav = true;
      }
      // Now disable elevated mode
      fm.elevated = false;
      // Kill root shell
      invoke("cmd_deauthenticate_admin").catch(() => {});
      // Navigate to home if current dir isn't readable
      if (needsNav) {
        try {
          const home = await invoke<string>("get_home_dir");
          await fm.navigate(home);
        } catch { /* ignore */ }
      } else {
        // Restart watcher for current (readable) directory
        fm.startWatch(fm.currentPath);
      }
      return;
    }
    if (authenticating) return;
    authenticating = true;
    try {
      await invoke("cmd_authenticate_admin");
      fm.elevated = true;
    } catch (_) {
      // Auth cancelled or failed
    } finally {
      authenticating = false;
    }
  }
</script>

<header class="titlebar" data-tauri-drag-region>
  <div class="left" data-tauri-drag-region>
    <div class="app-brand">
      <img src="/app-icon.png" alt="Luzumi" class="app-icon" />
      <span class="app-name">Luzumi</span>
    </div>
  </div>

  <div class="controls">
    <button
      class="ctrl-btn admin-btn"
      class:elevated={fm.elevated}
      class:authenticating={authenticating}
      title={fm.elevated ? "Admin mode (click to disable)" : "Enable admin mode"}
      onclick={toggleElevated}
    >
      {#if authenticating}
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none" class="spin">
          <path d="M8 2A6 6 0 1 0 14 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
        </svg>
      {:else if fm.elevated}
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
          <rect x="4" y="7" width="8" height="7" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/>
          <path d="M11 7V5C11 3.34 9.66 2 8 2S5 3.34 5 5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" fill="none"/>
          <circle cx="8" cy="10.5" r="1" fill="currentColor"/>
        </svg>
      {:else}
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
          <rect x="4" y="7" width="8" height="7" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/>
          <path d="M6 7V5C6 3.34 6.9 2 8 2s2 1.34 2 3v2" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" fill="none"/>
          <circle cx="8" cy="10.5" r="1" fill="currentColor"/>
        </svg>
      {/if}
    </button>
    <div class="sep"></div>
    <button class="ctrl-btn minimize" title="Minimize" onclick={() => win.minimize()}>
      <svg width="10" height="2" viewBox="0 0 10 2"><rect width="10" height="2" rx="1" fill="currentColor"/></svg>
    </button>
    <button class="ctrl-btn maximize" title={isMaximized ? "Restore" : "Maximize"} onclick={() => win.toggleMaximize()}>
      {#if isMaximized}
        <svg width="10" height="10" viewBox="0 0 10 10"><path d="M3 0H10V7H7V10H0V3H3V0ZM3 3H1V9H6V7H3V3ZM4 1V6H9V1H4Z" fill="currentColor"/></svg>
      {:else}
        <svg width="10" height="10" viewBox="0 0 10 10"><rect x="0.5" y="0.5" width="9" height="9" rx="1.5" stroke="currentColor" fill="none" stroke-width="1.2"/></svg>
      {/if}
    </button>
    <button class="ctrl-btn close" title="Close" onclick={() => win.close()}>
      <svg width="10" height="10" viewBox="0 0 10 10"><path d="M1 1L9 9M9 1L1 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
    </button>
  </div>
</header>

<style>
  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--app-bg);
    border-radius: 14px 14px 0 0;
    height: 38px;
    padding: 0 6px 0 14px;
    user-select: none;
    -webkit-user-select: none;
    border-bottom: 1px solid var(--border-subtle);
  }

  .left {
    display: flex;
    align-items: center;
    pointer-events: none;
  }

  .app-brand {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .app-icon {
    width: 18px;
    height: 18px;
    border-radius: 4px;
    object-fit: contain;
  }

  .app-name {
    font-size: 13px;
    font-weight: 600;
    color: var(--subtext1);
    letter-spacing: 0.02em;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .sep {
    width: 1px;
    height: 16px;
    background: var(--border-medium);
    margin: 0 4px;
  }

  .ctrl-btn {
    width: 32px;
    height: 32px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    transition: background 0.12s, color 0.12s;
  }

  .ctrl-btn:hover {
    background: var(--hover-bg);
    color: var(--text);
  }

  .close:hover {
    background: var(--danger-bg);
    color: var(--red);
  }

  .admin-btn.elevated {
    color: var(--peach);
  }

  .admin-btn.elevated:hover {
    background: rgba(245, 169, 127, 0.15);
    color: var(--peach);
  }

  .admin-btn.authenticating {
    opacity: 0.7;
    cursor: wait;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .spin {
    animation: spin 0.8s linear infinite;
  }
</style>
