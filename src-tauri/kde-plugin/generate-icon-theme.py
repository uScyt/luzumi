#!/usr/bin/env python3
"""
Generate a freedesktop-compatible icon theme from Luzumi's SVG icons.
Creates icons at ~/.local/share/icons/luzumi/ with proper MIME type naming.
"""

import os
import sys

THEME_DIR = os.path.expanduser("~/.local/share/icons/luzumi")

# Catppuccin Macchiato palette (matches Luzumi's app.css)
COLORS = {
    "blue":      "#8aadf4",
    "red":       "#ed8796",
    "green":     "#a6da95",
    "yellow":    "#eed49f",
    "peach":     "#f5a97f",
    "pink":      "#f5bde6",
    "mauve":     "#c6a0f6",
    "lavender":  "#b7bdf8",
    "sky":       "#91d7e3",
    "teal":      "#8bd5ca",
    "sapphire":  "#7dc4e4",
    "flamingo":  "#f0c6c6",
    "maroon":    "#ee99a0",
    "subtext0":  "#a5adcb",
    "overlay2":  "#939ab7",
    "overlay1":  "#8087a2",
    "overlay0":  "#6e738d",
    "rosewater": "#f4dbd6",
    "text":      "#cad3f5",
}

# SVG fragments from Luzumi's icons.ts (16x16 viewBox)
ICON_SVGS = {
    "Folder": '<path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" fill="currentColor" opacity=".15" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/>',
    "File": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>',
    "FileText": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 10H10M6 12.5H9" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>',
    "FileCode": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 10L8.5 12.5L6 15M10 10L7.5 12.5L10 15" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/>',
    "FileImage": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><circle cx="6.5" cy="10" r="1" fill="currentColor"/><path d="M5 14L8 11L10 13L11.5 11.5L13 13" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/>',
    "FileVector": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 10L8 12L10 10" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" fill="none"/><circle cx="6" cy="10" r="0.8" fill="currentColor"/><circle cx="10" cy="10" r="0.8" fill="currentColor"/><circle cx="8" cy="12" r="0.8" fill="currentColor"/>',
    "FilmStrip": '<rect x="3" y="3" width="10" height="11" rx="1" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M3 6H13M3 11H13" stroke="currentColor" stroke-width="1.3"/><rect x="1" y="5" width="2" height="2" rx=".5" fill="currentColor"/><rect x="1" y="10" width="2" height="2" rx=".5" fill="currentColor"/><rect x="13" y="5" width="2" height="2" rx=".5" fill="currentColor"/><rect x="13" y="10" width="2" height="2" rx=".5" fill="currentColor"/>',
    "MusicNote": '<path d="M9 12V4L13 3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><circle cx="7" cy="12" r="2" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".15"/>',
    "FilePdf": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><text x="4.5" y="13.5" font-size="5" font-weight="700" fill="currentColor" font-family="monospace">PDF</text>',
    "FileDoc": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 10H10M6 12.5H9" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><text x="6" y="9.5" font-size="3.5" font-weight="800" fill="currentColor" font-family="monospace">W</text>',
    "FileSheet": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M5.5 8.5H10.5M5.5 8.5V13H10.5V8.5M8 8.5V13M5.5 10.5H10.5M5.5 12H10.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>',
    "FilePresentation": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="5.5" y="9" width="5" height="3.5" rx=".8" stroke="currentColor" stroke-width="1.1"/><path d="M7.5 11L9 10.5V12L7.5 11Z" fill="currentColor" opacity=".8"/>',
    "FileArchive": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="6.5" y="9" width="4" height="5" rx=".8" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".1"/><path d="M7.5 10.5H9.5M7.5 12.5H9.5" stroke="currentColor" stroke-width="1" stroke-linecap="round"/><path d="M8 9V8.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>',
    "FileMarkdown": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M5 13V10L6.5 11.5L8 10V13M9.5 13V11L11.5 13V11" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>',
    "Database": '<ellipse cx="8" cy="4.5" rx="5" ry="2" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".15"/><path d="M3 4.5V8C3 9.1 5.24 10 8 10S13 9.1 13 8V4.5M3 8v3.5C3 12.6 5.24 13.5 8 13.5S13 12.6 13 11.5V8" stroke="currentColor" stroke-width="1.4" fill="none"/>',
    "FileFont": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6.5 13L8 9L9.5 13M7.2 11.5H8.8" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>',
    "Package": '<path d="M8 2L14 5.5V10.5L8 14L2 10.5V5.5L8 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".1"/><path d="M8 2V14M2 5.5L8 9L14 5.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>',
    "Disc": '<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><circle cx="8" cy="8" r="2" stroke="currentColor" stroke-width="1.4" fill="none"/>',
    "FileISO": '<circle cx="8" cy="8" r="6.5" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><circle cx="8" cy="8" r="2" stroke="currentColor" stroke-width="1.3" fill="none"/><circle cx="8" cy="8" r="4.5" stroke="currentColor" stroke-width=".7" fill="none" opacity=".4"/><circle cx="8" cy="8" r=".6" fill="currentColor"/>',
    "TerminalWindow": '<rect x="2" y="3" width="12" height="10" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M5 7L7 9L5 11M8 11H11" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>',
    "Gear": '<circle cx="8" cy="8" r="2.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M8 1v2M8 13v2M1 8h2M13 8h2M2.9 2.9l1.4 1.4M11.7 11.7l1.4 1.4M2.9 13.1l1.4-1.4M11.7 4.3l1.4-1.4" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>',
    "FileBook": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 9.5C7 8.8 9 8.8 10 9.5V13.5C9 12.8 7 12.8 6 13.5V9.5Z" stroke="currentColor" stroke-width="1.1" fill="none"/><path d="M8 9V13.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>',
    "FileApp": '<rect x="2" y="2" width="12" height="12" rx="3" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".12"/><path d="M6 5.5V10.5L11 8L6 5.5Z" fill="currentColor" opacity=".5" stroke="currentColor" stroke-width="1" stroke-linejoin="round"/>',
    "FileBinary": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><text x="8" y="12.5" font-size="4.5" font-weight="700" fill="currentColor" font-family="monospace" text-anchor="middle">0101</text>',
    "FileLib": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="5.5" y="9" width="5" height="4.5" rx=".8" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".1"/><path d="M8 9V13.5M5.5 11H10.5" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>',
    "FilePlaylist": '<path d="M3 4H10M3 7H10M3 10H8" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M12 10V7L14 6.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/><circle cx="11" cy="10.5" r="1.3" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".15"/>',
    "FileSubtitle": '<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="5" y="9.5" width="6" height="4" rx=".8" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".06"/><path d="M6 11H10M6.5 12.5H9.5" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>',
    "FileChip": '<rect x="5" y="5" width="6" height="6" rx="1" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M7 5V3M9 5V3M7 11V13M9 11V13M5 7H3M11 7H13M5 9H3M11 9H13" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>',
    "Link": '<path d="M6.5 10.5L9.5 7.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><path d="M9 11L9.5 11.5C10.88 12.88 13.12 12.88 14.5 11.5s1.38-3.62 0-5L13 5C11.62 3.62 9.38 3.62 8 5L7.5 5.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" fill="none"/><path d="M7 5L6.5 4.5C5.12 3.12 2.88 3.12 1.5 4.5S.12 8.12 1.5 9.5L3 11C4.38 12.38 6.62 12.38 8 11L8.5 10.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" fill="none"/>',
}

