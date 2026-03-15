<p align="center">
  <img src="src-tauri/icons/app-icon.png" width="120" alt="Luzumi" />
</p>

<h1 align="center">Luzumi</h1>

<p align="center">
  <strong>A modern, fast, and beautiful file manager for Linux.</strong>
</p>

<p align="center">
  Built with <a href="https://v2.tauri.app">Tauri 2</a> + <a href="https://svelte.dev">Svelte 5</a> + <a href="https://www.rust-lang.org">Rust</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/platform-Linux-blue?style=flat-square" alt="Linux" />
  <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
  <img src="https://img.shields.io/badge/version-0.1.0-purple?style=flat-square" alt="v0.1.0" />
</p>

---

## What is Luzumi?

Luzumi is a lightweight, native file manager that feels snappy and looks gorgeous. No Electron, no bloat — just a thin webview powered by Tauri and a Rust backend that talks directly to your filesystem.

It's designed for people who want something between a terminal and Nautilus: fast navigation, keyboard-driven workflows, and a UI that gets out of the way.

---

## Features

**Core**
- Dual view modes — list and grid with thumbnails
- Tabs — open multiple directories side by side
- Preview pane — quick look at text, images, and file metadata
- Breadcrumb navigation with editable address bar
- Drag & drop — move and copy files visually

**File operations**
- Copy, move, rename, duplicate, delete
- Secure deletion (multi-pass overwrite)
- Archive creation & extraction (ZIP)
- Bulk rename — find & replace, case conversion, numbering
- Duplicate file finder
- Undo / redo for file operations

**System integration**
- XDG Desktop Portal — use Luzumi as your system file picker
- Open With — launch files with any installed application
- Device management — mount, unmount, eject, unlock encrypted drives
- Admin mode — elevated operations via pkexec
- Set as default file manager
- Open terminal here

**Customization**
- Full CSS theming — drop a `.css` file in `~/.config/luzumi/themes/`
- Catppuccin Macchiato dark theme by default
- Light theme, Nord, and custom themes supported
- Icon colors adapt to your palette automatically

**Quality of life**
- Keyboard shortcuts for everything
- Pin folders to Quick Access
- Calculate folder sizes
- File properties with permissions, ownership, dates
- French & English interface

---

## Installation

### From source

**Prerequisites**
- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) (v18+)
- System libs: `webkit2gtk-4.1`, `libappindicator3`, `librsvg2` (see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

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

## Theming

Luzumi supports custom CSS themes. Create a `.css` file in `~/.config/luzumi/themes/`, then select it in **Settings > Theme**.

A theme file overrides CSS custom properties:

```css
:root {
  --base: #2e3440;
  --text: #eceff4;
  --accent: var(--blue);
  /* ... */
}
```

40+ variables are available — colors, backgrounds, borders, glass effects, shadows, and more.

See [THEMING.md](THEMING.md) for the full reference, examples (Nord, Catppuccin Latte), and tips.

---

## Keyboard shortcuts

| Action | Shortcut |
|--------|----------|
| New tab | `Ctrl+T` |
| Close tab | `Ctrl+W` |
| Navigate back | `Alt+Left` |
| Navigate forward | `Alt+Right` |
| Go up | `Alt+Up` |
| Reload | `F5` |
| Focus address bar | `Ctrl+L` |
| Search | `Ctrl+F` |
| Select all | `Ctrl+A` |
| Copy | `Ctrl+C` |
| Cut | `Ctrl+X` |
| Paste | `Ctrl+V` |
| Delete | `Delete` |
| Rename | `F2` |
| Properties | `Ctrl+I` |
| New folder | `Ctrl+Shift+N` |
| Toggle hidden files | `Ctrl+H` |
| List view | `Ctrl+1` |
| Grid view | `Ctrl+2` |
| Undo | `Ctrl+Z` |
| Redo | `Ctrl+Shift+Z` |

---

## Tech stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust |
| Frontend | Svelte 5 |
| Framework | Tauri 2 |
| Styling | CSS custom properties |
| Database | SQLite (file indexing) |
| Packaging | .deb, .rpm, AppImage |

---

## Project structure

```
luzumi/
  src/                     # Svelte frontend
    lib/
      components/          # UI components
      fileManager.svelte.ts  # Core state management
      i18n.ts              # Translations (en, fr)
      icons.ts             # SVG icon library
      types.ts             # TypeScript types
    App.svelte             # Root component
    app.css                # Theme variables & base styles
  src-tauri/               # Rust backend
    src/
      lib.rs               # Tauri commands
      filesystem.rs        # File operations & trash
      desktop_apps.rs      # Application launcher
    luzumi-portal/         # XDG Desktop Portal integration
  THEMING.md               # Theme creation guide
```

---

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

---

## License

MIT License - see [LICENSE](LICENSE) for details.
