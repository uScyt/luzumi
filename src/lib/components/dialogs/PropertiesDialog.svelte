<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fm } from "../../fileManager.svelte";
  import { formatSize, formatDate, getFileIcon, getFileColor } from "../../types";
  import type { FileEntry, FileDetails } from "../../types";
  import { t } from "../../i18n";

  let { entry }: { entry: FileEntry } = $props();

  let details = $state<FileDetails | null>(null);
  let thumb = $state<string | null>(null);
  let activeTab = $state<"general" | "permissions" | "checksums">("general");
  let checksums = $state<{ md5: string; sha256: string } | null>(null);
  let checksumLoading = $state(false);
  let detailsError = $state("");
  let editing = $state(false);
  let editMode = $state(0o644);
  let permError = $state("");
  let copiedHash = $state<string | null>(null);

  const IMAGE_EXTS = new Set(["png","jpg","jpeg","gif","webp","bmp","svg","ico","avif","heic"]);

  const kindLabel: Record<string, string> = {
    directory: t.directory,
    file: t.file,
    symlink: t.symbolicLink,
    other: t.specialFile,
  };

  const kindIcons: Record<string, string> = {
    directory: `<path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" fill="currentColor" opacity=".15" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round"/>`,
    file: `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>`,
    symlink: `<path d="M6.5 10.5L9.5 7.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><path d="M9 11L9.5 11.5C10.88 12.88 13.12 12.88 14.5 11.5s1.38-3.62 0-5L13 5C11.62 3.62 9.38 3.62 8 5L7.5 5.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>`,
  };

  $effect(() => {
    invoke<FileDetails>("cmd_get_file_details", { path: entry.path })
      .then((d) => { details = d; editMode = d.permissionsMode; })
      .catch((e) => { detailsError = String(e); });

    if (entry.kind === "file" && entry.extension && IMAGE_EXTS.has(entry.extension.toLowerCase())) {
      invoke<string>("cmd_read_thumbnail", { path: entry.path })
        .then((d) => { thumb = d; })
        .catch(() => {});
    }
  });

  function close() {
    fm.showProperties = null;
  }

  function getParentPath(p: string): string {
    const parts = p.split("/").filter(Boolean);
    parts.pop();
    return "/" + parts.join("/") || "/";
  }

  function formatOctal(perms: string): string {
    const map = (s: string) => {
      let v = 0;
      if (s[0] === 'r') v += 4;
      if (s[1] === 'w') v += 2;
      if (s[2] === 'x') v += 1;
      return v;
    };
    if (perms.length !== 9) return "";
    return `${map(perms.slice(0,3))}${map(perms.slice(3,6))}${map(perms.slice(6,9))}`;
  }

  function getMimeCategory(mime: string): string {
    if (mime.startsWith("image/")) return "Image";
    if (mime.startsWith("video/")) return "Video";
    if (mime.startsWith("audio/")) return "Audio";
    if (mime.startsWith("text/")) return "Text";
    if (mime.includes("pdf")) return "PDF";
    if (mime.includes("zip") || mime.includes("tar") || mime.includes("compress") || mime.includes("archive")) return "Archive";
    if (mime.includes("executable") || mime.includes("x-sharedlib")) return "Executable";
    return "File";
  }

  async function copyHash(hash: string) {
    await navigator.clipboard.writeText(hash);
    copiedHash = hash;
    setTimeout(() => { copiedHash = null; }, 1500);
  }

  async function loadChecksums() {
    if (checksums || checksumLoading) return;
    checksumLoading = true;
    try {
      checksums = await invoke<{ md5: string; sha256: string }>("cmd_compute_checksum", { path: entry.path });
    } catch {}
    checksumLoading = false;
  }

  function formatDateLong(ts: number | null | undefined): string {
    if (!ts) return "—";
    const d = new Date(ts * 1000);
    return d.toLocaleDateString(undefined, { year: "numeric", month: "long", day: "numeric" }) + " " + d.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit", second: "2-digit" });
  }

  function timeAgo(ts: number | null | undefined): string {
    if (!ts) return "";
    const diff = Math.floor(Date.now() / 1000 - ts);
    if (diff < 60) return "just now";
    if (diff < 3600) return `${Math.floor(diff / 60)} min ago`;
    if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
    if (diff < 604800) return `${Math.floor(diff / 86400)} day${Math.floor(diff / 86400) > 1 ? "s" : ""} ago`;
    if (diff < 2592000) return `${Math.floor(diff / 604800)} week${Math.floor(diff / 604800) > 1 ? "s" : ""} ago`;
    return `${Math.floor(diff / 2592000)} month${Math.floor(diff / 2592000) > 1 ? "s" : ""} ago`;
  }
