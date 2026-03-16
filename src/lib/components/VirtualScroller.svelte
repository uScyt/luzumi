<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    items,
    itemHeight,
    columns = 1,
    overscan = 5,
    children,
    class: className = "",
    style: extraStyle = "",
  }: {
    items: any[];
    itemHeight: number;
    columns?: number;
    overscan?: number;
    children: Snippet<[any[], number]>;
    class?: string;
    style?: string;
  } = $props();

  let scrollTop = $state(0);
  let clientHeight = $state(0);
  let containerEl = $state<HTMLElement | null>(null);

  const totalRows = $derived(Math.ceil(items.length / columns));
  const totalHeight = $derived(totalRows * itemHeight);
  const startRow = $derived(Math.max(0, Math.floor(scrollTop / itemHeight) - overscan));
  const endRow = $derived(Math.min(totalRows, Math.ceil((scrollTop + clientHeight) / itemHeight) + overscan));
  const startIndex = $derived(startRow * columns);
  const endIndex = $derived(Math.min(items.length, endRow * columns));
  const visibleItems = $derived(items.slice(startIndex, endIndex));
  const offsetY = $derived(startRow * itemHeight);

  function onScroll(e: Event) {
    scrollTop = (e.target as HTMLElement).scrollTop;
  }

  export function getContainer(): HTMLElement | null {
    return containerEl;
  }

  export function scrollToIndex(index: number) {
    if (!containerEl) return;
    const row = Math.floor(index / columns);
    containerEl.scrollTop = row * itemHeight;
  }
</script>

<div
  class="virtual-scroller {className}"
  style={extraStyle}
  bind:this={containerEl}
  bind:clientHeight
  onscroll={onScroll}
>
  <div class="virtual-spacer" style="height: {totalHeight}px">
    <div class="virtual-content" style="transform: translateY({offsetY}px)">
      {@render children(visibleItems, startIndex)}
    </div>
  </div>
</div>

<style>
  .virtual-scroller {
    overflow-y: auto;
    flex: 1;
  }

  .virtual-spacer {
    position: relative;
  }

  .virtual-content {
    will-change: transform;
  }
</style>
