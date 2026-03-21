import type { BookmarkEntry, DriveInfo } from "../types";

export interface TabSnapshot {
  id: string;
  path: string;
  history: string[];
  historyPos: number;
  searchQuery: string;
}

const MAX_TABS = 5;
const MAX_HISTORY = 100;
const MAX_RECENT = 20;
const RECENT_KEY = "luzumi_recent_locations";

export class NavigationState {
  currentPath = $state("");
  history = $state<string[]>([]);
  historyPos = $state(-1);
  tabs = $state<TabSnapshot[]>([]);
  activeTabIndex = $state(0);
  tabDropTarget = $state<number | null>(null);

  bookmarks = $state<BookmarkEntry[]>([]);
  drives = $state<DriveInfo[]>([]);
  quickAccess = $state<BookmarkEntry[]>([]);

  recentLocations = $state<string[]>([]);

  // Favorites (new)
  favorites = $state<Set<string>>(new Set());

  constructor() {
    this._loadRecentLocations();
    this._loadFavorites();
  }

  private _loadRecentLocations() {
    try {
      const raw = localStorage.getItem(RECENT_KEY);
      if (raw) this.recentLocations = JSON.parse(raw);
    } catch {}
  }

  private _loadFavorites() {
    try {
      const raw = localStorage.getItem("luzumi_favorites");
      if (raw) this.favorites = new Set(JSON.parse(raw));
    } catch {}
  }

  private _saveFavorites() {
    try {
      localStorage.setItem("luzumi_favorites", JSON.stringify([...this.favorites]));
    } catch {}
  }

  get canGoBack() { return this.historyPos > 0; }
  get canGoForward() { return this.historyPos < this.history.length - 1; }
  get canAddTab() { return this.tabs.length < MAX_TABS; }

  pushHistory(path: string) {
    let newHistory = [...this.history.slice(0, this.historyPos + 1), path];
    if (newHistory.length > MAX_HISTORY) {
      newHistory = newHistory.slice(newHistory.length - MAX_HISTORY);
    }
    this.history = newHistory;
    this.historyPos = newHistory.length - 1;
  }

  goBack(): string | null {
    if (!this.canGoBack) return null;
    this.historyPos--;
    return this.history[this.historyPos];
  }

  goForward(): string | null {
    if (!this.canGoForward) return null;
    this.historyPos++;
    return this.history[this.historyPos];
  }

  addRecentLocation(path: string) {
    const filtered = this.recentLocations.filter(p => p !== path);
    this.recentLocations = [path, ...filtered].slice(0, MAX_RECENT);
    try { localStorage.setItem(RECENT_KEY, JSON.stringify(this.recentLocations)); } catch {}
  }

  clearRecentLocations() {
    this.recentLocations = [];
    try { localStorage.removeItem(RECENT_KEY); } catch {}
  }

  toggleFavorite(path: string) {
    const next = new Set(this.favorites);
    if (next.has(path)) next.delete(path);
    else next.add(path);
    this.favorites = next;
    this._saveFavorites();
  }

  isFavorite(path: string): boolean {
    return this.favorites.has(path);
  }

  // Tab management
  initTab(path: string) {
    this.tabs = [{
      id: "1",
      path,
      history: [...this.history],
      historyPos: this.historyPos,
      searchQuery: "",
    }];
    this.activeTabIndex = 0;
  }

  saveActiveTab(searchQuery: string) {
    if (!this.tabs[this.activeTabIndex]) return;
    this.tabs[this.activeTabIndex] = {
      ...this.tabs[this.activeTabIndex],
      path: this.currentPath,
      history: [...this.history],
      historyPos: this.historyPos,
      searchQuery,
    };
  }

  prepareTabSwitch(index: number): TabSnapshot | null {
    if (index === this.activeTabIndex || index < 0 || index >= this.tabs.length) return null;
    this.activeTabIndex = index;
    const tab = this.tabs[index];
    if (!tab) return null;
    this.history = [...tab.history];
    this.historyPos = tab.historyPos;
    this.currentPath = tab.path;
    return tab;
  }

  addTab(path: string): TabSnapshot | null {
    if (this.tabs.length >= MAX_TABS) return null;
    const newTab: TabSnapshot = {
      id: Date.now().toString(),
      path,
      history: [path],
      historyPos: 0,
      searchQuery: "",
    };
    this.tabs = [...this.tabs, newTab];
    this.activeTabIndex = this.tabs.length - 1;
    this.history = [path];
    this.historyPos = 0;
    this.currentPath = "";
    return newTab;
  }

  closeTab(index: number): { newIndex: number; needsLoad: boolean } | null {
    if (this.tabs.length <= 1) return null;
    const newTabs = this.tabs.filter((_, i) => i !== index);
    const newIndex = Math.min(index, newTabs.length - 1);
    this.tabs = newTabs;
    if (index === this.activeTabIndex) {
      this.activeTabIndex = newIndex;
      return { newIndex, needsLoad: true };
    } else if (index < this.activeTabIndex) {
      this.activeTabIndex = this.activeTabIndex - 1;
    }
    return { newIndex, needsLoad: false };
  }

  getHistoryEntries(): Array<{ path: string; index: number; isCurrent: boolean }> {
    return this.history.map((path, index) => ({
      path,
      index,
      isCurrent: index === this.historyPos,
    }));
  }
}
