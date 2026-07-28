# Anime Cursor — Product Requirements Document

**Type:** Desktop App
**Stack:** Tauri v2 + React + Rust
**Core:** Desktop application for browsing, importing, and applying anime-style animated cursor (.ani) packs on Windows and Linux.

---

## 1. Problem Statement

- Changing system cursors on Windows/Linux requires deep Control Panel/registry navigation — not intuitive.
- .ani cursor packs are scattered across websites (Cursors-4U, DeviantArt) but no dedicated manager exists.
- No cross-platform tool that handles both Windows .ani and Linux Xcursor conversion in one UI.
- Backup/restore is absent from OS defaults — once you change it, reverting is manual.

---

## 2. Product Overview

### 2.1 Vision
A cross-platform desktop application that lets users browse, import, and apply anime-style animated cursor (.ani) packs — making it dead simple to customize their system cursor with anime aesthetics.

### 2.2 Target Users
- **Primary:** Anime fans who want animated anime cursors on their desktop
- **Secondary:** Power users who collect .ani cursor packs from sites like Cursors-4U
- **Tertiary:** Anyone who wants easy cursor management without digging into Control Panel/registry

### 2.3 Value Proposition
- One-click apply — no more manual Control Panel/registry edits
- Pack-based management — import ZIPs with multiple cursor variants
- Auto OS detection — no need to pick Windows vs Linux
- Safe restore — backup and restore system default cursor
- Open source, free, privacy-respecting (fully offline)

### 2.4 User Stories
- **US-01 First-time user:** As an anime fan, I want to install anime cursors easily so I can customize my desktop without messing with Control Panel. **Acceptance:** Open app → see empty dashboard → click "Import Your First Pack" → drag ZIP → see pack in grid → click → select variant → Apply → cursor changes immediately.
- **US-02 Power user:** As a collector, I want to manage multiple cursor packs so I can switch between them for different moods. **Acceptance:** Have 10+ packs in grid. Click each to preview variants. Apply any variant. Remove old packs. All operations complete in < 3 clicks.
- **US-03 Linux user:** As a Linux user, I want to use anime cursors even though .ani is a Windows format — without terminal commands. **Acceptance:** Same drag-and-drop import. App auto-converts .ani frames to Xcursor Linux format. Cursor applies after clicking "Apply".
- **US-04 Safety-concerned user:** As a cautious user, I want to restore my original cursor if I don't like the anime one. **Acceptance:** Open Settings → click "Restore Default Cursor" → cursor reverts to OS default → toast confirms success.

---

## 3. Goals

### Primary (v1)
- [ ] Pack import via drag-drop & file picker (ZIP + individual .ani)
- [ ] Dashboard grid browsing with search/sort
- [ ] One-click apply cursor with auto OS detection
- [ ] Automatic cursor backup before every apply
- [ ] Restore default cursor from Settings

### Secondary
- [ ] Language toggle (EN / ID / CN / JP)
- [ ] Light / Dark theme toggle
- [ ] Collapsible sidebar navigation
- [ ] Toast notifications for success/error states
- [ ] Settings screen with GitHub, Website, Donate links

---

## 4. Functional Requirements

### 4.1 Pack Import (P0)
| ID | Requirement | Notes |
|----|-------------|-------|
| F1 | Drag & drop ZIP file to import | Accept `.zip` only |
| F2 | Click to browse file picker | Native OS file dialog |
| F3 | Auto-extract ZIP to `~/.anime-cursor/packs/` | Create dir if not exist |
| F4 | Read `metadata.json` if present | Parse author, name, cursors, thumbnail |
| F5 | Fallback if no metadata.json | Use filename as pack name, generate thumbnail from first .ani frame |
| F6 | Show warning toast for missing metadata | Non-blocking |
| F7 | Support individual `.ani` file import | Auto-wrap into single-cursor pack |
| F8 | Prevent duplicate pack import | Compare by name hash |
| F9 | Progress bar during ZIP extraction | For large packs |

### 4.2 Dashboard (P0)
| ID | Requirement | Notes |
|----|-------------|-------|
| F10 | Grid display of all installed packs | Responsive auto-fill grid |
| F11 | Each card shows: thumbnail, pack name, OS badges | |
| F12 | Empty state with "Import Your First Pack" | CTA links to Import page |
| F13 | Click card → navigate to Pack Detail | |
| F14 | Sort packs by: recently added, name A-Z | Default: recently added |

### 4.3 Pack Detail (P0)
| ID | Requirement | Notes |
|----|-------------|-------|
| F15 | Show pack header: name, author, source URL | |
| F16 | Show all cursor variants in the pack | |
| F17 | Each variant shows: name, size label, preview thumbnail | |
| F18 | "Apply" button per variant | |
| F19 | "Remove Pack" button with confirmation | |
| F20 | Show current active variant indicator | |

