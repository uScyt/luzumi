<script lang="ts">
  import { fm } from "../fileManager.svelte";
  import { t } from "../i18n";
</script>

<div class="banner">
  <div class="banner-left">
    <svg width="13" height="13" viewBox="0 0 16 16" fill="none" class="banner-icon">
      <path d="M2 4.5C2 3.67 2.67 3 3.5 3H6.38C6.76 3 7.12 3.16 7.38 3.44L8.62 4.78C8.88 5.06 9.24 5.22 9.62 5.22H12.5C13.33 5.22 14 5.89 14 6.72V11.5C14 12.33 13.33 13 12.5 13H3.5C2.67 13 2 12.33 2 11.5V4.5Z" stroke="currentColor" stroke-width="1.4" fill="none"/>
    </svg>
    <span class="banner-text">{t.setDefaultBannerText}</span>
  </div>
  <div class="banner-right">
    <button class="set-btn" onclick={() => fm.setAsDefaultFileManager()}>{t.setDefaultBtn}</button>
    <button class="dismiss-btn" onclick={() => fm.dismissDefaultBanner()} aria-label={t.dismissClose}>
      <svg width="9" height="9" viewBox="0 0 10 10" fill="none">
        <path d="M1 1L9 9M9 1L1 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
    </button>
  </div>
</div>

<style>
  .banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    height: 36px;
    padding: 0 14px;
    background: var(--blue-subtle);
    border-bottom: 1px solid var(--blue-muted);
    flex-shrink: 0;
  }

  .banner-left {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .banner-icon {
    flex-shrink: 0;
    color: var(--blue);
    opacity: 0.8;
  }

  .banner-text {
    font-size: 12px;
    color: var(--subtext0);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .banner-right {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .set-btn {
    font-size: 11px;
    padding: 3px 10px;
    border-radius: 6px;
    background: var(--accent-glow);
    color: var(--mauve);
    transition: background 0.1s;
    white-space: nowrap;
  }

  .set-btn:hover {
    background: var(--accent-btn-hover);
  }

  .dismiss-btn {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--overlay0);
    transition: background 0.1s, color 0.1s;
  }

  .dismiss-btn:hover {
    background: var(--hover-bg);
    color: var(--subtext0);
  }
</style>
