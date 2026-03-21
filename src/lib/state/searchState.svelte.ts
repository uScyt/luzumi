import { invoke } from "@tauri-apps/api/core";
import type { FileEntry } from "../types";

export interface SearchFilter {
  types: string[];
  sizeMin?: number;
  sizeMax?: number;
  dateAfter?: number;
  dateBefore?: number;
  namePattern?: string;
  contentQuery?: string;
}

export interface SearchHistoryItem {
  query: string;
  filters: SearchFilter;
  timestamp: number;
  resultCount: number;
}

export interface ContentMatch {
  path: string;
  lineNumber: number;
  lineText: string;
  contextBefore: string[];
  contextAfter: string[];
}

const SEARCH_HISTORY_MAX = 50;
const SEARCH_HISTORY_KEY = "luzumi_search_history";

export class SearchState {
  query = $state("");
  mode = $state<"filename" | "content" | "advanced">("filename");
  filters = $state<SearchFilter>({ types: [] });
  results = $state<FileEntry[]>([]);
  contentResults = $state<ContentMatch[]>([]);
  isSearching = $state(false);
  showAdvancedPanel = $state(false);
  history = $state<SearchHistoryItem[]>([]);

  // Global search (index-based)
  globalResults = $state<FileEntry[]>([]);
  globalLoading = $state(false);

  private _debounce: ReturnType<typeof setTimeout> | null = null;

  constructor() {
    this._loadHistory();
  }

  private _loadHistory() {
    try {
      const raw = localStorage.getItem(SEARCH_HISTORY_KEY);
      if (raw) this.history = JSON.parse(raw);
    } catch {}
  }

  private _saveHistory() {
    try {
      localStorage.setItem(SEARCH_HISTORY_KEY, JSON.stringify(this.history.slice(0, SEARCH_HISTORY_MAX)));
    } catch {}
  }

  addToHistory(query: string, filters: SearchFilter, resultCount: number) {
    const item: SearchHistoryItem = { query, filters, timestamp: Date.now(), resultCount };
    this.history = [item, ...this.history.filter(h => h.query !== query)].slice(0, SEARCH_HISTORY_MAX);
    this._saveHistory();
  }

  clearHistory() {
    this.history = [];
    try { localStorage.removeItem(SEARCH_HISTORY_KEY); } catch {}
  }

  setQuery(query: string) {
    this.query = query;
  }

  setMode(mode: "filename" | "content" | "advanced") {
    this.mode = mode;
  }

  resetFilters() {
    this.filters = { types: [] };
  }

  triggerGlobalSearch(query: string, currentEntries: FileEntry[]) {
    if (this._debounce) clearTimeout(this._debounce);
    if (!query || query.length < 2) {
      this.globalResults = [];
      this.globalLoading = false;
      return;
    }
    this.globalLoading = true;
    this._debounce = setTimeout(async () => {
      try {
        const results = await invoke<FileEntry[]>("cmd_search_index", { query, limit: 50 });
        const currentPaths = new Set(currentEntries.map((e) => e.path));
        this.globalResults = results.filter((r) => !currentPaths.has(r.path));
      } catch {
        this.globalResults = [];
      }
      this.globalLoading = false;
    }, 300);
  }

  async searchAdvanced(query: string, filters: SearchFilter, path: string, recursive: boolean = true) {
    this.isSearching = true;
    this.results = [];
    try {
      const results = await invoke<FileEntry[]>("cmd_search_advanced", {
        query, filters, path, recursive,
      });
      this.results = results;
      this.addToHistory(query, filters, results.length);
    } catch (e) {
      this.results = [];
      throw e;
    } finally {
      this.isSearching = false;
    }
  }

  async searchContent(query: string, path: string, recursive: boolean = true) {
    this.isSearching = true;
    this.contentResults = [];
    try {
      const results = await invoke<ContentMatch[]>("cmd_search_content", {
        query, path, recursive, maxMatches: 500,
      });
      this.contentResults = results;
    } catch (e) {
      this.contentResults = [];
      throw e;
    } finally {
      this.isSearching = false;
    }
  }

  cancelSearch() {
    if (this._debounce) { clearTimeout(this._debounce); this._debounce = null; }
    invoke("cmd_search_cancel").catch(() => {});
    this.isSearching = false;
  }

  destroy() {
    if (this._debounce) { clearTimeout(this._debounce); this._debounce = null; }
  }
}
