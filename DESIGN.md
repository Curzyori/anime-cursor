---
version: alpha
name: anime-cursor-design
description: "A desktop app for managing and applying anime animated cursor (.ani) packs. Dark-first UI with pink-purple anime gradient accents. Supports Windows and Linux with automatic OS detection, ZIP pack import, and one-click apply/restore."
colors:
  primary: "#FF6B9D"
  primary-active: "#E05580"
  primary-muted: "rgba(255,107,157,0.15)"
  accent-purple: "#C44AFF"
  gradient-start: "#FF6B9D"
  gradient-end: "#C44AFF"
  ink: "#1A1A2E"
  body: "#E8E8F0"
  body-soft: "#9B9BB5"
  muted: "#6B6B85"
  surface: "#16162A"
  surface-elevated: "#1E1E38"
  surface-card: "#25254A"
  surface-hover: "#2E2E55"
  canvas-dark: "#0F0F23"
  canvas-dark-soft: "#13132A"
  hairline: "#2E2E55"
  hairline-soft: "#222244"
  on-primary: "#FFFFFF"
  on-dark: "#E8E8F0"
  on-dark-soft: "#9B9BB5"
  overlay: "rgba(15,15,35,0.6)"
  semantic-success: "#4ADE80"
  semantic-error: "#FF4D6D"
  semantic-warning: "#FBBF24"
  semantic-info: "#60A5FA"
typography:
  display-xl:
    fontFamily: "'Inter', system-ui, sans-serif"
    fontSize: 32px
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: -0.5px
  display-lg:
    fontFamily: "'Inter', system-ui, sans-serif"
    fontSize: 24px
    fontWeight: 600
    lineHeight: 1.3
    letterSpacing: -0.3px
  display-md:
    fontFamily: "'Inter', system-ui, sans-serif"
    fontSize: 20px
    fontWeight: 600
    lineHeight: 1.4
    letterSpacing: 0
  title-md:
    fontFamily: "'Inter', system-ui, sans-serif"
    fontSize: 16px
    fontWeight: 600
    lineHeight: 1.4
    letterSpacing: 0
  title-sm:
    fontFamily: "'Inter', system-ui, sans-serif"
    fontSize: 14px
    fontWeight: 500
    lineHeight: 1.4
    letterSpacing: 0
  body-md:
    fontFamily: "'Inter', system-ui, sans-serif"
    fontSize: 14px
    fontWeight: 400
    lineHeight: 1.55
    letterSpacing: 0
  body-sm:
    fontFamily: "'Inter', system-ui, sans-serif"
    fontSize: 12px
    fontWeight: 400
    lineHeight: 1.5
    letterSpacing: 0
  caption:
    fontFamily: "'Inter', system-ui, sans-serif"
    fontSize: 11px
    fontWeight: 500
    lineHeight: 1.4
    letterSpacing: 0.5px
    textTransform: uppercase
  code:
    fontFamily: "'JetBrains Mono', 'Fira Code', monospace"
    fontSize: 12px
    fontWeight: 400
    lineHeight: 1.6
    letterSpacing: 0
rounded:
  none: 0px
  xs: 4px
  sm: 6px
  md: 8px
  lg: 12px
  xl: 16px
  xxl: 24px
  pill: 9999px
spacing:
  xxs: 4px
  xs: 8px
  sm: 12px
  md: 16px
  lg: 20px
  xl: 24px
  xxl: 32px
  section: 48px
