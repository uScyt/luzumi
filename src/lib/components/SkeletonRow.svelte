<script lang="ts">
  let { count = 8, type = "list" }: { count?: number; type?: "list" | "grid" } = $props();
</script>

{#if type === "list"}
  <div class="skeleton-list">
    {#each Array(count) as _, i}
      <div class="skeleton-row" style="animation-delay: {i * 50}ms">
        <div class="skeleton-icon"></div>
        <div class="skeleton-name" style="width: {40 + Math.random() * 40}%"></div>
        <div class="skeleton-meta"></div>
        <div class="skeleton-meta short"></div>
      </div>
    {/each}
  </div>
{:else}
  <div class="skeleton-grid">
    {#each Array(count) as _, i}
      <div class="skeleton-card" style="animation-delay: {i * 50}ms">
        <div class="skeleton-thumb"></div>
        <div class="skeleton-label"></div>
      </div>
    {/each}
  </div>
{/if}

<style>
  .skeleton-list {
    padding: 4px 0;
  }

  .skeleton-row {
    display: flex;
    align-items: center;
    padding: 6px 16px;
    gap: 12px;
    animation: shimmer 1.5s ease-in-out infinite;
  }

  .skeleton-icon {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    background: var(--bg-tertiary, #45475a);
  }

  .skeleton-name {
    height: 14px;
    border-radius: 4px;
    background: var(--bg-tertiary, #45475a);
  }

  .skeleton-meta {
    width: 60px;
    height: 12px;
    border-radius: 4px;
    background: var(--bg-tertiary, #45475a);
    margin-left: auto;
  }

  .skeleton-meta.short {
    width: 40px;
    margin-left: 12px;
  }

  .skeleton-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    padding: 16px;
  }

  .skeleton-card {
    width: 80px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    animation: shimmer 1.5s ease-in-out infinite;
  }

  .skeleton-thumb {
    width: 56px;
    height: 56px;
    border-radius: 6px;
    background: var(--bg-tertiary, #45475a);
  }

  .skeleton-label {
    width: 60px;
    height: 10px;
    border-radius: 3px;
    background: var(--bg-tertiary, #45475a);
  }

  @keyframes shimmer {
    0%, 100% { opacity: 0.4; }
    50% { opacity: 0.8; }
  }
</style>
