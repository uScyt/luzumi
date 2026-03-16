import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { FileEntry, BookmarkEntry, DriveInfo, ContextMenuState } from "./types";
import { t } from "./i18n";

function parentDir(path: string): string {
  const idx = path.lastIndexOf("/");
  if (idx <= 0) return "/";
  return path.substring(0, idx);
}

function baseName(path: string): string {
  const idx = path.lastIndexOf("/");
  return idx === -1 ? path : path.substring(idx + 1);
}

interface FileFilter {
  name: string;
  patterns: string[];
}

interface PickerConfig {
  mode: "open" | "save";
  response_file: string;
  multiple: boolean;
  directory: boolean;
  title: string;
  filters: FileFilter[];
  current_folder: string | null;
  current_name: string | null;
}

interface TabSnapshot {
  id: string;
  path: string;
  history: string[];
  historyPos: number;
  searchQuery: string;
}

type UndoOp =
  | { type: "rename"; newPath: string; origName: string }
  | { type: "trash"; fileNames: string[] }
  | { type: "create"; path: string }
  | { type: "move"; moves: Array<{ newPath: string; origParent: string }> }
  | { type: "duplicate"; copies: string[] };

const ICON_SIZE_MIN = 48;
const ICON_SIZE_MAX = 128;
const ICON_SIZE_DEFAULT = 64;
const UNDO_STACK_LIMIT = 20;
const WATCH_INTERVAL_MS = 2500;
const DRIVE_REFRESH_MS = 3000;

class FileManager {
  currentPath = $state("");
  entries = $state<FileEntry[]>([]);
  selected = $state<Set<string>>(new Set());
  history = $state<string[]>([]);
  historyPos = $state(-1);
  searchQuery = $state("");
  showHidden = $state(false);
  viewMode = $state<"list" | "grid">("grid");
  bookmarks = $state<BookmarkEntry[]>([]);
  drives = $state<DriveInfo[]>([]);
  clipboard = $state<{ paths: string[]; mode: "copy" | "cut" } | null>(null);
  error = $state<string | null>(null);
  statusMessage = $state<string | null>(null);
  isLoading = $state(false);
  renameTarget = $state<string | null>(null);
  renameBuffer = $state("");
  contextMenu = $state<ContextMenuState | null>(null);
  showNewFolder = $state(false);
  newFolderName = $state("New Folder");
  showProperties = $state<FileEntry | null>(null);
  dragPaths = $state<string[]>([]);
  dropTarget = $state<string | null>(null);
  pendingDrop = $state<{ paths: string[]; dest: string } | null>(null);
  showDragDropDialog = $state(false);
  lastSelected = $state<string | null>(null);
  showSecureDeleteConfirm = $state(false);
  showUnlockDialog = $state<DriveInfo | null>(null);
  showSaveAsDialog = $state(false);
  saveAsSource = $state<string | null>(null);
  showNewFile = $state(false);
  newFileName = $state("untitled.txt");
  sortBy = $state<"name" | "size" | "date" | "type">("name");
  sortDir = $state<"asc" | "desc">("asc");
  driveRefreshInterval: ReturnType<typeof setInterval> | null = null;
  private _watchInterval: ReturnType<typeof setInterval> | null = null;
  private _watchUnlisten: (() => void) | null = null;
  private _trashUnlisten: (() => void) | null = null;
  private _splitUnlisten: (() => void) | null = null;
  elevated = $state(false);
  undoStack = $state<UndoOp[]>([]);
  redoStack = $state<UndoOp[]>([]);
  focusAddressBar = $state(false);
  focusSearch = $state(false);
  globalSearchResults = $state<FileEntry[]>([]);
  globalSearchLoading = $state(false);
  private _searchDebounce: ReturnType<typeof setTimeout> | null = null;
  deleteProgress = $state<{ current: string; done: number; total: number } | null>(null);
  copyMoveProgress = $state<{ current: string; done: number; total: number } | null>(null);
  diskSpace = $state<{ total: number; available: number; used: number } | null>(null);
  gitStatus = $state<{ is_repo: boolean; branch: string; modified: string[]; staged: string[]; untracked: string[] } | null>(null);
  splitView = $state(false);
  splitPath = $state("");
  splitEntries = $state<FileEntry[]>([]);
  splitFocused = $state(false);
  gridIconSize = $state((() => {
    try {
      const v = parseInt(localStorage.getItem("luzumi_icon_size") ?? String(ICON_SIZE_DEFAULT));
      return isNaN(v) ? ICON_SIZE_DEFAULT : Math.max(ICON_SIZE_MIN, Math.min(ICON_SIZE_MAX, v));
    } catch { return 64; }
  })());
  showHoverBox = $state((() => {
    try { return localStorage.getItem("luzumi_hover_box") === "true"; } catch { return false; }
  })());
  showSettings = $state(false);
  isDefaultFileManager = $state(true);
  showDefaultBanner = $state(false);
  private _statusTimer: ReturnType<typeof setTimeout> | null = null;
  private _iconSizeTimer: ReturnType<typeof setTimeout> | null = null;
  private _filteredCache: FileEntry[] | null = null;
  private _filterCacheKey = "";

  trashSize = $state(0);
  showTrashView = $state(false);
  trashEntries = $state<FileEntry[]>([]);
  showEmptyTrashConfirm = $state(false);

  quickAccess = $state<BookmarkEntry[]>([]);

  recentLocations = $state<string[]>((() => {
    try {
      const raw = localStorage.getItem("luzumi_recent_locations");
      return raw ? JSON.parse(raw) : [];
    } catch { return []; }
  })());

  folderSizes = $state<Map<string, number>>(new Map());

  showBulkRename = $state(false);
  showDuplicateFinder = $state(false);
  showKeyboardShortcuts = $state(false);
  showPreview = $state(false);
  showSelectPattern = $state(false);
  duplicateGroups = $state<Array<{ size: number; paths: string[] }>>([]);

  pendingSelect = $state<string | null>(null);
  pickerConfig = $state<PickerConfig | null>(null);
  pickerFileName = $state("");
  pickerFilterIndex = $state(0);

  get isPickerMode() { return this.pickerConfig !== null; }

  consumePendingSelect(container: HTMLElement | undefined) {
    if (!this.pendingSelect || !container) return;
    const path = this.pendingSelect;
    this.pendingSelect = null;
    requestAnimationFrame(() => {
      const el = container.querySelector<HTMLElement>(`[data-path="${CSS.escape(path)}"]`);
      if (el) {
        el.scrollIntoView({ block: "center", behavior: "smooth" });
        el.focus();
      }
    });
  }

