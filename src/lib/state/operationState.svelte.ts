import { invoke } from "@tauri-apps/api/core";
import { t } from "../i18n";

function parentDir(path: string): string {
  const idx = path.lastIndexOf("/");
  if (idx <= 0) return "/";
  return path.substring(0, idx);
}

function baseName(path: string): string {
  const idx = path.lastIndexOf("/");
  return idx === -1 ? path : path.substring(idx + 1);
}

export type UndoOp =
  | { type: "rename"; newPath: string; origName: string }
  | { type: "trash"; fileNames: string[] }
  | { type: "create"; path: string }
  | { type: "move"; moves: Array<{ newPath: string; origParent: string }> }
  | { type: "duplicate"; copies: string[] }
  | { type: "permissions"; path: string; oldMode: number; newMode: number }
  | { type: "archive"; archivePath: string; sourcePaths: string[] }
  | { type: "extract"; extractedPaths: string[]; archivePath: string }
  | { type: "group"; ops: UndoOp[] };

export interface OperationProgress {
  id: string;
  type: "copy" | "move" | "delete" | "archive" | "extract" | "search";
  current: string;
  done: number;
  total: number;
  paused: boolean;
  startTime: number;
}

const UNDO_STACK_LIMIT = 50;

export class OperationState {
  undoStack = $state<UndoOp[]>([]);
  redoStack = $state<UndoOp[]>([]);
  deleteProgress = $state<{ current: string; done: number; total: number } | null>(null);
  copyMoveProgress = $state<{ current: string; done: number; total: number } | null>(null);

  // Operation queue
  operations = $state<OperationProgress[]>([]);

  get canUndo() { return this.undoStack.length > 0; }
  get canRedo() { return this.redoStack.length > 0; }
  get hasActiveOperations() { return this.operations.length > 0; }

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
    if (op.type === "permissions") {
      return { type: "permissions", path: op.path, oldMode: op.newMode, newMode: op.oldMode };
    }
    if (op.type === "group") {
      const inverted = op.ops.map(o => this._invertOp(o)).filter((o): o is UndoOp => o !== null);
      return inverted.length > 0 ? { type: "group", ops: inverted.reverse() } : null;
    }
    return null;
  }

  pushUndo(op: UndoOp) {
    this.redoStack = [];
    this.undoStack = [...this.undoStack.slice(-(UNDO_STACK_LIMIT - 1)), op];
  }

  async undo(elevated: boolean): Promise<{ success: boolean; message: string }> {
    if (!this.undoStack.length) return { success: false, message: "" };
    const op = this.undoStack[this.undoStack.length - 1];
    this.undoStack = this.undoStack.slice(0, -1);
    const redoOp = this._invertOp(op);
    try {
      await this._executeUndo(op, elevated);
      if (redoOp) this.redoStack = [...this.redoStack.slice(-49), redoOp];
      else this.redoStack = [];
      return { success: true, message: t.undone };
    } catch (e) {
      return { success: false, message: t.undoError(String(e)) };
    }
  }

  private async _executeUndo(op: UndoOp, elevated: boolean) {
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
        for (const { newPath, origParent } of op.moves) {
          const cmd = elevated ? "cmd_elevated_move" : "cmd_move_entries";
          await invoke(cmd, { paths: [newPath], dest: origParent });
        }
        break;
      case "duplicate":
        await invoke("cmd_delete_entries", { paths: op.copies });
        break;
      case "permissions":
        await invoke("cmd_set_permissions", { path: op.path, mode: op.oldMode });
        break;
      case "group":
        for (const sub of op.ops) await this._executeUndo(sub, elevated);
        break;
    }
  }

  async redo(elevated: boolean): Promise<{ success: boolean; message: string }> {
    if (!this.redoStack.length) return { success: false, message: "" };
    const op = this.redoStack[this.redoStack.length - 1];
    this.redoStack = this.redoStack.slice(0, -1);
    const undoOp = this._invertOp(op);
    try {
      await this._executeRedo(op, elevated);
      if (undoOp) this.undoStack = [...this.undoStack.slice(-49), undoOp];
      return { success: true, message: t.redone };
    } catch (e) {
      return { success: false, message: t.redoError(String(e)) };
    }
  }

  private async _executeRedo(op: UndoOp, elevated: boolean) {
    switch (op.type) {
      case "rename":
        await invoke("cmd_rename_entry", { from: op.newPath, newName: op.origName });
        break;
      case "move":
        for (const { newPath, origParent } of op.moves) {
          const cmd = elevated ? "cmd_elevated_move" : "cmd_move_entries";
          await invoke(cmd, { paths: [newPath], dest: origParent });
        }
        break;
      case "permissions":
        await invoke("cmd_set_permissions", { path: op.path, mode: op.newMode });
        break;
      case "group":
        for (const sub of op.ops) await this._executeRedo(sub, elevated);
        break;
    }
  }

  // Operation queue management
  addOperation(op: OperationProgress) {
    this.operations = [...this.operations, op];
  }

  updateOperation(id: string, update: Partial<OperationProgress>) {
    this.operations = this.operations.map(op =>
      op.id === id ? { ...op, ...update } : op
    );
  }

  removeOperation(id: string) {
    this.operations = this.operations.filter(op => op.id !== id);
  }

  async pauseOperation(id: string) {
    try {
      await invoke("cmd_pause_operation", { id });
      this.updateOperation(id, { paused: true });
    } catch {}
  }

  async resumeOperation(id: string) {
    try {
      await invoke("cmd_resume_operation", { id });
      this.updateOperation(id, { paused: false });
    } catch {}
  }

  async cancelOperation() {
    try {
      await invoke("cmd_cancel_operation");
    } catch {}
  }
}
