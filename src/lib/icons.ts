/** Shared SVG icon paths for file type icons (16×16 viewBox) */
export const svgPaths: Record<string, string> = {
  // --- Base ---
  Folder:        `<path d="M2 5C2 4.45 2.45 4 3 4H7L9 6H13C13.55 6 14 6.45 14 7V12C14 12.55 13.55 13 13 13H3C2.45 13 2 12.55 2 12V5Z" fill="currentColor" opacity=".15" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/>`,
  File:          `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>`,
  Link:          `<path d="M6.5 10.5L9.5 7.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><path d="M9 11L9.5 11.5C10.88 12.88 13.12 12.88 14.5 11.5s1.38-3.62 0-5L13 5C11.62 3.62 9.38 3.62 8 5L7.5 5.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" fill="none"/><path d="M7 5L6.5 4.5C5.12 3.12 2.88 3.12 1.5 4.5S.12 8.12 1.5 9.5L3 11C4.38 12.38 6.62 12.38 8 11L8.5 10.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" fill="none"/>`,

  Vault:         `<rect x="2" y="7" width="12" height="8" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M5 7V5a3 3 0 016 0v2" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/><circle cx="8" cy="11" r="1" fill="currentColor"/><path d="M8 12V13" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>`,

  // --- Code & text ---
  FileCode:      `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 10L8.5 12.5L6 15M10 10L7.5 12.5L10 15" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/>`,
  FileText:      `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 10H10M6 12.5H9" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>`,
  FileMarkdown:  `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M5 13V10L6.5 11.5L8 10V13M9.5 13V11L11.5 13V11" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>`,

  // --- Media ---
  FileImage:     `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><circle cx="6.5" cy="10" r="1" fill="currentColor"/><path d="M5 14L8 11L10 13L11.5 11.5L13 13" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/>`,
  FileVector:    `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 10L8 12L10 10" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" fill="none"/><circle cx="6" cy="10" r="0.8" fill="currentColor"/><circle cx="10" cy="10" r="0.8" fill="currentColor"/><circle cx="8" cy="12" r="0.8" fill="currentColor"/>`,
  FilmStrip:     `<rect x="3" y="3" width="10" height="11" rx="1" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M3 6H13M3 11H13" stroke="currentColor" stroke-width="1.3"/><rect x="1" y="5" width="2" height="2" rx=".5" fill="currentColor"/><rect x="1" y="10" width="2" height="2" rx=".5" fill="currentColor"/><rect x="13" y="5" width="2" height="2" rx=".5" fill="currentColor"/><rect x="13" y="10" width="2" height="2" rx=".5" fill="currentColor"/>`,
  MusicNote:     `<path d="M9 12V4L13 3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><circle cx="7" cy="12" r="2" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".15"/>`,
  FilePlaylist:  `<path d="M3 4H10M3 7H10M3 10H8" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M12 10V7L14 6.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/><circle cx="11" cy="10.5" r="1.3" stroke="currentColor" stroke-width="1.2" fill="currentColor" opacity=".15"/>`,

  // --- Documents ---
  FilePdf:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><text x="4.5" y="13.5" font-size="5" font-weight="700" fill="currentColor" font-family="monospace">PDF</text>`,
  FileDoc:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 10H10M6 12.5H9" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><text x="6" y="9.5" font-size="3.5" font-weight="800" fill="currentColor" font-family="monospace">W</text>`,
  FileSheet:     `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M5.5 8.5H10.5M5.5 8.5V13H10.5V8.5M8 8.5V13M5.5 10.5H10.5M5.5 12H10.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,
  FilePresentation: `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="5.5" y="9" width="5" height="3.5" rx=".8" stroke="currentColor" stroke-width="1.1"/><path d="M7.5 11L9 10.5V12L7.5 11Z" fill="currentColor" opacity=".8"/>`,

  // --- Archives & packages ---
  FileArchive:   `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="6.5" y="9" width="4" height="5" rx=".8" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".1"/><path d="M7.5 10.5H9.5M7.5 12.5H9.5" stroke="currentColor" stroke-width="1" stroke-linecap="round"/><path d="M8 9V8.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,
  FileJar:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="6" y="9.5" width="5" height="4.5" rx="1" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".12"/><path d="M6.5 9.5V9C6.5 8.45 6.95 8 7.5 8H9.5C10.05 8 10.5 8.45 10.5 9V9.5" stroke="currentColor" stroke-width="1.1" fill="none"/><path d="M11 11.5H12C12.28 11.5 12.5 11.72 12.5 12V12.5C12.5 12.78 12.28 13 12 13H11" stroke="currentColor" stroke-width="1" fill="none" stroke-linecap="round"/>`,
  Package:       `<path d="M8 2L14 5.5V10.5L8 14L2 10.5V5.5L8 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".1"/><path d="M8 2V14M2 5.5L8 9L14 5.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>`,

  // --- System & tools ---
  Gear:          `<circle cx="8" cy="8" r="2.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M8 1v2M8 13v2M1 8h2M13 8h2M2.9 2.9l1.4 1.4M11.7 11.7l1.4 1.4M2.9 13.1l1.4-1.4M11.7 4.3l1.4-1.4" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>`,
  TerminalWindow:`<rect x="2" y="3" width="12" height="10" rx="1.5" stroke="currentColor" stroke-width="1.4" fill="none"/><path d="M5 7L7 9L5 11M8 11H11" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"/>`,
  Disc:          `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><circle cx="8" cy="8" r="2" stroke="currentColor" stroke-width="1.4" fill="none"/>`,
  Database:      `<ellipse cx="8" cy="4.5" rx="5" ry="2" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".15"/><path d="M3 4.5V8C3 9.1 5.24 10 8 10S13 9.1 13 8V4.5M3 8v3.5C3 12.6 5.24 13.5 8 13.5S13 12.6 13 11.5V8" stroke="currentColor" stroke-width="1.4" fill="none"/>`,

  // --- Special files ---
  FileFont:      `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6.5 13L8 9L9.5 13M7.2 11.5H8.8" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>`,
  FileBook:      `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 9.5C7 8.8 9 8.8 10 9.5V13.5C9 12.8 7 12.8 6 13.5V9.5Z" stroke="currentColor" stroke-width="1.1" fill="none"/><path d="M8 9V13.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,
  FileConfig:    `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><circle cx="8" cy="11.5" r="1.2" stroke="currentColor" stroke-width="1.1"/><path d="M8 9.2V10.3M8 12.7V13.8M5.9 10.5L6.8 11M9.2 12L10.1 12.5M5.9 12.5L6.8 12M9.2 11L10.1 10.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,

  // --- New icons ---
  FileDesign:    `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M8 9L5.5 13H10.5L8 9Z" stroke="currentColor" stroke-width="1.1" stroke-linejoin="round" fill="currentColor" opacity=".15"/><circle cx="8" cy="10.5" r="0.7" fill="currentColor"/>`,
  File3D:        `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M8 9L11 10.5V13L8 14.5L5 13V10.5L8 9Z" stroke="currentColor" stroke-width="1.1" stroke-linejoin="round" fill="currentColor" opacity=".1"/><path d="M8 9V14.5M5 10.5L8 12L11 10.5" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>`,
  FileCert:      `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M8 8.5L10.5 10L8 11.5L5.5 10L8 8.5Z" stroke="currentColor" stroke-width="1.1" stroke-linejoin="round" fill="currentColor" opacity=".2"/><path d="M7 12V14.5M9 12V14.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,
  FileNotebook:  `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="5.5" y="9" width="5" height="5" rx=".8" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".06"/><path d="M6.5 10.5H9.5M6.5 12H8.5" stroke="currentColor" stroke-width="1" stroke-linecap="round"/><circle cx="6" cy="13" r=".5" fill="currentColor"/>`,
  FileDocker:    `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="5" y="9.5" width="2" height="1.5" rx=".3" stroke="currentColor" stroke-width=".9"/><rect x="7.5" y="9.5" width="2" height="1.5" rx=".3" stroke="currentColor" stroke-width=".9"/><rect x="5" y="11.5" width="2" height="1.5" rx=".3" stroke="currentColor" stroke-width=".9"/><rect x="7.5" y="11.5" width="2" height="1.5" rx=".3" stroke="currentColor" stroke-width=".9"/><rect x="10" y="9.5" width="2" height="1.5" rx=".3" stroke="currentColor" stroke-width=".9"/>`,
  FileBuild:     `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 12L8 10L10 12" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/><path d="M6 14L8 12L10 14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>`,
  FileLock:      `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="6" y="11" width="4" height="3" rx=".8" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".15"/><path d="M9 11V10C9 9.17 8.55 8.5 8 8.5S7 9.17 7 10V11" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" fill="none"/>`,
  FileTorrent:   `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M8 9V13M6 11H10" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><path d="M5.5 9.5L8 9L10.5 9.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,
  FileData:      `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 9.5L7.5 11L6 12.5M8.5 9.5H10.5M8.5 12.5H10" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" stroke-linejoin="round"/>`,
  FileWeb:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><circle cx="8" cy="11.5" r="2.5" stroke="currentColor" stroke-width="1.1" fill="none"/><path d="M5.5 11.5H10.5M8 9C7 10 7 13 8 14M8 9C9 10 9 13 8 14" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>`,

  // --- Password vault / encrypted ---
  FileVault:     `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M8 8.5C6.9 8.5 6 9.28 6 10.2V11H10V10.2C10 9.28 9.1 8.5 8 8.5Z" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" fill="none"/><rect x="5.5" y="11" width="5" height="3.2" rx=".8" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".15"/><circle cx="8" cy="12.5" r=".6" fill="currentColor"/>`,

  // --- Private key / SSH ---
  FileKey:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><circle cx="7" cy="10.5" r="1.5" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".1"/><path d="M8.5 10.5H11.5M10 10.5V9.5M11 10.5V9.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,

  // --- GIS / map ---
  FileMap:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M8 9C9.1 9 10 9.9 10 11C10 12.5 8 14 8 14S6 12.5 6 11C6 9.9 6.9 9 8 9Z" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".1"/><circle cx="8" cy="11" r=".7" fill="currentColor"/>`,

  // --- Subtitle ---
  FileSubtitle:  `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="5" y="9.5" width="6" height="4" rx=".8" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".06"/><path d="M6 11H10M6.5 12.5H9.5" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>`,

  // --- Virtual machine ---
  FileVM:        `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="5.5" y="9" width="5" height="3.5" rx=".6" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".08"/><path d="M7 13.5V14.5M9 13.5V14.5M6 14.5H10" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>`,

  // --- Firmware / binary blob ---
  FileChip:      `<rect x="5" y="5" width="6" height="6" rx="1" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M7 5V3M9 5V3M7 11V13M9 11V13M5 7H3M11 7H13M5 9H3M11 9H13" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>`,

  // --- Log/journal ---
  FileLog:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 9.5H7M6 11H10M6 12.5H9M6 14H8" stroke="currentColor" stroke-width="1" stroke-linecap="round"/><circle cx="5.5" cy="9.5" r=".5" fill="currentColor"/>`,

  // --- Backup ---
  FileBackup:    `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 11.5C6 10.12 7.12 9 8.5 9C9.88 9 11 10.12 11 11.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" fill="none"/><path d="M6 11.5L5 10.5M6 11.5L5 12.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/><path d="M11 11.5L12 10.5M11 11.5L12 12.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,

  // --- Language-specific ---

  // JavaScript — "JS" badge
  LangJS:        `<rect x="1" y="1" width="14" height="14" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><text x="8" y="11.5" font-size="7" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">JS</text>`,

  // TypeScript — "TS" badge
  LangTS:        `<rect x="1" y="1" width="14" height="14" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><text x="8" y="11.5" font-size="7" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">TS</text>`,

  // JSON — { } braces
  LangJSON:      `<rect x="1" y="1" width="14" height="14" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M5 4.5C4 4.5 3.5 5 3.5 6V7C3.5 7.5 3 8 2.5 8C3 8 3.5 8.5 3.5 9V10C3.5 11 4 11.5 5 11.5M11 4.5C12 4.5 12.5 5 12.5 6V7C12.5 7.5 13 8 13.5 8C13 8 12.5 8.5 12.5 9V10C12.5 11 12 11.5 11 11.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>`,

  // Rust — crab gear ⚙ with R
  LangRust:      `<circle cx="8" cy="8" r="5.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M8 2V3.5M8 12.5V14M2 8H3.5M12.5 8H14M3.8 3.8L4.8 4.8M11.2 11.2L12.2 12.2M3.8 12.2L4.8 11.2M11.2 4.8L12.2 3.8" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/><text x="8" y="10.5" font-size="6.5" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">R</text>`,

  // Python — two intertwined snakes
  LangPython:    `<path d="M7.5 2C5.5 2 4.5 3 4.5 4.5V6.5H8.5V7.5H3.5C2 7.5 1 8.5 1 10.5C1 12.5 2 13.5 3.5 13.5H5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/><path d="M8.5 14C10.5 14 11.5 13 11.5 11.5V9.5H7.5V8.5H12.5C14 8.5 15 7.5 15 5.5C15 3.5 14 2.5 12.5 2.5H11" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/><circle cx="6" cy="4.5" r=".7" fill="currentColor"/><circle cx="10" cy="11.5" r=".7" fill="currentColor"/>`,

  // Go — "Go" text
  LangGo:        `<rect x="1" y="2.5" width="14" height="11" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="7.5" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">Go</text>`,

  // C — circle with C
  LangC:         `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="8" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">C</text>`,

  // C++ — circle with C+
  LangCpp:       `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".1"/><text x="7" y="11" font-size="7" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">C</text><path d="M11.5 6V10M9.5 8H13.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>`,

  // C# — circle with C#
  LangCSharp:    `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".1"/><text x="6.5" y="11" font-size="7" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">C</text><path d="M10 5.5V10.5M12 5.5V10.5M9 7H13M9 9H13" stroke="currentColor" stroke-width=".9" stroke-linecap="round"/>`,

  // Java — coffee cup
  LangJava:      `<path d="M5 3H11V9C11 11 9.5 12 8 12S5 11 5 9V3Z" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1" stroke-linejoin="round"/><path d="M11 5.5H12.5C13 5.5 13.5 6 13.5 6.5S13 7.5 12.5 7.5H11" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" fill="none"/><path d="M5.5 13H10.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><path d="M7 12V13M9 12V13" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,

  // PHP — <?php badge
  LangPHP:       `<rect x="1" y="3" width="14" height="10" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="10.8" font-size="5.5" font-weight="700" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">PHP</text>`,

  // Ruby — diamond shape
  LangRuby:      `<path d="M8 2L14 7L8 14L2 7L8 2Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="currentColor" opacity=".12"/><path d="M2 7H14M5 2L4 7L8 14M11 2L12 7L8 14" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/>`,

  // Swift — swift bird curve
  LangSwift:     `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M4.5 10.5C7 9 9 6.5 10 4C8 6 5.5 7 3.5 7C5 8.5 7 10 10 11C11.5 11.5 12.5 10.5 12 9" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>`,

  // Kotlin — K shape
  LangKotlin:    `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M5 12V4H7L11 8L7 12H5Z" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round" fill="currentColor" opacity=".15"/>`,

  // Dart — triangle dart shape
  LangDart:      `<path d="M3 11L8 2L13 11H3Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="currentColor" opacity=".1"/><path d="M8 2V14M3 11H13" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,

  // Lua — moon crescent
  LangLua:       `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M9 4C7.5 4.5 6.5 6 6.5 8S7.5 11.5 9 12C11 11 12 9.5 12 8S11 5 9 4Z" fill="currentColor" opacity=".25" stroke="currentColor" stroke-width="1"/><circle cx="5.5" cy="5" r=".8" fill="currentColor"/>`,

  // R — statistics
  LangR:         `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="8" font-weight="800" fill="currentColor" font-family="serif" text-anchor="middle">R</text>`,

  // Haskell — lambda
  LangHaskell:   `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="12" font-size="10" font-weight="400" fill="currentColor" font-family="serif" text-anchor="middle">λ</text>`,

  // Scala — S shape
  LangScala:     `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M5 5.5C5 4 6.5 3.5 8 3.5S11 4 11 5.5C11 7 5 7 5 8.5C5 10 6.5 12.5 8 12.5S11 12 11 10.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>`,

  // Elixir — drop
  LangElixir:    `<path d="M8 2C6 5 4 7.5 4 10C4 12.2 5.8 14 8 14S12 12.2 12 10C12 7.5 10 5 8 2Z" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12" stroke-linejoin="round"/><circle cx="7" cy="10.5" r="1.5" stroke="currentColor" stroke-width="1" fill="currentColor" opacity=".15"/>`,

  // Zig — Z
  LangZig:       `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M4.5 5H11.5L4.5 11H11.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>`,

  // Nim — crown
  LangNim:       `<path d="M3 12V5L5.5 8L8 4L10.5 8L13 5V12H3Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="currentColor" opacity=".12"/><circle cx="5.5" cy="5" r=".6" fill="currentColor"/><circle cx="8" cy="4" r=".6" fill="currentColor"/><circle cx="10.5" cy="5" r=".6" fill="currentColor"/>`,

  // Vue — V shape
  LangVue:       `<path d="M1 2H5L8 7.5L11 2H15L8 14L1 2Z" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round" fill="currentColor" opacity=".1"/><path d="M5 2L8 7.5L11 2" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round" fill="none"/>`,

  // Svelte — S flame
  LangSvelte:    `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M5.5 9.5C5.5 7.5 10.5 7.5 10.5 5.5C10.5 4 9 3.5 7.5 4M10.5 6.5C10.5 8.5 5.5 8.5 5.5 10.5C5.5 12 7 12.5 8.5 12" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" fill="none"/>`,

  // React — atom
  LangReact:     `<circle cx="8" cy="8" r="1.2" fill="currentColor"/><ellipse cx="8" cy="8" rx="6" ry="2.5" stroke="currentColor" stroke-width="1.1" fill="none"/><ellipse cx="8" cy="8" rx="6" ry="2.5" stroke="currentColor" stroke-width="1.1" fill="none" transform="rotate(60 8 8)"/><ellipse cx="8" cy="8" rx="6" ry="2.5" stroke="currentColor" stroke-width="1.1" fill="none" transform="rotate(120 8 8)"/>`,

  // Angular — A shape / shield
  LangAngular:   `<path d="M8 1L14 3.5L13 12L8 15L3 12L2 3.5L8 1Z" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round" fill="currentColor" opacity=".1"/><path d="M8 4L5 11H6.5L7.2 9H8.8L9.5 11H11L8 4Z" stroke="currentColor" stroke-width=".9" fill="currentColor" opacity=".2" stroke-linejoin="round"/><path d="M7.5 7.5H8.5" stroke="currentColor" stroke-width=".9" stroke-linecap="round"/>`,

  // CSS — style bracket
  LangCSS:       `<rect x="1" y="1" width="14" height="14" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M6 5C4.5 5 4 6 4 7S4.5 8 4 8.5S3 9 3 8C3 8 3 8.5 3.5 8.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" fill="none"/><text x="9" y="11.5" font-size="7.5" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">#</text>`,

  // HTML — angle brackets
  LangHTML:      `<rect x="1" y="1" width="14" height="14" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M4 5L2.5 8L4 11M12 5L13.5 8L12 11M9 4L7 12" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" fill="none"/>`,

  // Perl — camel hump / P
  LangPerl:      `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11.5" font-size="8.5" font-weight="800" fill="currentColor" font-family="serif" text-anchor="middle">P</text>`,

  // Erlang — Erl
  LangErlang:    `<rect x="1" y="2.5" width="14" height="11" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="6" font-weight="700" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">Erl</text>`,

  // OCaml — camel
  LangOCaml:     `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="6" font-weight="700" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">ML</text>`,

  // Clojure — parentheses
  LangClojure:   `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M6 4C4 5.5 4 10.5 6 12M10 4C12 5.5 12 10.5 10 12" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" fill="none"/>`,

  // Julia — three colored dots
  LangJulia:     `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/><circle cx="5.5" cy="9.5" r="1.5" fill="currentColor" opacity=".6"/><circle cx="10.5" cy="9.5" r="1.5" fill="currentColor" opacity=".4"/><circle cx="8" cy="5.5" r="1.5" fill="currentColor" opacity=".8"/>`,

  // D lang
  LangD:         `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="8" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">D</text>`,

  // V lang
  LangV:         `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><text x="8" y="11.5" font-size="9" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">V</text>`,

  // Crystal
  LangCrystal:   `<path d="M8 2L13 5.5V10.5L8 14L3 10.5V5.5L8 2Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="currentColor" opacity=".12"/><path d="M8 2L13 10.5H3L8 2Z" stroke="currentColor" stroke-width="1" stroke-linejoin="round" fill="currentColor" opacity=".1"/>`,

  // Groovy/Gradle — star
  LangGroovy:    `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M8 3L9.2 6.2L12.5 6.5L10 8.8L10.8 12L8 10.2L5.2 12L6 8.8L3.5 6.5L6.8 6.2L8 3Z" stroke="currentColor" stroke-width="1" stroke-linejoin="round" fill="currentColor" opacity=".2"/>`,

  // Objective-C
  LangObjC:      `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".1"/><text x="8" y="10.5" font-size="5.5" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">OC</text>`,

  // Fortran — F77
  LangFortran:   `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="8" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">F</text>`,

  // Ada
  LangAda:       `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="6" font-weight="700" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">Ada</text>`,

  // Assembly — hex/binary
  LangAsm:       `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="5.5" font-weight="700" fill="currentColor" font-family="monospace" text-anchor="middle">ASM</text>`,

  // VHDL / Verilog — chip
  LangHDL:       `<rect x="3" y="4" width="10" height="8" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M5 4V2.5M7 4V2.5M9 4V2.5M11 4V2.5M5 12V13.5M7 12V13.5M9 12V13.5M11 12V13.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><path d="M6 7H10M6 9H9" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>`,

  // Solidity — diamond / ethereum
  LangSolidity:  `<path d="M8 1L13 8L8 15L3 8L8 1Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="currentColor" opacity=".1"/><path d="M3 8H13M8 1V8" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>`,

  // WASM — hexagon
  LangWASM:      `<path d="M4 3H12L15 8L12 13H4L1 8L4 3Z" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round" fill="currentColor" opacity=".12"/><text x="8" y="10.5" font-size="4.5" font-weight="700" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">WA</text>`,

  // GDScript — game controller
  LangGDScript:  `<path d="M4 5H12C13.1 5 14 5.9 14 7V10C14 11.1 13.1 12 12 12H10L9 14H7L6 12H4C2.9 12 2 11.1 2 10V7C2 5.9 2.9 5 4 5Z" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1" stroke-linejoin="round"/><circle cx="5.5" cy="8.5" r=".8" fill="currentColor"/><circle cx="10.5" cy="8.5" r=".8" fill="currentColor"/><path d="M4 8.5H7M5.5 7V10" stroke="currentColor" stroke-width=".9" stroke-linecap="round"/>`,

  // Bash / Shell specific
  LangBash:      `<rect x="1.5" y="2.5" width="13" height="11" rx="2" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M4.5 6L7 8.5L4.5 11" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" fill="none"/><path d="M8 11H11.5" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><text x="12" y="6.5" font-size="3" font-weight="700" fill="currentColor" font-family="monospace">$</text>`,

  // SQL
  LangSQL:       `<ellipse cx="8" cy="4" rx="5.5" ry="2.2" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M2.5 4V12C2.5 13.2 5 14.2 8 14.2S13.5 13.2 13.5 12V4" stroke="currentColor" stroke-width="1.3" fill="none"/><path d="M2.5 8C2.5 9.2 5 10.2 8 10.2S13.5 9.2 13.5 8" stroke="currentColor" stroke-width="1.1" fill="none"/>`,

  // YAML
  LangYAML:      `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M4 5L6 8V11M12 5L10 8V11M8 8V11" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" fill="none"/>`,

  // TOML
  LangTOML:      `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M4 5H8M6 5V12" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><circle cx="11" cy="9" r="2.5" stroke="currentColor" stroke-width="1.2" fill="none"/>`,

  // XML
  LangXML:       `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M4 5.5L6.5 8L4 10.5M12 5.5L9.5 8L12 10.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" fill="none"/><path d="M7 11.5L9 5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round"/>`,

  // Nix — snowflake
  LangNix:       `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/><path d="M8 2.5V13.5M3 5.5L13 10.5M13 5.5L3 10.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><circle cx="8" cy="8" r="1" fill="currentColor"/>`,

  // Terraform — T
  LangTerraform: `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><path d="M5 4.5H11M8 4.5V12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/><path d="M5.5 7H10.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>`,

  // Env file — dotenv
  LangEnv:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 10H7.5M9 10H10M6 12H8M9.5 12H10" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><circle cx="5" cy="10" r=".5" fill="currentColor"/><circle cx="5" cy="12" r=".5" fill="currentColor"/>`,

  // GraphQL — hexagon with G
  LangGraphQL:   `<path d="M4 3H12L15 8L12 13H4L1 8L4 3Z" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round" fill="currentColor" opacity=".1"/><circle cx="4" cy="3" r=".8" fill="currentColor"/><circle cx="12" cy="3" r=".8" fill="currentColor"/><circle cx="15" cy="8" r=".8" fill="currentColor"/><circle cx="12" cy="13" r=".8" fill="currentColor"/><circle cx="4" cy="13" r=".8" fill="currentColor"/><circle cx="1" cy="8" r=".8" fill="currentColor"/>`,

  // Proto — protobuf
  LangProto:     `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M5 5H11M5 8H9M5 11H10" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><circle cx="4" cy="5" r=".5" fill="currentColor"/><circle cx="4" cy="8" r=".5" fill="currentColor"/><circle cx="4" cy="11" r=".5" fill="currentColor"/>`,

  // Raku (Perl 6) — butterfly
  LangRaku:      `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/><path d="M8 4V12M5 5C3.5 6 3.5 10 5 11M11 5C12.5 6 12.5 10 11 11" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" fill="none"/><circle cx="8" cy="8" r="1" fill="currentColor"/>`,

  // Cobol
  LangCobol:     `<rect x="1" y="2.5" width="14" height="11" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="10.8" font-size="4.5" font-weight="700" fill="currentColor" font-family="monospace" text-anchor="middle">COB</text>`,

  // Pascal / Delphi
  LangPascal:    `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11.5" font-size="8" font-weight="800" fill="currentColor" font-family="serif" text-anchor="middle">P</text><path d="M5 12H11" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>`,

  // Mojo — fire
  LangMojo:      `<path d="M8 2C7 4 5 5 5 8C5 10.5 6.5 12.5 8 13C9.5 12.5 11 10.5 11 8C11 5 9 4 8 2Z" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".15" stroke-linejoin="round"/><path d="M8 7C7.5 8 7 8.5 7 9.5C7 10.5 7.5 11 8 11.2C8.5 11 9 10.5 9 9.5C9 8.5 8.5 8 8 7Z" fill="currentColor" opacity=".4"/>`,

  // Gleam — sparkle
  LangGleam:     `<path d="M8 1L9.5 6.5L15 8L9.5 9.5L8 15L6.5 9.5L1 8L6.5 6.5L8 1Z" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round" fill="currentColor" opacity=".15"/>`,

  // Odin
  LangOdin:      `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="8" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">O</text>`,

  // CUDA
  LangCUDA:      `<rect x="3" y="3" width="10" height="10" rx="1.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M6 6V10M8 7V10M10 5V10" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M5 12H11" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>`,

  // Shader (GLSL/HLSL)
  LangShader:    `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M4 4L8 8L4 12M12 4L8 8L12 12" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round" fill="none"/><circle cx="8" cy="8" r="1.5" stroke="currentColor" stroke-width="1" fill="currentColor" opacity=".3"/>`,

  // Lisp / Scheme — nested parens
  LangLisp:      `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".08"/><path d="M5 3.5C3 5 3 11 5 12.5M7 4.5C5.5 6 5.5 10 7 11.5M11 3.5C13 5 13 11 11 12.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" fill="none"/>`,

  // Tcl/Tk
  LangTcl:       `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="6" font-weight="700" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">Tcl</text>`,

  // Prolog — ?
  LangProlog:    `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11.5" font-size="9" font-weight="700" fill="currentColor" font-family="serif" text-anchor="middle">?</text>`,

  // --- Additional language icons ---

  // F# — F sharp
  LangFSharp:    `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><text x="8" y="11.5" font-size="7.5" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">F#</text>`,

  // VB.NET — VB
  LangVB:        `<rect x="1" y="2.5" width="14" height="11" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="10.8" font-size="6" font-weight="700" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">VB</text>`,

  // Idris — dependent type lambda
  LangIdris:     `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="12" font-size="10" font-weight="400" fill="currentColor" font-family="serif" text-anchor="middle">Σ</text>`,

  // Lean — L
  LangLean:      `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M5 4.5V11.5H11" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>`,

  // Agda — A with overline
  LangAgda:      `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><path d="M5 12L8 4.5L11 12M6 10H10" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" fill="none"/><path d="M5 3.5H11" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>`,

  // Coq — rooster comb / Cq
  LangCoq:       `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="6" font-weight="700" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">Coq</text>`,

  // Forth — 4th
  LangForth:     `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="6" font-weight="700" fill="currentColor" font-family="monospace" text-anchor="middle">4th</text>`,

  // Red — R in circle
  LangRed:       `<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".12"/><text x="8" y="11" font-size="8" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">R</text><circle cx="11" cy="5" r="1" fill="currentColor"/>`,

  // Pony — P
  LangPony:      `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".12"/><text x="8" y="11.5" font-size="8" font-weight="800" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">P</text><path d="M5 3H11" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>`,

  // Chapel — Ch
  LangChapel:    `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="6.5" font-weight="700" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">Ch</text>`,

  // Factor — f
  LangFactor:    `<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11.5" font-size="9" font-weight="700" fill="currentColor" font-family="serif" text-anchor="middle">f</text>`,

  // AWK
  LangAwk:       `<rect x="1.5" y="2.5" width="13" height="11" rx="2" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="10.8" font-size="5.5" font-weight="700" fill="currentColor" font-family="monospace" text-anchor="middle">awk</text>`,

  // AppleScript — AS
  LangAppleScript:`<rect x="1.5" y="1.5" width="13" height="13" rx="2.5" stroke="currentColor" stroke-width="1.3" fill="currentColor" opacity=".1"/><text x="8" y="11" font-size="6.5" font-weight="700" fill="currentColor" font-family="system-ui,sans-serif" text-anchor="middle">AS</text>`,

  // --- Additional file type icons ---

  // ML model / weights — brain/neural
  FileML:        `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><circle cx="6.5" cy="10" r="1" stroke="currentColor" stroke-width=".9" fill="currentColor" opacity=".15"/><circle cx="9.5" cy="10" r="1" stroke="currentColor" stroke-width=".9" fill="currentColor" opacity=".15"/><circle cx="8" cy="13" r="1" stroke="currentColor" stroke-width=".9" fill="currentColor" opacity=".15"/><path d="M6.5 10L8 13M9.5 10L8 13M6.5 10H9.5" stroke="currentColor" stroke-width=".8" stroke-linecap="round"/>`,

  // Git file — branch icon
  FileGit:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><circle cx="7" cy="10" r="1" stroke="currentColor" stroke-width="1" fill="currentColor" opacity=".15"/><circle cx="10" cy="13" r="1" stroke="currentColor" stroke-width="1" fill="currentColor" opacity=".15"/><path d="M7 11V14M8 12.5C8 11.5 9 11 10 12" stroke="currentColor" stroke-width="1" stroke-linecap="round" fill="none"/>`,

  // License — scale/balance
  FileLicense:   `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M8 9V13.5M5.5 10.5L8 9L10.5 10.5" stroke="currentColor" stroke-width="1.1" stroke-linecap="round" stroke-linejoin="round" fill="none"/><path d="M5 10.5L4.5 12H6.5L6 10.5M10 10.5L9.5 12H11.5L11 10.5" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" fill="none"/>`,

  // Changelog — list with arrow
  FileChangelog: `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M7 10H10.5M7 12H9.5M7 14H10" stroke="currentColor" stroke-width="1" stroke-linecap="round"/><circle cx="5.5" cy="10" r=".5" fill="currentColor"/><circle cx="5.5" cy="12" r=".5" fill="currentColor"/><circle cx="5.5" cy="14" r=".5" fill="currentColor"/>`,

  // Readme — open book
  FileReadme:    `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M5.5 9C6.5 8.5 7.5 8.5 8 9V14C7.5 13.5 6.5 13.5 5.5 14V9Z" stroke="currentColor" stroke-width="1" fill="currentColor" opacity=".1"/><path d="M10.5 9C9.5 8.5 8.5 8.5 8 9V14C8.5 13.5 9.5 13.5 10.5 14V9Z" stroke="currentColor" stroke-width="1" fill="currentColor" opacity=".1"/>`,

  // Env file — key=value
  FileEnv:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 10H7.5M9 10H10M6 12H8M9.5 12H10" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><circle cx="5" cy="10" r=".5" fill="currentColor"/><circle cx="5" cy="12" r=".5" fill="currentColor"/>`,

  // ISO / disc image
  FileISO:       `<circle cx="8" cy="8" r="6.5" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><circle cx="8" cy="8" r="2" stroke="currentColor" stroke-width="1.3" fill="none"/><circle cx="8" cy="8" r="4.5" stroke="currentColor" stroke-width=".7" fill="none" opacity=".4"/><circle cx="8" cy="8" r=".6" fill="currentColor"/>`,

  // Binary file
  FileBinary:    `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><text x="8" y="12.5" font-size="4.5" font-weight="700" fill="currentColor" font-family="monospace" text-anchor="middle">0101</text>`,

  // DLL / shared library
  FileLib:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><rect x="5.5" y="9" width="5" height="4.5" rx=".8" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".1"/><path d="M8 9V13.5M5.5 11H10.5" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>`,

  // .desktop / application shortcut
  FileApp:       `<rect x="2" y="2" width="12" height="12" rx="3" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".12"/><path d="M6 5.5V10.5L11 8L6 5.5Z" fill="currentColor" opacity=".5" stroke="currentColor" stroke-width="1" stroke-linejoin="round"/>`,

  // Source map
  FileSourceMap: `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 10C7 9 9 9 10 10C9 11 7 11 6 10Z" stroke="currentColor" stroke-width="1.1" fill="currentColor" opacity=".15"/><path d="M5.5 12.5L8 11L10.5 12.5" stroke="currentColor" stroke-width="1" stroke-linecap="round" fill="none"/>`,

  // Diff / patch
  FileDiff:      `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><path d="M6 10H8M7 9V11" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/><path d="M9 12.5H11" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>`,

  // LaTeX / TeX
  FileTex:       `<path d="M4 2H10L14 6V14C14 14.55 13.55 15 13 15H4C3.45 15 3 14.55 3 14V3C3 2.45 3.45 2 4 2Z" stroke="currentColor" stroke-width="1.4" fill="currentColor" opacity=".08"/><path d="M9.5 2V6.5H14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/><text x="8.5" y="13.5" font-size="5.5" font-weight="700" fill="currentColor" font-family="serif" text-anchor="middle">TeX</text>`,
};

export function renderIcon(iconName: string): string {
  return svgPaths[iconName] ?? svgPaths["File"];
}
