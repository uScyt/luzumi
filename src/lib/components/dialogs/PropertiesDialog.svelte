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

  const IMAGE_EXTS = new Set(["png","jpg","jpeg","gif","webp","bmp","svg","ico"]);

  const kindLabel: Record<string, string> = {
    directory: t.directory,
    file: t.file,
    symlink: t.symbolicLink,
    other: t.specialFile,
  };

  const svgPaths: Record<string, string> = {
    Folder:        `<path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" fill="currentColor" opacity=".2" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round"/>`,
    File:          `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>`,
    FileCode:      `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><path d="M6 10L8.5 12.5L6 15M10 10L7.5 12.5L10 15" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" stroke-linejoin="round"/>`,
    FileText:      `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><path d="M6 10H10M6 12.5H9" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,
    FileImage:     `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><circle cx="6.5" cy="10" r="1" fill="currentColor"/><path d="M5 14L8 11L10 13L11.5 11.5L13 13" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" stroke-linejoin="round"/>`,
    FilmStrip:     `<rect x="3" y="3" width="10" height="11" rx="1" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M3 6H13M3 11H13" stroke="currentColor" stroke-width="1.1"/><rect x="1" y="5" width="2" height="2" rx=".5" fill="currentColor"/><rect x="1" y="10" width="2" height="2" rx=".5" fill="currentColor"/><rect x="13" y="5" width="2" height="2" rx=".5" fill="currentColor"/><rect x="13" y="10" width="2" height="2" rx=".5" fill="currentColor"/>`,
    MusicNote:     `<path d="M9 12V4L13 3" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/><circle cx="7" cy="12" r="2" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".15"/>`,
    FilePdf:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><text x="4.5" y="13.5" font-size="5" font-weight="700" fill="currentColor" font-family="monospace">PDF</text>`,
    FileArchive:   `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><path d="M7 8H9M7 10H9M7 12H9" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,
    Link:          `<path d="M6.5 10.5L9.5 7.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><path d="M9 11L9.5 11.5C10.88 12.88 13.12 12.88 14.5 11.5s1.38-3.62 0-5L13 5C11.62 3.62 9.38 3.62 8 5L7.5 5.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/><path d="M7 5L6.5 4.5C5.12 3.12 2.88 3.12 1.5 4.5S.12 8.12 1.5 9.5L3 11C4.38 12.38 6.62 12.38 8 11L8.5 10.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>`,
    Gear:          `<circle cx="8" cy="8" r="2.5" stroke="currentColor" stroke-width="1.2" fill="none"/><path d="M8 1v2M8 13v2M1 8h2M13 8h2M2.9 2.9l1.4 1.4M11.7 11.7l1.4 1.4M2.9 13.1l1.4-1.4M11.7 4.3l1.4-1.4" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>`,
    TerminalWindow:`<rect x="2" y="3" width="12" height="10" rx="1.5" stroke="currentColor" stroke-width="1.2" fill="none"/><path d="M5 7L7 9L5 11M8 11H11" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>`,
    Package:       `<path d="M8 2L14 5.5V10.5L8 14L2 10.5V5.5L8 2Z" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round" fill="currentColor" opacity=".1"/>`,
    Disc:          `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".1"/><circle cx="8" cy="8" r="2" stroke="currentColor" stroke-width="1.2" fill="none"/>`,
    Database:      `<ellipse cx="8" cy="4.5" rx="5" ry="2" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".15"/><path d="M3 4.5V8C3 9.1 5.24 10 8 10S13 9.1 13 8V4.5" stroke="currentColor" stroke-width="1.2" fill="none"/>`,
  };

  function renderIcon(iconName: string) {
    return svgPaths[iconName] ?? svgPaths["File"];
  }

  $effect(() => {
    invoke<FileDetails>("cmd_get_file_details", { path: entry.path })
      .then((d) => { details = d; editMode = d.permissionsMode; })
      .catch((e) => { detailsError = String(e); });

    if (entry.kind === "file" && entry.extension && IMAGE_EXTS.has(entry.extension.toLowerCase())) {
      invoke<string>("cmd_read_thumbnail", { path: entry.path })
        .then((d) => { thumb = d; })
        .catch(() => {}); // thumbnail failure is non-critical
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
</script>

<div class="overlay" role="dialog" aria-modal="true" tabindex="-1" onclick={close} onkeydown={(e) => { if (e.key === "Escape") close(); }}>
  <div class="dialog" role="presentation" onclick={(e) => e.stopPropagation()}>
    <div class="dialog-header">
      <div class="header-preview">
        <div class="preview-icon" style="color: {getFileColor(entry)}">
          {#if thumb}
            <img src={thumb} alt="" class="preview-thumb" />
          {:else}
            <svg width="44" height="44" viewBox="0 0 16 16">
              {@html renderIcon(getFileIcon(entry))}
            </svg>
          {/if}
        </div>
        <div class="header-info">
          <h2 title={entry.name}>{entry.name}</h2>
          <span class="header-type">{kindLabel[entry.kind] ?? entry.kind}{details?.mimeType && details.mimeType !== "application/octet-stream" ? ` — ${details.mimeType}` : ""}</span>
        </div>
      </div>
      <button class="close-btn" title={t.close} onclick={close}>
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
          <path d="M2 2L14 14M14 2L2 14" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
        </svg>
      </button>
    </div>

    <div class="tabs">
      <button class="tab" class:active={activeTab === "general"} onclick={() => activeTab = "general"}>{t.general}</button>
      <button class="tab" class:active={activeTab === "permissions"} onclick={() => activeTab = "permissions"}>{t.permissions}</button>
      {#if entry.kind === "file"}
        <button class="tab" class:active={activeTab === "checksums"} onclick={() => { activeTab = "checksums"; if (!checksums && !checksumLoading) { checksumLoading = true; invoke<{ md5: string; sha256: string }>("cmd_compute_checksum", { path: entry.path }).then(r => { checksums = r; }).catch(() => {}).finally(() => { checksumLoading = false; }); } }}>Checksums</button>
      {/if}
    </div>

    {#if activeTab === "general"}
      <div class="tab-content">
        <div class="info-section">
          <div class="prop-row">
            <span class="label">{t.location}</span>
            <span class="value mono" title={getParentPath(entry.path)}>{getParentPath(entry.path)}</span>
          </div>

          {#if entry.size != null}
            <div class="prop-row">
              <span class="label">{t.size}</span>
              <span class="value">{formatSize(entry.size)} <span class="dimmed">({entry.size.toLocaleString()} bytes)</span></span>
            </div>
          {/if}

          {#if details?.kind === "directory" && details.dirSize != null}
            <div class="prop-row">
              <span class="label">{t.contents}</span>
              <span class="value">
                {details.childrenCount ?? 0} item{(details.childrenCount ?? 0) !== 1 ? "s" : ""}, {formatSize(details.dirSize)} total
              </span>
            </div>
          {/if}

          {#if entry.extension}
            <div class="prop-row">
              <span class="label">{t.extension}</span>
              <span class="value">.{entry.extension}</span>
            </div>
          {/if}

          {#if details?.linkTarget}
            <div class="prop-row">
              <span class="label">{t.target}</span>
              <span class="value mono">
                {details.linkTarget}
                {#if details.linkTargetExists === false}
                  <span class="broken-link">(broken)</span>
                {/if}
              </span>
            </div>
          {/if}
        </div>

        <div class="divider"></div>

        <div class="info-section">
          <div class="prop-row">
            <span class="label">{t.modified}</span>
            <span class="value">{formatDate(entry.modified)}</span>
          </div>
          {#if details?.created}
            <div class="prop-row">
              <span class="label">{t.created}</span>
              <span class="value">{formatDate(details.created)}</span>
            </div>
          {/if}
          {#if details?.accessed}
            <div class="prop-row">
              <span class="label">{t.accessed}</span>
              <span class="value">{formatDate(details.accessed)}</span>
            </div>
          {/if}
        </div>
      </div>

    {:else if activeTab === "permissions"}
      <div class="tab-content">
        {#if detailsError}
          <div class="error-state">{detailsError}</div>
        {:else if details}
          <div class="info-section">
            <div class="prop-row">
              <span class="label">{t.owner}</span>
              <span class="value">{details.owner}</span>
            </div>
            <div class="prop-row">
              <span class="label">{t.group}</span>
              <span class="value">{details.group}</span>
            </div>
          </div>

          <div class="divider"></div>

          <div class="info-section">
            {#if editing}
              {@const bits = [
                { label: t.owner, r: 0o400, w: 0o200, x: 0o100 },
                { label: t.group, r: 0o040, w: 0o020, x: 0o010 },
                { label: t.others, r: 0o004, w: 0o002, x: 0o001 },
              ]}
              <div class="perm-display">
                <input class="perm-octal-input" type="text" value={((editMode & 0o777).toString(8)).padStart(3, '0')} oninput={(e) => { const v = parseInt((e.target as HTMLInputElement).value, 8); if (!isNaN(v) && v >= 0 && v <= 0o777) editMode = (editMode & ~0o777) | v; }} />
              </div>

              <div class="perm-grid">
                <div class="perm-header"></div>
                <div class="perm-header">{t.read}</div>
                <div class="perm-header">{t.write}</div>
                <div class="perm-header">{t.execute}</div>

                {#each bits as bit}
                  <div class="perm-label">{bit.label}</div>
                  <div class="perm-cell-edit"><input type="checkbox" checked={(editMode & bit.r) !== 0} onchange={() => { editMode = editMode ^ bit.r; }} /></div>
                  <div class="perm-cell-edit"><input type="checkbox" checked={(editMode & bit.w) !== 0} onchange={() => { editMode = editMode ^ bit.w; }} /></div>
                  <div class="perm-cell-edit"><input type="checkbox" checked={(editMode & bit.x) !== 0} onchange={() => { editMode = editMode ^ bit.x; }} /></div>
                {/each}
              </div>

              {#if permError}
                <div class="perm-error">{permError}</div>
              {/if}

              <div class="perm-actions">
                <button class="btn-cancel" onclick={() => { editing = false; editMode = details?.permissionsMode ?? 0o644; permError = ""; }}>Cancel</button>
                <button class="btn-save" onclick={async () => {
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
                }}>Save</button>
              </div>
            {:else}
              <div class="perm-display">
                <span class="perm-octal">{formatOctal(details.permissions)}</span>
                <span class="perm-string">{details.permissions}</span>
                <button class="edit-perm-btn" title="Edit permissions" onclick={() => { editing = true; editMode = details?.permissionsMode ?? 0o644; }}>
                  <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
                    <path d="M11.5 1.5L14.5 4.5L5 14H2V11L11.5 1.5Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                </button>
              </div>

              <div class="perm-grid">
                <div class="perm-header"></div>
                <div class="perm-header">{t.read}</div>
                <div class="perm-header">{t.write}</div>
                <div class="perm-header">{t.execute}</div>

                <div class="perm-label">{t.owner}</div>
                <div class="perm-cell" class:granted={details.permissions[0] === 'r'}>{details.permissions[0]}</div>
                <div class="perm-cell" class:granted={details.permissions[1] === 'w'}>{details.permissions[1]}</div>
                <div class="perm-cell" class:granted={details.permissions[2] === 'x'}>{details.permissions[2]}</div>

                <div class="perm-label">{t.group}</div>
                <div class="perm-cell" class:granted={details.permissions[3] === 'r'}>{details.permissions[3]}</div>
                <div class="perm-cell" class:granted={details.permissions[4] === 'w'}>{details.permissions[4]}</div>
                <div class="perm-cell" class:granted={details.permissions[5] === 'x'}>{details.permissions[5]}</div>

                <div class="perm-label">{t.others}</div>
                <div class="perm-cell" class:granted={details.permissions[6] === 'r'}>{details.permissions[6]}</div>
                <div class="perm-cell" class:granted={details.permissions[7] === 'w'}>{details.permissions[7]}</div>
                <div class="perm-cell" class:granted={details.permissions[8] === 'x'}>{details.permissions[8]}</div>
              </div>
            {/if}
          </div>

          <div class="divider"></div>

          <div class="info-section">
            <div class="badge-row">
              {#if details.isReadonly}
                <span class="attr-badge readonly">{t.readOnly}</span>
              {/if}
              {#if details.isExecutable}
                <span class="attr-badge executable">{t.executable}</span>
              {/if}
              {#if !details.isReadonly && !details.isExecutable}
                <span class="attr-badge">{t.standard}</span>
              {/if}
            </div>
          </div>
        {:else}
          <div class="loading-state">{t.loading}</div>
        {/if}
      </div>
    {:else if activeTab === "checksums"}
      <div class="tab-content">
        {#if checksumLoading}
          <div class="loading-state">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" class="spin">
              <path d="M8 2A6 6 0 1 0 14 8" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"/>
            </svg>
            Computing...
          </div>
        {:else if checksums}
          <div class="info-section">
            <div class="checksum-row">
              <span class="checksum-label">MD5</span>
              <div class="checksum-value-wrap">
                <code class="checksum-value">{checksums.md5}</code>
                <button class="copy-hash-btn" title="Copy" onclick={() => { if (checksums) navigator.clipboard.writeText(checksums.md5); }}>
                  <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
                    <rect x="5" y="5" width="9" height="9" rx="1.5" stroke="currentColor" stroke-width="1.5"/>
                    <path d="M11 5V3.5C11 2.67 10.33 2 9.5 2H3.5C2.67 2 2 2.67 2 3.5V9.5C2 10.33 2.67 11 3.5 11H5" stroke="currentColor" stroke-width="1.5"/>
                  </svg>
                </button>
              </div>
            </div>
            <div class="checksum-row">
              <span class="checksum-label">SHA-256</span>
              <div class="checksum-value-wrap">
                <code class="checksum-value">{checksums.sha256}</code>
                <button class="copy-hash-btn" title="Copy" onclick={() => { if (checksums) navigator.clipboard.writeText(checksums.sha256); }}>
                  <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
                    <rect x="5" y="5" width="9" height="9" rx="1.5" stroke="currentColor" stroke-width="1.5"/>
                    <path d="M11 5V3.5C11 2.67 10.33 2 9.5 2H3.5C2.67 2 2 2.67 2 3.5V9.5C2 10.33 2.67 11 3.5 11H5" stroke="currentColor" stroke-width="1.5"/>
                  </svg>
                </button>
              </div>
            </div>
          </div>
        {/if}
      </div>
    {/if}

    <div class="actions">
      <button class="btn-ok" onclick={close}>{t.close}</button>
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

  @keyframes fade { from { opacity: 0; } to { opacity: 1; } }

  .dialog {
    background: var(--glass-bg-strong);
    backdrop-filter: var(--glass-blur-strong);
    border: 1px solid var(--border-medium);
    border-radius: var(--radius-lg);
    padding: 0;
    width: 440px;
    max-height: 85vh;
    box-shadow: 0 20px 60px rgba(0,0,0,0.6), inset 0 1px 0 var(--border-light);
    animation: pop 0.18s cubic-bezier(0.16, 1, 0.3, 1);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  @keyframes pop {
    from { transform: scale(0.95) translateY(6px); opacity: 0; }
    to   { transform: scale(1) translateY(0); opacity: 1; }
  }

  .dialog-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 20px 20px 16px;
    border-bottom: 1px solid var(--border-light);
    background: var(--panel-bg-strong);
  }

  .header-preview {
    display: flex;
    align-items: center;
    gap: 14px;
    min-width: 0;
    flex: 1;
  }

  .preview-icon {
    width: 56px;
    height: 56px;
    border-radius: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--input-bg-focus);
    flex-shrink: 0;
    overflow: hidden;
  }

  .preview-thumb {
    width: 56px;
    height: 56px;
    object-fit: cover;
    border-radius: 14px;
  }

  .header-info {
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
    margin-bottom: 4px;
  }

  .header-type {
    font-size: 12px;
    color: var(--subtext0);
  }

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
    margin-left: 8px;
  }

  .close-btn:hover {
    background: var(--surface0);
    color: var(--text);
  }

  .tabs {
    display: flex;
    gap: 0;
    padding: 0 20px;
    border-bottom: 1px solid var(--border-light);
    background: var(--panel-bg-muted);
  }

  .tab {
    padding: 10px 16px;
    font-size: 12.5px;
    font-weight: 600;
    color: var(--overlay1);
    border-bottom: 2px solid transparent;
    transition: color 0.12s, border-color 0.12s;
    margin-bottom: -1px;
  }

  .tab:hover {
    color: var(--text);
  }

  .tab.active {
    color: var(--mauve);
    border-bottom-color: var(--mauve);
  }

  .tab-content {
    padding: 16px 20px;
    overflow-y: auto;
    flex: 1;
  }

  .info-section {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .divider {
    height: 1px;
    background: var(--border-light);
    margin: 14px 0;
  }

  .prop-row {
    display: grid;
    grid-template-columns: 85px 1fr;
    gap: 12px;
    align-items: baseline;
  }

  .label {
    font-size: 11.5px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--overlay0);
  }

  .value {
    font-size: 13px;
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .value.mono {
    font-family: monospace;
    font-size: 12px;
    color: var(--subtext0);
  }

  .dimmed {
    color: var(--subtext0);
    font-size: 11.5px;
  }

  .perm-display {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 6px;
  }

  .perm-octal {
    font-family: monospace;
    font-size: 22px;
    font-weight: 700;
    color: var(--mauve);
  }

  .perm-string {
    font-family: monospace;
    font-size: 14px;
    color: var(--subtext0);
    letter-spacing: 1px;
  }

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
  }

  .perm-cell {
    text-align: center;
    font-family: monospace;
    font-size: 13px;
    padding: 5px 0;
    border-radius: 6px;
    background: var(--input-bg);
    color: var(--overlay0);
    font-weight: 600;
  }

  .perm-cell.granted {
    background: var(--success-bg);
    color: var(--green);
  }

  .badge-row {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .attr-badge {
    font-size: 11px;
    font-weight: 600;
    padding: 3px 10px;
    border-radius: 20px;
    background: var(--input-bg-strong);
    color: var(--subtext0);
  }

  .attr-badge.readonly {
    background: var(--danger-muted);
    color: var(--maroon);
  }

  .attr-badge.executable {
    background: var(--success-bg);
    color: var(--green);
  }

  .loading-state {
    text-align: center;
    padding: 30px;
    color: var(--overlay1);
    font-size: 13px;
  }

  .actions {
    padding: 12px 20px;
    display: flex;
    justify-content: flex-end;
    border-top: 1px solid var(--border-light);
  }

  .btn-ok {
    padding: 7px 22px;
    border-radius: var(--radius-sm);
    font-size: 13px;
    color: var(--base);
    background: var(--mauve);
    font-weight: 600;
    transition: opacity 0.1s;
  }

  .btn-ok:hover { opacity: 0.88; }

  .checksum-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 14px;
  }

  .checksum-label {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--overlay0);
  }

  .checksum-value-wrap {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .checksum-value {
    font-family: monospace;
    font-size: 11px;
    color: var(--subtext0);
    background: var(--input-bg);
    padding: 6px 10px;
    border-radius: 6px;
    word-break: break-all;
    flex: 1;
    user-select: all;
  }

  .copy-hash-btn {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }

  .copy-hash-btn:hover {
    background: var(--hover-bg);
    color: var(--text);
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .spin {
    animation: spin 0.8s linear infinite;
  }

  .broken-link {
    color: var(--red);
    font-size: 11px;
    font-weight: 600;
    margin-left: 6px;
  }

  .error-state {
    text-align: center;
    padding: 30px;
    color: var(--red);
    font-size: 13px;
  }

  .edit-perm-btn {
    width: 24px;
    height: 24px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    transition: background 0.1s, color 0.1s;
    margin-left: 8px;
  }

  .edit-perm-btn:hover {
    background: var(--surface0);
    color: var(--text);
  }

  .perm-octal-input {
    font-family: monospace;
    font-size: 22px;
    font-weight: 700;
    color: var(--mauve);
    background: var(--input-bg);
    border: 1px solid var(--border-medium);
    border-radius: 6px;
    padding: 4px 10px;
    width: 80px;
    text-align: center;
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

  .btn-cancel {
    padding: 6px 16px;
    border-radius: var(--radius-sm);
    font-size: 12px;
    color: var(--subtext0);
    background: var(--surface0);
    font-weight: 600;
  }

  .btn-cancel:hover { background: var(--surface1); }

  .btn-save {
    padding: 6px 16px;
    border-radius: var(--radius-sm);
    font-size: 12px;
    color: var(--base);
    background: var(--green);
    font-weight: 600;
  }

  .btn-save:hover { opacity: 0.88; }

  .perm-error {
    color: var(--red);
    font-size: 12px;
    margin-top: 6px;
    padding: 6px 10px;
    background: var(--danger-subtle);
    border-radius: 6px;
  }
</style>