# Icon color: Luzumi icon name -> hex color
ICON_COLORS = {
    "Folder":           COLORS["blue"],
    "File":             COLORS["overlay2"],
    "FileText":         COLORS["subtext0"],
    "FileCode":         COLORS["mauve"],
    "FileImage":        COLORS["green"],
    "FileVector":       COLORS["teal"],
    "FilmStrip":        COLORS["peach"],
    "MusicNote":        COLORS["yellow"],
    "FilePdf":          COLORS["red"],
    "FileDoc":          COLORS["blue"],
    "FileSheet":        COLORS["green"],
    "FilePresentation": COLORS["peach"],
    "FileArchive":      COLORS["peach"],
    "FileMarkdown":     COLORS["sapphire"],
    "Database":         COLORS["sky"],
    "FileFont":         COLORS["lavender"],
    "Package":          COLORS["flamingo"],
    "Disc":             COLORS["lavender"],
    "FileISO":          COLORS["lavender"],
    "TerminalWindow":   COLORS["green"],
    "Gear":             COLORS["overlay1"],
    "FileBook":         COLORS["yellow"],
    "FileApp":          COLORS["blue"],
    "FileBinary":       COLORS["overlay1"],
    "FileLib":          COLORS["mauve"],
    "FilePlaylist":     COLORS["yellow"],
    "FileSubtitle":     COLORS["sky"],
    "FileChip":         COLORS["teal"],
    "Link":             COLORS["teal"],
}

