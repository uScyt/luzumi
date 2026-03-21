# Luzumi Plugin System

Luzumi supports plugins that extend its functionality with custom actions, context menu items, and event hooks. Plugins are written in JavaScript and run inside the application with a sandboxed API.

## Table of Contents

- [Quick Start](#quick-start)
- [Plugin Structure](#plugin-structure)
- [Manifest Reference](#manifest-reference)
- [Plugin API](#plugin-api)
- [Lifecycle Hooks](#lifecycle-hooks)
- [Permissions](#permissions)
- [Examples](#examples)
- [Installing Plugins](#installing-plugins)
- [Managing Plugins](#managing-plugins)
- [Development Tips](#development-tips)

---

## Quick Start

Create your first plugin in 3 steps:

### 1. Create the plugin directory

```bash
mkdir -p ~/.config/luzumi/plugins/hello-world
```

### 2. Create `manifest.json`

```json
{
  "name": "hello-world",
  "version": "1.0.0",
  "main": "index.js",
  "description": "A simple hello world plugin",
  "author": "Your Name",
  "permissions": ["ui.notify"],
  "hooks": ["onActivate"]
}
```

### 3. Create `index.js`

```javascript
// The `luzumi` object is injected automatically
luzumi.ui.notify("Hello from my plugin!", "success");

// Export lifecycle hooks by returning an object
return {
  onActivate() {
    console.log("Hello World plugin activated!");
  },
  onDeactivate() {
    console.log("Hello World plugin deactivated!");
  }
};
```

### 4. Enable it

Open Luzumi, go to **Settings** or use the **Command Palette** (`Ctrl+Shift+P`), search for "plugins", and enable your plugin.

---

## Plugin Structure

A plugin lives in its own directory under `~/.config/luzumi/plugins/`:

```
~/.config/luzumi/plugins/
  my-plugin/
    manifest.json      # Required: plugin metadata
    index.js           # Required: plugin entry point
    styles.css         # Optional: custom styles
    assets/            # Optional: any other files
      icon.png
```

The plugin directory name should match the `name` field in `manifest.json`.

---

## Manifest Reference

The `manifest.json` file describes your plugin:

```json
{
  "name": "my-plugin",
  "version": "1.0.0",
  "main": "index.js",
  "description": "What your plugin does",
  "author": "Your Name",
  "permissions": [
    "fs.read",
    "ui.notify",
    "ui.contextMenu",
    "nav.read"
  ],
  "hooks": [
    "onActivate",
    "onDeactivate",
    "onNavigate",
    "onFileOpen",
    "onContextMenu",
    "onSelectionChange"
  ]
}
```

| Field | Type | Required | Description |
|:---|:---|:---|:---|
| `name` | string | Yes | Unique plugin identifier (lowercase, hyphens OK) |
| `version` | string | Yes | Semver version (e.g. `1.0.0`) |
| `main` | string | No | Entry point file (default: `index.js`) |
| `description` | string | No | Short description shown in the plugin manager |
| `author` | string | No | Author name |
| `permissions` | string[] | No | List of permissions the plugin needs |
| `hooks` | string[] | No | Lifecycle hooks the plugin implements |

---

## Plugin API

Your plugin receives a `luzumi` object with these namespaces:

### `luzumi.fs` — Filesystem

| Method | Permission | Description |
|:---|:---|:---|
| `list(path)` | `fs.read` | List directory entries. Returns `FileEntry[]` |
| `exists(path)` | `fs.read` | Check if a path exists. Returns `boolean` |
| `readText(path, maxLines?)` | `fs.read` | Read file content as text (default: 100 lines) |
| `stat(path)` | `fs.read` | Get file metadata. Returns `FileEntry \| null` |

```javascript
// List files in home directory
const files = await luzumi.fs.list("/home/user/Documents");
for (const file of files) {
  console.log(file.name, file.size, file.kind);
}

// Check if a file exists
if (await luzumi.fs.exists("/home/user/.bashrc")) {
  const content = await luzumi.fs.readText("/home/user/.bashrc", 50);
  console.log(content);
}
```

### `luzumi.ui` — User Interface

| Method | Permission | Description |
|:---|:---|:---|
| `notify(message, type?)` | `ui.notify` | Show a toast notification. Type: `"success"`, `"error"`, or `"info"` |
| `setStatus(message)` | `ui.notify` | Set the status bar message |
| `addContextMenuItem(item)` | `ui.contextMenu` | Add an item to the right-click context menu |
| `removeContextMenuItem(id)` | `ui.contextMenu` | Remove a context menu item by ID |

```javascript
// Show notifications
luzumi.ui.notify("Operation complete!", "success");
luzumi.ui.notify("Something went wrong", "error");
luzumi.ui.setStatus("Processing files...");

// Add context menu item
luzumi.ui.addContextMenuItem({
  id: "my-action",
  label: "Do Something Cool",
  action: () => {
    luzumi.ui.notify("Cool action triggered!");
  },
  // Optional: only show for certain files
  when: (ctx) => ctx.extension === "md"
});
```

### `luzumi.nav` — Navigation

| Method | Permission | Description |
|:---|:---|:---|
| `getCurrentPath()` | `nav.read` | Get the current directory path |
| `navigate(path)` | `nav.navigate` | Navigate to a directory |
| `getSelected()` | `nav.read` | Get paths of selected files |

```javascript
// Get current location
const path = luzumi.nav.getCurrentPath();
console.log("Current directory:", path);

// Navigate somewhere
luzumi.nav.navigate("/home/user/Downloads");

// Get selected files
const selected = luzumi.nav.getSelected();
console.log(`${selected.length} files selected`);
```

### `luzumi.on()` — Events

Subscribe to events in the file manager:

```javascript
// Called when the user navigates to a new directory
const unsubscribe = luzumi.on("navigate", (path) => {
  console.log("Navigated to:", path);
});

// Available events:
// "navigate"        - directory changed
// "fileOpen"        - file was opened
// "selectionChange" - selection changed
// "contextMenu"     - context menu opened
// "fileCreate"      - file/folder created
// "fileDelete"      - file/folder deleted
// "fileRename"      - file/folder renamed

// Call unsubscribe() to stop listening
unsubscribe();
```

---

## Lifecycle Hooks

Your plugin can export lifecycle hooks by returning an object from the main script:

```javascript
// index.js
return {
  onActivate() {
    // Called when the plugin is enabled/loaded
    // Set up your plugin here
    luzumi.ui.notify("My Plugin is active!");
  },

  onDeactivate() {
    // Called when the plugin is disabled/unloaded
    // Clean up resources here
  },

  onNavigate(path) {
    // Called when user navigates to a new directory
    console.log("User went to:", path);
  },

  onFileOpen(path) {
    // Called when a file is opened
    console.log("File opened:", path);
  },

  onContextMenu(path, kind) {
    // Called when context menu is opened
    // Use this to dynamically add/remove menu items
  },

  onSelectionChange(paths) {
    // Called when file selection changes
    console.log(`${paths.length} files selected`);
  }
};
```

---

## Permissions

Plugins run with a restricted API. Each capability requires an explicit permission in the manifest. If a plugin tries to use a feature without the matching permission, an error is thrown.

| Permission | Grants access to |
|:---|:---|
| `fs.read` | Read files and list directories |
| `fs.write` | Create and modify files |
| `fs.delete` | Delete files and directories |
| `ui.notify` | Show notifications and status messages |
| `ui.contextMenu` | Add items to the right-click menu |
| `nav.read` | Read current path and selection |
| `nav.navigate` | Navigate the file manager to a path |

Only request the permissions your plugin actually needs.

---

## Examples

### Word Counter Plugin

Shows word count for text files in the status bar.

**manifest.json:**
```json
{
  "name": "word-counter",
  "version": "1.0.0",
  "description": "Shows word count for selected text files",
  "permissions": ["fs.read", "ui.notify", "nav.read"],
  "hooks": ["onActivate", "onSelectionChange"]
}
```

**index.js:**
```javascript
const TEXT_EXTS = ["txt", "md", "json", "js", "ts", "py", "rs", "go", "html", "css"];

async function countWords(path) {
  try {
    const content = await luzumi.fs.readText(path, 10000);
    return content.split(/\s+/).filter(w => w.length > 0).length;
  } catch {
    return null;
  }
}

return {
  onActivate() {
    luzumi.ui.setStatus("Word Counter ready");
  },

  async onSelectionChange(paths) {
    if (paths.length !== 1) return;
    const path = paths[0];
    const ext = path.split(".").pop()?.toLowerCase();
    if (!ext || !TEXT_EXTS.includes(ext)) return;

    const count = await countWords(path);
    if (count !== null) {
      luzumi.ui.setStatus(`${count.toLocaleString()} words`);
    }
  }
};
```

### Git Status Menu Plugin

Adds git-related actions to the context menu.

**manifest.json:**
```json
{
  "name": "git-actions",
  "version": "1.0.0",
  "description": "Git actions in the context menu",
  "permissions": ["ui.contextMenu", "ui.notify", "nav.read"],
  "hooks": ["onActivate", "onDeactivate"]
}
```

**index.js:**
```javascript
return {
  onActivate() {
    luzumi.ui.addContextMenuItem({
      id: "git-log",
      label: "View Git Log",
      action: () => {
        const selected = luzumi.nav.getSelected();
        if (selected.length > 0) {
          luzumi.ui.notify(`Git log for: ${selected[0].split("/").pop()}`);
        }
      },
      when: (ctx) => ctx.kind === "file"
    });

    luzumi.ui.addContextMenuItem({
      id: "git-blame",
      label: "Git Blame",
      action: () => {
        luzumi.ui.notify("Opening git blame...");
      },
      when: (ctx) => ctx.kind === "file"
    });
  },

  onDeactivate() {
    luzumi.ui.removeContextMenuItem("git-log");
    luzumi.ui.removeContextMenuItem("git-blame");
  }
};
```

### Quick Notes Plugin

Creates a "New Note" action in the context menu.

**manifest.json:**
```json
{
  "name": "quick-notes",
  "version": "1.0.0",
  "description": "Quickly create markdown notes",
  "permissions": ["ui.contextMenu", "ui.notify", "nav.read", "nav.navigate"],
  "hooks": ["onActivate", "onDeactivate"]
}
```

**index.js:**
```javascript
return {
  onActivate() {
    luzumi.ui.addContextMenuItem({
      id: "new-note",
      label: "New Note",
      action: () => {
        const dir = luzumi.nav.getCurrentPath();
        luzumi.ui.notify(`Create a note in ${dir}`, "info");
      }
    });
  },

  onDeactivate() {
    luzumi.ui.removeContextMenuItem("new-note");
  }
};
```

---

## Installing Plugins

### Manual installation

Copy the plugin folder into `~/.config/luzumi/plugins/`:

```bash
cp -r my-plugin ~/.config/luzumi/plugins/
```

### From a git repository

```bash
git clone https://github.com/user/luzumi-plugin-name.git \
  ~/.config/luzumi/plugins/plugin-name
```

### From the app

You can also install plugins from a local directory using the command palette or the plugin manager UI.

---

## Managing Plugins

### Via the Command Palette

1. Press `Ctrl+Shift+P` to open the command palette
2. Type "plugin" to see plugin-related commands
3. Enable, disable, or manage plugins

### Via the filesystem

- **Enable**: The plugin must exist in `~/.config/luzumi/plugins/<name>/` and be listed in `~/.config/luzumi/plugins/.enabled.json`
- **Disable**: Remove the plugin name from `.enabled.json`
- **Uninstall**: Delete the plugin directory

### Enabled plugins file

Luzumi tracks which plugins are enabled in `~/.config/luzumi/plugins/.enabled.json`:

```json
["word-counter", "git-actions", "quick-notes"]
```

---

## Development Tips

### Debugging

- Use `console.log()` / `console.error()` in your plugin — output goes to the browser console (open with `Ctrl+Shift+I` in dev mode)
- Use the Debug Panel (`Ctrl+Shift+D`) to see frontend logs

### Hot Reloading

During development, you can edit your plugin's `index.js` and disable/re-enable the plugin to reload it without restarting Luzumi.

### File Entry Type

The `FileEntry` object returned by `luzumi.fs.list()` and `luzumi.fs.stat()` has these fields:

```typescript
interface FileEntry {
  name: string;           // File name
  path: string;           // Full absolute path
  kind: "file" | "directory" | "symlink";
  size: number | null;    // Size in bytes (null for directories)
  modified: number | null; // Unix timestamp
  is_hidden: boolean;
  extension: string | null;
  is_writable: boolean;
  permissions_mode: number | null; // Unix permissions (e.g. 0o755)
  is_broken_link: boolean;
}
```

### Context Menu Item Type

```typescript
interface ContextMenuItem {
  id: string;            // Unique ID (prefixed with plugin name automatically)
  label: string;         // Display text
  icon?: string;         // Optional icon name
  action: () => void;    // Callback when clicked
  when?: (context: {     // Optional filter function
    path: string;
    kind: string;
    extension: string;
  }) => boolean;
}
```

### Security

- Plugins run in the main thread with a restricted API — they cannot access the DOM or Tauri APIs directly
- All filesystem access goes through the permission-gated `luzumi.fs` API
- Plugin names are sanitized to prevent path traversal
- Plugin execution has a 5-second timeout per hook call
- Plugin crashes are isolated with try/catch — a failing plugin won't crash the app
- Only install plugins from sources you trust

---

## API Quick Reference

```
luzumi.fs.list(path)                    → Promise<FileEntry[]>
luzumi.fs.exists(path)                  → Promise<boolean>
luzumi.fs.readText(path, maxLines?)     → Promise<string>
luzumi.fs.stat(path)                    → Promise<FileEntry|null>

luzumi.ui.notify(msg, type?)            → void
luzumi.ui.setStatus(msg)                → void
luzumi.ui.addContextMenuItem(item)      → void
luzumi.ui.removeContextMenuItem(id)     → void

luzumi.nav.getCurrentPath()             → string
luzumi.nav.navigate(path)               → void
luzumi.nav.getSelected()                → string[]

luzumi.on(event, handler)               → () => void
```