### 4.4 Apply Cursor (P0)
| ID | Requirement | Notes |
|----|-------------|-------|
| F21 | Auto-detect OS (Windows vs Linux) | |
| F22 | User only selects variant — not OS | |
| F23 | Windows: copy .ani to app data, set registry (HKCU), broadcast WM_SETTINGCHANGE | |
| F24 | Linux: parse .ani → Xcursor format, set via gsettings | |
| F25 | Backup current cursor before applying | Store in `~/.anime-cursor/backups/` |
| F26 | Toast "Applied!" on success | |
| F27 | Toast error message on failure with details | |
| F28 | Restore Default Cursor button in Settings | Rollback to OS default |

### 4.5 Settings (P0)
| ID | Requirement | Notes |
|----|-------------|-------|
| F29 | Language toggle: EN / ID / CN / JP | Default: EN |
| F30 | Theme toggle: Dark / Light | Default: Dark |
| F31 | GitHub link → `github.com/curzyori/anime-cursor` | Opens browser |
| F32 | Website link → `anime-cursor.curzy.dev` | Opens browser |
| F33 | Support/Donate link → `donate.curzy.dev` | Opens browser |
| F34 | Restore Default Cursor button | |
| F35 | Pack location (read-only path display) | Show `~/.anime-cursor/packs/` |
| F36 | App version display | v1.0.0 |

### 4.6 Sidebar Navigation (P1)
| ID | Requirement | Notes |
|----|-------------|-------|
| F37 | Collapsible sidebar with icon + label | |
| F38 | Nav items: Home, Import, Settings | |
| F39 | Active route highlight with pink accent | |
| F40 | Collapsed state: icons only (56px) | |

---

## 5. Non-Functional Requirements

### 5.1 Performance
| ID | Requirement |
|----|-------------|
| NF1 | App startup < 2 seconds |
| NF2 | Import: extract 10MB ZIP < 3 seconds |
| NF3 | Apply cursor: < 500ms |
| NF4 | Dashboard load: < 500ms with 20+ packs |
| NF5 | Memory usage: < 150MB idle, < 300MB with large pack loaded |

### 5.2 Security & Privacy
| ID | Requirement |
|----|-------------|
| NF6 | Fully offline — no network requests |
| NF7 | No telemetry, analytics, or tracking |
| NF8 | No admin/root privileges required |
| NF9 | Cursor backup before every apply — always recoverable |

### 5.3 Compatibility
| ID | Requirement |
|----|-------------|
| NF10 | Windows 10+ (x64) |
| NF11 | Linux — GNOME, KDE Plasma, XFCE (X11) |
| NF12 | Wayland (v1): GNOME Wayland via `gsettings`. KDE Wayland — partial (manual revert may be needed). XFCE Wayland not targeted. |
| NF13 | .ani format: RIFF animated cursor |
| NF14 | ZIP format: standard DEFLATE compression |

### 5.4 System Dependencies
| Platform | Required |
|----------|----------|
| Windows | None (Win32 API built-in) |
| Linux (Debian/Ubuntu) | `x11-xserver-utils` (provides `xcursorgen`), `gsettings` (built-in GNOME) |
| Linux (Arch) | `xcursorgen` package |
| Linux (KDE) | `kwriteconfig5` (part of `kf5-config`) |

### 5.5 Reliability
| ID | Requirement |
|----|-------------|
| NF15 | Graceful handling of corrupt ZIP files |
| NF16 | Graceful handling of malformed .ani files |
| NF17 | Cannot leave system with broken cursor — always restorable |
| NF18 | Error messages in user's selected language |

---

## 6. Screens & Settings Requirements

Layout AI-driven — tidak mendikte tata letak. Berikut daftar screen wajib + state fungsional per screen.

### 6.1 Dashboard (Home)
- **Fungsi:** Grid auto-fill semua pack ter-install, sort (recently added / name A-Z), CTA empty state.
- **Data:** tiap card = thumbnail + pack name + author + OS badges (Windows/Linux).
- **State:** Normal (grid loaded) · Empty (no packs + "Import Your First Pack" CTA) · Loading (skeleton saat fetch).

### 6.2 Import
- **Fungsi:** Drag-drop zone (.zip + .ani) + file picker, progress bar ekstraksi, toast hasil.
- **Data:** progress % ekstraksi, nama pack, jumlah cursor/variant.
- **State:** Normal (drop zone idle) · Loading (progress bar) · Success (toast + "Import another") · Error (toast corrupt).

