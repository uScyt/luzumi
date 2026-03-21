<h1 align="center">Theming Guide</h1>

<p align="center">
  Customize every pixel of Luzumi with CSS themes.
</p>

---

## Quick Start

1. Create a `.css` file in `~/.config/luzumi/themes/`
2. Open **Settings** in Luzumi and click **Reload**
3. Select your theme from the dropdown

Your theme applies instantly with a smooth transition. Only override the variables you want to change — everything else keeps its default value.

---

## Theme File Format

A theme file is a standard CSS file with a `:root` block:

```css
:root {
  --base:   #2e3440;
  --text:   #eceff4;
  --accent: var(--blue);
}
```

---

## Variable Reference

### Palette

The core colors used across the entire UI. Icons, text, badges, and states all derive from these.

| Variable | Default | Usage |
|---|---|---|
| `--rosewater` | `#f4dbd6` | Decorative accents |
| `--flamingo` | `#f0c6c6` | Package icons, Raku/Lisp |
| `--pink` | `#f5bde6` | C++, YAML, GraphQL, design files |
| `--mauve` | `#c6a0f6` | **Default accent** — C#, Kotlin, Haskell, Elixir |
| `--red` | `#ed8796` | Ruby, Scala, Angular, PDF, errors |
| `--maroon` | `#ee99a0` | Erlang, vault files |
| `--peach` | `#f5a97f` | Rust, Java, Swift, HTML, archives |
| `--yellow` | `#eed49f` | JS, JSON, music, data files |
| `--green` | `#a6da95` | Vue, Bash, images, terminal, certificates |
| `--teal` | `#8bd5ca` | Symlinks, Groovy, HDL, Proto |
| `--sky` | `#91d7e3` | Go, Dart, SQL, Docker |
| `--sapphire` | `#7dc4e4` | Markdown |
| `--blue` | `#8aadf4` | **Folders**, TypeScript, Python, CSS |
| `--lavender` | `#b7bdf8` | PHP, disc/font/VM files |

### Text & Overlays

| Variable | Default | Usage |
|---|---|---|
| `--text` | `#cad3f5` | Primary text |
| `--subtext1` | `#b8c0e0` | Secondary text |
| `--subtext0` | `#a5adce` | Tertiary text |
| `--overlay2` | `#939ab7` | Muted text, generic file icons |
| `--overlay1` | `#8087a2` | Disabled text, config/lock/log icons |
| `--overlay0` | `#8087a2` | Placeholder text (WCAG 3:1 minimum contrast) |

### Surfaces

| Variable | Default | Usage |
|---|---|---|
| `--crust` | `#181926` | Deepest background |
| `--mantle` | `#1e2030` | Deep background |
| `--base` | `#24273a` | Content area |
| `--surface0` | `#363a4f` | Elevated surfaces |
| `--surface1` | `#494d64` | Borders, dividers |
| `--surface2` | `#5b6078` | High contrast borders |

### App Backgrounds

| Variable | Default |
|---|---|
| `--app-bg` | `rgb(24, 26, 40)` |
| `--content-bg` | `var(--base)` |

### Glass Effects

| Variable | Default |
|---|---|
| `--glass-bg` | `rgba(30, 32, 48, 0.75)` |
| `--glass-bg-strong` | `rgba(22, 23, 36, 0.97)` |
| `--glass-border` | `rgba(255, 255, 255, 0.08)` |
| `--glass-blur` | `blur(24px) saturate(1.6)` |
| `--glass-blur-strong` | `blur(40px) saturate(1.8)` |

### Accent & Interactive

| Variable | Default |
|---|---|
| `--accent` | `var(--mauve)` |
| `--accent-muted` | `rgba(198, 160, 246, 0.14)` |
| `--accent-subtle` | `rgba(198, 160, 246, 0.08)` |
| `--accent-bg` | `rgba(198, 160, 246, 0.5)` |

### Hover & Borders

| Variable | Default |
|---|---|
| `--hover-bg` | `rgba(202, 211, 245, 0.08)` |
| `--hover-bg-subtle` | `rgba(202, 211, 245, 0.04)` |
| `--hover-bg-strong` | `rgba(202, 211, 245, 0.12)` |
| `--danger-bg` | `rgba(237, 135, 150, 0.2)` |
| `--border-subtle` | `rgba(255, 255, 255, 0.04)` |
| `--border-light` | `rgba(255, 255, 255, 0.06)` |
| `--border-medium` | `rgba(255, 255, 255, 0.08)` |

### Layout

| Variable | Default |
|---|---|
| `--radius-sm` | `6px` |
| `--radius-md` | `10px` |
| `--radius-lg` | `14px` |
| `--shadow-sm` | `0 2px 8px rgba(0,0,0,0.2)` |
| `--shadow-md` | `0 8px 24px rgba(0,0,0,0.35)` |
| `--shadow-lg` | `0 16px 48px rgba(0,0,0,0.5)` |

### Scrollbar & Selection

| Variable | Default |
|---|---|
| `--scrollbar-thumb` | `rgba(73, 77, 100, 0.4)` |
| `--scrollbar-thumb-hover` | `rgba(91, 96, 120, 0.7)` |
| `--selection-bg` | `rgba(198, 160, 246, 0.3)` |

---

## Icon Colors

Icons use `currentColor` and inherit from the palette via `getFileColor()`. Changing a palette variable changes all icons that reference it.

