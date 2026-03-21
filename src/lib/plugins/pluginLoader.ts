/**
 * Plugin Loader
 *
 * Loads and manages Luzumi plugins. Each plugin runs in its own scope
 * with a sandboxed API. Plugins are stored in ~/.config/luzumi/plugins/.
 */

import { invoke } from "@tauri-apps/api/core";
import { createPluginAPI, type PluginPermission, type ContextMenuItem } from "./pluginApi";

export interface LoadedPlugin {
  name: string;
  version: string;
  description: string;
  author: string;
  enabled: boolean;
  hooks: string[];
  permissions: PluginPermission[];
  instance: any | null;
  contextMenuItems: ContextMenuItem[];
}

export interface PluginInfo {
  name: string;
  version: string;
  description: string;
  author: string;
  enabled: boolean;
  path: string;
  permissions: string[];
  hooks: string[];
}

class PluginManager {
  private _plugins: Map<string, LoadedPlugin> = new Map();
  private _callbacks: {
    getPath: () => string;
    navigate: (path: string) => void;
    getSelected: () => string[];
    notify: (msg: string, type: string) => void;
    setStatus: (msg: string) => void;
  } = {
    getPath: () => "/",
    navigate: () => {},
    getSelected: () => [],
    notify: () => {},
    setStatus: () => {},
  };

  /** Set the callbacks that plugins will use to interact with the app */
  setCallbacks(cbs: typeof this._callbacks) {
    this._callbacks = cbs;
  }

  /** Load all enabled plugins from the backend */
  async loadAll() {
    try {
      const plugins = await invoke<PluginInfo[]>("cmd_list_plugins");
      for (const info of plugins) {
        if (info.enabled) {
          await this.loadPlugin(info.name);
        }
      }
    } catch (e) {
      console.error("Failed to load plugins:", e);
    }
  }

  /** Load and activate a single plugin */
  async loadPlugin(name: string) {
    try {
      const script = await invoke<string>("cmd_get_plugin_script", { name });
      const plugins = await invoke<PluginInfo[]>("cmd_list_plugins");
      const info = plugins.find(p => p.name === name);
      if (!info) return;

      const contextMenuItems: ContextMenuItem[] = [];

      const api = createPluginAPI(
        name,
        info.permissions as PluginPermission[],
        {
          ...this._callbacks,
          addContextMenuItem: (item) => { contextMenuItems.push(item); },
          removeContextMenuItem: (id) => {
            const idx = contextMenuItems.findIndex(i => i.id === id);
            if (idx >= 0) contextMenuItems.splice(idx, 1);
          },
        }
      );

      // Execute plugin script in a restricted scope
      // Block access to dangerous globals by shadowing them
      const pluginFn = new Function(
        "luzumi",
        "window", "document", "globalThis", "self",
        "fetch", "XMLHttpRequest", "WebSocket", "EventSource",
        "eval", "Function",
        "import", "require",
        `"use strict";\n${script}`
      );

      let instance: any;
      try {
        instance = pluginFn(
          api,
          undefined, undefined, undefined, undefined,
          undefined, undefined, undefined, undefined,
          undefined, undefined,
          undefined, undefined
        );
      } catch (initError) {
        console.error(`Plugin ${name} crashed during initialization:`, initError);
        return;
      }

      const loaded: LoadedPlugin = {
        name: info.name,
        version: info.version,
        description: info.description,
        author: info.author,
        enabled: true,
        hooks: info.hooks,
        permissions: info.permissions as PluginPermission[],
        instance,
        contextMenuItems,
      };

      this._plugins.set(name, loaded);

      // Call onActivate hook if the plugin exports it
      if (instance && typeof instance.onActivate === "function") {
        try { await instance.onActivate(); } catch (e) {
          console.error(`Plugin ${name} onActivate error:`, e);
        }
      }
    } catch (e) {
      console.error(`Failed to load plugin "${name}":`, e);
    }
  }

  /** Deactivate and unload a plugin */
  async unloadPlugin(name: string) {
    const plugin = this._plugins.get(name);
    if (!plugin) return;

    if (plugin.instance && typeof plugin.instance.onDeactivate === "function") {
      try { await plugin.instance.onDeactivate(); } catch (e) {
        console.error(`Plugin ${name} onDeactivate error:`, e);
      }
    }

    this._plugins.delete(name);
  }

  /** Enable a plugin (persists to disk) */
  async enablePlugin(name: string) {
    await invoke("cmd_enable_plugin", { name });
    await this.loadPlugin(name);
  }

  /** Disable a plugin (persists to disk) */
  async disablePlugin(name: string) {
    await this.unloadPlugin(name);
    await invoke("cmd_disable_plugin", { name });
  }

  /** Emit an event to all loaded plugins that have the relevant hook */
  async emit(event: string, ...args: any[]) {
    const PLUGIN_TIMEOUT = 5000; // 5s max per hook
    for (const plugin of this._plugins.values()) {
      if (!plugin.instance) continue;
      const handler = plugin.instance[event];
      if (typeof handler === "function") {
        try {
          await Promise.race([
            Promise.resolve(handler(...args)),
            new Promise((_, reject) =>
              setTimeout(() => reject(new Error(`Plugin ${plugin.name} timed out on ${event}`)), PLUGIN_TIMEOUT)
            ),
          ]);
        } catch (e) {
          console.error(`Plugin ${plugin.name} ${event} error:`, e);
        }
      }
    }
  }

  /** Get all context menu items from all loaded plugins */
  getContextMenuItems(context?: { path: string; kind: string; extension: string }): ContextMenuItem[] {
    const items: ContextMenuItem[] = [];
    for (const plugin of this._plugins.values()) {
      for (const item of plugin.contextMenuItems) {
        if (!context || !item.when || item.when(context)) {
          items.push(item);
        }
      }
    }
    return items;
  }

  /** Get all loaded plugins */
  getAll(): LoadedPlugin[] {
    return [...this._plugins.values()];
  }

  /** Get a specific plugin */
  get(name: string): LoadedPlugin | undefined {
    return this._plugins.get(name);
  }
}

export const pluginManager = new PluginManager();
