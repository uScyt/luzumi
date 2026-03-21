<p align="center">
  <img src="src-tauri/icons/archive.png" width="120" alt="Luzumi" />
</p>

<h1 align="center">Luzumi</h1>

<p align="center">
  <strong>A modern, fast, and beautiful file manager for Linux.</strong><br/>
  <sub>Built with <a href="https://v2.tauri.app">Tauri 2</a> + <a href="https://svelte.dev">Svelte 5</a> + <a href="https://www.rust-lang.org">Rust</a></sub>
</p>

<p align="center">
  <a href="#installation"><img src="https://img.shields.io/badge/platform-Linux-blue?style=flat-square&logo=linux&logoColor=white" alt="Linux" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" /></a>
  <img src="https://img.shields.io/badge/version-0.2.0-purple?style=flat-square" alt="v0.2.0" />
  <img src="https://img.shields.io/badge/Rust-backend-orange?style=flat-square&logo=rust&logoColor=white" alt="Rust" />
</p>

<br/>

<p align="center">
  <img src="gh-images/luzumi.png" alt="Luzumi screenshot" width="820" style="border-radius: 12px;" />
</p>

<br/>

> **No Electron. No bloat.** Just a thin webview powered by Tauri and a Rust backend that talks directly to your filesystem. Designed for people who want something between a terminal and Nautilus.

<br/>

## Highlights

<table>
<tr>
<td width="50%">

**Lightning fast**
- Rust backend with zero-copy file ops
- Virtual scrolling for 100k+ files
- Real-time file watching (inotify)

</td>
<td width="50%">

**Beautiful & themeable**
- Catppuccin Macchiato by default
- 40+ CSS variables for full control
- Drop-in custom themes

</td>
</tr>
<tr>
<td>

**Keyboard-driven**
- Every action has a shortcut
- Command palette (`Ctrl+Shift+P`) with natural language
- Grid/list arrow key navigation
- Bulk rename with regex

</td>
<td>

**Extensible**
- JavaScript plugin system
- Custom context menu actions
- Event hooks & lifecycle API

</td>
</tr>
</table>

<br/>

## Features

### Navigation & Views
- **Dual view modes** — Grid view with thumbnails + detailed list view
- **Keyboard navigation** — Arrow keys, Home/End in grid and list views
- **Tabs** — Multiple directories, `Ctrl+Tab` to switch
- **Split pane** — Side-by-side directory comparison
- **Preview pane** — Images, videos, PDF, Markdown, code (syntax highlighted), hex viewer
- **Breadcrumb navigation** with editable address bar
- **Virtual scrolling** — Handles massive directories smoothly
- **Navigation history** — Right-click back/forward for timeline dropdown
- **Workspaces** — Save and restore tab layouts
- **Type filter** — Filter files by category (Images, Videos, Audio, Documents, Code, Archives) from the toolbar

### Search
- **Fuzzy search** — Instant filename matching with Jaro-Winkler scoring
- **Content search** — Grep-like search inside files with line context
- **Advanced search panel** (`Ctrl+Shift+F`) — Filter by type, size, date
- **Natural language commands** — Type "sort by size" or "find duplicates" in the command palette
- **Search history** — Last 50 searches saved

### File Operations
- Copy, move, rename, duplicate, delete with **undo/redo** (50-level stack)
- **Inline rename** — New folder/file creates instantly and enters rename mode (like Nautilus/Dolphin)
- **Operation queue** — Pause, resume, cancel running operations
- **Conflict resolution** — Overwrite, auto-rename, or skip on copy/move conflicts
- **Secure deletion** — Cryptographically random multi-pass overwrite for sensitive files
- **Archive support** — Create & extract ZIP archives
- **Bulk rename** — Find & replace, case conversion, sequential numbering
- **Duplicate file finder** — Content-hash based detection
- **Drag & drop** — Internal moves + cross-app drag via `text/uri-list`
- **Folder comparison** — Side-by-side diff with left-only, right-only, modified, identical

### Tags & Organization
- **File tags** — Color-coded labels stored in SQLite
- **Tag manager** — Create, rename, delete tags with color picker
- **Smart suggestions** — Auto-analyze directories and suggest actions (organize, find duplicates, bulk rename)

### Filesystem & Permissions
- **Real-time file watching** — Instant detection via `notify` crate (auto-detects inotify on Linux)
- **Permission editing** — Visual rwx checkboxes + octal input
- **Auto-elevation** — Seamless pkexec when permission is denied
- **Symlink support** — Create, follow, and detect broken links