| Category | Variable |
|---|---|
| Folders | `--blue` |
| Symlinks | `--teal` |
| JS / JSON | `--yellow` |
| TS / Python / CSS | `--blue` |
| Rust / Java / HTML | `--peach` |
| C++ / YAML / GraphQL | `--pink` |
| C# / Kotlin / Haskell | `--mauve` |
| Ruby / Scala / Angular | `--red` |
| Go / Dart / Docker | `--sky` |
| Vue / Bash / Images | `--green` |
| Markdown | `--sapphire` |
| Archives | `--peach` |
| Config / Logs | `--overlay1` |
| Generic files | `--overlay2` |

> **Note:** `--blue` controls both folder icons and TS/Python/CSS icons. For a different folder color, be aware it will also affect those file types.

---

## Example Themes

### Catppuccin Latte (Light)

```css
/* ~/.config/luzumi/themes/catppuccin-latte.css */
:root {
  --rosewater: #dc8a78;  --flamingo: #dd7878;
  --pink:      #ea76cb;  --mauve:    #8839ef;
  --red:       #d20f39;  --maroon:   #e64553;
  --peach:     #fe640b;  --yellow:   #df8e1d;
  --green:     #40a02b;  --teal:     #179299;
  --sky:       #04a5e5;  --sapphire: #209fb5;
  --blue:      #1e66f5;  --lavender: #7287fd;

  --text:     #4c4f69;  --subtext1: #5c5f77;
  --subtext0: #6c6f85;  --overlay2: #7c7f93;
  --overlay1: #8c8fa1;  --overlay0: #9ca0b0;
  --surface2: #acb0be;  --surface1: #bcc0cc;
  --surface0: #ccd0da;
  --base:     #eff1f5;  --mantle:   #e6e9ef;  --crust: #dce0e8;

  --app-bg:          #dce0e8;
  --content-bg:      var(--base);
  --glass-bg:        rgba(230, 233, 239, 0.8);
  --glass-bg-strong: rgba(220, 224, 232, 0.97);
  --glass-border:    rgba(0, 0, 0, 0.08);

  --accent:        var(--mauve);
  --accent-muted:  rgba(136, 57, 239, 0.12);
  --accent-subtle: rgba(136, 57, 239, 0.06);
  --accent-bg:     rgba(136, 57, 239, 0.4);

  --hover-bg:        rgba(76, 79, 105, 0.08);
  --hover-bg-subtle: rgba(76, 79, 105, 0.04);
  --hover-bg-strong: rgba(76, 79, 105, 0.12);
  --danger-bg:       rgba(210, 15, 57, 0.15);

  --border-subtle: rgba(0, 0, 0, 0.04);
  --border-light:  rgba(0, 0, 0, 0.06);
  --border-medium: rgba(0, 0, 0, 0.08);

  --scrollbar-thumb:       rgba(172, 176, 190, 0.5);
  --scrollbar-thumb-hover: rgba(140, 143, 161, 0.7);
  --selection-bg:          rgba(136, 57, 239, 0.2);

  --shadow-sm: 0 2px 8px rgba(0,0,0,0.08);
  --shadow-md: 0 8px 24px rgba(0,0,0,0.12);
  --shadow-lg: 0 16px 48px rgba(0,0,0,0.18);
}
```

### Nord

```css
/* ~/.config/luzumi/themes/nord.css */
:root {
  --rosewater: #d08770;  --flamingo: #d08770;
  --pink:      #b48ead;  --mauve:    #b48ead;
  --red:       #bf616a;  --maroon:   #bf616a;
  --peach:     #d08770;  --yellow:   #ebcb8b;
  --green:     #a3be8c;  --teal:     #8fbcbb;
  --sky:       #88c0d0;  --sapphire: #81a1c1;
  --blue:      #81a1c1;  --lavender: #b48ead;

  --text:     #eceff4;  --subtext1: #e5e9f0;
  --subtext0: #d8dee9;  --overlay2: #a5b1c2;
  --overlay1: #8892a5;  --overlay0: #6b7489;
  --surface2: #4c566a;  --surface1: #434c5e;
  --surface0: #3b4252;
  --base:     #2e3440;  --mantle:   #292e39;  --crust: #242933;

  --app-bg:          #242933;
  --content-bg:      var(--base);
  --glass-bg:        rgba(46, 52, 64, 0.8);
  --glass-bg-strong: rgba(36, 41, 51, 0.97);
  --glass-border:    rgba(255, 255, 255, 0.06);

  --accent:        var(--blue);
  --accent-muted:  rgba(129, 161, 193, 0.14);
  --accent-subtle: rgba(129, 161, 193, 0.08);
  --accent-bg:     rgba(129, 161, 193, 0.5);

  --hover-bg:        rgba(236, 239, 244, 0.08);
  --hover-bg-subtle: rgba(236, 239, 244, 0.04);
  --hover-bg-strong: rgba(236, 239, 244, 0.12);
  --danger-bg:       rgba(191, 97, 106, 0.2);

  --border-subtle: rgba(255, 255, 255, 0.04);
  --border-light:  rgba(255, 255, 255, 0.06);
  --border-medium: rgba(255, 255, 255, 0.08);

  --scrollbar-thumb:       rgba(76, 86, 106, 0.5);
  --scrollbar-thumb-hover: rgba(67, 76, 94, 0.8);
  --selection-bg:          rgba(129, 161, 193, 0.3);
}
```

---

## Tips

- **Live editing** — Edit your theme file on disk, then re-select it in Settings to see changes instantly
- **Minimal themes** — You only need to override the variables you want to change
- **Light themes** — Remember to also override `--app-bg`, `--glass-*`, `--hover-*`, `--border-*`, `--shadow-*`, and `--scrollbar-*` with lighter values
- **Accent color** — Change `--accent` to restyle buttons, selections, and active states across the entire UI
- **Reload** — Click the reload button in Settings after modifying a theme file
- **Animations** — All transitions respect `prefers-reduced-motion`. Users can also disable animations entirely in Settings
