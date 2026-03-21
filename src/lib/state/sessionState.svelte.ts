import { invoke } from "@tauri-apps/api/core";

export interface SessionData {
  tabs: Array<{
    path: string;
    scrollPos: number;
    viewMode: string;
    searchQuery: string;
  }>;
  activeTabIndex: number;
  splitView: { enabled: boolean; path: string; ratio: number };
  sidebarWidth: number;
  previewOpen: boolean;
  windowBounds: { x: number; y: number; width: number; height: number };
  showHidden: boolean;
  sortBy: string;
  sortDir: string;
  viewMode: string;
}

const SESSION_SAVE_DEBOUNCE = 30000; // 30s auto-save

export class SessionState {
  private _saveTimer: ReturnType<typeof setTimeout> | null = null;
  private _lastSaved: SessionData | null = null;
  restoring = $state(false);

  async saveSession(data: SessionData) {
    try {
      await invoke("cmd_save_session", { session: JSON.stringify(data) });
      this._lastSaved = data;
    } catch {}
  }

  async loadSession(): Promise<SessionData | null> {
    try {
      const raw = await invoke<string | null>("cmd_load_session");
      if (raw) return JSON.parse(raw);
    } catch {}
    return null;
  }

  startAutoSave(getSessionData: () => SessionData) {
    this.stopAutoSave();
    this._saveTimer = setInterval(() => {
      const data = getSessionData();
      this.saveSession(data);
    }, SESSION_SAVE_DEBOUNCE);
  }

  stopAutoSave() {
    if (this._saveTimer) { clearInterval(this._saveTimer); this._saveTimer = null; }
  }

  destroy() {
    this.stopAutoSave();
  }
}