### 6.3 Pack Detail
- **Fungsi:** Header pack (nama, author, source URL), list variant (thumbnail + nama + size label + Apply), Remove Pack (confirm modal), active variant indicator.
- **Data:** variant list, current active variant ID.
- **State:** Normal (list loaded, hover = preview) · Empty (pack kosong) · Loading.

### 6.4 Settings
- **Fungsi:** General (Language EN/ID/CN/JP, Theme Dark/Light), Cursor (Restore Default + read-only pack path), Links (GitHub/Website/Donate), About (version).
- **Data:** config.json (lang, theme), version string.
- **State:** Normal · Loading (save pref) · Confirm modal (restore).

---

## 7. User Journey Flows

### 7.1 First-run (empty state → cursor applied)
```
Open app → Dashboard empty state → click "Import Your First Pack"
→ File picker opens (or drag ZIP onto import zone)
→ Progress bar → toast "Pack imported!"
→ Back to Dashboard → pack card visible → click card
→ Pack Detail → variant list → click "Apply" on a variant
→ Toast "Applied!" → cursor changes immediately
```
**Total user clicks:** 5-6. **Time from open:** ~30s (assuming ZIP on desktop).

### 7.2 Returning user (apply different cursor)
```
Open app → Dashboard grid shows packs → click a pack card
→ Pack Detail → see "ACTIVE" badge on current variant
→ Scroll to another variant → click "Apply"
→ Toast "Applied!" → cursor switches
```

### 7.3 Power user (switch between packs)
```
Open app → Dashboard → click Pack A → Apply variant
→ Back → click Pack B → Apply variant
→ Back → click Pack C → Apply variant
```
Each switch: 3 clicks, < 3s.

### 7.4 Safety flow (restore default)
```
Settings → "Restore Default Cursor" → modal confirmation "Revert to OS default?"
→ Confirm → cursor restored → toast
```

---

## 8. Technical Architecture

### 8.1 Stack
- **Desktop Framework:** Tauri v2 (Rust + webview)
- **Frontend:** React 18+ with TypeScript, Tailwind CSS
- **State Management:** Zustand (app state: theme, lang, sidebar) + React Query (async Tauri commands)
- **Routing:** React Router v6
- **Rust Crates:** `serde`, `serde_json`, `zip`, `tauri`, `regex`, `dirs`, `winreg`, `windows-sys`, `xcursor`
- **Build Tool:** Vite

### 8.2 Directory Structure
```
anime-cursor/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs      # Tauri entry, commands
│   │   ├── parser.rs    # .ANI RIFF parser
│   │   ├── applier.rs   # Cursor apply/restore (Win + Linux)
│   │   ├── pack.rs      # ZIP extract, metadata read
│   │   └── backup.rs    # Cursor backup management
│   ├── icons/           # App icons per platform
│   └── Cargo.toml
├── src/                 # React frontend
│   ├── App.tsx
│   ├── main.tsx
│   ├── pages/
│   │   ├── Dashboard.tsx
│   │   ├── Import.tsx
│   │   ├── PackDetail.tsx
│   │   └── Settings.tsx
│   ├── components/
│   │   ├── Sidebar.tsx
│   │   ├── PackCard.tsx
│   │   ├── ImportZone.tsx
│   │   ├── Toast.tsx
│   │   ├── Badge.tsx
│   │   ├── VariantItem.tsx
│   │   ├── PackHeader.tsx
│   │   ├── EmptyState.tsx
│   │   ├── Modal.tsx
│   │   ├── ProgressBar.tsx
│   │   ├── Dropdown.tsx
│   │   └── LinkRow.tsx
│   ├── i18n/
│   │   ├── en.json
│   │   ├── id.json
│   │   ├── zh.json
│   │   └── ja.json
│   │
│   │   i18n key structure: flat dot-notation keys with `{placeholder}` interpolation.
│   │   Example: `"pack.apply": "Apply"`, `"toast.import.success": "Pack \"{name}\" imported!"`.
│   │   All UI text uses `t("key")` wrapper — no hardcoded strings in components except numbers/dates.
│   │
│   ├── hooks/
│   │   └── usePacks.ts
│   ├── styles/
│   │   ├── globals.css
│   │   └── themes.css
│   └── types/
│       └── index.ts
├── public/
│   ├── logo.svg
│   └── icons/
├── package.json
├── tailwind.config.js
├── tsconfig.json
└── tauri.conf.json
```

