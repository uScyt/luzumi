<script lang="ts">
  let { size = 24, color = "var(--accent, #89b4fa)" }: { size?: number; color?: string } = $props();
</script>

<svg
  class="spinner"
  width={size}
  height={size}
  viewBox="0 0 24 24"
  style="--spinner-color: {color}"
>
  <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" opacity="0.2" />
  <path
    d="M12 2a10 10 0 0 1 10 10"
    stroke="var(--spinner-color)"
    stroke-width="3"
    fill="none"
    stroke-linecap="round"
  />
</svg>

<style>
  .spinner {
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