  tabs = $state<TabSnapshot[]>([]);
  activeTabIndex = $state(0);
  tabDropTarget = $state<number | null>(null);

  get canRedo() { return this.redoStack.length > 0; }

  get canGoBack() {
    return this.historyPos > 0;
  }
  get canGoForward() {
    return this.historyPos < this.history.length - 1;
  }

  setStatus(msg: string) {
    this.statusMessage = msg;
    if (this._statusTimer) clearTimeout(this._statusTimer);
    this._statusTimer = setTimeout(() => { this.statusMessage = null; }, 4000);
  }

  addRecentLocation(path: string) {
    const filtered = this.recentLocations.filter(p => p !== path);
    this.recentLocations = [path, ...filtered].slice(0, 20);
    try { localStorage.setItem("luzumi_recent_locations", JSON.stringify(this.recentLocations)); } catch {}
  }

  clearRecentLocations() {
    this.recentLocations = [];
    try { localStorage.removeItem("luzumi_recent_locations"); } catch {}
  }

  async calculateFolderSize(path: string) {
    this.folderSizes.set(path, -1); // -1 = loading
    this.folderSizes = new Map(this.folderSizes);
    try {
      const size = await invoke<number>("cmd_calculate_dir_size", { path });
      this.folderSizes.set(path, size);
      this.folderSizes = new Map(this.folderSizes);
    } catch {
      this.folderSizes.delete(path);
      this.folderSizes = new Map(this.folderSizes);
    }
  }

  async findDuplicates(recursive: boolean = true) {
    this.duplicateGroups = [];
    this.showDuplicateFinder = true;
    try {
      const groups = await invoke<Array<{ size: number; paths: string[] }>>("cmd_find_duplicates", { path: this.currentPath, recursive });
      this.duplicateGroups = groups;
    } catch (e) {
      this.error = String(e);
    }
  }

  bulkRename() {
    if (this.selected.size < 2) return;
    this.showBulkRename = true;
  }

  async compressSelected() {
    const paths = [...this.selected];
    if (paths.length === 0) return;
    const firstName = baseName(paths[0]);
    const archiveName = paths.length === 1 ? `${firstName}.zip` : "archive.zip";
    const dest = this.currentPath + "/" + archiveName;
    this.setStatus(t.compressing);
    try {
      await invoke("cmd_create_archive", { paths, dest });
      await this.reload();
      this.setStatus(t.archiveCreated(archiveName));
    } catch (e) {
      this.error = String(e);
    }
  }

  async extractArchive(path: string) {
    this.setStatus(t.extracting);
    try {
      await invoke("cmd_extract_archive", { archive: path, dest: this.currentPath });
      await this.reload();
      const name = baseName(path);
      this.setStatus(t.archiveExtracted(name));
    } catch (e) {
      this.error = String(e);
    }
  }

  invertSelection() {
    const all = new Set(this.entries.map(e => e.path));
    const inverted = new Set<string>();
    for (const p of all) {
      if (!this.selected.has(p)) inverted.add(p);
    }
    this.selected = inverted;
  }

  selectByPattern(pattern: string) {
    if (!pattern) return;
    const regex = new RegExp(
      "^" + pattern.replace(/[.+^${}()|[\]\\]/g, "\\$&").replace(/\*/g, ".*").replace(/\?/g, ".") + "$",
      "i"
    );
    const matched = new Set<string>();
    for (const e of this.entries) {
      if (regex.test(e.name)) matched.add(e.path);
    }
    this.selected = matched;
  }

  private _invertOp(op: UndoOp): UndoOp | null {
    if (op.type === "rename") {
      const dir = parentDir(op.newPath);
      const newName = baseName(op.newPath);
      return { type: "rename", newPath: dir + "/" + op.origName, origName: newName };
    }
    if (op.type === "move") {
      return {
        type: "move",
        moves: op.moves.map(m => {
          const name = baseName(m.newPath);
          return { newPath: m.origParent + "/" + name, origParent: parentDir(m.newPath) };
        })
      };
    }
    return null;
  }

  pushUndo(op: UndoOp) {
    this.redoStack = [];
    this.undoStack = [...this.undoStack.slice(-(UNDO_STACK_LIMIT - 1)), op];
  }

  async undo() {
    if (!this.undoStack.length) return;
    const op = this.undoStack[this.undoStack.length - 1];
    this.undoStack = this.undoStack.slice(0, -1);
    const redoOp = this._invertOp(op);
    try {
      switch (op.type) {
        case "rename":
          await invoke("cmd_rename_entry", { from: op.newPath, newName: op.origName });
          break;
        case "trash":
          for (const name of op.fileNames)
            await invoke("cmd_restore_from_trash", { fileName: name });
          break;
        case "create":
          await invoke("cmd_delete_entries", { paths: [op.path] });
          break;
        case "move":
          for (const { newPath, origParent } of op.moves)
            await invoke("cmd_move_entries", { paths: [newPath], dest: origParent });
          break;
        case "duplicate":
          await invoke("cmd_delete_entries", { paths: op.copies });
          break;
      }
      if (redoOp) this.redoStack = [...this.redoStack.slice(-19), redoOp];
      else this.redoStack = [];
      this.setStatus(t.undone);
      await this.reload();
    } catch (e) { this.error = t.undoError(String(e)); }
  }

  async redo() {
    if (!this.redoStack.length) return;
    const op = this.redoStack[this.redoStack.length - 1];
    this.redoStack = this.redoStack.slice(0, -1);
    const undoOp = this._invertOp(op);
    try {
      switch (op.type) {
        case "rename":
          await invoke("cmd_rename_entry", { from: op.newPath, newName: op.origName });
          break;
        case "move":
          for (const { newPath, origParent } of op.moves)
            await invoke("cmd_move_entries", { paths: [newPath], dest: origParent });
          break;
      }
      if (undoOp) this.undoStack = [...this.undoStack.slice(-19), undoOp];
      this.setStatus(t.redone);
      await this.reload();
    } catch (e) { this.error = t.redoError(String(e)); }
  }

  setSort(field: "name" | "size" | "date" | "type") {
    if (this.sortBy === field) {
      this.sortDir = this.sortDir === "asc" ? "desc" : "asc";
    } else {
      this.sortBy = field;
      this.sortDir = "asc";
    }
    try { localStorage.setItem("luzumi_sort_by", this.sortBy); } catch (_) {}
    try { localStorage.setItem("luzumi_sort_dir", this.sortDir); } catch (_) {}
  }

