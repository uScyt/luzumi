<script lang="ts">
  let { name, color, removable = false, onremove }: {
    name: string;
    color: string;
    removable?: boolean;
    onremove?: () => void;
  } = $props();
</script>

<span class="tag-badge" style="--tag-color: {color}">
  <span class="tag-dot"></span>
  <span class="tag-name">{name}</span>
  {#if removable && onremove}
    <button class="tag-remove" aria-label="Remove tag" onclick={onremove}>
      <svg viewBox="0 0 24 24" width="10" height="10"><path fill="currentColor" d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>
    </button>
  {/if}
</span>

<style>
  .tag-badge {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 1px 7px;
    border-radius: 10px;
    background: color-mix(in srgb, var(--tag-color) 15%, transparent);
    border: 1px solid color-mix(in srgb, var(--tag-color) 30%, transparent);
    font-size: 10px;
    white-space: nowrap;
    max-width: 100px;
  }

  .tag-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--tag-color);
    flex-shrink: 0;
  }

  .tag-name {
    color: var(--text-primary, #cdd6f4);
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tag-remove {
    background: none;
    border: none;
    color: var(--text-secondary, #a6adc8);
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .tag-remove:hover {
    color: var(--danger, #f38ba8);
  }
</style>
