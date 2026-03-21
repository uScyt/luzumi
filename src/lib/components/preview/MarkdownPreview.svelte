<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let { filePath }: { filePath: string } = $props();

  let html = $state("");
  let loading = $state(true);
  let error = $state<string | null>(null);

  $effect(() => {
    if (filePath) loadMarkdown(filePath);
  });

  async function loadMarkdown(path: string) {
    loading = true;
    error = null;
    try {
      const [content] = await invoke<[string, string]>("cmd_read_file_preview", { path, maxLines: 2000 });
      html = await invoke<string>("cmd_render_markdown", { content });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }
</script>

<div class="markdown-preview">
  {#if loading}
    <div class="md-loading">Loading...</div>
  {:else if error}
    <div class="md-error">{error}</div>
  {:else}
    <div class="md-content">
      {@html html}
    </div>
  {/if}
</div>

<style>
  .markdown-preview {
    height: 100%;
    overflow: auto;
  }

  .md-content {
    padding: 16px 20px;
    color: var(--text-primary, #cdd6f4);
    font-size: 14px;
    line-height: 1.7;
  }

  .md-content :global(h1) { font-size: 1.6em; margin: 0.8em 0 0.4em; border-bottom: 1px solid var(--border-color, #45475a); padding-bottom: 0.3em; }
  .md-content :global(h2) { font-size: 1.3em; margin: 0.7em 0 0.3em; }
  .md-content :global(h3) { font-size: 1.1em; margin: 0.6em 0 0.3em; }
  .md-content :global(p) { margin: 0.5em 0; }
  .md-content :global(code) { background: var(--bg-tertiary, #45475a); padding: 2px 5px; border-radius: 3px; font-size: 0.9em; }
  .md-content :global(pre) { background: var(--bg-tertiary, #45475a); padding: 12px; border-radius: 6px; overflow-x: auto; }
  .md-content :global(pre code) { background: none; padding: 0; }
  .md-content :global(a) { color: var(--accent, #89b4fa); }
  .md-content :global(blockquote) { border-left: 3px solid var(--accent, #89b4fa); margin: 0.5em 0; padding: 4px 12px; color: var(--text-secondary, #a6adc8); }
  .md-content :global(ul), .md-content :global(ol) { padding-left: 24px; }
  .md-content :global(li) { margin: 4px 0; }
  .md-content :global(table) { border-collapse: collapse; width: 100%; margin: 0.5em 0; }
  .md-content :global(th), .md-content :global(td) { border: 1px solid var(--border-color, #45475a); padding: 6px 10px; text-align: left; }
  .md-content :global(th) { background: var(--bg-tertiary, #45475a); }
  .md-content :global(img) { max-width: 100%; }
  .md-content :global(hr) { border: none; border-top: 1px solid var(--border-color, #45475a); margin: 1em 0; }
  .md-content :global(input[type="checkbox"]) { margin-right: 6px; }

  .md-loading, .md-error {
    padding: 24px;
    text-align: center;
    color: var(--text-secondary, #a6adc8);
  }

  .md-error { color: var(--danger, #f38ba8); }
</style>
