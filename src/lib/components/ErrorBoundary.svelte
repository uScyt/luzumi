<script lang="ts">
  import { onMount } from "svelte";
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();
  let crashed = $state(false);
  let errorMessage = $state("");
  let errorStack = $state("");

  onMount(() => {
    function handleError(event: ErrorEvent) {
      crashed = true;
      errorMessage = event.message || "Unknown error";
      errorStack = event.error?.stack ?? "";
      event.preventDefault();
    }

    function handleRejection(event: PromiseRejectionEvent) {
      // Don't crash for unhandled rejections, just log them
      console.error("Unhandled rejection:", event.reason);
    }

    window.addEventListener("error", handleError);
    window.addEventListener("unhandledrejection", handleRejection);

    return () => {
      window.removeEventListener("error", handleError);
      window.removeEventListener("unhandledrejection", handleRejection);
    };
  });

  function reload() {
    window.location.reload();
  }

  function dismiss() {
    crashed = false;
    errorMessage = "";
    errorStack = "";
  }
</script>

{#if crashed}
  <div class="error-boundary">
    <div class="error-content">
      <svg viewBox="0 0 24 24" width="48" height="48" class="error-icon">
        <path fill="currentColor" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
      </svg>
      <h2>Something went wrong</h2>
      <p class="error-msg">{errorMessage}</p>
      {#if errorStack}
        <pre class="error-stack">{errorStack}</pre>
      {/if}
      <div class="error-actions">
        <button class="reload-btn" onclick={reload}>Reload Application</button>
        <button class="dismiss-btn" onclick={dismiss}>Try to Continue</button>
      </div>
    </div>
  </div>
{:else}
  {@render children()}
{/if}

<style>
  .error-boundary {
    position: fixed;
    inset: 0;
    background: var(--bg-primary, #181825);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 99999;
  }

  .error-content {
    text-align: center;
    max-width: 500px;
    padding: 40px;
  }

  .error-icon {
    color: var(--danger, #f38ba8);
    margin-bottom: 16px;
  }

  h2 {
    color: var(--text-primary, #cdd6f4);
    font-size: 22px;
    margin: 0 0 12px;
  }

  .error-msg {
    color: var(--text-secondary, #a6adc8);
    font-size: 14px;
    margin: 0 0 16px;
    word-break: break-word;
  }

  .error-stack {
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--border-color, #45475a);
    border-radius: 8px;
    padding: 12px;
    font-size: 11px;
    color: var(--text-secondary, #a6adc8);
    text-align: left;
    overflow: auto;
    max-height: 200px;
    font-family: monospace;
    margin-bottom: 20px;
  }

  .error-actions {
    display: flex;
    gap: 12px;
    justify-content: center;
  }

  .reload-btn {
    background: var(--accent, #89b4fa);
    color: var(--bg-primary, #1e1e2e);
    border: none;
    border-radius: 8px;
    padding: 10px 24px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
  }

  .reload-btn:hover { opacity: 0.9; }

  .dismiss-btn {
    background: none;
    border: 1px solid var(--border-color, #45475a);
    color: var(--text-secondary, #a6adc8);
    border-radius: 8px;
    padding: 10px 24px;
    font-size: 14px;
    cursor: pointer;
  }

  .dismiss-btn:hover {
    background: var(--bg-hover, #313244);
    color: var(--text-primary, #cdd6f4);
  }
</style>
