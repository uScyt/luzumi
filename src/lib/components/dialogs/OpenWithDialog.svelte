<script lang="ts">
  import { fm } from "../../fileManager.svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface DesktopApp {
    name: string;
    exec: string;
    icon: string;
    desktop_id: string;
  }

  let apps = $state<DesktopApp[]>([]);
  let customCommand = $state("");
  let loading = $state(true);
  let setDefault = $state(false);

  let {
    filePath,
    mimeType,
    onClose,
  }: {
    filePath: string;
    mimeType?: string;
    onClose: () => void;
  } = $props();

  $effect(() => {
    if (filePath) loadApps();
  });

  async function loadApps() {
    loading = true;
    try {
      apps = await invoke<DesktopApp[]>("cmd_list_open_with_apps", { path: filePath });
    } catch {
      apps = [];
    }
    loading = false;
  }

  async function openWith(app: DesktopApp) {
    try {
      await invoke("cmd_open_with_app", { path: filePath, desktopId: app.desktop_id });
      if (setDefault && mimeType) {
        try {
          await invoke("cmd_set_default_app", { mimeType, desktopId: app.desktop_id });
        } catch {}
      }
      onClose();
    } catch (e) {
      fm.ui.error = `Failed to open: ${e}`;
    }
  }

  async function openCustom() {
    if (!customCommand.trim()) return;
    try {
      await invoke("cmd_open_with_command", { path: filePath, command: customCommand.trim() });
      onClose();
    } catch (e) {
      fm.ui.error = `Failed to open: ${e}`;
    }
  }

  function fileName(): string {
    return filePath.split("/").pop() ?? filePath;
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") openCustom();
    if (e.key === "Escape") onClose();
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="openwith-overlay" onclick={onClose}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="openwith-dialog" onclick={(e) => e.stopPropagation()}>
    <div class="openwith-header">
      <h3>Open With</h3>
      <button class="close-btn" onclick={onClose} aria-label="Close">
        <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
      </button>
    </div>

    <p class="openwith-file">Open <strong>{fileName()}</strong> with:</p>

    <div class="app-list">
      {#if loading}
        <div class="app-loading">Loading applications...</div>
      {:else if apps.length === 0}
        <div class="app-empty">No applications found</div>
      {:else}
        {#each apps as app (app.desktop_id)}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div class="app-item" ondblclick={() => openWith(app)} onclick={() => openWith(app)}>
            <span class="app-icon">
              {#if app.icon}
                <img src={app.icon} alt="" width="24" height="24" onerror={(e) => (e.currentTarget as HTMLImageElement).style.display = 'none'} />
              {:else}
                <svg viewBox="0 0 24 24" width="24" height="24"><path fill="currentColor" d="M6 2c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6H6z"/></svg>
              {/if}
            </span>
            <div class="app-info">
              <span class="app-name">{app.name}</span>
              <span class="app-exec">{app.exec}</span>
            </div>
          </div>
        {/each}
      {/if}
    </div>

    <div class="custom-section">
      <input
        bind:value={customCommand}
        type="text"
        placeholder="Custom command..."
        onkeydown={onKeydown}
      />
      <button class="run-btn" onclick={openCustom} disabled={!customCommand.trim()}>Run</button>
    </div>

    {#if mimeType}
      <label class="default-check">
        <input type="checkbox" bind:checked={setDefault} />
        Set as default for {mimeType}
      </label>
    {/if}
  </div>
</div>

<style>
  .openwith-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 10000;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .openwith-dialog {
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 12px;
    width: 380px;
    max-height: 70vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.5);
  }

  .openwith-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .openwith-header h3 { margin: 0; font-size: 16px; color: var(--text-primary, #cdd6f4); }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
  }

  .close-btn:hover { background: var(--bg-hover, #313244); }

  .openwith-file {
    padding: 10px 20px;
    font-size: 13px;
    color: var(--text-secondary, #a6adc8);
    margin: 0;
  }

  .openwith-file strong { color: var(--text-primary, #cdd6f4); }

  .app-list {
    flex: 1;
    overflow-y: auto;
    padding: 0 8px;
    max-height: 300px;
  }

  .app-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    border-radius: 6px;
    cursor: pointer;
  }

  .app-item:hover { background: var(--bg-hover, #313244); }

  .app-icon {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary, #a6adc8);
    flex-shrink: 0;
  }

  .app-info {
    flex: 1;
    min-width: 0;
  }

  .app-name {
    display: block;
    font-size: 13px;
    color: var(--text-primary, #cdd6f4);
    font-weight: 500;
  }

  .app-exec {
    display: block;
    font-size: 10px;
    color: var(--text-secondary, #a6adc8);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .app-loading, .app-empty {
    padding: 20px;
    text-align: center;
    font-size: 13px;
    color: var(--text-secondary, #a6adc8);
  }

  .custom-section {
    display: flex;
    gap: 6px;
    padding: 10px 20px;
    border-top: 1px solid var(--border-color, #45475a);
  }

  .custom-section input {
    flex: 1;
    background: var(--bg-primary, #181825);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 6px;
    color: var(--text-primary, #cdd6f4);
    padding: 6px 10px;
    font-size: 12px;
    outline: none;
  }

  .run-btn {
    background: var(--accent, #89b4fa);
    color: var(--bg-primary, #1e1e2e);
    border: none;
    border-radius: 6px;
    padding: 6px 14px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
  }

  .run-btn:disabled { opacity: 0.4; cursor: default; }

  .default-check {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 20px 14px;
    font-size: 12px;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
  }
</style>
