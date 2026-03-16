<p align="center">
  <img src="src-tauri/icons/app-icon.png" width="140" alt="Luzumi" />
</p>

<h1 align="center">Luzumi</h1>

<p align="center">
  <em>A modern, fast, and beautiful file manager for Linux.</em><br/>
  Built with <a href="https://v2.tauri.app">Tauri 2</a> + <a href="https://svelte.dev">Svelte 5</a> + <a href="https://www.rust-lang.org">Rust</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/platform-Linux-blue?style=for-the-badge&logo=linux&logoColor=white" alt="Linux" />
  <img src="https://img.shields.io/badge/license-MIT-green?style=for-the-badge" alt="MIT License" />
  <img src="https://img.shields.io/badge/version-0.1.0-purple?style=for-the-badge" alt="v0.1.0" />
  <img src="https://img.shields.io/badge/Rust-backend-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
</p>

<p align="center">
  <img src="gh-images/luzumi.png" alt="Luzumi screenshot" width="800" />
</p>

---

Luzumi is a lightweight, native file manager that feels snappy and looks gorgeous. No Electron, no bloat — just a thin webview powered by Tauri and a Rust backend that talks directly to your filesystem.

Designed for people who want something between a terminal and Nautilus: fast navigation, keyboard-driven workflows, and a UI that stays out of the way.

---

## Features

### Navigation & Views

- **Dual view modes** — List view and grid view with image thumbnails
- **Tabs** — Open multiple directories, switch with `Ctrl+Tab`
- **Split pane** — Side-by-side directory comparison
- **Preview pane** — Quick look at text, images, and file metadata
- **Breadcrumb navigation** with editable address bar
- **Virtual scrolling** — Handles directories with 100k+ files smoothly

### File Operations

- Copy, move, rename, duplicate, delete with undo/redo
- **Secure deletion** — Multi-pass overwrite for sensitive files
- **Archive support** — Create & extract ZIP archives
- **Bulk rename** — Find & replace, case conversion, sequential numbering
- **Duplicate file finder** — Detect identical files by content hash
- **Drag & drop** — Move and copy files visually, including native drag

### Filesystem & Permissions

- **Real-time file watching** — Powered by `notify`, no polling needed. Instant detection of external changes to files, permissions, and sizes
- **Permission editing** — Visual rwx checkboxes + octal input in Properties dialog
- **Auto-elevation** — Seamless pkexec escalation when permission is denied
- **Symlink support** — Create symlinks, detect and display broken links with visual indicators
- **Broken link detection** — Strikethrough + red highlighting for dangling symlinks

### Trash & Drives

- **Freedesktop Trash spec** — Full compliance with proper percent-encoding
- **External drive trash** — Scans `.Trash-<uid>/` on all mounted volumes
- **Trash metadata** — Shows original path and deletion date from `.trashinfo`
- **Real-time trash monitoring** — Trash size updates automatically via file watcher
- **Device management** — Mount, unmount, eject removable drives, unlock LUKS encrypted volumes
- **Dynamic mount detection** — Reads `/proc/mounts` for accurate filesystem types
- **GVFS support** — Detects GNOME network mounts (SMB, NFS, SSHFS)

### System Integration

- **XDG Desktop Portal** — Use Luzumi as your system-wide file picker
- **Open With** — Launch files with any installed application
- **Admin mode** — Elevated operations via pkexec
- **Set as default file manager** — One-click setup
- **Open terminal here** — Launch your terminal in the current directory

### Customization

- **Full CSS theming** — Drop a `.css` file in `~/.config/luzumi/themes/`
- **Catppuccin Macchiato** dark theme by default
- Built-in Light, Nord, and custom theme support
- 40+ CSS variables — Colors, glass effects, shadows, borders
- Icon colors adapt to your palette automatically

### Quality of Life

- Keyboard shortcuts for everything
- Pin folders to Quick Access
- Calculate folder sizes on demand
- File properties with permissions, ownership, checksums, and dates
- French & English interface
- Error messages shown inline — no more silent failures

---

## Installation

### From source

**Prerequisites:**
- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) (v18+)
- System libs: `webkit2gtk-4.1`, `libappindicator3`, `librsvg2`
  (see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

```bash
git clone https://github.com/uScyt/luzumi.git
cd luzumi
npm install
npm run tauri build
```

The built binary will be in `src-tauri/target/release/luzumi`.
Packages (`.deb`, `.rpm`) are generated in `src-tauri/target/release/bundle/`.

### AppImage

```bash
npm run tauri build
cd scripts
chmod +x build-appimage.sh
./build-appimage.sh
```

---

## Development

```bash
npm install
npm run tauri dev
```

Hot-reload is enabled — edit Svelte components and see changes instantly. Rust changes trigger a recompile.

---

## Keyboard Shortcuts

| Action | Shortcut |
|---|---|
| New tab | `Ctrl+T` |
| Close tab | `Ctrl+W` |
| Navigate back / forward | `Alt+Left` / `Alt+Right` |
| Go up | `Alt+Up` |
| Reload | `F5` |
| Focus address bar | `Ctrl+L` |
| Search | `Ctrl+F` |
| Select all | `Ctrl+A` |
| Copy / Cut / Paste | `Ctrl+C` / `Ctrl+X` / `Ctrl+V` |
| Delete | `Delete` |
| Rename | `F2` |
| Properties | `Ctrl+I` |
| New folder | `Ctrl+Shift+N` |
| Toggle hidden files | `Ctrl+H` |
| List / Grid view | `Ctrl+1` / `Ctrl+2` |
| Undo / Redo | `Ctrl+Z` / `Ctrl+Shift+Z` |

---

## Theming

Luzumi supports custom CSS themes. Create a `.css` file in `~/.config/luzumi/themes/`, then select it in **Settings > Theme**.

See **[THEMING.md](THEMING.md)** for the full variable reference, example themes (Nord, Catppuccin Latte), and tips.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Backend | Rust |
| Frontend | Svelte 5 |
| Framework | Tauri 2 |
| File watching | `notify` crate (inotify/fanotify) |
| Trash spec | Freedesktop + `percent-encoding` |
| Database | SQLite (file indexing) |
| Styling | CSS custom properties |
| Packaging | `.deb`, `.rpm`, AppImage |

---

## Project Structure

```
luzumi/
  src/                          # Svelte frontend
    lib/
      components/               # UI components
        VirtualScroller.svelte  # Virtual scrolling for large dirs
        FileGrid.svelte         # Grid view
        FileList.svelte         # List view
        TrashView.svelte        # Trash management
        dialogs/                # Properties, bulk rename, etc.
      fileManager.svelte.ts     # Core state management
      i18n.ts                   # Translations (en, fr)
      icons.ts                  # SVG icon library (200+ icons)
      types.ts                  # TypeScript types
    App.svelte                  # Root component
    app.css                     # Theme variables & base styles
  src-tauri/                    # Rust backend
    src/
      lib.rs                    # Tauri commands & file watchers
      filesystem.rs             # File ops, trash, permissions, mounts
      desktop_apps.rs           # Application launcher
    luzumi-portal/              # XDG Desktop Portal integration
  THEMING.md                    # Theme creation guide
```

---

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

---

## License

MIT License — see [LICENSE](LICENSE) for details.
