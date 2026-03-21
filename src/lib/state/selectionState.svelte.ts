import type { FileEntry } from "../types";

export class SelectionState {
  selected = $state<Set<string>>(new Set());
  lastSelected = $state<string | null>(null);
  clipboard = $state<{ paths: string[]; mode: "copy" | "cut" } | null>(null);
  dragPaths = $state<string[]>([]);
  dropTarget = $state<string | null>(null);
  pendingDrop = $state<{ paths: string[]; dest: string } | null>(null);

  get hasClipboard() { return this.clipboard !== null; }
  get clipboardCount() { return this.clipboard?.paths.length ?? 0; }
  get clipboardMode() { return this.clipboard?.mode ?? null; }
  get selectedCount() { return this.selected.size; }

  select(path: string) {
    this.selected = new Set([path]);
    this.lastSelected = path;
  }

  toggleSelect(path: string, ctrl: boolean, shift: boolean, sortedPaths: string[]) {
    if (shift && this.lastSelected) {
      const from = sortedPaths.indexOf(this.lastSelected);
      const to = sortedPaths.indexOf(path);
      if (from !== -1 && to !== -1) {
        const [s, end] = from < to ? [from, to] : [to, from];
        const range = new Set(sortedPaths.slice(s, end + 1));
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

  selectAll(entries: FileEntry[]) {
    this.selected = new Set(entries.map((e) => e.path));
  }

  clearSelection() {
    this.selected = new Set();
    this.lastSelected = null;
  }

  invertSelection(entries: FileEntry[]) {
    const inverted = new Set<string>();
    for (const e of entries) {
      if (!this.selected.has(e.path)) inverted.add(e.path);
    }
    this.selected = inverted;
  }

  selectByPattern(pattern: string, entries: FileEntry[]) {
    if (!pattern) return;
    const regex = new RegExp(
      "^" + pattern.replace(/[.+^${}()|[\]\\]/g, "\\$&").replace(/\*/g, ".*").replace(/\?/g, ".") + "$",
      "i"
    );
    const matched = new Set<string>();
    for (const e of entries) {
      if (regex.test(e.name)) matched.add(e.path);
    }
    this.selected = matched;
  }

  setClipboard(mode: "copy" | "cut") {
    if (this.selected.size === 0) return;
    this.clipboard = { paths: [...this.selected], mode };
  }

  clearClipboard() {
    this.clipboard = null;
  }

  setDragPaths(entry: FileEntry) {
    if (this.selected.has(entry.path)) {
      this.dragPaths = [...this.selected];
    } else {
      this.dragPaths = [entry.path];
    }
  }

  clearDrag() {
    this.dragPaths = [];
    this.dropTarget = null;
    this.pendingDrop = null;
  }
}