### Trash & Drives
- **Freedesktop Trash spec** — Full compliance with percent-encoding
- **External drive trash** — Scans `.Trash-<uid>/` on mounted volumes
- **Device management** — Mount, unmount, eject, unlock LUKS volumes
- **GVFS support** — GNOME network mounts (SMB, NFS, SSHFS)

### System Integration
- **XDG Desktop Portal** — System-wide file picker with preview pane
- **KDE Plasma desktop plugin** — Right-click context menu on the desktop (new folder/file with name prompt, paste, open terminal, open Luzumi)
- **Open With** — Launch files with any installed app, set defaults, custom commands
- **Set as default file manager** — One-click setup
- **Open terminal here** — Launch terminal in current directory
- **Integrity check** — Verify and auto-repair Luzumi installation from Settings

### Icons & Preview
- **120+ file type icons** — Languages, archives, media, configs, ML models...
- **Image thumbnails** in grid view (PNG, JPG, WebP, AVIF, HEIC...)
- **Video thumbnails** via ffmpeg (MP4, MKV, AVI, MOV, WebM...)
- **PDF preview** — Page-by-page viewing via poppler
- **Markdown preview** — Rendered HTML with full styling
- **Code preview** — Syntax-highlighted source files
- **Hex viewer** — Binary file inspection with offset/hex/ASCII columns
- **Skeleton loading** — Placeholder rows while loading large directories

### Accessibility
- **Reduced motion** — Respects `prefers-reduced-motion` system setting
- **WCAG contrast** — Improved overlay contrast ratios
- **ARIA attributes** — Proper roles and live regions for screen readers
- **Keyboard-accessible** — All views, dialogs, and controls support keyboard navigation

### Customization
- **Full CSS theming** — Drop a `.css` file in `~/.config/luzumi/themes/`
- Built-in Light, Nord, Catppuccin themes
- **Density modes** — Comfortable, Compact, Dense
- **Font size & line height** sliders
- **High contrast mode** and **animation toggle**
- **Plugin system** — Extend Luzumi with JavaScript plugins
- French & English interface

<br/>

## Installation

### From source