### 8.3 Data Flow
```
User drops ZIP
  → Tauri drag-drop event fires
  → Rust: validate ZIP, extract to ~/.anime-cursor/packs/<id>/
  → Rust: parse metadata.json (or fallback)
  → Rust: return Pack struct to frontend via Tauri command
  → React: update dashboard grid (optimistic)

User clicks Apply
  → React: send variant selection to Rust
  → Rust: backup current cursor
  → Rust: detect OS → call platform-specific applier
  → Rust: return success/failure
  → React: show toast
```

### 8.4 Data Storage
```
~/.anime-cursor/
├── packs/
│   └── <pack-id>/
│       ├── cursor.ani
│       ├── cursor_48.ani
│       ├── preview.png
│       └── metadata.json
├── backups/
│   └── <timestamp>/
│       ├── (registry backup or cursor theme backup)
└── config.json    # app preferences (lang, theme)
```

### 8.5 Backend Logic

#### 8.5.1 .ani Parser (`parser.rs`)

.ani is a RIFF container format (same family as .wav, .avi):

```
RIFF ('ACON' chunk)
├── LIST ('INFO')         — metadata (name, artist, comments)
├── anih (26 bytes)       — animation header:
│   ├── cbSize            — size of this struct (usually 36)
│   ├── frames            — total frame count (1–100+)
│   ├── steps             — frames to show in a cycle
│   ├── cx/cy             — cursor hotspot coordinates
│   ├── bitCount          — color depth (usually 32 for RGBA)
│   └── ...flags, display rate
├── LIST ('fram')         — frame data
│   ├── icon - icon data per frame (DIB bitmap)
│   ├── icon - icon data per frame
│   └── ...
└── LIST ('seq ')         — optional sequence override
```

**Parse strategy:**
1. Validate RIFF header + "ACON" magic — reject if invalid (corrupt file)
2. Read `anih` chunk → get frame count, hotspot (cx/cy), display rate
3. Iterate `fram` LIST → extract each frame as RGBA icon data
4. If `seq ` LIST present → apply sequence order; else assume sequential
5. Return `AniCursor { frames: Vec<Frame>, hotspot: (u32,u32), rate: u32 }`

**Error handling:**
- Missing RIFF header → skip variant, toast warning
- Truncated frame data → log, continue with partial frames
- Zero frames → skip variant entirely

#### 8.5.2 Apply Flow (`applier.rs`)

