import type { FileEntry, DriveInfo, ContextMenuState } from "../types";

const ICON_SIZE_MIN = 48;
const ICON_SIZE_MAX = 128;
const ICON_SIZE_DEFAULT = 64;

export type ViewDensity = "comfortable" | "compact" | "dense";

export class UiState {
  viewMode = $state<"list" | "grid">("grid");
  showHidden = $state(false);
  sortBy = $state<"name" | "size" | "date" | "type">("name");
  sortDir = $state<"asc" | "desc">("asc");

  // Dialogs
  showNewFolder = $state(false);
  newFolderName = $state("New Folder");
  showNewFile = $state(false);
  newFileName = $state("untitled.txt");
  showProperties = $state<FileEntry | null>(null);
  showSecureDeleteConfirm = $state(false);
  showUnlockDialog = $state<DriveInfo | null>(null);
  showSaveAsDialog = $state(false);
  saveAsSource = $state<string | null>(null);
  showBulkRename = $state(false);
  showDuplicateFinder = $state(false);
  showKeyboardShortcuts = $state(false);
  showSelectPattern = $state(false);
  showEmptyTrashConfirm = $state(false);
  showDragDropDialog = $state(false);
  showSettings = $state(false);
  showCommandPalette = $state(false);
  showTagManager = $state(false);
  showFolderCompare = $state(false);
  showOpenWith = $state<string | null>(null);

  // Panels
  showPreview = $state(false);
  showDebugPanel = $state(false);
  splitView = $state(false);

  // Context menu
  contextMenu = $state<ContextMenuState | null>(null);

  // Rename
  renameTarget = $state<string | null>(null);
  renameBuffer = $state("");

  // Icons & Appearance
  gridIconSize = $state(ICON_SIZE_DEFAULT);
  showHoverBox = $state(false);
  density = $state<ViewDensity>("comfortable");
  fontSize = $state(14);
  lineHeight = $state(1.5);
  highContrast = $state(false);
  animationsEnabled = $state(true);

  // Status
  error = $state<string | null>(null);
  statusMessage = $state<string | null>(null);
  isLoading = $state(false);
  focusAddressBar = $state(false);
  focusSearch = $state(false);

  // Default file manager
  isDefaultFileManager = $state(true);
  showDefaultBanner = $state(false);

  // Portal
  portalInstalled = $state(false);

  // Themes
  currentTheme = $state("Default");
  availableThemes = $state<string[]>([]);

  // Sidebar
  showRecents = $state(false);

  // Onboarding
  onboardingDone = $state(false);
  showOnboarding = $state(false);

  // Elevated
  elevated = $state(false);

  private _statusTimer: ReturnType<typeof setTimeout> | null = null;
  private _iconSizeTimer: ReturnType<typeof setTimeout> | null = null;

  constructor() {
    this._loadPreferences();
  }

  private _loadPreferences() {
    try {
      // Icon size
      const v = parseInt(localStorage.getItem("luzumi_icon_size") ?? String(ICON_SIZE_DEFAULT));
      this.gridIconSize = isNaN(v) ? ICON_SIZE_DEFAULT : Math.max(ICON_SIZE_MIN, Math.min(ICON_SIZE_MAX, v));

      // Hover box
      this.showHoverBox = localStorage.getItem("luzumi_hover_box") === "true";

      // Sort
      const sb = localStorage.getItem("luzumi_sort_by");
      const sd = localStorage.getItem("luzumi_sort_dir");
      if (sb === "name" || sb === "size" || sb === "date" || sb === "type") this.sortBy = sb;
      if (sd === "asc" || sd === "desc") this.sortDir = sd;

      // Theme
      this.currentTheme = localStorage.getItem("luzumi_theme") ?? "Default";

      // Density
      const den = localStorage.getItem("luzumi_density");
      if (den === "comfortable" || den === "compact" || den === "dense") this.density = den;

      // Font size
      const fs = parseInt(localStorage.getItem("luzumi_font_size") ?? "14");
      if (!isNaN(fs) && fs >= 10 && fs <= 22) this.fontSize = fs;

      // Line height
      const lh = parseFloat(localStorage.getItem("luzumi_line_height") ?? "1.5");
      if (!isNaN(lh) && lh >= 1.0 && lh <= 2.5) this.lineHeight = lh;

      // High contrast
      this.highContrast = localStorage.getItem("luzumi_high_contrast") === "true";

      // Animations
      this.animationsEnabled = localStorage.getItem("luzumi_animations") !== "false";

      // Recents sidebar
      this.showRecents = localStorage.getItem("luzumi_show_recents") === "true";

      // Onboarding
      this.onboardingDone = localStorage.getItem("luzumi_onboarding_done") === "true";
    } catch {}
  }

  setSort(field: "name" | "size" | "date" | "type") {
    if (this.sortBy === field) {
      this.sortDir = this.sortDir === "asc" ? "desc" : "asc";
    } else {
      this.sortBy = field;
      this.sortDir = "asc";
    }
    try { localStorage.setItem("luzumi_sort_by", this.sortBy); } catch {}
    try { localStorage.setItem("luzumi_sort_dir", this.sortDir); } catch {}
  }

  setGridIconSize(size: number) {
    this.gridIconSize = Math.max(ICON_SIZE_MIN, Math.min(ICON_SIZE_MAX, size));
    if (this._iconSizeTimer) clearTimeout(this._iconSizeTimer);
    this._iconSizeTimer = setTimeout(() => {
      try { localStorage.setItem("luzumi_icon_size", String(this.gridIconSize)); } catch {}
    }, 300);
  }

  toggleHoverBox() {
    this.showHoverBox = !this.showHoverBox;
    try { localStorage.setItem("luzumi_hover_box", String(this.showHoverBox)); } catch {}
  }

  setDensity(density: ViewDensity) {
    this.density = density;
    try { localStorage.setItem("luzumi_density", density); } catch {}
  }

  setFontSize(size: number) {
    this.fontSize = Math.max(10, Math.min(22, size));
    try { localStorage.setItem("luzumi_font_size", String(this.fontSize)); } catch {}
  }

  setLineHeight(height: number) {
    this.lineHeight = Math.max(1.0, Math.min(2.5, height));
    try { localStorage.setItem("luzumi_line_height", String(this.lineHeight)); } catch {}
  }

  toggleHighContrast() {
    this.highContrast = !this.highContrast;
    try { localStorage.setItem("luzumi_high_contrast", String(this.highContrast)); } catch {}
  }

  toggleRecents() {
    this.showRecents = !this.showRecents;
    try { localStorage.setItem("luzumi_show_recents", String(this.showRecents)); } catch {}
  }

  toggleAnimations() {
    this.animationsEnabled = !this.animationsEnabled;
    try { localStorage.setItem("luzumi_animations", String(this.animationsEnabled)); } catch {}
  }

  setStatus(msg: string) {
    this.statusMessage = msg;
    if (this._statusTimer) clearTimeout(this._statusTimer);
    this._statusTimer = setTimeout(() => { this.statusMessage = null; }, 4000);
  }

  clearError() { this.error = null; }
  clearStatus() { this.statusMessage = null; }

  completeOnboarding() {
    this.onboardingDone = true;
    this.showOnboarding = false;
    try { localStorage.setItem("luzumi_onboarding_done", "true"); } catch {}
  }

  destroy() {
    if (this._statusTimer) { clearTimeout(this._statusTimer); this._statusTimer = null; }
    if (this._iconSizeTimer) { clearTimeout(this._iconSizeTimer); this._iconSizeTimer = null; }
  }
}