</script>

<div class="overlay" role="dialog" aria-modal="true" tabindex="-1" onclick={close} onkeydown={(e) => { if (e.key === "Escape") close(); }}>
  <div class="dialog" role="presentation" onclick={(e) => e.stopPropagation()}>

    <!-- Header with large icon + name -->
    <div class="dialog-header">
      <div class="header-left">
        <div class="icon-wrap" style="color: {getFileColor(entry)}">
          {#if thumb}
            <img src={thumb} alt="" class="thumb" />
          {:else}
            <svg width="36" height="36" viewBox="0 0 16 16">
              {@html kindIcons[entry.kind] ?? kindIcons["file"]}
            </svg>
          {/if}
        </div>
        <div class="header-text">
          <h2 title={entry.name}>{entry.name}</h2>
          <div class="header-meta">
            <span class="meta-badge kind">{kindLabel[entry.kind] ?? entry.kind}</span>
            {#if entry.extension}
              <span class="meta-badge ext">.{entry.extension}</span>
            {/if}
            {#if details?.mimeType && details.mimeType !== "application/octet-stream"}
              <span class="meta-badge mime">{getMimeCategory(details.mimeType)}</span>
            {/if}
            {#if details?.isExecutable && entry.kind === "file"}
              <span class="meta-badge exec">Executable</span>
            {/if}
            {#if details?.isReadonly}
              <span class="meta-badge readonly">Read-only</span>
            {/if}
          </div>
        </div>
      </div>
      <button class="close-btn" title={t.close} onclick={close}>
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
          <path d="M3 3L13 13M13 3L3 13" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
      </button>
    </div>

    <!-- Tabs -->
    <div class="tabs">
      <button class="tab" class:active={activeTab === "general"} onclick={() => activeTab = "general"}>
        <svg width="13" height="13" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4"/><path d="M8 5V8.5M8 11V11.5" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"/></svg>
        {t.general}
      </button>
      <button class="tab" class:active={activeTab === "permissions"} onclick={() => activeTab = "permissions"}>
        <svg width="13" height="13" viewBox="0 0 16 16" fill="none"><rect x="3" y="7" width="10" height="7" rx="1.5" stroke="currentColor" stroke-width="1.3"/><path d="M5.5 7V5C5.5 3.62 6.62 2.5 8 2.5s2.5 1.12 2.5 2.5v2" stroke="currentColor" stroke-width="1.3"/></svg>
        {t.permissions}
      </button>
      {#if entry.kind === "file"}
        <button class="tab" class:active={activeTab === "checksums"} onclick={() => { activeTab = "checksums"; loadChecksums(); }}>
          <svg width="13" height="13" viewBox="0 0 16 16" fill="none"><path d="M2 4H14M2 8H10M2 12H7" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M12 9L10.5 12.5L13.5 11L12 14.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/></svg>
          Checksums
        </button>
      {/if}
    </div>

    <!-- Content -->
    <div class="tab-content">
      {#if activeTab === "general"}
        <!-- Size summary card -->
        {#if entry.size != null || (details?.kind === "directory" && details.dirSize != null)}
          <div class="size-card">
            <div class="size-main">
              {#if details?.kind === "directory" && details.dirSize != null}
                {formatSize(details.dirSize)}
              {:else if entry.size != null}
                {formatSize(entry.size)}
              {/if}
            </div>
            <div class="size-detail">
              {#if details?.kind === "directory" && details.childrenCount != null}
                {details.childrenCount.toLocaleString()} item{details.childrenCount !== 1 ? "s" : ""}
              {:else if entry.size != null}
                {entry.size.toLocaleString()} bytes
              {/if}
            </div>
          </div>
        {/if}

        <!-- Location -->
        <div class="section">
          <div class="section-title">Location</div>
          <div class="path-display">
            <svg width="13" height="13" viewBox="0 0 16 16" fill="none" class="path-icon"><path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="none"/></svg>
            <span class="path-text" title={entry.path}>{entry.path}</span>
            <button class="copy-btn" title="Copy path" onclick={() => navigator.clipboard.writeText(entry.path)} aria-label="Copy path">
              <svg width="11" height="11" viewBox="0 0 16 16" fill="none"><rect x="5" y="5" width="9" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4"/><path d="M11 5V3.5C11 2.67 10.33 2 9.5 2H3.5C2.67 2 2 2.67 2 3.5V9.5C2 10.33 2.67 11 3.5 11H5" stroke="currentColor" stroke-width="1.4"/></svg>
            </button>
          </div>
        </div>

        {#if details?.mimeType}
          <div class="section">
            <div class="section-title">Type</div>
            <div class="info-value">{details.mimeType}</div>
          </div>
        {/if}

        {#if details?.linkTarget}
          <div class="section">
            <div class="section-title">{t.target}</div>
            <div class="path-display">
              <svg width="13" height="13" viewBox="0 0 16 16" fill="none" class="path-icon"><path d="M6.5 10.5L9.5 7.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><path d="M7 5L8 4C9.1 2.9 10.9 2.9 12 4s1.1 2.9 0 4L11 9" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/><path d="M9 11L8 12C6.9 13.1 5.1 13.1 4 12s-1.1-2.9 0-4L5 7" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/></svg>
              <span class="path-text" title={details.linkTarget}>
                {details.linkTarget}
              </span>
              {#if details.linkTargetExists === false}
                <span class="broken-badge">broken</span>
              {/if}
            </div>
          </div>
        {/if}

        <!-- Dates -->
        <div class="section">
          <div class="section-title">Dates</div>
          <div class="dates-grid">
            <div class="date-row">
              <svg width="13" height="13" viewBox="0 0 16 16" fill="none" class="date-icon"><path d="M8 3V8L11 10" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3"/></svg>
              <div class="date-info">
                <span class="date-label">{t.modified}</span>
                <span class="date-value">{formatDateLong(entry.modified)}</span>
                {#if entry.modified}
                  <span class="date-ago">{timeAgo(entry.modified)}</span>
                {/if}
              </div>
            </div>
            {#if details?.created}
              <div class="date-row">
                <svg width="13" height="13" viewBox="0 0 16 16" fill="none" class="date-icon"><path d="M8 3V8M12 8A4 4 0 1 1 4 8" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><path d="M6 2h4" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/></svg>
                <div class="date-info">
                  <span class="date-label">{t.created}</span>
                  <span class="date-value">{formatDateLong(details.created)}</span>
                  <span class="date-ago">{timeAgo(details.created)}</span>
                </div>
              </div>
            {/if}
            {#if details?.accessed}
              <div class="date-row">
                <svg width="13" height="13" viewBox="0 0 16 16" fill="none" class="date-icon"><circle cx="8" cy="8" r="3" stroke="currentColor" stroke-width="1.3"/><circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3"/><circle cx="8" cy="8" r="1" fill="currentColor"/></svg>
                <div class="date-info">
                  <span class="date-label">{t.accessed}</span>
                  <span class="date-value">{formatDateLong(details.accessed)}</span>
                  <span class="date-ago">{timeAgo(details.accessed)}</span>
                </div>
              </div>
            {/if}
          </div>
        </div>

      {:else if activeTab === "permissions"}
        {#if detailsError}
          <div class="empty-state error">{detailsError}</div>
        {:else if details}
          <!-- Owner & Group -->
          <div class="section">
            <div class="section-title">Ownership</div>
            <div class="owner-grid">
              <div class="owner-card">
                <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="5" r="3" stroke="currentColor" stroke-width="1.3"/><path d="M3 14C3 11.24 5.24 9 8 9s5 2.24 5 5" stroke="currentColor" stroke-width="1.3"/></svg>
                <div>
                  <span class="owner-label">{t.owner}</span>
                  <span class="owner-value">{details.owner}</span>
                </div>
              </div>
              <div class="owner-card">
                <svg width="14" height="14" viewBox="0 0 16 16" fill="none"><circle cx="6" cy="5" r="2.5" stroke="currentColor" stroke-width="1.2"/><path d="M2 13C2 10.8 3.8 9 6 9s4 1.8 4 4" stroke="currentColor" stroke-width="1.2"/><circle cx="11" cy="5.5" r="2" stroke="currentColor" stroke-width="1.2"/><path d="M13 9c1.1 0 2 .9 2 2" stroke="currentColor" stroke-width="1.2"/></svg>
                <div>
                  <span class="owner-label">{t.group}</span>
                  <span class="owner-value">{details.group}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Permission matrix -->
          <div class="section">
            <div class="section-title-row">
              <span class="section-title">Permissions</span>
              {#if !editing}
                <div class="perm-summary">
                  <span class="perm-octal">{formatOctal(details.permissions)}</span>
                  <span class="perm-string">{details.permissions}</span>
                  <button class="edit-btn" title="Edit permissions" onclick={() => { editing = true; editMode = details?.permissionsMode ?? 0o644; }}>
                    <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><path d="M11.5 1.5L14.5 4.5L5 14H2V11L11.5 1.5Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
                  </button>
                </div>
              {:else}
                <div class="perm-summary">
                  <label for="perm-octal-edit" class="sr-only">Octal permissions</label>
                  <input id="perm-octal-edit" class="perm-octal-input" type="text" value={((editMode & 0o777).toString(8)).padStart(3, '0')} oninput={(e) => { const v = parseInt((e.target as HTMLInputElement).value, 8); if (!isNaN(v) && v >= 0 && v <= 0o777) editMode = (editMode & ~0o777) | v; }} />
                </div>
              {/if}
            </div>

            <div class="perm-grid">
              <div class="perm-corner"></div>
              <div class="perm-header">{t.read}</div>
              <div class="perm-header">{t.write}</div>
              <div class="perm-header">{t.execute}</div>

              {#each [
                { label: t.owner, r: 0o400, w: 0o200, x: 0o100 },
                { label: t.group, r: 0o040, w: 0o020, x: 0o010 },
                { label: t.others, r: 0o004, w: 0o002, x: 0o001 },
              ] as bit, i}
                <div class="perm-label">{bit.label}</div>
                {#if editing}
                  <div class="perm-cell-edit"><input type="checkbox" checked={(editMode & bit.r) !== 0} onchange={() => { editMode = editMode ^ bit.r; }} /></div>
                  <div class="perm-cell-edit"><input type="checkbox" checked={(editMode & bit.w) !== 0} onchange={() => { editMode = editMode ^ bit.w; }} /></div>
                  <div class="perm-cell-edit"><input type="checkbox" checked={(editMode & bit.x) !== 0} onchange={() => { editMode = editMode ^ bit.x; }} /></div>
                {:else}
                  <div class="perm-cell" class:granted={details.permissions[i * 3] === 'r'}>
                    {#if details.permissions[i * 3] === 'r'}
                      <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><path d="M3 8L6.5 11.5L13 5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                    {:else}
                      <svg width="10" height="10" viewBox="0 0 16 16" fill="none"><path d="M4 4L12 12M12 4L4 12" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></svg>
                    {/if}
                  </div>
                  <div class="perm-cell" class:granted={details.permissions[i * 3 + 1] === 'w'}>
                    {#if details.permissions[i * 3 + 1] === 'w'}
                      <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><path d="M3 8L6.5 11.5L13 5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                    {:else}
                      <svg width="10" height="10" viewBox="0 0 16 16" fill="none"><path d="M4 4L12 12M12 4L4 12" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></svg>
                    {/if}
                  </div>
                  <div class="perm-cell" class:granted={details.permissions[i * 3 + 2] === 'x'}>
                    {#if details.permissions[i * 3 + 2] === 'x'}
                      <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><path d="M3 8L6.5 11.5L13 5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                    {:else}
                      <svg width="10" height="10" viewBox="0 0 16 16" fill="none"><path d="M4 4L12 12M12 4L4 12" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></svg>
                    {/if}
                  </div>
                {/if}
              {/each}
            </div>

            {#if permError}
              <div class="perm-error">{permError}</div>
            {/if}

            {#if editing}
              <div class="perm-actions">
                <button class="btn-secondary" onclick={() => { editing = false; editMode = details?.permissionsMode ?? 0o644; permError = ""; }}>Cancel</button>
                <button class="btn-primary" onclick={async () => {
                  try {
                    await invoke("cmd_set_permissions", { path: entry.path, mode: editMode });
                    const d = await invoke<FileDetails>("cmd_get_file_details", { path: entry.path });
                    details = d;
                    editMode = d.permissionsMode;
                    editing = false;
                    permError = "";
                  } catch (e) {
                    const msg = String(e);
                    if (msg.includes("ermission")) {
                      try {
                        await invoke("cmd_elevated_set_permissions", { path: entry.path, mode: editMode });
                        const d = await invoke<FileDetails>("cmd_get_file_details", { path: entry.path });
                        details = d;
                        editMode = d.permissionsMode;
                        editing = false;
                        permError = "";
                      } catch (e2) { permError = String(e2); }
                    } else {
                      permError = msg;
                    }
                  }
                }}>
                  <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><path d="M3 8L6.5 11.5L13 5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                  Save
                </button>
              </div>
            {/if}
          </div>

        {:else}
          <div class="empty-state">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" class="spin"><path d="M8 2A6 6 0 1 0 14 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></svg>
            {t.loading}
          </div>
        {/if}

      {:else if activeTab === "checksums"}
        {#if checksumLoading}
          <div class="empty-state">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" class="spin"><path d="M8 2A6 6 0 1 0 14 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/></svg>
            Computing hashes...
          </div>
        {:else if checksums}
          <div class="section">
            <div class="hash-card">
              <div class="hash-header">
                <span class="hash-algo">MD5</span>
                <button class="copy-btn" class:copied={copiedHash === checksums.md5} title="Copy" onclick={() => copyHash(checksums!.md5)} aria-label="Copy MD5">
                  {#if copiedHash === checksums.md5}
                    <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><path d="M3 8L6.5 11.5L13 5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                  {:else}
                    <svg width="11" height="11" viewBox="0 0 16 16" fill="none"><rect x="5" y="5" width="9" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4"/><path d="M11 5V3.5C11 2.67 10.33 2 9.5 2H3.5C2.67 2 2 2.67 2 3.5V9.5C2 10.33 2.67 11 3.5 11H5" stroke="currentColor" stroke-width="1.4"/></svg>
                  {/if}
                </button>
              </div>
              <code class="hash-value">{checksums.md5}</code>
            </div>

            <div class="hash-card">
              <div class="hash-header">
                <span class="hash-algo">SHA-256</span>
                <button class="copy-btn" class:copied={copiedHash === checksums.sha256} title="Copy" onclick={() => copyHash(checksums!.sha256)} aria-label="Copy SHA-256">
                  {#if copiedHash === checksums.sha256}
                    <svg width="12" height="12" viewBox="0 0 16 16" fill="none"><path d="M3 8L6.5 11.5L13 5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
                  {:else}
                    <svg width="11" height="11" viewBox="0 0 16 16" fill="none"><rect x="5" y="5" width="9" height="9" rx="1.5" stroke="currentColor" stroke-width="1.4"/><path d="M11 5V3.5C11 2.67 10.33 2 9.5 2H3.5C2.67 2 2 2.67 2 3.5V9.5C2 10.33 2.67 11 3.5 11H5" stroke="currentColor" stroke-width="1.4"/></svg>
                  {/if}
                </button>
              </div>
              <code class="hash-value">{checksums.sha256}</code>
            </div>
          </div>
        {/if}
      {/if}
    </div>

    <div class="footer">
      <button class="btn-close" onclick={close}>{t.close}</button>
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.55);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 600;
    animation: fade 0.15s ease;
  }
  @keyframes fade { from { opacity: 0; } }

  .dialog {
    background: var(--glass-bg-strong);
    backdrop-filter: var(--glass-blur-strong);
    border: 1px solid var(--border-medium);
    border-radius: 16px;
    width: 460px;
    max-width: calc(100vw - 40px);
    max-height: 85vh;
    box-shadow: 0 24px 80px rgba(0,0,0,0.6), inset 0 1px 0 var(--border-light);
    animation: pop 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  @keyframes pop { from { transform: scale(0.96) translateY(8px); opacity: 0; } }

  /* Header */
  .dialog-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 20px 20px 16px;
    background: var(--panel-bg-strong);
    border-bottom: 1px solid var(--border-light);
  }

  .header-left {
    display: flex;
    gap: 14px;
    min-width: 0;
    flex: 1;
    align-items: center;
  }

  .icon-wrap {
    width: 52px;
    height: 52px;
    border-radius: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--input-bg-focus);
    flex-shrink: 0;
    overflow: hidden;
  }

  .thumb {
    width: 52px;
    height: 52px;
    object-fit: cover;
    border-radius: 14px;
  }

  .header-text {
    min-width: 0;
    flex: 1;
  }

  h2 {
    font-size: 15px;
    font-weight: 600;
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 6px;
  }

  .header-meta {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
  }

  .meta-badge {
    font-size: 10px;
    font-weight: 600;
    padding: 2px 7px;
    border-radius: 4px;
    background: var(--hover-bg-strong);
    color: var(--subtext0);
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }
  .meta-badge.kind { background: var(--accent-muted); color: var(--accent); }
  .meta-badge.ext { background: var(--blue-muted); color: var(--blue); }
  .meta-badge.exec { background: var(--success-bg); color: var(--green); }
  .meta-badge.readonly { background: var(--danger-muted); color: var(--maroon); }

  .close-btn {
    width: 28px;
    height: 28px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    transition: background 0.1s, color 0.1s;
    flex-shrink: 0;
  }
  .close-btn:hover { background: var(--surface0); color: var(--text); }

  /* Tabs */
  .tabs {
    display: flex;
    padding: 0 16px;
    gap: 2px;
    border-bottom: 1px solid var(--border-light);
    background: var(--panel-bg-muted);
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 9px 14px;
    font-size: 12px;
    font-weight: 600;
    color: var(--overlay1);
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
    transition: color 0.12s, border-color 0.12s;
  }
  .tab:hover { color: var(--text); }
  .tab.active { color: var(--accent); border-bottom-color: var(--accent); }

  /* Content */
  .tab-content {
    padding: 16px 20px;
    overflow-y: auto;
    flex: 1;
  }

  .section {
    margin-bottom: 16px;
  }

  .section-title {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--overlay0);
    margin-bottom: 8px;
  }

  .section-title-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }

  .section-title-row .section-title { margin-bottom: 0; }

  .info-value {
    font-size: 12.5px;
    color: var(--subtext0);
    font-family: monospace;
  }

  /* Size card */
  .size-card {
    background: var(--accent-muted);
    border: 1px solid var(--accent-border);
    border-radius: 10px;
    padding: 14px 16px;
    margin-bottom: 16px;
    text-align: center;
  }

  .size-main {
    font-size: 24px;
    font-weight: 700;
    color: var(--accent);
    letter-spacing: -0.02em;
  }

  .size-detail {
    font-size: 11.5px;
    color: var(--subtext0);
    margin-top: 2px;
  }

  /* Path display */
  .path-display {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--input-bg);
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    padding: 8px 10px;
  }

  .path-icon { color: var(--overlay1); flex-shrink: 0; }

  .path-text {
    flex: 1;
    font-size: 12px;
    font-family: monospace;
    color: var(--subtext0);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    user-select: all;
    -webkit-user-select: all;
  }

  .copy-btn {
    width: 24px;
    height: 24px;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }
  .copy-btn:hover { background: var(--hover-bg); color: var(--text); }
  .copy-btn.copied { color: var(--green); }

  .broken-badge {
    font-size: 10px;
    font-weight: 700;
    color: var(--red);
    background: var(--danger-muted);
    padding: 1px 6px;
    border-radius: 4px;
    flex-shrink: 0;
  }

  /* Dates */
  .dates-grid {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .date-row {
    display: flex;
    gap: 10px;
    align-items: flex-start;
  }

  .date-icon {
    color: var(--overlay1);
    flex-shrink: 0;
    margin-top: 1px;
  }

  .date-info {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 0;
  }

  .date-label {
    font-size: 10.5px;
    font-weight: 600;
    color: var(--overlay0);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .date-value {
    font-size: 12.5px;
    color: var(--text);
  }

  .date-ago {
    font-size: 11px;
    color: var(--overlay1);
    font-style: italic;
  }

  /* Permissions */
  .owner-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .owner-card {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    background: var(--input-bg);
    border-radius: 8px;
    border: 1px solid var(--border-subtle);
  }

  .owner-card svg { color: var(--overlay1); flex-shrink: 0; }

  .owner-label {
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--overlay0);
    display: block;
  }

  .owner-value {
    font-size: 13px;
    color: var(--text);
    font-weight: 500;
    display: block;
  }

  .perm-summary {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .perm-octal {
    font-family: monospace;
    font-size: 16px;
    font-weight: 700;
    color: var(--accent);
  }

  .perm-string {
    font-family: monospace;
    font-size: 12px;
    color: var(--subtext0);
    letter-spacing: 1px;
  }

  .perm-octal-input {
    font-family: monospace;
    font-size: 16px;
    font-weight: 700;
    color: var(--accent);
    background: var(--input-bg);
    border: 1px solid var(--accent-border);
    border-radius: 6px;
    padding: 2px 8px;
    width: 56px;
    text-align: center;
  }

  .edit-btn {
    width: 24px;
    height: 24px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    transition: background 0.1s, color 0.1s;
  }
  .edit-btn:hover { background: var(--hover-bg); color: var(--text); }

  .perm-grid {
    display: grid;
    grid-template-columns: 70px repeat(3, 1fr);
    gap: 4px;
  }

  .perm-header {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--overlay0);
    text-align: center;
    padding: 4px 0;
  }

  .perm-label {
    font-size: 12px;
    color: var(--subtext0);
    display: flex;
    align-items: center;
    padding: 4px 0;
    font-weight: 500;
  }

  .perm-cell {
    text-align: center;
    padding: 5px 0;
    border-radius: 6px;
    background: var(--input-bg);
    color: var(--overlay0);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .perm-cell.granted {
    background: var(--success-bg);
    color: var(--green);
  }

  .perm-cell-edit {
    text-align: center;
    padding: 5px 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .perm-cell-edit input[type="checkbox"] {
    width: 16px;
    height: 16px;
    accent-color: var(--green);
    cursor: pointer;
  }

  .perm-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 12px;
  }

  .perm-error {
    color: var(--red);
    font-size: 12px;
    margin-top: 8px;
    padding: 8px 10px;
    background: var(--danger-subtle);
    border-radius: 8px;
    border: 1px solid var(--danger-border);
  }

  /* Checksums */
  .hash-card {
    background: var(--input-bg);
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    padding: 10px 12px;
    margin-bottom: 10px;
  }

  .hash-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 6px;
  }

  .hash-algo {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--overlay0);
  }

  .hash-value {
    font-family: monospace;
    font-size: 11px;
    color: var(--subtext0);
    word-break: break-all;
    line-height: 1.5;
    user-select: all;
    -webkit-user-select: all;
  }

  /* Buttons */
  .btn-secondary {
    padding: 6px 16px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    color: var(--subtext0);
    background: var(--surface0);
    transition: background 0.1s;
  }
  .btn-secondary:hover { background: var(--surface1); }

  .btn-primary {
    padding: 6px 16px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    color: var(--base);
    background: var(--green);
    display: flex;
    align-items: center;
    gap: 5px;
    transition: opacity 0.1s;
  }
  .btn-primary:hover { opacity: 0.88; }

  .footer {
    padding: 12px 20px;
    display: flex;
    justify-content: flex-end;
    border-top: 1px solid var(--border-light);
  }

  .btn-close {
    padding: 7px 24px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 600;
    color: var(--base);
    background: var(--accent);
    transition: opacity 0.1s;
  }
  .btn-close:hover { opacity: 0.88; }

  /* States */
  .empty-state {
    text-align: center;
    padding: 30px;
    color: var(--overlay1);
    font-size: 13px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
  }

  .empty-state.error { color: var(--red); }

  @keyframes spin { to { transform: rotate(360deg); } }
  .spin { animation: spin 0.8s linear infinite; }

  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    border: 0;
  }
</style>
