<script lang="ts">
  import { fm } from "../fileManager.svelte";

  const steps = [
    {
      target: ".sidebar",
      title: "Sidebar",
      description: "Navigate your filesystem with bookmarks, quick access, drives, and trash. Right-click folders to pin them.",
      position: "right" as const,
    },
    {
      target: ".toolbar",
      title: "Toolbar",
      description: "Navigate back/forward, go up, switch views, search files, and access settings.",
      position: "bottom" as const,
    },
    {
      target: ".address-bar",
      title: "Address Bar",
      description: "Click any breadcrumb to navigate. Press Ctrl+L to type a path directly.",
      position: "bottom" as const,
    },
    {
      target: ".search-bar",
      title: "Search",
      description: "Press Ctrl+F to search. Type to filter files in the current directory, or search globally via the index.",
      position: "bottom" as const,
    },
    {
      target: ".file-list, .file-grid",
      title: "Files",
      description: "Double-click to open. Right-click for context menu. Ctrl+Click for multi-select. Drag to move files.",
      position: "top" as const,
    },
    {
      target: ".status-bar",
      title: "Tips",
      description: "Use Ctrl+Shift+P for the command palette. Press ? to see all keyboard shortcuts. Enjoy Luzumi!",
      position: "top" as const,
    },
  ];

  let currentStep = $state(0);

  function next() {
    if (currentStep < steps.length - 1) {
      currentStep++;
    } else {
      finish();
    }
  }

  function skip() {
    finish();
  }

  function finish() {
    fm.ui.completeOnboarding();
  }

  const step = $derived(steps[currentStep]);
  const isLast = $derived(currentStep === steps.length - 1);
</script>

{#if fm.ui.showOnboarding && !fm.ui.onboardingDone}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="onboarding-overlay" onclick={skip}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="onboarding-tooltip" onclick={(e) => e.stopPropagation()}>
      <div class="onboarding-step-indicator">
        {#each steps as _, i}
          <div class="step-dot" class:active={i === currentStep} class:done={i < currentStep}></div>
        {/each}
      </div>
      <h3 class="onboarding-title">{step.title}</h3>
      <p class="onboarding-desc">{step.description}</p>
      <div class="onboarding-actions">
        <button class="onboarding-skip" onclick={skip}>Skip</button>
        <button class="onboarding-next" onclick={next}>
          {isLast ? "Done" : "Next"}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .onboarding-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    z-index: 10000;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .onboarding-tooltip {
    background: var(--bg-secondary, #1e1e2e);
    border: 1px solid var(--accent, #89b4fa);
    border-radius: 12px;
    padding: 24px;
    max-width: 380px;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.5);
  }

  .onboarding-step-indicator {
    display: flex;
    gap: 6px;
    margin-bottom: 16px;
    justify-content: center;
  }

  .step-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--bg-tertiary, #45475a);
    transition: background 0.2s;
  }

  .step-dot.active {
    background: var(--accent, #89b4fa);
    transform: scale(1.2);
  }

  .step-dot.done {
    background: var(--success, #a6e3a1);
  }

  .onboarding-title {
    margin: 0 0 8px;
    font-size: 18px;
    color: var(--text-primary, #cdd6f4);
    font-weight: 600;
  }

  .onboarding-desc {
    margin: 0 0 20px;
    font-size: 13px;
    color: var(--text-secondary, #a6adc8);
    line-height: 1.6;
  }

  .onboarding-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
  }

  .onboarding-skip {
    background: none;
    border: 1px solid var(--border-color, #45475a);
    color: var(--text-secondary, #a6adc8);
    padding: 6px 16px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
  }

  .onboarding-skip:hover {
    background: var(--bg-hover, #313244);
  }

  .onboarding-next {
    background: var(--accent, #89b4fa);
    border: none;
    color: var(--bg-primary, #1e1e2e);
    padding: 6px 20px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
    font-weight: 600;
  }

  .onboarding-next:hover {
    opacity: 0.9;
  }
</style>
