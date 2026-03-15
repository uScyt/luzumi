<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { t } from "../i18n";

  let expanded = $state(false);

  const progress = $derived(() => {
    if (!fm.deleteProgress || fm.deleteProgress.total === 0) return 0;
    return Math.round((fm.deleteProgress.done / fm.deleteProgress.total) * 100);
  });

  const filename = $derived(() => {
    const c = fm.deleteProgress?.current ?? "";
    return c.split("/").pop() || c;
  });
</script>

{#if fm.deleteProgress}
  <div class="progress-bar-wrap" class:expanded>
    {#if expanded}
      <div class="progress-expanded">
        <div class="progress-header">
          <div class="progress-title">
            <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
              <path d="M3 5H13M5.5 5V3.5C5.5 3.22 5.72 3 6 3H10C10.28 3 10.5 3.22 10.5 3.5V5M12 5V13C12 13.55 11.55 14 11 14H5C4.45 14 4 13.55 4 13V5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            {t.secureDeleting}
          </div>
          <button class="toggle-btn" onclick={() => expanded = false} title={t.collapse}>
            <svg width="11" height="11" viewBox="0 0 16 16" fill="none">
              <path d="M3 10L8 5L13 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
        </div>
        <div class="progress-file">{fm.deleteProgress.current || t.preparing}</div>
        <div class="progress-track">
          <div class="progress-fill" style="width: {progress()}%"></div>
        </div>
        <div class="progress-stats">
          <span>{fm.deleteProgress.done} / {fm.deleteProgress.total} {t.files}</span>
          <span>{progress()}%</span>
        </div>
      </div>
    {:else}
      <div class="progress-mini">
        <svg width="11" height="11" viewBox="0 0 16 16" fill="none" style="flex-shrink:0">
          <path d="M3 5H13M5.5 5V3.5C5.5 3.22 5.72 3 6 3H10C10.28 3 10.5 3.22 10.5 3.5V5M12 5V13C12 13.55 11.55 14 11 14H5C4.45 14 4 13.55 4 13V5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <div class="mini-track">
          <div class="mini-fill" style="width: {progress()}%"></div>
        </div>
        <span class="mini-pct">{progress()}%</span>
        <span class="mini-name">{filename()}</span>
        <button class="toggle-btn" onclick={() => expanded = true} title={t.expand}>
          <svg width="11" height="11" viewBox="0 0 16 16" fill="none">
            <path d="M3 6L8 11L13 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
      </div>
    {/if}
  </div>
{/if}

<style>
  .progress-bar-wrap {
    position: fixed;
    bottom: 29px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 600;
    animation: slideUp 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes slideUp {
    from { opacity: 0; transform: translateX(-50%) translateY(8px); }
    to   { opacity: 1; transform: translateX(-50%) translateY(0); }
  }

  .progress-mini {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 32px;
    padding: 0 12px;
    background: var(--glass-bg-strong);
    backdrop-filter: blur(20px) saturate(1.5);
    border: 1px solid var(--danger-bg);
    border-radius: 10px;
    box-shadow: 0 4px 20px rgba(0,0,0,0.4);
    color: var(--maroon);
    white-space: nowrap;
    min-width: 260px;
    max-width: 440px;
  }

  .mini-track {
    flex: 1;
    height: 4px;
    background: var(--danger-bg);
    border-radius: 2px;
    overflow: hidden;
    min-width: 60px;
  }

  .mini-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--maroon), var(--red));
    border-radius: 2px;
    transition: width 0.3s ease;
  }

  .mini-pct {
    font-size: 11px;
    font-weight: 700;
    color: var(--red);
    font-variant-numeric: tabular-nums;
    min-width: 30px;
    text-align: right;
  }

  .mini-name {
    font-size: 11px;
    color: var(--subtext0);
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 140px;
  }

  .toggle-btn {
    width: 22px;
    height: 22px;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay1);
    transition: background 0.1s, color 0.1s;
    flex-shrink: 0;
  }

  .toggle-btn:hover {
    background: var(--hover-bg);
    color: var(--text);
  }

  .progress-expanded {
    width: 320px;
    background: var(--glass-bg-strong);
    backdrop-filter: blur(24px) saturate(1.5);
    border: 1px solid var(--danger-bg);
    border-radius: 12px;
    padding: 14px 16px 12px;
    box-shadow: 0 8px 32px rgba(0,0,0,0.5);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .progress-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .progress-title {
    display: flex;
    align-items: center;
    gap: 7px;
    font-size: 12px;
    font-weight: 600;
    color: var(--maroon);
  }

  .progress-file {
    font-size: 11.5px;
    color: var(--subtext0);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .progress-track {
    height: 6px;
    background: var(--danger-bg);
    border-radius: 3px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--maroon), var(--red));
    border-radius: 3px;
    transition: width 0.3s ease;
  }

  .progress-stats {
    display: flex;
    justify-content: space-between;
    font-size: 10.5px;
    color: var(--overlay1);
    font-variant-numeric: tabular-nums;
  }
</style>