def make_svg(svg_content, color):
    """Wrap SVG fragment in full SVG with color applied."""
    colored = svg_content.replace("currentColor", color)
    return f'''<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none">
{colored}
</svg>'''

# Mapping: freedesktop MIME icon name -> (Luzumi icon, optional color override)
# KDE uses these names for desktop file icons
MIME_MAP = {
    # -- Directories (places category) --
    "inode-directory":                    ("Folder", None),
    "folder":                             ("Folder", None),
    "folder-open":                        ("Folder", None),
    "user-home":                          ("Folder", COLORS["yellow"]),
    "user-desktop":                       ("Folder", COLORS["mauve"]),
    "folder-download":                    ("Folder", COLORS["green"]),
    "folder-documents":                   ("Folder", COLORS["blue"]),
    "folder-music":                       ("Folder", COLORS["yellow"]),
    "folder-pictures":                    ("Folder", COLORS["green"]),
    "folder-videos":                      ("Folder", COLORS["peach"]),
    "folder-publicshare":                 ("Folder", COLORS["flamingo"]),
    "folder-templates":                   ("Folder", COLORS["overlay1"]),
    "folder-remote":                      ("Folder", COLORS["sky"]),
    "folder-bookmark":                    ("Folder", COLORS["mauve"]),
    "folder-favorites":                   ("Folder", COLORS["yellow"]),
    "folder-drag-accept":                 ("Folder", None),
    "folder-new":                         ("Folder", COLORS["green"]),

    # -- Generic file types --
    "text-plain":                         ("FileText", None),
    "text-x-generic":                     ("FileText", None),
    "text-html":                          ("FileCode", COLORS["peach"]),
    "text-css":                           ("FileCode", COLORS["blue"]),
    "text-x-script":                      ("FileCode", COLORS["green"]),
    "text-x-python":                      ("FileCode", COLORS["blue"]),
    "text-x-python3":                     ("FileCode", COLORS["blue"]),
    "text-x-java":                        ("FileCode", COLORS["peach"]),
    "text-x-csrc":                        ("FileCode", COLORS["blue"]),
    "text-x-c++src":                      ("FileCode", COLORS["pink"]),
    "text-x-chdr":                        ("FileCode", COLORS["blue"]),
    "text-x-c++hdr":                      ("FileCode", COLORS["pink"]),
    "text-x-csharp":                      ("FileCode", COLORS["mauve"]),
    "text-x-rust":                        ("FileCode", COLORS["peach"]),
    "text-x-go":                          ("FileCode", COLORS["sky"]),
    "text-x-ruby":                        ("FileCode", COLORS["red"]),
    "text-x-php":                         ("FileCode", COLORS["lavender"]),
    "text-x-perl":                        ("FileCode", COLORS["sky"]),
    "text-x-lua":                         ("FileCode", COLORS["blue"]),
    "text-x-r":                           ("FileCode", COLORS["blue"]),
    "text-x-haskell":                     ("FileCode", COLORS["mauve"]),
    "text-x-scala":                       ("FileCode", COLORS["red"]),
    "text-x-kotlin":                      ("FileCode", COLORS["mauve"]),
    "text-x-swift":                       ("FileCode", COLORS["peach"]),
    "text-x-dart":                        ("FileCode", COLORS["sky"]),
    "text-x-elixir":                      ("FileCode", COLORS["mauve"]),
    "text-x-zig":                         ("FileCode", COLORS["peach"]),
    "text-x-nim":                         ("FileCode", COLORS["yellow"]),
    "text-x-objc":                        ("FileCode", COLORS["blue"]),
    "text-x-fortran":                     ("FileCode", COLORS["mauve"]),
    "text-x-pascal":                      ("FileCode", COLORS["blue"]),
    "text-x-ada":                         ("FileCode", COLORS["green"]),
    "text-x-verilog":                     ("FileCode", COLORS["teal"]),
    "text-x-vhdl":                        ("FileCode", COLORS["teal"]),
    "text-x-clojure":                     ("FileCode", COLORS["green"]),
    "text-x-erlang":                      ("FileCode", COLORS["maroon"]),
    "text-x-julia":                       ("FileCode", COLORS["mauve"]),
    "text-x-ocaml":                       ("FileCode", COLORS["peach"]),
    "text-x-makefile":                    ("FileCode", COLORS["mauve"]),
    "text-x-cmake":                       ("FileCode", COLORS["mauve"]),
    "text-x-qml":                         ("FileCode", COLORS["sky"]),

    # Scripting / shell
    "application-x-shellscript":          ("TerminalWindow", None),
    "text-x-shellscript":                 ("TerminalWindow", None),

    # Markup / config
    "text-xml":                           ("FileCode", COLORS["peach"]),
    "application-xml":                    ("FileCode", COLORS["peach"]),
    "application-json":                   ("FileCode", COLORS["yellow"]),
    "application-x-yaml":                 ("FileCode", COLORS["pink"]),
    "text-x-yaml":                        ("FileCode", COLORS["pink"]),
    "application-toml":                   ("FileCode", COLORS["peach"]),
    "text-markdown":                      ("FileMarkdown", None),
    "text-x-markdown":                    ("FileMarkdown", None),
    "text-x-tex":                         ("FileCode", COLORS["teal"]),
    "text-x-bibtex":                      ("FileCode", COLORS["teal"]),

    # JavaScript / TypeScript / Web
    "application-javascript":             ("FileCode", COLORS["yellow"]),
    "text-javascript":                    ("FileCode", COLORS["yellow"]),
    "application-x-javascript":           ("FileCode", COLORS["yellow"]),
    "application-typescript":             ("FileCode", COLORS["blue"]),
    "text-x-typescript":                  ("FileCode", COLORS["blue"]),

    # Documents
    "application-pdf":                    ("FilePdf", None),
    "application-msword":                 ("FileDoc", None),
    "application-vnd.openxmlformats-officedocument.wordprocessingml.document": ("FileDoc", None),
    "application-vnd.oasis.opendocument.text": ("FileDoc", None),
    "application-vnd.ms-excel":           ("FileSheet", None),
    "application-vnd.openxmlformats-officedocument.spreadsheetml.sheet": ("FileSheet", None),
    "application-vnd.oasis.opendocument.spreadsheet": ("FileSheet", None),
    "application-vnd.ms-powerpoint":      ("FilePresentation", None),
    "application-vnd.openxmlformats-officedocument.presentationml.presentation": ("FilePresentation", None),
    "application-vnd.oasis.opendocument.presentation": ("FilePresentation", None),
    "application-rtf":                    ("FileDoc", None),
    "application-epub+zip":               ("FileBook", None),
    "application-x-mobipocket-ebook":     ("FileBook", None),

    # Images
    "image-png":                          ("FileImage", None),
    "image-jpeg":                         ("FileImage", None),
    "image-gif":                          ("FileImage", None),
    "image-bmp":                          ("FileImage", None),
    "image-webp":                         ("FileImage", None),
    "image-tiff":                         ("FileImage", None),
    "image-x-icon":                       ("FileImage", None),
    "image-vnd.adobe.photoshop":          ("FileImage", COLORS["pink"]),
    "image-x-xcf":                        ("FileImage", COLORS["pink"]),
    "image-svg+xml":                      ("FileVector", None),
    "image-x-eps":                        ("FileVector", None),
    "image-x-generic":                    ("FileImage", None),

    # Video
    "video-mp4":                          ("FilmStrip", None),
    "video-x-matroska":                   ("FilmStrip", None),
    "video-x-msvideo":                    ("FilmStrip", None),
    "video-quicktime":                    ("FilmStrip", None),
    "video-webm":                         ("FilmStrip", None),
    "video-x-flv":                        ("FilmStrip", None),
    "video-x-generic":                    ("FilmStrip", None),

    # Audio
    "audio-mpeg":                         ("MusicNote", None),
    "audio-x-flac":                       ("MusicNote", None),
    "audio-x-wav":                        ("MusicNote", None),
    "audio-ogg":                          ("MusicNote", None),
    "audio-x-vorbis+ogg":                 ("MusicNote", None),
    "audio-aac":                          ("MusicNote", None),
    "audio-mp4":                          ("MusicNote", None),
    "audio-x-generic":                    ("MusicNote", None),
    "audio-x-mpegurl":                    ("FilePlaylist", None),
    "application-x-cue":                  ("FilePlaylist", None),

    # Archives
    "application-zip":                    ("FileArchive", None),
    "application-x-tar":                  ("FileArchive", None),
    "application-gzip":                   ("FileArchive", None),
    "application-x-gzip":                 ("FileArchive", None),
    "application-x-bzip2":               ("FileArchive", None),
    "application-x-xz":                   ("FileArchive", None),
    "application-x-compressed-tar":       ("FileArchive", None),
    "application-x-bzip-compressed-tar":  ("FileArchive", None),
    "application-x-xz-compressed-tar":    ("FileArchive", None),
    "application-x-7z-compressed":        ("FileArchive", None),
    "application-x-rar":                  ("FileArchive", None),
    "application-vnd.rar":                ("FileArchive", None),
    "application-zstd":                   ("FileArchive", None),
    "application-x-zstd-compressed-tar":  ("FileArchive", None),
    "application-x-rpm":                  ("Package", None),
    "application-x-deb":                  ("Package", None),
    "application-vnd.flatpak":            ("Package", None),
    "application-vnd.snap":               ("Package", None),
    "application-x-appimage":             ("Package", None),

    # Database
    "application-x-sqlite3":              ("Database", None),
    "application-sql":                    ("Database", None),

    # Disc images
    "application-x-cd-image":             ("FileISO", None),
    "application-x-raw-disk-image":       ("FileISO", None),

    # Fonts
    "font-sfnt":                          ("FileFont", None),
    "application-x-font-ttf":             ("FileFont", None),
    "application-x-font-otf":             ("FileFont", None),
    "application-font-woff":              ("FileFont", None),
    "application-font-woff2":             ("FileFont", None),

    # Executables / binaries
    "application-x-executable":           ("FileBinary", None),
    "application-x-sharedlib":            ("FileLib", None),
    "application-x-object":               ("FileBinary", None),
    "application-x-desktop":              ("FileApp", None),

    # System
    "application-x-firmware":             ("FileChip", None),
    "application-octet-stream":           ("FileBinary", None),

    # Symlink
    "inode-symlink":                      ("Link", None),

    # Subtitles
    "application-x-subrip":               ("FileSubtitle", None),

    # Generic fallbacks
    "unknown":                            ("File", None),
    "application-x-generic":              ("File", None),
    "text-x-generic-template":            ("FileText", None),
}

