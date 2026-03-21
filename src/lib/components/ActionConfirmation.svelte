<script lang="ts">
  import { fm } from "../fileManager.svelte";

  let visible = $state(false);
  let message = $state("");
  let icon = $state<"success" | "error" | "info">("success");
  let timer: ReturnType<typeof setTimeout> | null = null;

  // Watch for status changes
  $effect(() => {
    const msg = fm.statusMessage;
    if (msg) {
      message = msg;
      icon = "success";
      visible = true;
      if (timer) clearTimeout(timer);
      timer = setTimeout(() => { visible = false; }, 3500);
    }
  });

  $effect(() => {
    const err = fm.error;
    if (err) {
      message = err;
      icon = "error";
      visible = true;
      if (timer) clearTimeout(timer);
      timer = setTimeout(() => { visible = false; fm.clearError(); }, 5000);
    }
  });
</script>

{#if visible}
  <div class="toast" class:error={icon === "error"} class:success={icon === "success"}>
    <div class="toast-icon">
      {#if icon === "success"}
        <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/></svg>
      {:else if icon === "error"}
        <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/></svg>
      {:else}
        <svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/></svg>
      {/if}
    </div>
    <span class="toast-message">{message}</span>
    <button class="toast-close" aria-label="Close" onclick={() => { visible = false; fm.clearError(); }}>
      <svg viewBox="0 0 24 24" width="14" height="14"><path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
    </button>
  </div>
{/if}

<style>
  .toast {
    position: fixed;
    bottom: 40px;
    right: 20px;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 16px;
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 8px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
    z-index: 9998;
    animation: slideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    max-width: 400px;
  }

  .toast.success {
    border-left: 3px solid var(--success, #a6e3a1);
  }

  .toast.error {
    border-left: 3px solid var(--danger, #f38ba8);
  }

  .toast-icon {
    flex-shrink: 0;
  }

  .toast.success .toast-icon { color: var(--success, #a6e3a1); }
  .toast.error .toast-icon { color: var(--danger, #f38ba8); }

  .toast-message {
    flex: 1;
    color: var(--text-primary, #cdd6f4);
    font-size: 13px;
    line-height: 1.4;
  }

  .toast-close {
    background: none;
    border: none;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
    padding: 2px;
    border-radius: 4px;
    flex-shrink: 0;
  }

  .toast-close:hover {
    background: var(--bg-hover, #313244);
    color: var(--text-primary, #cdd6f4);
  }

  @keyframes slideIn {
    from {
      transform: translateY(20px) scale(0.95);
      opacity: 0;
    }
    to {
      transform: translateY(0) scale(1);
      opacity: 1;
    }
  }
</style>