  setGridIconSize(size: number) {
    this.gridIconSize = Math.max(ICON_SIZE_MIN, Math.min(ICON_SIZE_MAX, size));
    if (this._iconSizeTimer) clearTimeout(this._iconSizeTimer);
    this._iconSizeTimer = setTimeout(() => {
      try { localStorage.setItem("luzumi_icon_size", String(this.gridIconSize)); } catch (_) {}
    }, 300);
  }

  toggleHoverBox() {
    this.showHoverBox = !this.showHoverBox;
    try { localStorage.setItem("luzumi_hover_box", String(this.showHoverBox)); } catch (_) {}
  }

  loadSortPrefs() {
    try {
      const sb = localStorage.getItem("luzumi_sort_by");
      const sd = localStorage.getItem("luzumi_sort_dir");
      if (sb === "name" || sb === "size" || sb === "date" || sb === "type") this.sortBy = sb;
      if (sd === "asc" || sd === "desc") this.sortDir = sd;
    } catch (_) {}
  }

  filteredEntries(): FileEntry[] {
    const key = `${this.entries.length}|${this.searchQuery}|${this.sortBy}|${this.sortDir}|${this.entries[0]?.modified ?? 0}|${this.entries[this.entries.length - 1]?.path ?? ""}`;
    if (this._filterCacheKey === key && this._filteredCache) return this._filteredCache;
    let entries = this.searchQuery
      ? this.entries.filter((e) => e.name.toLowerCase().includes(this.searchQuery.toLowerCase()))
      : [...this.entries];
    const dir = this.sortDir === "asc" ? 1 : -1;
    entries.sort((a, b) => {
      if (a.kind !== b.kind) return a.kind === "directory" ? -1 : 1;
      let cmp = 0;
      switch (this.sortBy) {
        case "name": cmp = a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: "base" }); break;
        case "size": cmp = (a.size ?? 0) - (b.size ?? 0); break;
        case "date": cmp = (a.modified ?? 0) - (b.modified ?? 0); break;
        case "type": cmp = (a.extension ?? "").localeCompare(b.extension ?? "", undefined, { sensitivity: "base" }); break;
      }
      return cmp * dir;
    });
    this._filteredCache = entries;
    this._filterCacheKey = key;
    return entries;
  }

  async init() {
    this.loadSortPrefs();
    try {
      const [home, bm, dr, qa] = await Promise.all([
        invoke<string>("get_home_dir"),
        invoke<BookmarkEntry[]>("cmd_get_bookmarks"),
        invoke<DriveInfo[]>("cmd_get_drives"),
        invoke<BookmarkEntry[]>("cmd_get_quick_access"),
      ]);
      this.bookmarks = bm;
      this.drives = dr;
      this.quickAccess = qa;
      const startupPath = await invoke<string>("cmd_get_startup_path");
      const startupSelect = await invoke<string | null>("cmd_get_startup_select");
      await this.navigate(startupPath || home);
      if (startupSelect) {
        const fullPath = this.currentPath.replace(/\/$/, "") + "/" + startupSelect;
        this.selected = new Set([fullPath]);
        this.pendingSelect = fullPath;
      }
      this.tabs = [{
        id: "1",
        path: this.currentPath,
        history: [...this.history],
        historyPos: this.historyPos,
        searchQuery: "",
      }];
      this.activeTabIndex = 0;
      await this.initPicker();
      this.startDriveRefresh();
      this.refreshTrashSize();
      this.startTrashWatch();
      this.checkDefaultFileManager();
      this.checkPortalInstalled();
      this.loadThemeList();
      if (this.currentTheme !== "Default") this.applyTheme(this.currentTheme);
    } catch (e) {
      this.error = String(e);
    }
  }

  private listDir(path: string, showHidden: boolean): Promise<FileEntry[]> {
    const cmd = this.elevated ? "cmd_elevated_list_directory" : "list_directory";
    return invoke<FileEntry[]>(cmd, { path, showHidden });
  }

  private _dirSignature(arr: FileEntry[]): string {
    // Lightweight signature for huge dirs: count + first/last entry info
    if (arr.length > 10000) {
      const first = arr[0];
      const last = arr[arr.length - 1];
      return `${arr.length}|${first?.modified ?? 0}|${first?.path ?? ""}|${last?.modified ?? 0}|${last?.path ?? ""}`;
    }
    return arr.map(e => e.path + e.size + (e.modified ?? 0) + (e.permissionsMode ?? 0)).sort().join("|");
  }

  async startWatch(path: string) {
    this.stopWatch();
    try {
      await invoke("cmd_watch_directory", { path });
      const unlisten = await listen("fs-changed", async () => {
        if (this.isLoading || this.deleteProgress) return;
        try {
          const fresh = await this.listDir(path, this.showHidden);
          if (this._dirSignature(fresh) !== this._dirSignature(this.entries)) {
            this.entries = fresh;
          }
        } catch (_) {}
      });
      this._watchUnlisten = unlisten;
    } catch (_) {
      // Fallback to polling if notify fails
      this._watchInterval = setInterval(async () => {
        if (this.isLoading || this.deleteProgress) return;
        try {
          const fresh = await this.listDir(path, this.showHidden);
          if (this._dirSignature(fresh) !== this._dirSignature(this.entries)) {
            this.entries = fresh;
          }
        } catch (_) {}
      }, WATCH_INTERVAL_MS);
    }
  }

  stopWatch() {
    if (this._watchUnlisten) { this._watchUnlisten(); this._watchUnlisten = null; }
    if (this._watchInterval) { clearInterval(this._watchInterval); this._watchInterval = null; }
    invoke("cmd_unwatch_directory", {}).catch(() => {});
  }

  async startTrashWatch() {
    try {
      const trashPath = await invoke<string>("get_home_dir") + "/.local/share/Trash/files";
      await invoke("cmd_watch_directory", { path: trashPath, channel: "trash-changed" });
      const unlisten = await listen("trash-changed", () => {
        this.refreshTrashSize();
      });
      this._trashUnlisten = unlisten;
    } catch (_) {}
  }

  stopTrashWatch() {
    if (this._trashUnlisten) { this._trashUnlisten(); this._trashUnlisten = null; }
    invoke("cmd_unwatch_directory", { channel: "trash-changed" }).catch(() => {});
  }

  destroy() {
    this.stopWatch();
    this.stopTrashWatch();
    if (this._splitUnlisten) { this._splitUnlisten(); this._splitUnlisten = null; }
    if (this.driveRefreshInterval) { clearInterval(this.driveRefreshInterval); this.driveRefreshInterval = null; }
    if (this._searchDebounce) { clearTimeout(this._searchDebounce); this._searchDebounce = null; }
    if (this._statusTimer) { clearTimeout(this._statusTimer); this._statusTimer = null; }
    if (this._iconSizeTimer) { clearTimeout(this._iconSizeTimer); this._iconSizeTimer = null; }
  }

  startDriveRefresh() {
    if (this.driveRefreshInterval) return;
    this.driveRefreshInterval = setInterval(() => this.refreshDrives(), DRIVE_REFRESH_MS);
  }

  async refreshDrives() {
    try {
      this.drives = await invoke<DriveInfo[]>("cmd_get_drives");
    } catch (_) {}
  }

  async navigate(path: string) {
    if (path === this.currentPath && !this.showTrashView) return;
    this.showTrashView = false;
    this.stopWatch();
    this.isLoading = true;
    try {
      const entries = await this.listDir(path, this.showHidden);
      let newHistory = [...this.history.slice(0, this.historyPos + 1), path];
      if (newHistory.length > 100) {
        newHistory = newHistory.slice(newHistory.length - 100);
      }
      this.history = newHistory;
      this.historyPos = newHistory.length - 1;
      this.currentPath = path;
      if (entries.length > 50000) {
        this.error = `Warning: ${entries.length.toLocaleString()} items — performance may be degraded`;
      }
      this.entries = entries;
      this.selected = new Set();
      this.renameTarget = null;
      if (entries.length <= 50000) this.error = null;
      this.globalSearchResults = [];
      this.searchQuery = "";
      this.folderSizes.clear();
      this.addRecentLocation(path);
      this.startWatch(path);
      this.refreshDiskSpace(path);
      this.refreshGitStatus(path);
    } catch (e) {
      this.error = String(e);
    } finally {
      this.isLoading = false;
    }
  }

  async reload() {
    this.isLoading = true;
    try {
      this.entries = await this.listDir(this.currentPath, this.showHidden);
      this.error = null;
    } catch (e) {
      this.error = String(e);
    } finally {
      this.isLoading = false;
    }
  }

  async goBack() {
    if (!this.canGoBack) return;
    this.historyPos--;
    const path = this.history[this.historyPos];
    this.isLoading = true;
    try {
      this.entries = await this.listDir(path, this.showHidden);
      this.currentPath = path;
      this.selected = new Set();
    } catch (e) {
      this.error = String(e);
    } finally {
      this.isLoading = false;
    }
  }

  async goForward() {
    if (!this.canGoForward) return;
    this.historyPos++;
    const path = this.history[this.historyPos];
    this.isLoading = true;
    try {
      this.entries = await this.listDir(path, this.showHidden);
      this.currentPath = path;
      this.selected = new Set();
    } catch (e) {
      this.error = String(e);
    } finally {
      this.isLoading = false;
    }
  }

  async goUp() {
    if (this.currentPath === "/") return;
    const parts = this.currentPath.split("/").filter(Boolean);
    parts.pop();
    const parent = "/" + parts.join("/") || "/";
    await this.navigate(parent);
  }

  async open(entry: FileEntry) {
    if (entry.kind === "directory" || entry.kind === "symlink") {
      await this.navigate(entry.path);
    } else {
      try {
        await invoke("open_file", { path: entry.path });
      } catch (e) {
        this.error = String(e);
      }
    }
  }

  triggerGlobalSearch(query: string) {
    if (this._searchDebounce) clearTimeout(this._searchDebounce);
    if (!query || query.length < 2) {
      this.globalSearchResults = [];
      this.globalSearchLoading = false;
      return;
    }
    this.globalSearchLoading = true;
    this._searchDebounce = setTimeout(async () => {
      try {
        const results = await invoke<FileEntry[]>("cmd_search_index", { query, limit: 50 });
        const currentPaths = new Set(this.entries.map((e) => e.path));
            this.globalSearchResults = results.filter(
          (r) => !currentPaths.has(r.path)
        );
      } catch {
        this.globalSearchResults = [];
      }
      this.globalSearchLoading = false;
    }, 300);
  }

  async rename(from: string, newName: string) {
    try {
      const origName = baseName(from);
      const parent = parentDir(from);
      const cmd = this.elevated ? "cmd_elevated_rename" : "cmd_rename_entry";
      await invoke(cmd, { from, newName });
      this.pushUndo({ type: "rename", newPath: parent + "/" + newName, origName });
      this.setStatus(t.renamedTo(newName));
      this.renameTarget = null;
      await this.reload();
    } catch (e) {
      this.error = String(e);
    }
  }

  async deleteSelected() {
    const paths = [...this.selected];
    if (paths.length === 0) return;
    try {
      if (this.elevated) {
        await invoke("cmd_elevated_delete", { paths });
        this.setStatus(t.deletedItems(paths.length));
      } else {
        const fileNames = paths.map(p => baseName(p));
        await invoke("cmd_delete_entries", { paths });
        this.pushUndo({ type: "trash", fileNames });
        this.setStatus(t.trashedItems(paths.length));
      }
      this.selected = new Set();
      await this.reload();
      this.refreshTrashSize();
    } catch (e) {
      this.error = String(e);
    }
  }

  async secureDeleteSelected() {
    const paths = [...this.selected];
    if (paths.length === 0) return;
    this.showSecureDeleteConfirm = false;
    this.deleteProgress = { current: "", done: 0, total: paths.length };
    try {
      await invoke("cmd_secure_delete_streamed", { paths });
      this.setStatus(t.secureDeletedItems(paths.length));
      this.selected = new Set();
      await this.reload();
    } catch (e) {
      this.error = String(e);
    } finally {
      this.deleteProgress = null;
    }
  }

  triggerSecureDelete() {
    try {
      if (localStorage.getItem("luzumi_skip_secure_confirm") === "1") {
        this.secureDeleteSelected();
        return;
      }
    } catch (_) {}
    this.showSecureDeleteConfirm = true;
  }

  async createFolder(name: string) {
    try {
      const cmd = this.elevated ? "cmd_elevated_create_directory" : "cmd_create_directory";
      await invoke(cmd, { parent: this.currentPath, name });
      this.pushUndo({ type: "create", path: this.currentPath + "/" + name });
      this.setStatus(t.folderCreated(name));
      await this.reload();
    } catch (e) {
      this.error = String(e);
    }
  }

  copySelected() {
    if (this.selected.size === 0) return;
    this.clipboard = { paths: [...this.selected], mode: "copy" };
    this.setStatus(t.copiedItems(this.selected.size));
  }

  cutSelected() {
    if (this.selected.size === 0) return;
    this.clipboard = { paths: [...this.selected], mode: "cut" };
    this.setStatus(t.cutItemsMsg(this.selected.size));
  }

  async paste() {
    if (!this.clipboard) return;
    const { paths, mode } = this.clipboard;
    const toProcess = paths.filter((p) => {
      const parent = parentDir(p);
      return parent !== this.currentPath;
    });
    if (toProcess.length === 0) return;
    const destDir = this.currentPath;
    try {
      if (mode === "copy") {
        const cmd = this.elevated ? "cmd_elevated_copy" : "cmd_copy_entries";
        await invoke(cmd, { paths: toProcess, dest: destDir });
        this.clipboard = null;
      } else {
        const moves = toProcess.map(p => ({
          newPath: destDir + "/" + baseName(p),
          origParent: (parentDir(p)),
        }));
        const cmd = this.elevated ? "cmd_elevated_move" : "cmd_move_entries";
        await invoke(cmd, { paths: toProcess, dest: destDir });
        this.clipboard = null;
        this.pushUndo({ type: "move", moves });
      }
      this.setStatus(t.pastedItems(toProcess.length));
      await this.reload();
    } catch (e) {
      this.error = String(e);
    }
  }

  selectAll() {
    this.selected = new Set(this.entries.map((e) => e.path));
  }

  toggleSelect(path: string, ctrl: boolean, shift?: boolean) {
    if (shift && this.lastSelected) {
      const paths = this.filteredEntries().map((e) => e.path);
      const from = paths.indexOf(this.lastSelected);
      const to = paths.indexOf(path);
      if (from !== -1 && to !== -1) {
        const [s, end] = from < to ? [from, to] : [to, from];
        const range = new Set(paths.slice(s, end + 1));
        this.selected = ctrl ? new Set([...this.selected, ...range]) : range;
        return;
      }
    }
    if (ctrl) {
      const next = new Set(this.selected);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      this.selected = next;
    } else {
      this.selected = new Set([path]);
    }
    this.lastSelected = path;
  }

  startRename(entry: FileEntry) {
    this.renameTarget = entry.path;
    this.renameBuffer = entry.name;
  }

  async toggleHidden() {
    this.showHidden = !this.showHidden;
    await this.reload();
  }

  requestDrop(dest: string) {
    if (this.dragPaths.length === 0) return;
    const allAlreadyInDest = this.dragPaths.every((p) => {
      const parent = parentDir(p);
      return parent === dest;
    });
    if (allAlreadyInDest) {
      this.dragPaths = [];
      this.dropTarget = null;
      return;
    }
    this.pendingDrop = { paths: [...this.dragPaths], dest };
    this.showDragDropDialog = true;
  }

  async confirmDropMove() {
    if (!this.pendingDrop) return;
    const { paths, dest } = this.pendingDrop;
    this.pendingDrop = null;
    this.showDragDropDialog = false;
    try {
      const moves = paths.map(p => ({
        newPath: dest + "/" + baseName(p),
        origParent: (parentDir(p)),
      }));
      const cmd = this.elevated ? "cmd_elevated_move" : "cmd_move_entries";
      await invoke(cmd, { paths, dest });
      this.pushUndo({ type: "move", moves });
      this.setStatus(t.movedItems(paths.length));
      this.dragPaths = [];
      this.dropTarget = null;
      await this.reload();
    } catch (e) {
      this.error = String(e);
    }
  }

  async confirmDropCopy() {
    if (!this.pendingDrop) return;
    const { paths, dest } = this.pendingDrop;
    this.pendingDrop = null;
    this.showDragDropDialog = false;
    try {
      const cmd = this.elevated ? "cmd_elevated_copy" : "cmd_copy_entries";
      await invoke(cmd, { paths, dest });
      this.setStatus(t.copiedItems(paths.length));
      this.dragPaths = [];
      this.dropTarget = null;
      await this.reload();
    } catch (e) {
      this.error = String(e);
    }
  }

  cancelDrop() {
    this.dragPaths = [];
    this.dropTarget = null;
    this.pendingDrop = null;
    this.showDragDropDialog = false;
  }

  setDragPaths(entry: FileEntry) {
    if (this.selected.has(entry.path)) {
      this.dragPaths = [...this.selected];
    } else {
      this.dragPaths = [entry.path];
    }
  }

  async startNativeDrag(entry: FileEntry) {
    const paths = this.selected.has(entry.path) ? [...this.selected] : [entry.path];
    try {
      await invoke("cmd_start_drag", { paths });
    } catch (_) {}
  }

  async refreshDiskSpace(path: string) {
    try {
      this.diskSpace = await invoke<{ total: number; available: number; used: number }>("cmd_get_disk_space", { path });
    } catch { this.diskSpace = null; }
  }

  async refreshGitStatus(path: string) {
    try {
      this.gitStatus = await invoke<{ is_repo: boolean; branch: string; modified: string[]; staged: string[]; untracked: string[] } | null>("cmd_get_git_status", { path });
    } catch { this.gitStatus = null; }
  }

  getGitFileStatus(filePath: string): string | null {
    if (!this.gitStatus?.is_repo) return null;
    const name = baseName(filePath);
    if (this.gitStatus.staged.some(f => f === name || f.endsWith("/" + name))) return "staged";
    if (this.gitStatus.modified.some(f => f === name || f.endsWith("/" + name))) return "modified";
    if (this.gitStatus.untracked.some(f => f === name || f.endsWith("/" + name))) return "untracked";
    return null;
  }

  toggleSplitView() {
    this.splitView = !this.splitView;
    if (this.splitView && !this.splitPath) {
      this.splitPath = this.currentPath;
      this.loadSplitEntries();
    }
  }

  async loadSplitEntries() {
    if (!this.splitPath) return;
    try {
      const entries: FileEntry[] = await invoke("cmd_list_directory", {
        path: this.splitPath,
        showHidden: this.showHidden,
      });
      this.splitEntries = entries;
    } catch {
      this.splitEntries = [];
    }
  }

  async navigateSplit(path: string) {
    this.splitPath = path;
    await this.loadSplitEntries();
  }

  async dropOnto(targetDir: string) {
    if (this.dragPaths.length === 0) return;
    if (this.dragPaths.includes(targetDir)) return;
    try {
      const dragged = [...this.dragPaths];
      const moves = dragged.map(p => ({
        newPath: targetDir + "/" + baseName(p),
        origParent: (parentDir(p)),
      }));
      const cmd = this.elevated ? "cmd_elevated_move" : "cmd_move_entries";
      await invoke(cmd, { paths: dragged, dest: targetDir });
      this.pushUndo({ type: "move", moves });
      this.setStatus(t.movedItems(dragged.length));
      this.dragPaths = [];
      this.dropTarget = null;
      await this.reload();
    } catch (e) {
      this.error = String(e);
    }
  }

  async mountDrive(drive: DriveInfo) {
    if (drive.isEncrypted && !drive.isMounted) {
      this.showUnlockDialog = drive;
      return;
    }
    try {
      const mountPoint = await invoke<string>("cmd_mount_drive", { device: drive.device });
      this.setStatus(t.mountedOn(mountPoint));
      await this.refreshDrives();
      if (mountPoint) await this.navigate(mountPoint);
    } catch (e) {
      this.error = String(e);
    }
  }

  async unlockDrive(device: string, password: string) {
    try {
      const mountPoint = await invoke<string>("cmd_unlock_drive", { device, password });
      this.setStatus(t.unlockedMounted(mountPoint));
      this.showUnlockDialog = null;
      await this.refreshDrives();
      if (mountPoint) await this.navigate(mountPoint);
    } catch (e) {
      this.error = String(e);
      this.showUnlockDialog = null;
    }
  }

  async ejectDrive(drive: DriveInfo) {
    try {
      await invoke("cmd_eject_drive", { device: drive.device });
      this.setStatus(t.ejected(drive.name));
      await this.refreshDrives();
    } catch (e) {
      this.error = String(e);
    }
  }

  clearError() {
    this.error = null;
  }

  clearStatus() {
    this.statusMessage = null;
  }

  async checkDefaultFileManager() {
    try {
      this.isDefaultFileManager = await invoke<boolean>("cmd_check_default_file_manager");
      if (!this.isDefaultFileManager) {
        try {
          this.showDefaultBanner = localStorage.getItem("luzumi_default_dismissed") !== "1";
        } catch { this.showDefaultBanner = true; }
      }
    } catch (_) {}
  }

  async setAsDefaultFileManager() {
    try {
      await invoke("cmd_set_as_default_file_manager");
      this.isDefaultFileManager = true;
      this.showDefaultBanner = false;
      this.setStatus(t.setAsDefaultMsg);
    } catch (e) { this.error = String(e); }
  }

  dismissDefaultBanner() {
    this.showDefaultBanner = false;
    try { localStorage.setItem("luzumi_default_dismissed", "1"); } catch (_) {}
  }

  async refreshTrashSize() {
    try {
      this.trashSize = await invoke<number>("cmd_get_trash_size");
    } catch (_) {}
  }

  async openTrash() {
    this.showTrashView = true;
    try {
      this.trashEntries = await invoke<FileEntry[]>("cmd_list_trash");
    } catch (e) {
      this.error = String(e);
    }
  }

  async emptyTrash() {
    try {
      await invoke("cmd_empty_trash");
      this.trashEntries = [];
      this.trashSize = 0;
      this.showEmptyTrashConfirm = false;
      this.showTrashView = false;
      this.setStatus(t.trashEmptied);
    } catch (e) {
      this.error = String(e);
      this.showEmptyTrashConfirm = false;
    }
  }

  async restoreFromTrash(fileName: string) {
    try {
      await invoke("cmd_restore_from_trash", { fileName });
      this.trashEntries = this.trashEntries.filter(e => e.name !== fileName);
      this.setStatus(t.restoredItem(fileName));
      await this.refreshTrashSize();
    } catch (e) {
      this.error = String(e);
    }
  }

  async addQuickAccess(name: string, path: string) {
    try {
      await invoke("cmd_add_quick_access", { name, path, icon: "Folder" });
      this.quickAccess = await invoke<BookmarkEntry[]>("cmd_get_quick_access");
    } catch (e) {
      this.error = String(e);
    }
  }

  async removeQuickAccess(path: string) {
    try {
      await invoke("cmd_remove_quick_access", { path });
      this.quickAccess = await invoke<BookmarkEntry[]>("cmd_get_quick_access");
    } catch (e) {
      this.error = String(e);
    }
  }

  openSaveAs(path: string) {
    this.saveAsSource = path;
    this.showSaveAsDialog = true;
  }

  async executeSaveAs(destDir: string, newName: string) {
    if (!this.saveAsSource || !newName.trim()) return;
    const dest = destDir.endsWith("/") ? destDir + newName : destDir + "/" + newName;
    try {
      await invoke("cmd_copy_as", { source: this.saveAsSource, dest });
      this.setStatus(t.savedCopyAs(newName));
      await this.reload();
    } catch (e) {
      this.error = String(e);
    }
    this.showSaveAsDialog = false;
    this.saveAsSource = null;
  }

  async createFile(name: string) {
    try {
      await invoke("cmd_create_file", { parent: this.currentPath, name });
      this.pushUndo({ type: "create", path: this.currentPath + "/" + name });
      this.setStatus(t.fileCreated(name));
      await this.reload();
    } catch (e) {
      this.error = String(e);
    }
  }

  async duplicateSelected() {
    const paths = [...this.selected];
    if (paths.length === 0) return;
    const copies: string[] = [];
    try {
      for (const p of paths) {
        const name = p.split("/").pop() ?? "";
        const dotIdx = name.lastIndexOf(".");
        const ext = dotIdx > 0 ? name.slice(dotIdx) : "";
        const base = ext ? name.slice(0, -ext.length) : name;
        const newName = `${base}_copy${ext}`;
        const dest = parentDir(p) + "/" + newName;
        await invoke("cmd_copy_as", { source: p, dest });
        copies.push(dest);
      }
      this.pushUndo({ type: "duplicate", copies });
      this.setStatus(t.duplicatedItems(paths.length));
      await this.reload();
    } catch (e) {
      this.error = String(e);
    }
  }

  async openTerminal(path?: string) {
    try {
      await invoke("cmd_open_terminal", { path: path ?? this.currentPath });
    } catch (e) {
      this.error = String(e);
    }
  }

  // ── Picker mode ──

  async initPicker() {
    try {
      const config = await invoke<PickerConfig | null>("cmd_get_picker_config");
      if (config) {
        this.pickerConfig = config;
        this.pickerFileName = config.current_name ?? "";
        if (config.current_folder) {
          await this.navigate(config.current_folder);
        }
      }
    } catch (_) {}
  }

  async submitPicker() {
    if (!this.pickerConfig) return;
    let uris: string[];
    if (this.pickerConfig.mode === "save") {
      const name = this.pickerFileName.trim();
      if (!name) return;
      uris = [`file://${this.currentPath}/${name}`];
    } else {
      // open mode
      const paths = [...this.selected];
      if (paths.length === 0) return;
      uris = paths.map(p => `file://${p}`);
    }
    try {
      await invoke("cmd_submit_picker_result", { uris });
    } catch (e) {
      this.error = String(e);
    }
  }

  async cancelPicker() {
    try {
      await invoke("cmd_cancel_picker");
    } catch (_) {}
  }

  pickerFilteredEntries(): FileEntry[] {
    if (!this.pickerConfig) return this.filteredEntries();
    const entries = this.filteredEntries();
    if (this.pickerConfig.directory) {
      return entries.filter(e => e.kind === "directory" || e.kind === "symlink");
    }
    const filters = this.pickerConfig.filters;
    if (filters.length === 0 || this.pickerFilterIndex >= filters.length) return entries;
    const active = filters[this.pickerFilterIndex];
    if (!active) return entries;
    const pats = active.patterns.map(p =>
      new RegExp("^" + p.replace(/[.+^${}()|[\]\\]/g, "\\$&").replace(/\*/g, ".*") + "$", "i")
    );
    return entries.filter(e => {
      if (e.kind === "directory" || e.kind === "symlink") return true;
      return pats.some(r => r.test(e.name));
    });
  }

  // ── Portal install/uninstall ──

  portalInstalled = $state(false);

  currentTheme = $state<string>((() => {
    try { return localStorage.getItem("luzumi_theme") ?? "Default"; } catch { return "Default"; }
  })());
  availableThemes = $state<string[]>([]);

  async checkPortalInstalled() {
    try {
      this.portalInstalled = await invoke<boolean>("cmd_check_portal_installed");
    } catch (_) {}
  }

  async installPortal() {
    try {
      await invoke("cmd_install_portal");
      this.portalInstalled = true;
      this.setStatus(t.installPortal);
    } catch (e) {
      this.error = String(e);
    }
  }

  async uninstallPortal() {
    try {
      await invoke("cmd_uninstall_portal");
      this.portalInstalled = false;
      this.setStatus(t.uninstallPortal);
    } catch (e) {
      this.error = String(e);
    }
  }

  async loadThemeList() {
    try {
      this.availableThemes = await invoke<string[]>("cmd_list_themes");
    } catch { this.availableThemes = []; }
  }

  async applyTheme(name: string) {
    this.currentTheme = name;
    try { localStorage.setItem("luzumi_theme", name); } catch {}

    document.documentElement.classList.add("theme-transitioning");

    let el = document.getElementById("luzumi-theme");
    if (name === "Default") {
      el?.remove();
    } else {
      try {
        const css = await invoke<string>("cmd_read_theme", { name });
        if (!el) {
          el = document.createElement("style");
          el.id = "luzumi-theme";
          document.head.appendChild(el);
        }
        el.textContent = css;
      } catch (e) {
        this.error = String(e);
      }
    }

    setTimeout(() => document.documentElement.classList.remove("theme-transitioning"), 500);
  }

  async openThemesFolder() {
    try {
      const dir = await invoke<string>("cmd_get_themes_dir");
      await this.navigate(dir);
      this.showSettings = false;
    } catch (e) { this.error = String(e); }
  }

  private _saveActiveTab() {
    if (!this.tabs[this.activeTabIndex]) return;
    this.tabs[this.activeTabIndex] = {
      ...this.tabs[this.activeTabIndex],
      path: this.currentPath,
      history: [...this.history],
      historyPos: this.historyPos,
      searchQuery: this.searchQuery,
    };
  }

  private async _loadTab(index: number) {
    const tab = this.tabs[index];
    if (!tab) return;
    this.history = [...tab.history];
    this.historyPos = tab.historyPos;
    this.currentPath = tab.path;
    this.searchQuery = tab.searchQuery;
    this.selected = new Set();
    this.renameTarget = null;
    this.stopWatch();
    await this.reload();
    this.startWatch(this.currentPath);
  }

  async switchTab(index: number) {
    if (index === this.activeTabIndex || index < 0 || index >= this.tabs.length) return;
    this._saveActiveTab();
    this.activeTabIndex = index;
    await this._loadTab(index);
  }

  async addTab(path?: string) {
    if (this.tabs.length >= 5) return;
    this._saveActiveTab();
    const targetPath = path ?? this.currentPath;
    const newTab: TabSnapshot = {
      id: Date.now().toString(),
      path: targetPath,
      history: [targetPath],
      historyPos: 0,
      searchQuery: "",
    };
    this.tabs = [...this.tabs, newTab];
    this.activeTabIndex = this.tabs.length - 1;
    this.history = [targetPath];
    this.historyPos = 0;
    this.currentPath = "";
    this.searchQuery = "";
    this.globalSearchResults = [];
    this.selected = new Set();
    await this.navigate(targetPath);
  }

  async dropOnTab(tabIndex: number) {
    const tab = this.tabs[tabIndex];
    if (!tab || !this.dragPaths.length) return;
    this.tabDropTarget = null;
    if (tabIndex !== this.activeTabIndex) {
      await this.switchTab(tabIndex);
    }
    this.requestDrop(tab.path);
  }

  async closeTab(index: number) {
    if (this.tabs.length <= 1) return;
    const newTabs = this.tabs.filter((_, i) => i !== index);
    const newIndex = Math.min(index, newTabs.length - 1);
    this.tabs = newTabs;
    if (index === this.activeTabIndex) {
      this.activeTabIndex = newIndex;
      await this._loadTab(newIndex);
    } else if (index < this.activeTabIndex) {
      this.activeTabIndex = this.activeTabIndex - 1;
    }
  }

  // --- Native context menu ---
  private _nativeMenuTarget: FileEntry | null = null;
  private _nativeMenuListenerSetup = false;

  private _setupNativeMenuListener() {
    if (this._nativeMenuListenerSetup) return;
    this._nativeMenuListenerSetup = true;
    listen<string>("context-menu-action", (event) => {
      this._handleNativeMenuAction(event.payload);
    });
  }

  private _handleNativeMenuAction(actionId: string) {
    const target = this._nativeMenuTarget;

    // Handle "Open With" app selections
    if (actionId.startsWith("ow:") && target) {
      const desktopId = actionId.slice(3);
      invoke("cmd_open_with_app", { path: target.path, desktopId }).catch(() => {});
      this._nativeMenuTarget = null;
      return;
    }

    switch (actionId) {
      case "open":
        if (target) this.open(target);
        break;
      case "open-with":
        if (target) this._showOpenWithMenu(target);
        break;
      case "enable-admin":
        invoke("cmd_authenticate_admin").then(() => { this.elevated = true; }).catch(() => {});
        break;
      case "pin-quick-access":
        if (target) this.addQuickAccess(target.name, target.path);
        break;
      case "unpin-quick-access":
        if (target) this.removeQuickAccess(target.path);
        break;
      case "rename":
        if (target) this.startRename(target);
        break;
      case "bulk-rename":
        this.bulkRename();
        break;
      case "calculate-size":
        if (target) this.calculateFolderSize(target.path);
        break;
      case "copy":
        this.copySelected();
        break;
      case "duplicate":
        this.duplicateSelected();
        break;
      case "save-copy-as":
        if (target) this.openSaveAs(target.path);
        break;
      case "cut":
        this.cutSelected();
        break;
      case "paste":
        this.paste();
        break;
      case "copy-path":
        if (target) navigator.clipboard.writeText(target.path);
        break;
      case "open-terminal":
        this.openTerminal(target?.kind === "directory" ? target.path : undefined);
        break;
      case "compress":
        this.compressSelected();
        break;
      case "extract-here":
        if (target) this.extractArchive(target.path);
        break;
      case "move-to-trash":
        this.deleteSelected();
        break;
      case "delete-permanently":
        this.triggerSecureDelete();
        break;
      case "properties":
        if (target) this.showProperties = target;
        break;
      case "new-folder":
        this.showNewFolder = true;
        this.newFolderName = "New Folder";
        break;
      case "new-file":
        this.showNewFile = true;
        this.newFileName = "untitled.txt";
        break;
      case "pin-current":
        {
          const name = this.currentPath.split("/").filter(Boolean).pop() || this.currentPath;
          this.addQuickAccess(name, this.currentPath);
        }
        break;
      case "invert-selection":
        this.invertSelection();
        break;
      case "select-by-pattern":
        this.showSelectPattern = true;
        break;
      case "find-duplicates":
        this.findDuplicates();
        break;
    }
    this._nativeMenuTarget = null;
  }

  async showNativeContextMenu(entry: FileEntry | null) {
    this._setupNativeMenuListener();
    this._nativeMenuTarget = entry;

    const items: Array<{ id: string; label: string; enabled: boolean; separator: boolean }> = [];
    const sep = () => items.push({ id: "", label: "", enabled: false, separator: true });
    const item = (id: string, label: string, enabled = true) =>
      items.push({ id, label, enabled, separator: false });

    const hasClipboard = !!this.clipboard;
    const singleSelected = this.selected.size === 1
      ? this.entries.find((e) => this.selected.has(e.path)) ?? null
      : null;
    const isPinned = entry?.kind === "directory"
      ? this.quickAccess.some(qa => qa.path === entry?.path)
      : false;

    if (entry) {
      // Target menu
      item("open", t.open);
      item("open-with", entry.kind === "directory" ? t.openFolderWith : t.openWith);

      if (!entry.isWritable && !this.elevated) {
        item("enable-admin", t.enableAdmin);
      }

      if (entry.kind === "directory") {
        item(isPinned ? "unpin-quick-access" : "pin-quick-access",
             isPinned ? t.unpinQuickAccess : t.pinQuickAccess);
      }

      sep();

      if (singleSelected) {
        item("rename", `${t.rename}    F2`);
      }
      if (this.selected.size >= 2) {
        item("bulk-rename", t.bulkRename);
      }
      if (singleSelected && singleSelected.kind === "directory") {
        item("calculate-size", t.calculateSize);
      }

      sep();
      item("copy", `${t.copy}    Ctrl+C`);
      item("duplicate", `${t.duplicate}    Ctrl+D`);
      if (entry.kind !== "directory") {
        item("save-copy-as", t.saveCopyAs);
      }
      item("cut", `${t.cut}    Ctrl+X`);

      if (hasClipboard) {
        item("paste", `${t.paste}    Ctrl+V`);
      }

      sep();
      item("copy-path", t.copyPath);
      if (entry.kind === "directory") {
        item("open-terminal", t.openTerminal);
      }
      item("compress", t.compress);
      if (singleSelected && singleSelected.extension?.toLowerCase() === "zip") {
        item("extract-here", t.extractHere);
      }

      sep();
      item("move-to-trash", `${t.moveToTrash}    Del`);
      item("delete-permanently", `${t.deletePermanently}    Shift+Del`);
      sep();
      item("properties", t.properties);
    } else {
      // Background menu
      if (hasClipboard) {
        item("paste", `${t.paste}    Ctrl+V`);
        sep();
      }
      item("open-terminal", t.openTerminal);
      item("new-folder", t.newFolder);
      item("new-file", t.newFile);
      item("pin-current", t.pinQuickAccess);
      sep();
      item("invert-selection", t.invertSelection);
      item("select-by-pattern", t.selectByPattern);
      sep();
      item("find-duplicates", t.findDuplicates);
    }

    try {
      await invoke("cmd_show_context_menu", { items });
    } catch (e) {
      console.error("Native context menu failed:", e);
    }
  }

  private async _showOpenWithMenu(target: FileEntry) {
    try {
      const apps = await invoke<Array<{ desktop_id: string; name: string; icon: string; is_default: boolean }>>(
        "cmd_list_open_with_apps", { path: target.path }
      );
      if (!apps || apps.length === 0) return;

      const items: Array<{ id: string; label: string; enabled: boolean; separator: boolean }> = [];
      for (const app of apps) {
        const label = app.is_default ? `${app.name} (${t.open})` : app.name;
        items.push({ id: `ow:${app.desktop_id}`, label, enabled: true, separator: false });
      }

      this._nativeMenuTarget = target;
      await invoke("cmd_show_context_menu", { items });
    } catch (e) {
      console.error("Open with menu failed:", e);
    }
  }
}

export const fm = new FileManager();