def main():
    # Sizes to generate (scalable SVGs, but KDE looks for specific size dirs)
    sizes = [16, 22, 24, 32, 48, 64, 128, 256]

    # Create theme directory
    os.makedirs(THEME_DIR, exist_ok=True)

    # Write index.theme
    size_dirs = []
    for s in sizes:
        size_dirs.append(f"{s}x{s}/mimetypes")
        size_dirs.append(f"{s}x{s}/places")

    directories_str = ",".join(size_dirs)

    index_content = f"""[Icon Theme]
Name=Luzumi
Comment=Luzumi file manager icon theme
Inherits=breeze,hicolor
Example=folder

Directories={directories_str}

"""
    for s in sizes:
        index_content += f"""[{s}x{s}/mimetypes]
Size={s}
Context=MimeTypes
Type=Scalable
MinSize={s}
MaxSize={s * 2}

[{s}x{s}/places]
Size={s}
Context=Places
Type=Scalable
MinSize={s}
MaxSize={s * 2}

"""

    with open(os.path.join(THEME_DIR, "index.theme"), "w") as f:
        f.write(index_content)

    # Create directories and generate SVGs
    count = 0
    for size in sizes:
        for category in ["mimetypes", "places"]:
            dirpath = os.path.join(THEME_DIR, f"{size}x{size}", category)
            os.makedirs(dirpath, exist_ok=True)

    for icon_name, (luzumi_icon, color_override) in MIME_MAP.items():
        svg_content = ICON_SVGS.get(luzumi_icon)
        if not svg_content:
            continue

        color = color_override or ICON_COLORS.get(luzumi_icon, COLORS["overlay2"])
        svg = make_svg(svg_content, color)

        # Determine category
        is_place = icon_name.startswith("folder") or icon_name in (
            "user-home", "user-desktop", "inode-directory"
        )
        category = "places" if is_place else "mimetypes"

        for size in sizes:
            filepath = os.path.join(THEME_DIR, f"{size}x{size}", category, f"{icon_name}.svg")
            with open(filepath, "w") as f:
                f.write(svg)
        count += 1

    print(f"Generated {count} icon mappings in {THEME_DIR}")
    print(f"Sizes: {', '.join(str(s) for s in sizes)}")
    print()
    print("To apply: System Settings → Appearance → Icons → select 'Luzumi'")
    print("Or run: plasma-apply-icontheme luzumi")

if __name__ == "__main__":
    main()
