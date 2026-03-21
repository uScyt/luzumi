<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fm } from "../fileManager.svelte";
  import { getFileIcon, getFileColor } from "../types";
  import type { FileEntry } from "../types";
  import { t } from "../i18n";
  import { renderIcon } from "../icons";
  import FileTooltip from "./FileTooltip.svelte";
  import VirtualScroller from "./VirtualScroller.svelte";

  const IMAGE_EXTS = new Set(["png","jpg","jpeg","gif","webp","bmp","svg","ico","tiff","tif","heic","heif","avif","jxl"]);
  const VIDEO_EXTS = new Set(["mp4","m4v","mkv","avi","mov","webm","flv","wmv","mpg","mpeg","ogv","ogg","3gp","ts","mts","m2ts","vob"]);

  function hasThumbnail(entry: FileEntry): boolean {
    if (entry.kind !== "file" || !entry.extension) return false;
    const ext = entry.extension.toLowerCase();
    return IMAGE_EXTS.has(ext) || VIDEO_EXTS.has(ext);
  }

  // Thumbnail cache — non-reactive internals to avoid cascade re-renders
  let thumbnails = $state<Map<string, string>>(new Map());
  const requestedThumbs = new Set<string>();
  let pendingThumbs: FileEntry[] = [];
  let activeLoads = 0;
  let thumbBatchTimer: ReturnType<typeof setTimeout> | null = null;
  const MAX_CONCURRENT_THUMBS = 6;

  function queueThumbnail(entry: FileEntry) {
    if (requestedThumbs.has(entry.path)) return;
    requestedThumbs.add(entry.path);
    pendingThumbs.push(entry);
    drainThumbQueue();
  }

  function drainThumbQueue() {
    while (activeLoads < MAX_CONCURRENT_THUMBS && pendingThumbs.length > 0) {
      const entry = pendingThumbs.shift()!;
      activeLoads++;
      invoke<string>("cmd_read_thumbnail", { path: entry.path })
        .then((data) => {
          thumbnails.set(entry.path, data);
          // Batch reactive updates: only trigger one re-render per frame
          if (!thumbBatchTimer) {
            thumbBatchTimer = setTimeout(() => {
              thumbBatchTimer = null;
              thumbnails = new Map(thumbnails);
            }, 50);
          }
        })
        .catch(() => {
          // Mark as failed so we show the fallback icon
          thumbnails.set(entry.path, "");
          if (!thumbBatchTimer) {
            thumbBatchTimer = setTimeout(() => {
              thumbBatchTimer = null;
              thumbnails = new Map(thumbnails);
            }, 50);
          }
        })
        .finally(() => {
          activeLoads--;
          drainThumbQueue();
        });
    }
  }

  // Rubber band selection
  let containerEl: HTMLElement;
  let rubberBox = $state<{ x1: number; y1: number; x2: number; y2: number } | null>(null);

  function onContainerMouseDown(e: MouseEvent) {
    if ((e.target as HTMLElement).closest(".grid-card")) return;
    if (e.button !== 0) return;
    e.preventDefault();
    if (!e.ctrlKey) fm.selected = new Set();
    rubberBox = { x1: e.clientX, y1: e.clientY, x2: e.clientX, y2: e.clientY };

    const onMove = (ev: MouseEvent) => {
      rubberBox = { ...rubberBox!, x2: ev.clientX, y2: ev.clientY };
      updateRubberSelection(ev.ctrlKey);
    };
    const onUp = () => {
      rubberBox = null;
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
    };
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
  }

  function updateRubberSelection(ctrl: boolean) {
    if (!rubberBox || !containerEl) return;
    const { x1, y1, x2, y2 } = rubberBox;
    const minX = Math.min(x1, x2), maxX = Math.max(x1, x2);
    const minY = Math.min(y1, y2), maxY = Math.max(y1, y2);
    const newSel = ctrl ? new Set(fm.selected) : new Set<string>();
    const scroller = containerEl.querySelector<HTMLElement>(".virtual-scroller") ?? containerEl;
    const rect = scroller.getBoundingClientRect();
    const scrollTop = scroller.scrollTop;
    const relMinY = minY - rect.top + scrollTop;
    const relMaxY = maxY - rect.top + scrollTop;
    const relMinX = minX - rect.left;
    const relMaxX = maxX - rect.left;
    const cols = gridColumns;
    const colWidth = (gridContainerWidth - 32) / cols; // 32=padding
    const startRow = Math.max(0, Math.floor(relMinY / cardRowHeight));
    const endRow = Math.min(Math.ceil(displayed.length / cols) - 1, Math.floor(relMaxY / cardRowHeight));
    const startCol = Math.max(0, Math.floor(relMinX / colWidth));
    const endCol = Math.min(cols - 1, Math.floor(relMaxX / colWidth));
    for (let row = startRow; row <= endRow; row++) {
      for (let col = startCol; col <= endCol; col++) {
        const idx = row * cols + col;
        if (idx < displayed.length) {
          newSel.add(displayed[idx].path);
        }
      }
    }
    fm.selected = newSel;
  }

  let renameInput = $state<HTMLInputElement | null>(null);

  // Tooltip
  let tooltipEntry = $state<FileEntry | null>(null);
  let tooltipPos = $state({ x: 0, y: 0 });
  let tooltipTimer: ReturnType<typeof setTimeout> | null = null;

  function onCardMouseEnter(e: MouseEvent, entry: FileEntry) {
    if (tooltipTimer) clearTimeout(tooltipTimer);
    tooltipTimer = setTimeout(() => {
      tooltipEntry = entry;
      tooltipPos = { x: e.clientX, y: e.clientY };
    }, 500);
  }

  function onCardMouseLeave() {
    if (tooltipTimer) { clearTimeout(tooltipTimer); tooltipTimer = null; }
    tooltipEntry = null;
  }

  function onCardMouseMove(e: MouseEvent) {
    if (tooltipEntry) tooltipPos = { x: e.clientX, y: e.clientY };
  }

  function startRename(entry: FileEntry) {
    fm.startRename(entry);
    requestAnimationFrame(() => {
      if (renameInput) {
        renameInput.focus();
        renameInput.select();
      }
    });
  }

  function commitRename() {
    if (!fm.renameTarget) return;
    const buf = fm.renameBuffer.trim();
    const entry = fm.entries.find((e) => e.path === fm.renameTarget);
    if (buf && entry && buf !== entry.name) {
      fm.rename(fm.renameTarget, buf);
    } else {
      fm.renameTarget = null;
    }
  }

  function onCardClick(e: MouseEvent, entry: FileEntry) {
    fm.toggleSelect(entry.path, e.ctrlKey || e.metaKey, e.shiftKey);
  }

  function onContextMenu(e: MouseEvent, entry: FileEntry | null) {
    e.preventDefault();
    e.stopPropagation();
    if (entry && !fm.selected.has(entry.path)) {
      fm.selected = new Set([entry.path]);
    }
    fm.contextMenu = {
      x: e.clientX,
      y: e.clientY,
      target: entry,
    };
  }

  function onDragStart(e: DragEvent, entry: FileEntry) {
    if (!e.dataTransfer) return;
    e.dataTransfer.effectAllowed = "copyMove";
    e.dataTransfer.setData("text/plain", entry.path);
    const paths = fm.selected.has(entry.path) ? [...fm.selected] : [entry.path];
    const uris = paths.map(p => "file://" + encodeURI(p)).join("\r\n");
    e.dataTransfer.setData("text/uri-list", uris);
    fm.setDragPaths(entry);
  }

  function onBgDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) e.dataTransfer.dropEffect = fm.dragPaths.length > 0 ? "move" : "copy";
    fm.dropTarget = null;
  }

  function onBgDrop(e: DragEvent) {
    e.preventDefault();
    if (fm.dragPaths.length > 0) {
      fm.requestDrop(fm.currentPath);
      return;
    }
    handleExternalDrop(e, fm.currentPath);
  }

  function handleExternalDrop(e: DragEvent, dest: string) {
    const uriList = e.dataTransfer?.getData("text/uri-list");
    if (uriList) {
      const paths = uriList.split("\r\n")
        .filter(u => u && !u.startsWith("#"))
        .map(u => decodeURI(u.replace(/^file:\/\//, "")))
        .filter(Boolean);
      if (paths.length > 0) {
        fm.handleExternalDrop(paths, dest);
        return;
      }
    }
  }

  function onCardDragEnter(e: DragEvent, entry: FileEntry) {
    if (entry.kind !== "directory") return;
    if (fm.dragPaths.includes(entry.path)) return;
    e.preventDefault();
    e.stopPropagation();
    fm.dropTarget = entry.path;
  }

  function onCardDragOver(e: DragEvent, entry: FileEntry) {
    if (entry.kind !== "directory") return;
    if (fm.dragPaths.includes(entry.path)) return;
    e.preventDefault();
    e.stopPropagation();
    if (e.dataTransfer) e.dataTransfer.dropEffect = fm.dragPaths.length > 0 ? "move" : "copy";
    fm.dropTarget = entry.path;
  }

  function onCardDrop(e: DragEvent, entry: FileEntry) {
    if (entry.kind !== "directory") return;
    e.preventDefault();
    e.stopPropagation();
    if (fm.dragPaths.length > 0) {
      fm.requestDrop(entry.path);
    } else {
      handleExternalDrop(e, entry.path);
    }
  }

  const displayed = $derived(fm.filteredEntries());

  // Virtual scrolling: compute grid columns from container width
  let gridContainerWidth = $state(0);
  const cardMinWidth = $derived(fm.gridIconSize + 40);
  // card padding (14+10=24 vertical) + icon + gap(7) + name (~32) => approximate row height
  const cardRowHeight = $derived(fm.gridIconSize + 24 + 7 + 32 + 4); // +4 for gap
  const gridColumns = $derived(Math.max(1, Math.floor((gridContainerWidth - 32) / (cardMinWidth + 4)) || 1)); // 32=padding, 4=gap

  let scrollerEl: VirtualScroller;

  // Only reload thumbnails when directory changes, not on every scroll/filter
  let lastThumbPath = "";
  $effect(() => {
    const path = fm.currentPath;
    if (path === lastThumbPath) return;
    lastThumbPath = path;
    // Clear caches for new directory
    requestedThumbs.clear();
    pendingThumbs = [];
    thumbnails = new Map();
    // Queue visible images
    for (const entry of fm.entries) {
      const ext = entry.extension?.toLowerCase() ?? "";
      const sizeLimit = VIDEO_EXTS.has(ext) ? 2 * 1024 * 1024 * 1024 : 10 * 1024 * 1024;
      if (hasThumbnail(entry) && (entry.size ?? 0) < sizeLimit) {
        queueThumbnail(entry);
      }
    }
  });

  $effect(() => {
    if (fm.pendingSelect && displayed.length > 0) {
      fm.consumePendingSelect(containerEl);
    }
  });

  function onScrollerResize(width: number, _height: number) {
    gridContainerWidth = width;
  }
</script>

<div class="grid-toolbar">
  {#each [['name', t.name], ['date', t.date], ['size', t.size], ['type', t.type]] as [field, label]}
    <button
      class="sort-btn"
      class:active={fm.sortBy === field}
      onclick={() => fm.setSort(field as "name" | "size" | "date" | "type")}
    >
      {label}
      {#if fm.sortBy === field}<span class="sort-arrow">{fm.sortDir === 'asc' ? '↑' : '↓'}</span>{/if}
    </button>
  {/each}
</div>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  class="grid-container"
  class:show-hover={fm.showHoverBox}
  style="--icon-size: {fm.gridIconSize}px; --card-min: {fm.gridIconSize + 40}px"
  role="grid"
  tabindex="0"
  bind:this={containerEl}
  oncontextmenu={(e) => onContextMenu(e, null)}
  ondragover={onBgDragOver}
  ondrop={onBgDrop}
  onmousedown={onContainerMouseDown}
  onkeydown={() => {}}
  onclick={(e) => { if (!(e.target as HTMLElement).closest(".grid-card")) fm.selected = new Set(); }}
>
  {#if displayed.length === 0}
    <div class="empty-state">
      <svg width="48" height="48" viewBox="0 0 16 16" fill="none" opacity=".2">
        <path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1" fill="currentColor" opacity=".2"/>
      </svg>
      <span>{fm.searchQuery ? t.noResults : t.emptyDirectory}</span>
    </div>
  {:else}
    <VirtualScroller items={displayed} itemHeight={cardRowHeight} columns={gridColumns} class="grid-scroll-area" onresize={onScrollerResize}>
      {#snippet children(visibleItems, _startIndex)}
        <div class="grid" style="grid-template-columns: repeat({gridColumns}, 1fr)">
          {#each visibleItems as entry (entry.path)}
            {@const iconName = getFileIcon(entry)}
            {@const iconColor = getFileColor(entry)}
            {@const isSelected = fm.selected.has(entry.path)}
            {@const isRenaming = fm.renameTarget === entry.path}
            {@const thumb = thumbnails.get(entry.path)}
            {@const showThumb = hasThumbnail(entry) && thumb}
            <div
              class="grid-card glass-card"
              class:selected={isSelected}
              class:cut={fm.clipboard?.mode === "cut" && fm.clipboard.paths.includes(entry.path)}
              class:drop-target={fm.dropTarget === entry.path}
              class:dragging={fm.dragPaths.includes(entry.path)}
              role="gridcell"
              tabindex="0"
              data-path={entry.path}
              draggable={!isRenaming ? "true" : undefined}
              onclick={(e) => onCardClick(e, entry)}
              ondblclick={() => { if (!isRenaming) fm.open(entry); }}
              oncontextmenu={(e) => onContextMenu(e, entry)}
              onkeydown={(e) => {
                if (e.key === "Enter") { fm.open(entry); return; }
                if (e.key === "F2") { startRename(entry); return; }
                if (["ArrowUp","ArrowDown","ArrowLeft","ArrowRight"].includes(e.key)) {
                  e.preventDefault();
                  const entries = fm.filteredEntries();
                  const idx = entries.findIndex(x => x.path === entry.path);
                  let next: typeof entry | undefined;
                  if (e.key === "ArrowRight" || e.key === "ArrowDown") next = entries[idx + 1];
                  else next = entries[idx - 1];
                  if (!next) return;
                  fm.toggleSelect(next.path, e.ctrlKey || e.metaKey, e.shiftKey);
                  requestAnimationFrame(() => {
                    containerEl?.querySelector<HTMLElement>(`[data-path="${CSS.escape(next!.path)}"]`)?.focus();
                  });
                }
              }}
              ondragstart={(e) => onDragStart(e, entry)}
              ondragend={() => { fm.dragPaths = []; fm.dropTarget = null; }}
              ondragenter={(e) => onCardDragEnter(e, entry)}
              ondragover={(e) => onCardDragOver(e, entry)}
              ondragleave={() => { if (fm.dropTarget === entry.path) fm.dropTarget = null; }}
              ondrop={(e) => onCardDrop(e, entry)}
              onmouseenter={(e) => onCardMouseEnter(e, entry)}
              onmouseleave={onCardMouseLeave}
              onmousemove={onCardMouseMove}
            >
              <div class="card-icon" class:folder={entry.kind === "directory"} class:has-thumb={showThumb}>
                {#if showThumb}
                  <img
                    src={thumb}
                    alt=""
                    class="thumb"
                    style="width: {fm.gridIconSize}px; height: {fm.gridIconSize}px"
                    onerror={() => { thumbnails.set(entry.path, ""); thumbnails = new Map(thumbnails); }}
                  />
                {:else}
                  <svg width={fm.gridIconSize} height={fm.gridIconSize} viewBox="0 0 16 16" style="color: {iconColor}">
                    {@html renderIcon(iconName)}
                  </svg>
                {/if}
                {#if !entry.isWritable}
                  <div class="lock-badge" title={t.noWritePermission}>
                    <svg width="8" height="8" viewBox="0 0 16 16" fill="none">
                      <rect x="3" y="7" width="10" height="8" rx="2" fill="var(--crust)" stroke="var(--peach)" stroke-width="2"/>
                      <path d="M6 7V5C6 3.34 6.9 2 8 2s2 1.34 2 3v2" stroke="var(--peach)" stroke-width="2" stroke-linecap="round" fill="none"/>
                    </svg>
                  </div>
                {/if}
              </div>
              {#if isRenaming}
                <input
                  class="rename-input"
                  bind:this={renameInput}
                  bind:value={fm.renameBuffer}
                  onclick={(e) => e.stopPropagation()}
                  ondblclick={(e) => e.stopPropagation()}
                  onkeydown={(e) => {
                    if (e.key === "Enter") { e.preventDefault(); commitRename(); }
                    if (e.key === "Escape") { e.preventDefault(); fm.renameTarget = null; }
                  }}
                  onblur={commitRename}
                />
              {:else}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <span
                  class="card-name"
                  class:broken-link={entry.isBrokenLink}
                  title={entry.isBrokenLink ? `${entry.name} (broken link)` : entry.name}
                  ondblclick={(e) => { e.stopPropagation(); startRename(entry); }}
                >{entry.name}</span>
                {@const gitSt = fm.getGitFileStatus(entry.path)}
                {#if gitSt}
                  <span class="git-dot git-{gitSt}"></span>
                {/if}
              {/if}
            </div>
          {/each}
        </div>
      {/snippet}
    </VirtualScroller>
  {/if}
</div>

{#if rubberBox}
  {@const left = Math.min(rubberBox.x1, rubberBox.x2)}
  {@const top = Math.min(rubberBox.y1, rubberBox.y2)}
  {@const width = Math.abs(rubberBox.x2 - rubberBox.x1)}
  {@const height = Math.abs(rubberBox.y2 - rubberBox.y1)}
  <div
    class="rubber-band"
    style="left:{left}px; top:{top}px; width:{width}px; height:{height}px;"
  ></div>
{/if}

{#if tooltipEntry}
  <FileTooltip entry={tooltipEntry} x={tooltipPos.x} y={tooltipPos.y} />
{/if}

<style>
  .grid-toolbar {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 5px 14px;
    background: var(--panel-bg);
    backdrop-filter: blur(12px);
    border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }

  .sort-btn {
    display: flex;
    align-items: center;
    gap: 3px;
    padding: 4px 10px;
    border-radius: 7px;
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--overlay1);
    background: none;
    cursor: pointer;
    transition: color 0.1s, background 0.1s;
  }

  .sort-btn:hover {
    color: var(--subtext1);
    background: var(--hover-bg);
  }

  .sort-btn.active {
    color: var(--mauve);
    background: var(--accent-subtle);
  }

  .sort-arrow {
    font-size: 10px;
  }

  .grid-container {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    user-select: none;
  }

  :global(.grid-scroll-area) {
    flex: 1;
    overflow-y: auto;
    padding: 14px 16px;
  }

  .grid {
    display: grid;
    gap: 4px;
  }

  .glass-card {
    background: transparent;
    border: 1px solid transparent;
  }

  .grid-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 7px;
    padding: 14px 8px 10px;
    border-radius: 12px;
    cursor: default;
    transition: background 0.12s, box-shadow 0.12s, border-color 0.12s, transform 0.1s;
    outline: none;
    position: relative;
  }

  .show-hover .grid-card:hover {
    background: var(--hover-bg);
    border-color: var(--border-light);
  }

  .grid-card.selected {
    background: var(--accent-muted);
    border-color: var(--accent-border);
    box-shadow: 0 0 0 1px var(--accent-glow), 0 4px 16px rgba(0, 0, 0, 0.12);
  }

  .grid-card.selected:hover {
    background: var(--accent-muted);
  }

  .grid-card.cut {
    opacity: 0.4;
  }

  .grid-card.dragging {
    opacity: 0.35;
    transform: scale(0.95);
  }

  .grid-card.drop-target {
    background: var(--blue-bg);
    border-color: var(--blue-border-strong);
    box-shadow: 0 0 0 1px var(--blue-bg-strong), 0 6px 20px var(--blue-glow);
    transform: scale(1.02);
  }

  .card-icon {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: var(--icon-size, 64px);
    height: var(--icon-size, 64px);
    border-radius: 14px;
    transition: transform 0.15s cubic-bezier(0.16, 1, 0.3, 1);
    overflow: visible;
    position: relative;
  }

  .grid-card:active .card-icon {
    transform: scale(0.94);
  }

  .card-icon.folder {
    background: transparent;
  }

  .card-icon.folder svg {
    transform: scale(1.18);
  }

  .card-icon.has-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 12px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.25);
    overflow: hidden;
  }

  .lock-badge {
    position: absolute;
    bottom: -2px;
    left: -2px;
    width: 14px;
    height: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    background: var(--crust);
    box-shadow: 0 1px 4px rgba(0,0,0,0.5);
  }

  .thumb {
    width: 56px;
    height: 56px;
    object-fit: cover;
    border-radius: 12px;
  }

  .card-name {
    font-size: 11.5px;
    color: var(--text);
    text-align: center;
    width: 100%;
    overflow: hidden;
    display: -webkit-box;
    line-clamp: 2;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    line-height: 1.4;
    word-break: break-word;
  }

  .card-name.broken-link {
    color: var(--red);
    opacity: 0.7;
    text-decoration: line-through;
  }

  .rename-input {
    width: 100%;
    background: var(--surface1);
    border: 1.5px solid var(--mauve);
    border-radius: 6px;
    padding: 2px 6px;
    color: var(--text);
    font-size: 11.5px;
    outline: none;
    text-align: center;
    height: 24px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    height: 300px;
    color: var(--overlay1);
    font-size: 14px;
  }

  .git-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    display: inline-block;
    flex-shrink: 0;
  }

  .git-dot.git-modified {
    background: var(--yellow);
  }

  .git-dot.git-staged {
    background: var(--green);
  }

  .git-dot.git-untracked {
    background: var(--overlay1);
  }

  :global(.rubber-band) {
    position: fixed;
    background: var(--accent-subtle);
    border: 1px solid var(--accent-border-strong);
    border-radius: 3px;
    pointer-events: none;
    z-index: 500;
  }
</style>
