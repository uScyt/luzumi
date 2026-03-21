/**
 * Luzumi Plugin API
 *
 * This module defines the API surface exposed to plugins running inside
 * sandboxed Web Workers. Plugins interact with Luzumi exclusively through
 * the `luzumi` global object injected into their worker scope.
 */

import { invoke } from "@tauri-apps/api/core";
import type { FileEntry } from "../types";

export interface LuzumiPluginAPI {
  /** Filesystem operations (read-only by default, write requires permission) */
  fs: {
    list(path: string): Promise<FileEntry[]>;
    exists(path: string): Promise<boolean>;
    readText(path: string, maxLines?: number): Promise<string>;
    stat(path: string): Promise<FileEntry | null>;
  };

  /** UI interactions */
  ui: {
    notify(message: string, type?: "success" | "error" | "info"): void;
    setStatus(message: string): void;
    addContextMenuItem(item: ContextMenuItem): void;
    removeContextMenuItem(id: string): void;
  };

  /** Navigation */
  nav: {
    getCurrentPath(): string;
    navigate(path: string): void;
    getSelected(): string[];
  };

  /** Events */
  on(event: PluginEvent, handler: (...args: any[]) => void): () => void;
}

export interface ContextMenuItem {
  id: string;
  label: string;
  icon?: string;
  action: () => void;
  when?: (context: { path: string; kind: string; extension: string }) => boolean;
}

export type PluginEvent =
  | "navigate"
  | "fileOpen"
  | "selectionChange"
  | "contextMenu"
  | "fileCreate"
  | "fileDelete"
  | "fileRename";

export interface PluginManifest {
  name: string;
  version: string;
  main: string;
  description?: string;
  author?: string;
  permissions: PluginPermission[];
  hooks: PluginHook[];
}

export type PluginPermission =
  | "fs.read"
  | "fs.write"
  | "fs.delete"
  | "ui.notify"
  | "ui.contextMenu"
  | "nav.navigate"
  | "nav.read";

export type PluginHook =
  | "onActivate"
  | "onDeactivate"
  | "onNavigate"
  | "onFileOpen"
  | "onContextMenu"
  | "onSelectionChange";

/**
 * Creates the API object that will be exposed to a plugin.
 * Each plugin gets its own scoped API instance.
 */
export function createPluginAPI(
  pluginName: string,
  permissions: PluginPermission[],
  callbacks: {
    getPath: () => string;
    navigate: (path: string) => void;
    getSelected: () => string[];
    notify: (msg: string, type: string) => void;
    setStatus: (msg: string) => void;
    addContextMenuItem: (item: ContextMenuItem) => void;
    removeContextMenuItem: (id: string) => void;
  }
): LuzumiPluginAPI {
  const listeners = new Map<string, Set<(...args: any[]) => void>>();

  function requirePermission(perm: PluginPermission) {
    if (!permissions.includes(perm)) {
      throw new Error(`Plugin "${pluginName}" lacks permission: ${perm}`);
    }
  }

  return {
    fs: {
      async list(path: string) {
        requirePermission("fs.read");
        return invoke<FileEntry[]>("cmd_list_directory", { path, showHidden: false });
      },
      async exists(path: string) {
        requirePermission("fs.read");
        try {
          await invoke("cmd_get_file_details", { path });
          return true;
        } catch {
          return false;
        }
      },
      async readText(path: string, maxLines = 100) {
        requirePermission("fs.read");
        return invoke<string>("cmd_read_text_preview", { path, maxLines });
      },
      async stat(path: string) {
        requirePermission("fs.read");
        try {
          return await invoke<FileEntry>("cmd_get_file_details", { path });
        } catch {
          return null;
        }
      },
    },

    ui: {
      notify(message: string, type: "success" | "error" | "info" = "info") {
        requirePermission("ui.notify");
        callbacks.notify(message, type);
      },
      setStatus(message: string) {
        requirePermission("ui.notify");
        callbacks.setStatus(message);
      },
      addContextMenuItem(item: ContextMenuItem) {
        requirePermission("ui.contextMenu");
        callbacks.addContextMenuItem({ ...item, id: `${pluginName}:${item.id}` });
      },
      removeContextMenuItem(id: string) {
        requirePermission("ui.contextMenu");
        callbacks.removeContextMenuItem(`${pluginName}:${id}`);
      },
    },

    nav: {
      getCurrentPath() {
        requirePermission("nav.read");
        return callbacks.getPath();
      },
      navigate(path: string) {
        requirePermission("nav.navigate");
        callbacks.navigate(path);
      },
      getSelected() {
        requirePermission("nav.read");
        return callbacks.getSelected();
      },
    },

    on(event: PluginEvent, handler: (...args: any[]) => void) {
      if (!listeners.has(event)) {
        listeners.set(event, new Set());
      }
      listeners.get(event)!.add(handler);
      return () => { listeners.get(event)?.delete(handler); };
    },
  };
}
