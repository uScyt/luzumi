<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let { filePath }: { filePath: string } = $props();

  let hexData = $state<{ offset: number; bytes: number[]; total_size: number } | null>(null);
  let currentOffset = $state(0);
  let loading = $state(true);
  let error = $state<string | null>(null);

  const BYTES_PER_PAGE = 256; // 16 rows of 16 bytes

  $effect(() => {
    if (filePath) {
      currentOffset = 0;
      loadHex(filePath, 0);
    }
  });

  async function loadHex(path: string, offset: number) {
    loading = true;
    error = null;
    try {
      hexData = await invoke<{ offset: number; bytes: number[]; total_size: number }>("cmd_read_hex", {
        path, offset, length: BYTES_PER_PAGE,
      });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function prevPage() {
    const newOffset = Math.max(0, currentOffset - BYTES_PER_PAGE);
    currentOffset = newOffset;
    loadHex(filePath, newOffset);
  }

  function nextPage() {
    if (!hexData) return;
    const newOffset = currentOffset + BYTES_PER_PAGE;
    if (newOffset < hexData.total_size) {
      currentOffset = newOffset;
      loadHex(filePath, newOffset);
    }
  }

  function toHex(byte: number): string {
    return byte.toString(16).padStart(2, "0").toUpperCase();
  }

  function toAscii(byte: number): string {
    return byte >= 32 && byte < 127 ? String.fromCharCode(byte) : ".";
  }

  function formatOffset(offset: number): string {
    return offset.toString(16).padStart(8, "0").toUpperCase();
  }

  const rows = $derived.by(() => {
    if (!hexData?.bytes.length) return [];
    const result: Array<{ offset: number; hex: string[]; ascii: string }> = [];
    for (let i = 0; i < hexData.bytes.length; i += 16) {
      const slice = hexData.bytes.slice(i, i + 16);
      result.push({
        offset: hexData.offset + i,
        hex: slice.map(b => toHex(b)),
        ascii: slice.map(b => toAscii(b)).join(""),
      });
    }
    return result;
  });
</script>

<div class="hex-viewer">
  {#if error}
    <div class="hex-error">{error}</div>
  {:else}
    <div class="hex-toolbar">
      <button onclick={prevPage} disabled={currentOffset <= 0}>Prev</button>
      <span class="hex-offset-info">
        Offset: 0x{formatOffset(currentOffset)}
        {#if hexData} / {hexData.total_size.toLocaleString()} bytes{/if}
      </span>
      <button onclick={nextPage} disabled={!hexData || currentOffset + BYTES_PER_PAGE >= hexData.total_size}>Next</button>
    </div>
    <div class="hex-content">
      {#if loading}
        <div class="hex-loading">Loading...</div>
      {:else}
        <table class="hex-table">
          <thead>
            <tr>
              <th class="offset-col">Offset</th>
              {#each Array(16) as _, i}
                <th class="hex-col">{toHex(i)}</th>
              {/each}
              <th class="ascii-col">ASCII</th>
            </tr>
          </thead>
          <tbody>
            {#each rows as row}
              <tr>
                <td class="offset-col">{formatOffset(row.offset)}</td>
                {#each Array(16) as _, i}
                  <td class="hex-col">{row.hex[i] ?? ""}</td>
                {/each}
                <td class="ascii-col">{row.ascii}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  {/if}
</div>

<style>
  .hex-viewer {
    height: 100%;
    display: flex;
    flex-direction: column;
    font-family: "JetBrains Mono", "Fira Code", monospace;
    font-size: 11px;
  }

  .hex-toolbar {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 6px 12px;
    border-bottom: 1px solid var(--border-color, #45475a);
    flex-shrink: 0;
  }

  .hex-toolbar button {
    background: var(--bg-tertiary, #45475a);
    border: none;
    color: var(--text-primary, #cdd6f4);
    border-radius: 4px;
    padding: 3px 10px;
    cursor: pointer;
    font-size: 11px;
  }

  .hex-toolbar button:disabled { opacity: 0.3; cursor: default; }
  .hex-toolbar button:not(:disabled):hover { background: var(--bg-hover, #313244); }

  .hex-offset-info {
    color: var(--text-secondary, #a6adc8);
    font-size: 11px;
  }

  .hex-content {
    flex: 1;
    overflow: auto;
    padding: 4px;
  }

  .hex-table {
    border-collapse: collapse;
    width: 100%;
  }

  .hex-table th, .hex-table td {
    padding: 1px 4px;
    text-align: center;
    white-space: nowrap;
  }

  .hex-table th {
    color: var(--accent, #89b4fa);
    font-weight: 600;
    position: sticky;
    top: 0;
    background: var(--bg-primary, #1e1e2e);
    border-bottom: 1px solid var(--border-color, #45475a);
  }

  .offset-col {
    text-align: left !important;
    color: var(--text-secondary, #a6adc8);
    padding-right: 12px !important;
  }

  .hex-col {
    color: var(--text-primary, #cdd6f4);
  }

  .ascii-col {
    text-align: left !important;
    color: var(--success, #a6e3a1);
    padding-left: 12px !important;
    letter-spacing: 0.5px;
  }

  .hex-loading, .hex-error {
    padding: 24px;
    text-align: center;
    color: var(--text-secondary, #a6adc8);
  }

  .hex-error { color: var(--danger, #f38ba8); }
</style>