components:
  sidebar:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.body}"
    width: 220px
    collapsedWidth: 56px
    typography: "{typography.title-sm}"
    padding: "{spacing.md}"
    item-active-bg: "{colors.surface-card}"
    item-hover-bg: "{colors.surface-hover}"
  dashboard-grid:
    backgroundColor: transparent
    gap: "{spacing.lg}"
    card-min-width: 180px
  pack-card:
    backgroundColor: "{colors.surface-card}"
    textColor: "{colors.body}"
    typography: "{typography.title-md}"
    rounded: "{rounded.lg}"
    padding: "{spacing.md}"
    border: "1px solid {colors.hairline}"
    hover-border: "1px solid {colors.primary}"
  import-zone:
    backgroundColor: "{colors.surface-elevated}"
    textColor: "{colors.body-soft}"
    rounded: "{rounded.xl}"
    padding: "{spacing.section}"
    border: "2px dashed {colors.hairline}"
    hover-border: "2px dashed {colors.primary}"
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.on-primary}"
    typography: "{typography.title-sm}"
    rounded: "{rounded.md}"
    padding: "10px 18px"
    height: 36px
  button-secondary:
    backgroundColor: "{colors.surface-card}"
    textColor: "{colors.body}"
    typography: "{typography.title-sm}"
    rounded: "{rounded.md}"
    padding: "10px 18px"
    height: 36px
    border: "1px solid {colors.hairline}"
  button-ghost:
    backgroundColor: transparent
    textColor: "{colors.body-soft}"
    typography: "{typography.title-sm}"
    rounded: "{rounded.md}"
    padding: "8px 12px"
    height: 36px
    hover-bg: "{colors.surface-hover}"
  text-input:
    backgroundColor: "{colors.surface-elevated}"
    textColor: "{colors.body}"
    typography: "{typography.body-md}"
    rounded: "{rounded.md}"
    padding: "10px 14px"
    height: 36px
    border: "1px solid {colors.hairline}"
    focus-border: "1px solid {colors.primary}"
  dropdown:
    backgroundColor: "{colors.surface-elevated}"
    textColor: "{colors.body}"
    typography: "{typography.body-md}"
    rounded: "{rounded.sm}"
    minWidth: 180px
    optionPadding: "{spacing.xs} {spacing.sm}"
    border: "1px solid {colors.hairline}"
  modal-dialog:
    backgroundColor: "{colors.surface-card}"
    textColor: "{colors.body}"
    typography: "{typography.display-md}"
    rounded: "{rounded.lg}"
    maxWidth: 400px
    padding: "{spacing.xl}"
    overlay: "{colors.overlay}"
  progress-bar:
    trackColor: "{colors.surface-elevated}"
    fillColor: "{colors.primary}"
    height: 4px
    rounded: "{rounded.pill}"
  empty-state:
    textColor: "{colors.body-soft}"
    typography: "{typography.body-md}"
    illustrationSize: 80px
    padding: "{spacing.section}"
  variant-list:
    backgroundColor: transparent
    gap: "{spacing.md}"
  variant-item:
    backgroundColor: "{colors.surface-elevated}"
    textColor: "{colors.body}"
    rounded: "{rounded.md}"
    padding: "{spacing.sm}"
    border: "1px solid {colors.hairline}"
    hover-bg: "{colors.surface-hover}"
    thumbnail-size: 48px
  active-variant-indicator:
    leftBorder: "3px solid {colors.primary}"
  link-row:
    backgroundColor: transparent
    textColor: "{colors.body}"
    typography: "{typography.body-md}"
    height: 36px
    rounded: "{rounded.md}"
    hover-bg: "{colors.surface-hover}"
  settings-section:
    backgroundColor: transparent
    textColor: "{colors.body}"
    typography: "{typography.title-md}"
    padding: "{spacing.md} 0"
  divider:
    backgroundColor: "{colors.hairline-soft}"
    height: 1px
    margin: "{spacing.md} 0"
  badge:
    backgroundColor: "{colors.primary-muted}"
    textColor: "{colors.primary}"
    typography: "{typography.caption}"
    rounded: "{rounded.pill}"
    padding: "2px 10px"
  pack-header:
    backgroundColor: transparent
    textColor: "{colors.body}"
    typography: "{typography.display-lg}"
    padding: "{spacing.md} 0"
    author-size: "{typography.body-md}"
    source-url-size: "{typography.body-sm}"
  link-row:
    backgroundColor: transparent
    textColor: "{colors.body}"
    typography: "{typography.body-md}"
    padding: "{spacing.sm} 0"
    hover-bg: "{colors.surface-hover}"
    rounded: "{rounded.sm}"
    iconSize: 20px
  toast-success:
    backgroundColor: "{colors.semantic-success}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    rounded: "{rounded.md}"
    padding: "12px 16px"
  toast-error:
    backgroundColor: "{colors.semantic-error}"
    textColor: "{colors.on-primary}"
    typography: "{typography.body-sm}"
    rounded: "{rounded.md}"
    padding: "12px 16px"
