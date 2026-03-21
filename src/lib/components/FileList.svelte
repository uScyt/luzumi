<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { getFileIcon, getFileColor, formatSize, formatDate } from "../types";
  import type { FileEntry } from "../types";
  import { t } from "../i18n";
  import { renderIcon } from "../icons";
  import FileTooltip from "./FileTooltip.svelte";
  import SkeletonRow from "./SkeletonRow.svelte";
  import VirtualScroller from "./VirtualScroller.svelte";

  // Rubber band selection
  let containerEl: HTMLElement;
  let rubberBox = $state<{ x1: number; y1: number; x2: number; y2: number } | null>(null);

  function onContainerMouseDown(e: MouseEvent) {
    if ((e.target as HTMLElement).closest(".file-row")) return;
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
    const { y1, y2 } = rubberBox;
    const minY = Math.min(y1, y2), maxY = Math.max(y1, y2);
    const newSel = ctrl ? new Set(fm.selected) : new Set<string>();
    const scroller = containerEl.querySelector<HTMLElement>(".virtual-scroller") ?? containerEl;
    const rect = scroller.getBoundingClientRect();
    const scrollTop = scroller.scrollTop;
    const relMinY = minY - rect.top + scrollTop;
    const relMaxY = maxY - rect.top + scrollTop;
    const startIdx = Math.max(0, Math.floor(relMinY / listRowHeight));
    const endIdx = Math.min(displayed.length - 1, Math.floor(relMaxY / listRowHeight));
    for (let i = startIdx; i <= endIdx; i++) {
      newSel.add(displayed[i].path);
    }
    fm.selected = newSel;
  }

  function onRowDragEnter(e: DragEvent, entry: FileEntry) {
    if (entry.kind !== "directory") return;
    if (fm.dragPaths.includes(entry.path)) return;
    e.preventDefault();
    e.stopPropagation();
    fm.dropTarget = entry.path;
  }

  let renameInput = $state<HTMLInputElement | null>(null);

  $effect(() => {
    if (renameInput && fm.renameTarget) {
      renameCommitted = false;
      renameInput.focus();
      renameInput.select();
    }
  });

  // Tooltip
  let tooltipEntry = $state<FileEntry | null>(null);
  let tooltipPos = $state({ x: 0, y: 0 });
  let tooltipTimer: ReturnType<typeof setTimeout> | null = null;

  function onRowMouseEnter(e: MouseEvent, entry: FileEntry) {
    if (tooltipTimer) clearTimeout(tooltipTimer);
    tooltipTimer = setTimeout(() => {
      tooltipEntry = entry;
      tooltipPos = { x: e.clientX, y: e.clientY };
    }, 500);
  }

  function onRowMouseLeave() {
    if (tooltipTimer) { clearTimeout(tooltipTimer); tooltipTimer = null; }
    tooltipEntry = null;
  }

  function onRowMouseMove(e: MouseEvent) {
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

  let renameCommitted = false;

  function commitRename() {
    if (renameCommitted || !fm.renameTarget) return;
    renameCommitted = true;
    const target = fm.renameTarget;
    const buf = fm.renameBuffer.trim();
    const entry = fm.entries.find((e) => e.path === target);
    if (buf && entry && buf !== entry.name) {
      fm.rename(target, buf);
    } else {
      fm.renameTarget = null;
    }
  }

  function onRowClick(e: MouseEvent, entry: FileEntry) {
    fm.toggleSelect(entry.path, e.ctrlKey || e.metaKey, e.shiftKey);
  }

  function onRowDblClick(entry: FileEntry) {
    fm.open(entry);
  }

  function onRowKeyDown(e: KeyboardEvent, entry: FileEntry) {
    if (e.key === "Enter") { if (!fm.renameTarget) onRowDblClick(entry); return; }
    if (e.key === "F2") {
      e.preventDefault();
      fm.startRename(entry);
      requestAnimationFrame(() => { renameInput?.focus(); renameInput?.select(); });
      return;
    }
    if (e.key === "ArrowUp" || e.key === "ArrowDown") {
      e.preventDefault();
      const entries = fm.filteredEntries();
      const idx = entries.findIndex(x => x.path === entry.path);
      const next = e.key === "ArrowUp" ? entries[idx - 1] : entries[idx + 1];
      if (!next) return;
      fm.toggleSelect(next.path, e.ctrlKey || e.metaKey, e.shiftKey);
      requestAnimationFrame(() => {
        containerEl?.querySelector<HTMLElement>(`[data-path="${CSS.escape(next.path)}"]`)?.focus();
      });
    }
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

  const displayed = $derived(fm.filteredEntries());

  const listIconSize = $derived(Math.max(16, Math.min(28, Math.round(fm.gridIconSize * 0.38))));
  const listRowHeight = $derived(Math.max(30, Math.round(fm.gridIconSize * 0.53)));
  const colIconWidth = $derived(listIconSize + 14);

  $effect(() => {
    if (fm.pendingSelect && displayed.length > 0) {
      fm.consumePendingSelect(containerEl);
    }
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
<div
  class="list-container"
  role="listbox"
  aria-label="File list"
  tabindex="0"
  style="--row-h: {listRowHeight}px; --icon-s: {listIconSize}px; --col-icon-w: {colIconWidth}px"
  bind:this={containerEl}
  oncontextmenu={(e) => onContextMenu(e, null)}
  ondragover={(e) => { e.preventDefault(); if (e.dataTransfer) e.dataTransfer.dropEffect = fm.dragPaths.length > 0 ? "move" : "copy"; fm.dropTarget = null; }}
  ondrop={(e) => { e.preventDefault(); if (fm.dragPaths.length > 0) { fm.requestDrop(fm.currentPath); } else { const uriList = e.dataTransfer?.getData("text/uri-list"); if (uriList) { const paths = uriList.split("\r\n").filter((u: string) => u && !u.startsWith("#")).map((u: string) => decodeURI(u.replace(/^file:\/\//, ""))).filter(Boolean); if (paths.length > 0) fm.handleExternalDrop(paths, fm.currentPath); } } }}
  onmousedown={onContainerMouseDown}
  onclick={(e) => { if (!(e.target as HTMLElement).closest(".file-row")) fm.selected = new Set(); }}
  onkeydown={(e) => { if (e.key === "Escape") fm.selected = new Set(); }}
>
  <div class="list-header">
    <div class="col-icon"></div>
    <button class="col-name col-sort" class:active-sort={fm.sortBy === 'name'} onclick={() => fm.setSort('name')}>
      {t.name}
      {#if fm.sortBy === 'name'}<span class="sort-arrow">{fm.sortDir === 'asc' ? '↑' : '↓'}</span>{/if}
    </button>
    <button class="col-size col-sort" class:active-sort={fm.sortBy === 'size'} onclick={() => fm.setSort('size')}>
      {t.size}
      {#if fm.sortBy === 'size'}<span class="sort-arrow">{fm.sortDir === 'asc' ? '↑' : '↓'}</span>{/if}
    </button>
    <button class="col-date col-sort" class:active-sort={fm.sortBy === 'date'} onclick={() => fm.setSort('date')}>
      {t.modified}
      {#if fm.sortBy === 'date'}<span class="sort-arrow">{fm.sortDir === 'asc' ? '↑' : '↓'}</span>{/if}
    </button>
  </div>

  {#if fm.isLoading && displayed.length === 0}
    <div class="list-body">
      <SkeletonRow type="list" count={10} />
    </div>
  {:else if displayed.length === 0}
    <div class="list-body">
      <div class="empty-state">
        <svg width="40" height="40" viewBox="0 0 16 16" fill="none" opacity=".25">
          <path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1" fill="currentColor" opacity=".2"/>
        </svg>
        <span>{fm.searchQuery ? t.noResults : t.emptyDirectory}</span>
      </div>
    </div>
  {:else}
    <VirtualScroller items={displayed} itemHeight={listRowHeight} class="list-body" style="padding: 4px 8px">
      {#snippet children(visibleItems, _startIndex)}
        {#each visibleItems as entry (entry.path)}
          {@const iconName = getFileIcon(entry)}
          {@const iconColor = getFileColor(entry)}
          {@const isSelected = fm.selected.has(entry.path)}
          {@const isRenaming = fm.renameTarget === entry.path}
          <div
            class="file-row"
            class:selected={isSelected}
            class:cut={fm.clipboard?.mode === "cut" && fm.clipboard.paths.includes(entry.path)}
            class:drop-target={fm.dropTarget === entry.path}
            class:dragging={fm.dragPaths.includes(entry.path)}
            class:hidden-entry={entry.isHidden}
            role="row"
            tabindex="0"
            data-path={entry.path}
            draggable={!isRenaming ? "true" : undefined}
            onclick={(e) => onRowClick(e, entry)}
            ondblclick={() => onRowDblClick(entry)}
            oncontextmenu={(e) => onContextMenu(e, entry)}
            onkeydown={(e) => onRowKeyDown(e, entry)}
            ondragstart={(e) => { if (e.dataTransfer) { e.dataTransfer.effectAllowed = "copyMove"; e.dataTransfer.setData("text/plain", entry.path); const paths = fm.selected.has(entry.path) ? [...fm.selected] : [entry.path]; e.dataTransfer.setData("text/uri-list", paths.map(p => "file://" + encodeURI(p)).join("\r\n")); } fm.setDragPaths(entry); }}
            ondragend={() => { fm.dragPaths = []; fm.dropTarget = null; }}
            ondragenter={(e) => onRowDragEnter(e, entry)}
            ondragover={(e) => { if (entry.kind === "directory" && !fm.dragPaths.includes(entry.path)) { e.preventDefault(); e.stopPropagation(); if (e.dataTransfer) e.dataTransfer.dropEffect = fm.dragPaths.length > 0 ? "move" : "copy"; fm.dropTarget = entry.path; } }}
            ondragleave={() => { if (fm.dropTarget === entry.path) fm.dropTarget = null; }}
            ondrop={(e) => { if (entry.kind === "directory") { e.preventDefault(); e.stopPropagation(); if (fm.dragPaths.length > 0) { fm.requestDrop(entry.path); } else { const uriList = e.dataTransfer?.getData("text/uri-list"); if (uriList) { const paths = uriList.split("\r\n").filter((u: string) => u && !u.startsWith("#")).map((u: string) => decodeURI(u.replace(/^file:\/\//, ""))).filter(Boolean); if (paths.length > 0) fm.handleExternalDrop(paths, entry.path); } } } }}
            onmouseenter={(e) => onRowMouseEnter(e, entry)}
            onmouseleave={onRowMouseLeave}
            onmousemove={onRowMouseMove}
          >
            <div class="col-icon">
              <div class="icon-wrap">
                <svg width={listIconSize} height={listIconSize} viewBox="0 0 16 16" style="color: {iconColor}">
                  {@html renderIcon(iconName)}
                </svg>
                {#if !entry.isWritable}
                  <div class="lock-badge" title={t.noWritePermission}>
                    <svg width="7" height="7" viewBox="0 0 16 16" fill="none">
                      <rect x="3" y="7" width="10" height="8" rx="2" fill="var(--crust)" stroke="var(--peach)" stroke-width="2"/>
                      <path d="M6 7V5C6 3.34 6.9 2 8 2s2 1.34 2 3v2" stroke="var(--peach)" stroke-width="2" stroke-linecap="round" fill="none"/>
                    </svg>
                  </div>
                {/if}
              </div>
            </div>

            <div class="col-name">
              {#if isRenaming}
                <input
                  class="rename-input"
                  bind:this={renameInput}
                  bind:value={fm.renameBuffer}
                  onclick={(e) => e.stopPropagation()}
                  onkeydown={(e) => {
                    e.stopPropagation();
                    if (e.key === "Enter") { e.preventDefault(); commitRename(); }
                    if (e.key === "Escape") { e.preventDefault(); fm.renameTarget = null; }
                  }}
                  onblur={commitRename}
                />
              {:else}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <span
                  class="file-name"
                  class:directory={entry.kind === "directory"}
                  class:symlink={entry.kind === "symlink"}
                  class:broken-link={entry.isBrokenLink}
                  ondblclick={(e) => { e.stopPropagation(); startRename(entry); }}
                  title={entry.isBrokenLink ? `${entry.name} (broken link)` : entry.name}
                >{entry.name}</span>
                {@const gitSt = fm.getGitFileStatus(entry.path)}
                {#if gitSt}
                  <span class="git-dot git-{gitSt}" title={gitSt}></span>
                {/if}
              {/if}
            </div>

            <div class="col-size">
              {#if entry.kind === "directory" && fm.folderSizes.has(entry.path)}
                {@const s = fm.folderSizes.get(entry.path)!}
                {#if s === -1}
                  <svg width="12" height="12" viewBox="0 0 16 16" fill="none" class="spin-icon"><path d="M8 2A6 6 0 1 0 14 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></svg>
                {:else}
                  {formatSize(s)}
                {/if}
              {:else if entry.kind === "directory"}
                <button class="calc-size" onclick={(e) => { e.stopPropagation(); fm.calculateFolderSize(entry.path); }} title={t.calculateSize}>&mdash;</button>
              {:else}
                {entry.size != null ? formatSize(entry.size) : "\u2014"}
              {/if}
            </div>
            <div class="col-date">
              {formatDate(entry.modified)}
            </div>
          </div>
        {/each}
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
  .list-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
    user-select: none;
  }

  .list-header {
    display: grid;
    grid-template-columns: var(--col-icon-w, 34px) minmax(0, 1fr) minmax(60px, 100px) minmax(100px, 180px);
    gap: 6px;
    padding: 0 14px;
    height: 32px;
    align-items: center;
    background: var(--panel-bg-strong);
    backdrop-filter: blur(12px);
    border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }

  .list-header div, .list-header button {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--overlay1);
  }

  .col-sort {
    display: flex;
    align-items: center;
    gap: 4px;
    background: none;
    border: none;
    cursor: pointer;
    padding: 3px 6px;
    margin: -3px -6px;
    border-radius: 5px;
    text-align: left;
    transition: color 0.1s, background 0.1s;
  }

  .col-sort:hover {
    color: var(--subtext1);
    background: var(--hover-bg);
  }

  .col-sort.active-sort {
    color: var(--mauve);
  }

  .sort-arrow {
    font-size: 10px;
    color: var(--mauve);
    font-weight: 700;
  }

  .list-body, :global(.list-body) {
    flex: 1;
    overflow-y: auto;
    padding: 4px 8px;
  }

  .file-row {
    display: grid;
    grid-template-columns: var(--col-icon-w, 34px) minmax(0, 1fr) minmax(60px, 100px) minmax(100px, 180px);
    gap: 6px;
    padding: 0 6px;
    height: var(--row-h, 33px);
    align-items: center;
    border-radius: 8px;
    cursor: default;
    transition: background 0.08s;
    outline: none;
  }

  .file-row:hover {
    background: var(--hover-bg);
  }

  .file-row.selected {
    background: var(--hover-bg-strong);
  }

  .file-row.selected:hover {
    background: var(--accent-muted);
  }

  .file-row.hidden-entry {
    opacity: 0.55;
  }

  .file-row.cut {
    opacity: 0.45;
  }

  .file-row.dragging {
    opacity: 0.4;
  }

  .file-row.drop-target {
    background: var(--blue-bg);
    box-shadow: inset 0 0 0 1.5px var(--blue-border-strong);
  }

  .col-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .icon-wrap {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .lock-badge {
    position: absolute;
    bottom: -3px;
    right: -3px;
    width: 12px;
    height: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 3px;
    background: var(--crust);
    box-shadow: 0 1px 3px rgba(0,0,0,0.4);
  }

  .col-name {
    overflow: hidden;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .git-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .git-modified { background: var(--yellow); }
  .git-staged { background: var(--green); }
  .git-untracked { background: var(--overlay1); }

  .file-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 13px;
    color: var(--text);
  }

  .file-name.directory {
    color: var(--blue);
    font-weight: 500;
  }

  .file-name.symlink {
    color: var(--teal);
    font-style: italic;
  }

  .file-name.broken-link {
    color: var(--red);
    opacity: 0.7;
    text-decoration: line-through;
  }

  .col-size, .col-date {
    font-size: 12px;
    color: var(--subtext0);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .rename-input {
    width: 100%;
    background: var(--surface1);
    border: 1.5px solid var(--mauve);
    border-radius: 5px;
    padding: 2px 6px;
    color: var(--text);
    font-size: 13px;
    outline: none;
    height: 26px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    height: 200px;
    color: var(--overlay1);
    font-size: 14px;
  }

  :global(.rubber-band) {
    position: fixed;
    background: var(--accent-subtle);
    border: 1px solid var(--accent-border-strong);
    border-radius: 3px;
    pointer-events: none;
    z-index: 500;
  }

  @keyframes spin { to { transform: rotate(360deg); } }
  .spin-icon { animation: spin 0.8s linear infinite; color: var(--overlay1); }

  .calc-size {
    cursor: pointer;
    opacity: 0.5;
    transition: opacity 0.1s;
    background: none;
    border: none;
    color: inherit;
    font: inherit;
    padding: 0;
  }
  .calc-size:hover { opacity: 1; }
</style>
