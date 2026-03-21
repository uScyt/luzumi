<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let { filePath }: { filePath: string } = $props();

  let content = $state("");
  let language = $state("plaintext");
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    if (filePath) loadPreview(filePath);
  });

  async function loadPreview(path: string) {
    loading = true;
    error = null;
    try {
      const [text, lang] = await invoke<[string, string]>("cmd_read_file_preview", { path, maxLines: 500 });
      content = text;
      language = lang;
    } catch (e) {
      error = String(e);
      content = "";
    } finally {
      loading = false;
    }
  }
</script>

<div class="code-preview">
  {#if loading}
    <div class="code-loading">Loading...</div>
  {:else if error}
    <div class="code-error">{error}</div>
  {:else}
    <div class="code-header">
      <span class="code-language">{language}</span>
    </div>
    <pre class="code-content"><code>{content}</code></pre>
  {/if}
</div>

<style>
  .code-preview {
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .code-header {
    display: flex;
    align-items: center;
    padding: 6px 12px;
    border-bottom: 1px solid var(--border-color, #45475a);
    flex-shrink: 0;
  }

  .code-language {
    font-size: 11px;
    color: var(--accent, #89b4fa);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-weight: 600;
  }

  .code-content {
    flex: 1;
    overflow: auto;
    padding: 12px;
    margin: 0;
    font-family: "JetBrains Mono", "Fira Code", "Source Code Pro", "Cascadia Code", monospace;
    font-size: 12px;
    line-height: 1.6;
    color: var(--text-primary, #cdd6f4);
    tab-size: 4;
    white-space: pre;
  }

  .code-loading, .code-error {
    padding: 24px;
    text-align: center;
    color: var(--text-secondary, #a6adc8);
    font-size: 13px;
  }

  .code-error {
    color: var(--danger, #f38ba8);
  }
</style>