i18n:
  locales: ["en", "id", "zh", "ja"]
  default: "en"
  localeMap:
    en: "EN - English"
    id: "ID - Bahasa Indonesia"
    zh: "ZH - 中文"
    ja: "JA - 日本語"
---

## Overview

Anime Cursor is a desktop application for browsing, importing, and applying anime-style animated cursor (.ani) packs. The UI follows a **dark-first** design system with a pink-purple anime gradient accent palette, inspired by anime aesthetics and modern developer tools.

The dark canvas (`{colors.canvas-dark}` — #0F0F23) provides a deep, immersive background that makes the gradient pink-purple accents and pack preview thumbnails stand out. Card surfaces use progressively lighter navy tones to create depth without shadows.

**Key Characteristics:**
- Dark canvas (`{colors.canvas-dark}` — #0F0F23) with multiple navy surface layers for depth
- Pink-purple gradient (`{colors.gradient-start}` → `{colors.gradient-end}`) as the signature brand voltage
- Primary pink (`{colors.primary}` — #FF6B9D) used on CTAs, active states, and the logo
- Inter sans-serif for all UI type; JetBrains Mono for code/file paths
- Border radius hierarchy: `{rounded.md}` (8px) for buttons, `{rounded.lg}` (12px) for cards, `{rounded.xl}` (16px) for import zone
- 48px section rhythm — tighter than standard SaaS, appropriate for a utility desktop app

## Colors

### Brand & Accent
- **Pink / Primary** (`{colors.primary}` — #FF6B9D): Signature anime pink. Used on primary CTAs, active nav items, hover card borders.
- **Purple** (`{colors.accent-purple}` — #C44AFF): Secondary brand voltage. Used on gradient overlays, secondary badges.
- **Gradient** (`{colors.gradient-start}` → `{colors.gradient-end}`): Pink→Purple gradient used on the logo background, hero bands.

### Surface (Dark Mode Default)
- **Canvas Dark** (`{colors.canvas-dark}` — #0F0F23): Deep navy-black — the default page floor. Always dark.
- **Canvas Dark Soft** (`{colors.canvas-dark-soft}` — #13132A): Slightly lighter, used for secondary page bands.
- **Surface** (`{colors.surface}` — #16162A): Sidebar background, secondary panels.
- **Surface Elevated** (`{colors.surface-elevated}` — #1E1E38): Import zone, input fields.
- **Surface Card** (`{colors.surface-card}` — #25254A): Pack cards, settings cards, dialog backgrounds.
- **Surface Hover** (`{colors.surface-hover}` — #2E2E55): Hover state for interactive cards.

### Surface (Light Mode Alternative)
When the user toggles to Light Mode:
- **Canvas** becomes `#FAFAFE` (near-white with hint of lavender)
- **Surface** becomes `#F0F0F8`
- **Surface Elevated** becomes `#E8E8F5`
- **Surface Card** becomes `#FFFFFF`
- **Ink** stays near-black for readability
- All other accent colors remain the same

### Text
- **Ink** (`{colors.ink}` — #1A1A2E): All headlines and primary text. Only used in light mode body.
- **Body** (`{colors.body}` — #E8E8F0): Default text on dark surfaces.
- **Body Soft** (`{colors.body-soft}` — #9B9BB5): Secondary text, descriptions.
- **Muted** (`{colors.muted}` — #6B6B85): Captions, fine-print, disabled states.
- **On Primary** (`{colors.on-primary}` — #FFFFFF): Text on pink buttons.
- **On Dark Soft** (`{colors.on-dark-soft}` — #9B9BB5): Footer body, secondary labels.

### Semantic
- **Success** (`{colors.semantic-success}` — #4ADE80): "Applied!" confirmation, restore success.
- **Error** (`{colors.semantic-error}` — #FF4D6D): Import errors, apply failures.
- **Warning** (`{colors.semantic-warning}` — #FBBF24): Corrupt ZIP, missing metadata.
- **Info** (`{colors.semantic-info}` — #60A5FA): Tips, "metadata not found" suggestions.

## Typography

### Font Family
The system uses **Inter** as the single sans-serif family for all UI type — display, body, navigation, buttons. **JetBrains Mono** is reserved for code/file paths and terminal output. Fallback: `system-ui, -apple-system, sans-serif`.

### Hierarchy

| Token | Size | Weight | Line Height | Letter Spacing | Use |
|---|---|---|---|---|---|
| `{typography.display-xl}` | 32px | 700 | 1.2 | -0.5px | Page headlines (Dashboard title) |
| `{typography.display-lg}` | 24px | 600 | 1.3 | -0.3px | Section heads, pack detail title |
| `{typography.display-md}` | 20px | 600 | 1.4 | 0 | Dialog titles, settings section heads |
| `{typography.title-md}` | 16px | 600 | 1.4 | 0 | Card titles, sidebar labels |
| `{typography.title-sm}` | 14px | 500 | 1.4 | 0 | Button labels, nav items |
| `{typography.body-md}` | 14px | 400 | 1.55 | 0 | Default running-text |
| `{typography.body-sm}` | 12px | 400 | 1.5 | 0 | Footer, pack metadata |
| `{typography.caption}` | 11px | 500 | 1.4 | 0.5px, uppercase | Badges, section labels |
| `{typography.code}` | 12px | 400 | 1.6 | 0 | File paths, code — JetBrains Mono |

### Principles
- Display weights stay at 600–700 for clear hierarchy but never go above 700.
- Body text is comfortable (`{typography.body-md}` 14px at 1.55 line height) for desktop reading.
- Caption is always uppercase for consistency across badges and section headers.
- Code surfaces use JetBrains Mono at 12px for compact file path display.

## Layout

### Spacing System
- **Base unit:** 4px.
- **Tokens:** `{spacing.xxs}` 4px · `{spacing.xs}` 8px · `{spacing.sm}` 12px · `{spacing.md}` 16px · `{spacing.lg}` 20px · `{spacing.xl}` 24px · `{spacing.xxl}` 32px · `{spacing.section}` 48px.
- **Section padding:** `{spacing.section}` (48px) — appropriate for a utility desktop app with limited vertical space.
- **Card internal padding:** `{spacing.md}` (16px) for pack cards, `{spacing.xl}` (24px) for detail views.

### App Layout
```
┌─────────────┬──────────────────────────────────────┐
│   Sidebar   │  Main Content Area                    │
│  220px      │                                       │
│             │  ┌──────┐ ┌──────┐ ┌──────┐          │
│  🏠 Home    │  │      │ │      │ │      │          │
│  📦 Import  │  │Pack1 │ │Pack2 │ │Pack3 │          │
│  ⚙️ Settings│  │      │ │      │ │      │          │
│             │  └──────┘ └──────┘ └──────┘          │
│             │                                       │
└─────────────┴──────────────────────────────────────┘
```

### Grid & Container
- **Sidebar:** Fixed 220px width when expanded, 56px when collapsed (icons only).
- **Main content:** Fluid — fills remaining width after sidebar.
- **Dashboard grid:** Auto-fill responsive grid, card min-width 180px.
- **Pack detail:** Single-column layout with preview carousel at top, variant list below.

## Elevation & Depth

The system uses **color contrast for depth** as the primary elevation method. Surfaces get progressively lighter navy to indicate elevation:

| Level | Token | Use |
|---|---|---|
| Floor | `{colors.canvas-dark}` (#0F0F23) | Page background |
| Panel | `{colors.surface}` (#16162A) | Sidebar |
| Elevated | `{colors.surface-elevated}` (#1E1E38) | Import zone, inputs |
| Card | `{colors.surface-card}` (#25254A) | Pack cards, dialogs |
| Hover | `{colors.surface-hover}` (#2E2E55) | Hover states |

**Floating UI layer** — elements above the card level (modals, toasts, tooltips) get a subtle box-shadow to distinguish them from content cards. Use `0 4px 24px rgba(0,0,0,.4)` for the modal overlay and `0 2px 8px rgba(0,0,0,.4)` for toasts/tooltips. This prevents confusion between floating elements and elevated surfaces.

## Shapes

### Border Radius Scale

| Token | Value | Use |
|---|---|---|
| `{rounded.xs}` | 4px | Inline tags, badges |
| `{rounded.sm}` | 6px | Compact rows, tooltips, dropdown options |
| `{rounded.md}` | 8px | Buttons, text inputs, sidebar items |
| `{rounded.lg}` | 12px | Pack cards, settings cards, dialogs |
| `{rounded.xl}` | 16px | Import zone, large containers |
| `{rounded.xxl}` | 24px | App icon background |
| `{rounded.pill}` | 9999px | Badge pills, version tag |

### Iconography
- The app icon (`public/logo.svg`) is a pink-purple gradient rounded rect (rx=96 on 512px) containing an anime girl silhouette with a cursor pointer overlay in the top-right corner, plus sparkle dots. Matches spec.
- Navigation icons use Feather-style line icons at 20px with 1.5px stroke.
- Empty states use simple illustration placeholders (inline SVG, no external assets).

## Motion & Animation

### Core Animation Parameters
- **Easing default:** `cubic-bezier(0.16, 1, 0.3, 1)` — snappy ease-out for all UI transitions.
- **Easing aggressive:** `cubic-bezier(0.32, 0, 0.67, 0)` — quick ease-in for fade-out/dismiss.
- **Duration baseline:** 200ms for most interactions, 300ms for compound transitions.

### Sidebar Collapse
- Width transition: 200ms ease-out on the `width` property.
- Icon labels fade out (100ms) before collapse starts, fade in (100ms) after expand completes.
- Toggle button rotates 180° on collapse.

### Page Transitions
- Route change: 200ms fade on content area (React Router `AnimatePresence` or CSS `@view-transition`).
- Sidebar active highlight: instant background swap, 150ms color fade on accent border.

### Card Hover
- Border color: 150ms ease-out from `{colors.hairline}` to `{colors.primary}`.
- Optional: subtle `transform: scale(1.02)` with 200ms ease-out — only on pack cards in the dashboard grid, not on buttons.

### Toast Notifications
- Enter: slide-in-right 250ms (translateX 20px → 0).
- Dismiss: fade-out 300ms (opacity 1 → 0).
- Delay before auto-dismiss: 3s (success), persistent (error).

### Import Zone
- Dashed border color transition: 150ms ease-out.
- Background glow: pulsing `box-shadow inset 0 0 20px rgba(255, 107, 157, 0.1)` on active drag — 1s infinite alternate.

### Cursor Preview (Pack Detail)
- .ani preview auto-plays on hover over variant thumbnail.
- Thumbnail border transition: 150ms `{colors.hairline}` → `{colors.primary}` on selected variant.

### Progress Bar
- Width transition on fill: 300ms ease-out — smooth percentage change during ZIP extraction.

## Components

### Sidebar Navigation

**`sidebar`** — Fixed left panel, `{colors.surface}` background, 220px width (56px collapsed). Layout: app logo + name at top, nav items with icon + label, settings at bottom. Active item uses `{colors.surface-card}` background + `{colors.primary}` left border accent. Navigation items: Home, Import, Settings.

### Buttons

**`button-primary`** — The signature pink CTA. Background `{colors.primary}` (#FF6B9D), text `{colors.on-primary}` (white), type `{typography.title-sm}` (14px / 500), padding 10px × 18px, height 36px, rounded `{rounded.md}` (8px). Hover darkens slightly to `#E05580`.

**`button-secondary`** — Card-colored outline button. Background `{colors.surface-card}`, text `{colors.body}`, 1px `{colors.hairline}` border, same size as primary.

**`button-ghost`** — Transparent button for secondary actions. Background transparent, text `{colors.body-soft}`, shows `{colors.surface-hover}` background on hover.

### Pack Cards

**`pack-card`** — Used in the Dashboard grid. Background `{colors.surface-card}`, 1px `{colors.hairline}` border, rounded `{rounded.lg}` (12px), padding `{spacing.md}` (16px). Contains a 120×120 thumbnail area at top, pack name in `{typography.title-md}`, author/badge below. Hover state: border shifts to `{colors.primary}`.

### Import Zone

**`import-zone`** — Full-width drop target. Background `{colors.surface-elevated}`, 2px dashed `{colors.hairline}` border, rounded `{rounded.xl}` (16px), padding `{spacing.section}` (48px). Text: "Drop your .zip pack here or click to browse". Hover/active: border shifts to dashed `{colors.primary}`, background gets subtle pink glow. Shows progress bar during extraction.

### Modal Dialog

**`modal-dialog`** — Floating overlay for confirmation/actions. Overlay: `{colors.overlay}` (60% ink at 60% opacity). Dialog box: `{colors.surface-card}` background, max-width 400px, rounded `{rounded.lg}` (12px), padding `{spacing.xl}` (24px). Title in `{typography.display-md}`, body text in `{typography.body-md}`, action buttons right-aligned (ghost + primary). Box-shadow: `0 4px 24px rgba(0,0,0,.4)`. Used for: pack removal confirmation, restore cursor confirmation.

### Progress Bar

**`progress-bar`** — Indeterminate or determinate fill. Track: `{colors.surface-elevated}`, 4px height, rounded `{rounded.pill}`. Fill: `{colors.primary}` with width transition at 300ms ease-out. Indeterminate state: looping gradient animation (primary → accent-purple → primary, 1.5s). Used in: ZIP extraction, .ani→Xcursor conversion.

### Empty State

**`empty-state`** — Centered placeholder when no packs are installed. 80px illustration (inline SVG), `{colors.body-soft}` body text in `{typography.body-md}`, padding `{spacing.section}` (48px) vertically. Contains a `button-primary` CTA: "Import Your First Pack" linking to Import page.

### Pack Detail

**`variant-list`** — Vertical list container for cursor variants in a pack. Background transparent, gap `{spacing.md}` (16px).

**`variant-item`** — Row in the variant list. Background `{colors.surface-elevated}`, 1px `{colors.hairline}` border, rounded `{rounded.md}` (8px), padding `{spacing.sm}` (12px). Layout: horizontal flex — 48×48 thumbnail on left, name + size label in center, `button-primary` ("Apply") on right. Hover: `{colors.surface-hover}` background.

**`active-variant-indicator`** — Current active variant marker. `{colors.primary}` 3px left border on `variant-item`, plus a small "ACTIVE" badge (green, `{colors.semantic-success}` background, white text, `{typography.caption}`, pill rounded) replacing the Apply button.

### Link Row

**`link-row`** — External link in Settings. Background transparent, height 36px, horizontal flex: label on left (`{typography.body-md}`, `{colors.body}`), chevron/arrow icon on right (`{colors.body-soft}`, 16px). Hover: `{colors.surface-hover}` background, rounded `{rounded.md}` (8px). Cursor pointer. Click: `tauri::api::shell::open`. Used for: GitHub, Website, Donate links in Settings.

### Badges

**`badge`** — Small uppercase pill. Background `{colors.primary-muted}` (15% opacity pink), text `{colors.primary}`, type `{typography.caption}` (11px / 500 / uppercase). Used for: "NEW", "Windows", "Linux", cursor variant size labels.

### Dropdown / Select

**`dropdown`** — Native select replacement. Container: `{colors.surface-elevated}` background, 1px `{colors.hairline}` border, rounded `{rounded.sm}` (6px), min-width 180px. Option row: `{spacing.xs} {spacing.sm}` padding, `{typography.body-md}` text. Hover option: `{colors.surface-hover}`. Used in: Settings (sort preference, theme toggle, **language selector EN / ID / ZH / JP**).

**Language Selector (Settings)** — 4 options: `EN` English · `ID` Bahasa Indonesia · `ZH` 中文 · `JP` 日本語. Selected option shows a check icon. Each option prefixed with an **inline SVG flag** (not emoji): EN=UK/US flag, ID=Indonesia flag, ZH=China flag, JP=Japan flag. Default: `EN`. The selector controls `t()` locale loading from `i18n/{en,id,zh,ja}.json`.

### Pack Detail Page

**Layout** — Two sections stacked vertically:
1. **`pack-header`** — Top section: pack name in `{typography.display-lg}`, author line in `{typography.body-md}` (e.g. "by MyWaifuCollection"), source URL in `{typography.body-sm}`. All left-aligned, `{spacing.md}` padding bottom.
2. **`variant-list`** — Scrollable list of cursor variants. Gap between items: `{spacing.sm}`.

**`variant-item`** — Each row in the variant list. Background `{colors.surface-elevated}`, 1px `{colors.hairline}` border, rounded `{rounded.lg}` (12px), padding `{spacing.md}`. Layout: thumbnail (48×48) | variant name + size label | Apply button (`button-primary`, compact 32px height). Hover: background shifts to `{colors.surface-hover}`.

**`active-variant-indicator`** — The currently applied variant gets a pink left-border (`3px solid {colors.primary}`) and/or a small pink dot (8px) next to the variant name. Border on hover stays — this indicator is persistent until another variant is applied.

**"Remove Pack" button** — `button-secondary` style (red variant: `{colors.semantic-error}` border). Click opens `modal-dialog` confirmation.

### Link Row

**`link-row`** — Used in Settings screen to display external links. Transparent background, `{typography.body-md}` text, 20px icon (Feather-style external link/code/heart) at start, row padding `{spacing.sm} 0`. Hover shows `{colors.surface-hover}` background with `{rounded.sm}` rounding. Chevron icon at right (→ or ↗). Full row clickable → opens via `tauri::api::shell::open`.

### Toast Notifications

**`toast-success`** — Green toast. Background `{colors.semantic-success}`, text `{colors.ink}`. Appears at top-right, auto-dismisses after 3s. Enter animation: slide-in-right 250ms.

**`toast-error`** — Red toast. Background `{colors.semantic-error}`, text white. Persistent until dismissed. Enter animation: slide-in-right 250ms.

## Light Mode Overrides

When the user toggles to Light Mode via Settings:

| Token | Dark Value | Light Value |
|---|---|---|
| canvas-dark | #0F0F23 | #FAFAFE |
| canvas-dark-soft | #13132A | #F5F5FC |
| surface | #16162A | #F0F0F8 |
| surface-elevated | #1E1E38 | #E8E8F5 |
| surface-card | #25254A | #FFFFFF |
| surface-hover | #2E2E55 | #E8E8F5 |
| body | #E8E8F0 | #1A1A2E |
| body-soft | #9B9BB5 | #5A5A7A |
| muted | #6B6B85 | #8A8AAA |
| hairline | #2E2E55 | #E0E0F0 |
| hairline-soft | #222244 | #D8D8E8 |
| overlay | rgba(15,15,35,0.6) | rgba(90,90,122,0.2) |

The accent colors (pink, purple, gradient) remain unchanged in light mode — only the surface and text tokens invert.

## Implementation Notes — Tauri v2 + React + Rust

### CSS Custom Properties
Define all color, spacing, and radius tokens as CSS custom properties on `:root`, with `[data-theme="light"]` overrides:

```css
:root {
  --color-primary: #FF6B9D;
  --color-primary-active: #E05580;
  --color-surface: #16162A;
  --spacing-md: 16px;
  --rounded-md: 8px;
  /* ... etc */
}

[data-theme="light"] {
  --color-surface: #F0F0F8;
  /* ... light overrides */
}
```

### Tailwind Config Mapping
Map tokens to Tailwind v3 `theme.extend`:

```js
// tailwind.config.js
module.exports = {
  extend: {
    colors: {
      primary: { DEFAULT: '#FF6B9D', active: '#E05580', muted: 'rgba(255,107,157,0.15)' },
      accent: { purple: '#C44AFF' },
      surface: { DEFAULT: '#16162A', elevated: '#1E1E38', card: '#25254A', hover: '#2E2E55' },
      canvas: { dark: '#0F0F23', 'dark-soft': '#13132A' },
      ink: { DEFAULT: '#1A1A2E', body: '#E8E8F0', soft: '#9B9BB5', muted: '#6B6B85' },
      semantic: { success: '#4ADE80', error: '#FF4D6D', warning: '#FBBF24', info: '#60A5FA' },
      hairline: { DEFAULT: '#2E2E55', soft: '#222244' },
    },
    spacing: { xxs: '4px', section: '48px' },
    borderRadius: { xs: '4px', sm: '6px', lg: '12px', xl: '16px', xxl: '24px' },
    fontFamily: { code: ["'JetBrains Mono'", "'Fira Code'", 'monospace'] },
  },
}
```

### Tauri-Specific
- **Window chrome:** Custom titlebar via `data-tauri-drag-region`. Titlebar background matches `{colors.surface}`.
- **Tray icon:** Uses the app icon as a 32×32 variant. Context menu: Show/Hide, Quit.
- **Native file dialog:** Via `tauri::api::dialog::file::FileDialogBuilder` — filter for `.zip` and `.ani`.
- **Dark mode detection:** Read `Window::theme()` on startup to set initial `data-theme`. Overridden by user preference.
- **Shell open:** `tauri::api::shell::open` for GitHub/website/donate links — opens the OS default browser (not the app's webview).

### React Frontend
- **Router:** React Router v6 with `createHashRouter` (Tauri filesystem-friendly).
- **Global styles:** Import CSS vars in `index.css` via `@apply` or direct `var()`.
- **Animation:** CSS transitions/animations preferred over JS animation libraries — lighter, GPU-accelerated, sufficient for utility app.
- **State:** Zustand for app state (theme, language, sidebar collapsed) + React Query for async Tauri commands.

### Rust Backend
- .ani parsing: `tauri::command` functions for read/extract/convert.
- Windows cursor apply: registry write via `winreg` crate, `WM_SETTINGCHANGE` via `windows-sys`.
- Linux cursor apply: `.ani` → Xcursor conversion via `xcursor` crate, `gsettings` call via `std::process::Command`.
  - GNOME (X11 + Wayland): `gsettings set org.gnome.desktop.interface cursor-theme` works on both.
  - KDE (X11): `kwriteconfig5` + `plasma-apply-cursortheme`. KDE Wayland: partial — manual revert may be needed (Wayland protocol restricts programmatic cursor changes).

## Accessibility & Contrast Notes

- Primary pink (#FF6B9D) on surface-card (#25254A): **~5.5:1** — WCAG AA for normal text ✓.
- Primary pink (#FF6B9D) on surface-elevated (#1E1E38): **~4.8:1** — WCAG AA for large text only. Avoid using pink as body text at small sizes on elevated surfaces.
- Body (#E8E8F0) on canvas-dark (#0F0F23): **~13:1** — excellent contrast ✓.
- Body-soft (#9B9BB5) on canvas-dark (#0F0F23): **~6:1** — WCAG AA ✓.
- Semantic colors on dark surfaces all exceed 7:1 — safe for status indicators.
- Light mode contrasts are slightly lower but still above 4.5:1 across all pairs.

## Do's and Don'ts

### Do
- Use `{colors.primary}` (#FF6B9D) on primary CTAs and active states only.
- Keep the dark canvas as default. Light mode is an opt-in preference.
- Use `{rounded.md}` (8px) consistently for all interactive elements (buttons, inputs).
- Maintain 48px section rhythm — the app is a utility, not a marketing site.
- Use subtle box-shadow for floating elements (modal, toast, tooltip) above the card layer.
- Match color tokens exactly to the CSS vars or Tailwind config — no inlined hex values.

### Don't
- Don't introduce a secondary action color beyond pink and purple.
- Don't use drop shadows for surface elevation — use color contrast instead.
- Don't use emoji as icons. Use inline SVG line icons (Feather-style).
- Don't mix border-radius styles — cards stay at `{rounded.lg}`, buttons at `{rounded.md}`.
- Don't make the sidebar non-collapsible — users may want more preview space.
- Don't animate route transitions longer than 200ms — this is a utility, not a showcase.