**Windows:**
1. Copy selected `.ani` file to `%APPDATA%\anime-cursor\active\`
2. Open `HKEY_CURRENT_USER\Control Panel\Cursors`
3. Write registry values for each system cursor slot (Arrow, Wait, Help, etc.) pointing to the copied .ani
4. Call `SystemParametersInfoW(SPI_SETCURSORS, 0, 0, SPIF_SENDCHANGE)` — broadcasts WM_SETTINGCHANGE
5. OS picks up change immediately (no reboot needed)

**Linux (X11):**
1. Parse .ani frames via `parser.rs` → get frame bitmaps + hotspot
2. Convert each frame to Xcursor `.xcur` format:
   - Write Xcursor header + TOC
   - For each frame: write RGBA pixels as a chunk
3. Convert in-process via `xcursor` crate to compile the Xcursor theme
4. Write theme to `~/.icons/anime-cursor/cursors/`
5. Run `gsettings set org.gnome.desktop.interface cursor-theme 'anime-cursor'`
6. KDE: set via `kwriteconfig5 --file ~/.config/kcminputrc --group Mouse --key cursorTheme 'anime-cursor'`

#### 8.5.3 Backup / Restore (`backup.rs`)

**Pre-apply backup (automatic):**
- Windows: export `HKCU\Control Panel\Cursors` to `~/.anime-cursor/backups/<timestamp>/cursors.reg`
- Linux: copy current cursor theme name from gsettings + backup `~/.icons/<current-theme>/cursors/` directory

**Restore:**
- Windows: import `cursors.reg` via `reg.exe import`, then broadcast SPI_SETCURSORS
- Linux: restore theme directory from backup, re-run gsettings or kwriteconfig5

**Safety guarantees:**
- Backup always runs BEFORE apply — if backup fails, apply is aborted (NF16)
- At least 1 backup retained per session; old backups auto-clean after 30 days
- Restore from any timestamp available in Settings

### 8.6 Default Reference Packs

Project ships with sample `.ani` packs di `.example/ani/` sebagai referensi
format dan struktur pack. Semua bersumber dari Cursors-4U — bisa dipakai
sebagai test data untuk parser dan fallback default jika user belum import.

| Pack | Author | Variants |
|------|--------|----------|
| Madoka Magica (Miki Sayaka) | Unknown | 32-48-64, 72-96-128, 256 |
| Minji - NewJeans Powerpuff Girls | josuegrotesco | 32-48-64, 72-96-128, 256 |
| Danielle - NewJeans Powerpuff Girls | josuegrotesco | 32-48-64, 72-96-128, 256 |
| Haerin - NewJeans Powerpuff Girls | josuegrotesco | 32-48-64, 72-96-128, 256 |
| My Custom Cursor 8 | josuegrotesco | 32-48-64, 72-96-128, 256 |
| Slapping Cat | Unknown | 32-48-64, 72-96-128, 200 |
| Wait (Hatsune Miku Chibi) | supermariofps | 32-48-64, 96 |

Setiap pack berisi `readme.txt` + `.ani` file multi-size. ZIP original juga
disertakan di `.example/zip/` untuk test import flow.

---

## 9. Error Handling

| Scenario | Handling |
|----------|----------|
| Corrupt ZIP file (invalid header) | Show error toast: "Invalid or corrupt .zip file. No files extracted." |
| Malformed .ani (bad RIFF header) | Skip variant, show warning: "Skipped malformed cursor: {filename}". Other variants still apply. |
| Windows: registry write fails | Catch Win32 error, show error toast with message. Cursor stays at current. |
| Windows: WM_SETTINGCHANGE broadcast fails | Non-fatal — cursor already written. Log silently. |
| Linux: .ani→Xcursor conversion fails | Show error toast with variant name. Conversion debug in log. |
| Linux: xcursorgen not found | Show error toast: "xcursorgen not found. Install xcursorgen package." |
| Backup fails before apply | Block apply. Show error toast: "Failed to backup current cursor. Aborted." |
| Permission denied on pack dir | Show error toast: "Cannot write to ~/.anime-cursor/packs/. Check permissions." |

---

## 10. Distribution & URL Pattern

### Format per Platform

| Platform | Default Format |
|----------|---------------|
| Windows | `.exe` (Tauri bundle) |
| Linux Desktop | `.deb` package |
| macOS | N/A (excluded per v1 scope) |

### URL Pattern

| Link | URL |
|------|-----|
| GitHub | `github.com/curzyori/anime-cursor` |
| Website | `anime-cursor.curzy.dev` |
| Donate | `donate.curzy.dev` |

---

## 11. Out of Scope (v1)

| Feature | Reason |
|---------|--------|
| macOS support | .ani not native; custom format conversion too complex for v1 |
| Auto-updater | Manual download for v1; Tauri updater later |
| Online pack gallery | Fully offline app. Maybe later as optional |
| Cursor editor/creator | Out of scope — manager only |
| In-app cursor animation preview | Static thumbnail from first frame; full animation playback is complex |
| Multiple simultaneous packs | One cursor theme active at a time |
| CLI mode | GUI-only for v1 |

---

## 12. Open Questions

|| Question | Status |
||----------|--------|
|| .ANI RIFF parser — build custom or use crate? | **Custom parser** (ref `references/ani-riff-structure.md`) — ~100 lines, no crate dependency, full control over chunk parsing |
|| Linux Xcursor conversion — embed xcursorgen or ship binary? | **`xcursor` crate** — pure Rust, no external binary needed, handles Xcursor format generation in-process |
|| Language files — should i18n keys be in code or separate files? | Separate JSON files (en.json, id.json, zh.json, ja.json) |

---

## 13. Decision Log

| # | Decision | Alternatives | Rationale |
|---|----------|-------------|-----------|
| 1 | Tauri v2 over Electron | Electron, PyQt6, Flutter | Smaller binary (~8MB vs ~150MB), Rust performance for .ani parsing, cross-platform via webview |
| 2 | React frontend | Vanilla JS, Svelte, Vue | Familiar, component-based, good for multi-page app with state management |
| 3 | Auto OS detection | Manual OS selector | User requested: "pake auto detect aja" |
| 4 | Dark theme default | Light theme default | User requested: prioritaskan dark |
| 5 | EN default language | ID default | User requested: prioritaskan en |
| 6 | ZIP pack format | Bare .ani files | 1 ZIP = multiple variants + metadata + preview |
| 7 | macOS excluded | Full cross-platform | .ani is Windows native; Xcursor on Linux is convertible; macOS cursor format is incompatible |
| 8 | Settings links: GitHub, Website, Donate | Just in README | User required it in app's Settings screen |
| 9 | No animated preview | GIF preview, video preview | YAGNI — static thumbnail from first frame is sufficient for v1 |
| 10 | Offline-only | Online pack gallery | v1 focus: local management. Gallery could be v2 |
| 11 | 4-language i18n (EN/ID/CN/JP) | 2-language (EN/ID) | Curzy default convention across 50-Projects |