**Prerequisites:**
- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) (v18+)
- [ffmpeg](https://ffmpeg.org/) (optional, for video thumbnails)
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
cd scripts && chmod +x build-appimage.sh && ./build-appimage.sh
```

<br/>

## Development

```bash
npm install
npm run tauri dev
```

Hot-reload is enabled — edit Svelte components and see changes instantly. Rust changes trigger a recompile.

<br/>

## Keyboard Shortcuts

| Action | Shortcut |
|:---|:---|
| Command palette | `Ctrl+Shift+P` |
| New tab | `Ctrl+T` |
| Close tab | `Ctrl+W` |
| Navigate back / forward | `Alt+Left` / `Alt+Right` |
| Go up | `Alt+Up` |
| Reload | `F5` |
| Focus address bar | `Ctrl+L` |
| Search | `Ctrl+F` |
| Advanced search | `Ctrl+Shift+F` |
| Select all | `Ctrl+A` |
| Invert selection | `Ctrl+Shift+A` |
| Select by pattern | `Ctrl+G` |
| Copy / Cut / Paste | `Ctrl+C` / `Ctrl+X` / `Ctrl+V` |
| Duplicate | `Ctrl+D` |
| Delete | `Delete` |
| Secure delete | `Shift+Delete` |
| Rename | `F2` |
| New folder | `Ctrl+Shift+N` |
| Toggle hidden files | `Ctrl+H` |
| Toggle preview | `Ctrl+P` |
| Split view | `F3` |
| List / Grid view | `Ctrl+1` / `Ctrl+2` |
| Undo / Redo | `Ctrl+Z` / `Ctrl+Shift+Z` |
| Settings | `Ctrl+,` |
| Debug panel | `Ctrl+Shift+D` |
| Keyboard shortcuts | `?` |

<br/>

## Theming

Luzumi supports custom CSS themes. Create a `.css` file in `~/.config/luzumi/themes/`, then select it in **Settings > Theme**.

See **[THEMING.md](THEMING.md)** for the full variable reference and example themes.

<br/>

## Plugins

Luzumi has a JavaScript plugin system for adding custom functionality. Plugins can:

- Add items to the right-click context menu
- React to navigation, file open, and selection events
- Read files and directories through a sandboxed API
- Show notifications and status messages

**Quick example** — create `~/.config/luzumi/plugins/my-plugin/manifest.json`:

```json
{
  "name": "my-plugin",
  "version": "1.0.0",
  "permissions": ["ui.notify", "nav.read"],
  "hooks": ["onNavigate"]
}
```

And `index.js`:

```javascript
return {
  onNavigate(path) {
    luzumi.ui.notify("Now in: " + path);
  }
};
```

See **[PLUGINS.md](PLUGINS.md)** for the full API reference, permissions, lifecycle hooks, and example plugins.

<br/>

## Tech Stack

| Layer | Technology |
|:---|:---|
| Backend | Rust |
| Frontend | Svelte 5 (runes) |
| Framework | Tauri 2 |
| File watching | `notify` crate (auto-detects inotify/fsevents) |
| Search | `strsim` (fuzzy), `regex` (glob), `memmap2` (content) |
| Tags | SQLite (`rusqlite`) |
| Preview | `pulldown-cmark` (md), `poppler` (pdf) |
| Logging | `tracing` + `tracing-subscriber` |
| Thumbnails | ffmpeg (video), base64 (images) |
| Trash spec | Freedesktop + `percent-encoding` |
| Styling | CSS custom properties (Catppuccin) |
| Plugins | JavaScript with sandboxed API |
| CI/CD | GitHub Actions |
| Packaging | `.deb`, `.rpm`, AppImage |

<br/>

## Project Structure

```
luzumi/
  src/                              # Svelte frontend
    lib/
      components/                   # UI components (30+)
        preview/                    # Preview sub-components
          CodePreview.svelte        # Syntax-highlighted code
          MarkdownPreview.svelte    # Rendered markdown
          PdfPreview.svelte         # PDF page viewer
          HexViewer.svelte          # Binary hex viewer
        dialogs/                    # Dialog components (15+)
        CommandPalette.svelte       # Ctrl+Shift+P command launcher
        DebugPanel.svelte           # Performance & logs panel
        ErrorBoundary.svelte        # Crash recovery
        OperationQueue.svelte       # Operation progress tracker
        SuggestionBar.svelte        # Smart directory suggestions
        WorkspaceSwitcher.svelte    # Workspace save/load
        ...                         # FileGrid, FileList, PreviewPane, etc.
      state/                        # Reactive state modules
        searchState.svelte.ts       # Search query, results, history
        selectionState.svelte.ts    # Selection & clipboard
        navigationState.svelte.ts   # History, tabs, bookmarks
        operationState.svelte.ts    # Undo/redo, operation queue
        uiState.svelte.ts           # UI preferences & dialogs
        sessionState.svelte.ts      # Session persistence
        debugState.svelte.ts        # Performance tracking
      plugins/                      # Plugin system
        pluginApi.ts                # Sandboxed API for plugins
        pluginLoader.ts             # Plugin lifecycle management
      commandRegistry.ts            # Command palette registry
      naturalCommand.ts             # Natural language command parser
      errors.ts                     # Centralized error handling
      transitions.ts                # Motion-aware transitions
      eventBus.ts                   # Cross-component event system
      fileManager.svelte.ts         # Core state (composes modules)
      i18n.ts                       # Translations (en, fr)
      icons.ts                      # SVG icon library (120+ icons)
      types.ts                      # Types & file icon/color mapping
    App.svelte                      # Root component
    app.css                         # Theme variables & base styles
  src-tauri/                        # Rust backend
    src/
      lib.rs                        # Tauri commands (60+), integrity check
      filesystem.rs                 # File ops, trash, permissions
      search.rs                     # Fuzzy & content search (with ReDoS protection)
      tags.rs                       # SQLite tag system
      compare.rs                    # Folder comparison
      preview.rs                    # PDF, markdown, hex preview
      vault.rs                      # Encrypted vault system
      plugins.rs                    # Plugin management
      logging.rs                    # Structured logging (tracing)
      operation_queue.rs            # Async operation queue
      errors.rs                     # Typed error system
      desktop_apps.rs               # Application launcher
    luzumi-portal/                  # XDG Desktop Portal integration
    kde-plugin/                     # KDE Plasma desktop context menu plugin
  .github/workflows/               # CI/CD
    ci.yml                          # Lint, clippy, test, audit
    release.yml                     # Tag-based release
```

<br/>

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

<br/>

## License

MIT License — see [LICENSE](LICENSE) for details.
