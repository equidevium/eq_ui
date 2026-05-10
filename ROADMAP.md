# eq_ui Roadmap

> A portable Dioxus 0.7 component library following atomic design principles.
> This roadmap is organized by priority: **Now**, **Next**, and **Later**.

---

## What We Have Today

### Atoms (19)
- `EqText` - Text rendering with variants (H1, H2, H3, Body, Muted, Caption, Emphasis, Mono)
- `EqLabel` - Form labels
- `EqLink` - Styled anchor links
- `EqInput` - Text, email, password, textarea inputs
- `EqIcon` - Icon wrapper with size variants (Sm, Md, Lg)
- `EqImage` - Full-featured image atom (sizing, aspect ratios, object-fit, rounded corners)
- `EqScrollableSpace` - Scrollable container with themed scrollbar
- `EqDivider` - Separator with variants (Solid, Dashed, Dotted, Spacer), weights, and spacing
- `EqVideo` - Video atom with poster overlay (EqImage + play icon), autoplay, muted, loop, controls, aspect ratio
- `EqCheckbox` - Themed checkbox with Checked, Unchecked, and Indeterminate states. Icon-based rendering using Phosphor square icons, optional label, disabled state, size variants
- `EqButton` - Themed button atom with five variants (Primary, Ghost, Outline, Card, Danger) and three sizes (Sm, Md, Lg). Gradient variants animate via background-position shift on hover. Native `<button>` element for accessibility
- `EqProgress` - Progress bar with determinate/indeterminate modes, 4 color variants, gradient fill, and shimmer animation
- `EqTab` - Tab bar with underline, pill, and card variants; badges; disabled state; WAI-ARIA tablist pattern with roving tabindex
- `EqRadioGroup` - Radio button group with mutually exclusive selection, three sizes (Sm, Md, Lg), vertical/horizontal layout, WAI-ARIA radiogroup pattern with roving tabindex
- `EqSwitch` - Toggle switch with pill track and sliding thumb, three sizes, WAI-ARIA switch role
- `EqSlider` - Range slider with native `<input type="range">`, accent-color theming, three sizes (Sm, Md, Lg), optional value label, disabled state
- `EqAvatar` - User avatar with image, initials fallback, icon fallback, four sizes (Sm, Md, Lg, Xl), online/offline/busy status dot, selection ring
- `EqTooltip` - Hover/focus tooltip with four positions (Top, Bottom, Left, Right), pure CSS positioning, ARIA describedby, keyboard accessible
- `EqSelect` - Styled dropdown select with search, placeholder, disabled options, keyboard navigation, WAI-ARIA combobox pattern, check mark on selected option

### Molecules (14)
- `EqCard` - Card with header, body, footer slots
- `EqImageCard` - Image card with caption modes (Below, Overlay)
- `EqCarousel` - Generic content carousel with Default and Peek modes, WAI-ARIA carousel pattern
- `EqTree` - Collapsible tree view with select, expand/collapse, child count, WAI-ARIA tree pattern with full keyboard navigation
- `EqAccordion` - Collapsible panels with single-expand and multi-expand modes, smooth CSS grid animation, element headers, WAI-ARIA accordion pattern with keyboard navigation
- `EqNavItem` - Navigation item with icon, label, active state, size variants
- `EqCta` - Call-to-action section with title, description, action slot, and two layout modes (Inline, Centered)
- `EqModal` - Modal dialog with backdrop, five size presets, close-on-backdrop/Escape, focus management, WAI-ARIA dialog pattern
- `EqToastList` - Toast notification stack with four severity levels, auto-dismiss via JS setTimeout, six position anchors, manual close, WAI-ARIA status/alert pattern
- `EqDropdown` - Dropdown menu with selectable items, separators, disabled state, keyboard navigation (arrows/Enter/Escape), two positions, close-on-outside-click
- `EqDatePicker` - Date picker with calendar popup, month navigation, today highlight, formatted display, pure Rust date math, WAI-ARIA dialog pattern
- `EqCalendar` - Standalone calendar with month & week views, event dots, timed events, month/year picker drill-down, WAI-ARIA grid pattern
- `EqVirtualList` - High-performance windowed list rendering only visible items, fixed-size rows, overscan buffer, scroll-to-index, sticky section headers, vertical/horizontal modes
- `EqDeviceFrame` - Static iPhone 16 / 16 Pro chrome with Dynamic Island, status bar, home indicator, painted side buttons. Pure presentation wrapper for showcasing mobile-only components in the playground; no event callbacks. Tier 1 Blitz-ready (CSS + inline SVG only)

### Organisms (12)
- `EqAppShell` - Full-page layout (header + main + footer)
- `EqHeader` - Sticky header with nav slot and backdrop blur
- `EqFooter` - Footer with link groups
- `EqNavbar` - Navigation bar
- `EqHeroShell` - Hero section with optional background image, overlay, custom title/subtitle colors, WAI-ARIA landmark region
- `EqPageSection` - Content section with title and description
- `EqDrawer` - Slide-in panel from any screen edge (Left, Right, Top, Bottom), four size presets, header/body/footer slots, backdrop overlay, close-on-Escape, WAI-ARIA dialog
- `EqGrid` - Feature-rich, type-safe data grid organism with:
  - Sorting (multi-column, custom comparators)
  - Column filters (per-column text) and global quick filter
  - Pagination with configurable page size
  - Row virtualization with dynamic row height measurement, split-table layout, synced colgroup, and info footer
  - Row selection (None, Single, Multi) with EqCheckbox integration
  - Column resizing with drag handles and min/max constraints
  - Bulk actions toolbar: delete, export (CSV/JSON/TXT/ODS), change status, clipboard copy, aggregation summaries, custom action slots
  - Drag-and-drop between grids via shared context (bidirectional, with playground demo)
  - Row reordering via drag handles (grip icon column, visual insertion indicator)
  - GridNavigation enum (Standard, Paginate, Virtualize)
  - Density presets (Compact, Normal, Comfortable)
  - Loading overlay and empty state
  - Full WAI-ARIA table semantics (aria-sort, aria-selected, aria-rowcount/colcount, aria-busy, live region announcements for reorder)
  - Full theme integration via CSS variables
- `EqFilePicker` - File/folder picker with drag-and-drop zone, single/multiple/folder modes, file type filter, max size validation, image thumbnails, upload progress, abstracted FilePickerBackend trait (web + native)
- `EqToolbar` - Mobile header with start / title / end slots and an optional secondary row for search, segmented controls, or a progress bar. Pure layout; consumer drives slot content
- `EqBottomNav` - Bottom-anchored mobile tab bar with icon + label items, count or dot badges, disabled state, WAI-ARIA tablist pattern. Active state owned by the consumer
- `EqMobileAppShell` - Three-region mobile layout (toolbar + scrollable body + bottom nav) with iOS safe-area padding via `env(safe-area-inset-*)`. Both fixed slots optional

### Accessibility
- **100% ARIA coverage** on all 45 implemented components (v0.4.2 + EqDeviceFrame, EqToolbar, EqBottomNav, EqMobileAppShell on `main`)
- WAI-ARIA patterns: Tree View, Accordion, Tablist, Radiogroup, Carousel, Data Grid, Dialog, Combobox, Switch
- Roving tabindex with keyboard navigation on all composite widgets
- Decorative elements marked with `aria-hidden`
- Live regions for dynamic content updates (pagination, drag-and-drop, toasts)

### Theming
- `EqTheme` - Theme enum with custom CSS support
- `EqThemeRenderer` - Runtime theme switcher using `document::Style`
- Theme context via `use_theme_provider()` / `use_theme()` / `set_theme()`
- Built-in themes (26): Unghosty, Burgundy, Gold, PurplePink, Monochrome, Watermelon, Sunset, Ocean, Spacetime, Gruvbox, Monokai, Hellas, Egypt, Dometrain, Catppuccin, Dracula, Nord, OneDark, RosePine, SolarizedDark, TokyoNight, Warcraft, SweetRush, Cloud, Synthwave, Limbotron (default)

### Infrastructure
- Co-located `_styles.rs` pattern for all components
- `theme.rs` shared style tokens + `merge_classes()` utility
- CSS variable system (`buttons.css`, `index.css`) - 57+ variables per theme covering core darks, labels, gradients, accents, states, borders, inputs, surfaces, code, buttons, interactive states, transitions, scrollbar, and grid
- Theme Showcase in EqPlayground - displays all CSS variables with color swatches, gradient previews, live button variants, and interactive card demos
- Tailwind CSS v4 with `@source` directives scanning `.rs` files
- `class` prop on every component for style overrides via `merge_classes()`
- EqPlayground - interactive component playground with prop controls, variant galleries, CSS documentation, and usage examples for every component
  - ComponentDescriptor pattern for extensible component registration
  - In-app Getting Started guide
  - Modular architecture (playground_helpers, playground_types)
- Published on crates.io: `eq_ui` v0.4.2, `eq_ui_build` v0.1.0
- Phosphor icon system with SVG path data constants and copy-on-demand workflow (ICON_REGISTRY.md)
- `components.json` - single source of truth for component metadata, ARIA status, platform support, and Blitz tier
- `eq_ui_macros` proc-macro crate with `#[playground(...)]` attribute and `PlaygroundEnum` derive

---

## Now - Active Development

These are the items currently being worked on or immediately planned.

### Quality (foundational, blocks v0.5)
- [ ] **Automated testing** - At minimum, smoke tests that every component renders without panicking, plus snapshot/visual regression on a critical subset (EqGrid, EqCalendar, EqVirtualList, EqModal). 41 components on crates.io with zero tests is a defect, not a "next priority."
- [ ] **CI/CD pipeline** - GitHub Actions running `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`, and a playground build on every push and PR. Required before more components ship.

### Platform
- [ ] **Blitz renderer compatibility** - CSS gap analysis filed upstream on DioxusLabs/blitz. Awaiting resolution of `@media(hover: hover)` (issue #252) and CSS transition support. Refactor `document::eval()` usage to use `web-sys-x` when available.

### Refactoring
- [ ] **EqCard macro unification** - The four EqCard sub-components are structurally identical wrappers. Unify via a declarative macro to reduce duplication.

### Documentation & Tooling
- [ ] **Integration guide** - Step-by-step instructions for adding eq_ui to a consuming project (git dependency, Tailwind `@source`, theme setup, `EqThemeRenderer` wiring).

---

## Next - Short-Term Priorities

Items to tackle once the "Now" batch stabilizes.

### Components
- [ ] **EqSkeleton** (Atom) - Loading placeholder with shimmer animation.
- [ ] **EqBadge** (Atom) - Small status indicator/label.

### Theming & Customization
- [ ] **Dark/light mode toggle** - Allow themes to define both dark and light variants with a toggle mechanism.
- [ ] **Theme creation guide** - Document how to create new built-in themes (CSS variable structure, naming conventions).

### Infrastructure
- [x] _Automated testing and CI/CD moved to **Now**. See the Quality section above._

---

## Later - Future Vision

Longer-term ideas and aspirations.

### Components

#### Form and input

- [ ] **EqForm** (Organism) - Form builder with validation, field groups, and submission handling.
- [ ] **EqValidation** (Pattern) - Field validation primitives. Most likely a trait derivable from a struct with annotated form-state fields, plus a `Validator` type that composes per-field rules and surfaces errors. Integrates with EqInput / EqSelect / EqRadioGroup / EqCheckbox so any form-builder picks up validation for free.
- [ ] **EqRangeSlider** (Molecule) - Two-handle slider for selecting a numeric range. Builds on EqSlider.
- [ ] **EqTimePicker** (Molecule) - Time selection with formatted display, minute / hour granularity, 12 and 24-hour modes.
- [ ] **EqOneTimeInput** (Molecule) - Multi-digit OTP entry with paste support and per-cell focus.
- [ ] **EqPasswordStrength** (Molecule) - Password strength meter with configurable policy rules and rule-by-rule feedback.
- [ ] **EqSearch** (Molecule) - Search input with autocomplete dropdown driven by a context-supplied result source.
- [ ] **EqRating** (Atom) - Star or symbol rating with 5, 7, and 10 variants.

#### Display and layout

- [ ] **EqTypography** (Atom) - Text wrapper exposing size, alignment, wrapping, overflow, and case-transform props for one-stop styling.
- [ ] **EqLoading** (Atom) - Spinner / activity indicator with size and color variants.
- [ ] **EqQr** (Atom) - QR code renderer from a string. SVG output, configurable size and error-correction level.
- [ ] **EqSidebar** (Organism) - Collapsible sidebar navigation.
- [ ] **EqBreadcrumb** (Molecule) - Navigation breadcrumb trail.
- [ ] **EqStepper** (Molecule) - Multi-step progress indicator.
- [ ] **EqCommandPalette** (Organism) - Keyboard-driven command palette (Cmd+K style).
- [ ] **EqContextMenu** (Molecule) - Right-click context menu.
- [ ] **EqHoverCard** (Molecule) - Hover card with rich content preview.
- [ ] **EqFreeGridColumns** (Organism) - Multi-column free-form layout where each column renders independent content. Closer to a magazine spread than a data grid.
- [ ] **EqNewspaper** (Organism) - Article-style layout combining EqFreeGridColumns with inline image placement for editorial content.

#### Editors and complex widgets

- [ ] **EqRichTextEditor** (Organism) - Rich text editor via JS engine init.
- [ ] **EqSignature** (Atom) - Canvas-based signature drawing.
- [ ] **EqPdfViewer** (Organism) - PDF rendering with page navigation and zoom. Likely wraps the browser's built-in viewer plus a thin control layer.
- [ ] **EqSchedule** (Organism) - Calendar event scheduler with day / week / month views, drag-to-create, and event editing. Significantly larger than EqCalendar.
- [ ] **EqSnippingTool** (Organism) - Screen-region capture overlay similar to OS-level snipping tools. Browser support varies; may need platform-specific backends.
- [ ] **EqCaptcha** (Organism) - Captcha challenge wrapper. Most likely an integration layer for hCaptcha or similar rather than rolling our own; security-sensitive features should not be DIY.

### Platform
- [ ] **Desktop-specific components** - Components optimized for Dioxus desktop (native menus, system tray integration).
- [ ] **Mobile-responsive patterns** - Ensure all components work well on mobile viewports. Add mobile-specific organisms (bottom sheet, swipe gestures).
- [ ] **Animation system** - Shared transition/animation utilities for component enter/exit states.
- [ ] **Blitz native rendering** - Full support once CSS gaps are resolved upstream.

### Ecosystem
- [ ] **Documentation site** - Dedicated docs site built with eq_ui itself (dogfooding).
- [ ] **Starter templates** - Project templates for web, desktop, and fullstack Dioxus apps using eq_ui.
- [ ] **Community themes** - Allow third-party theme contributions with a standard format.
- [ ] **EqPlayground public version** - Host EqPlayground as a public site where users can explore components, tweak props, and preview themes without cloning the repo.

---

## Release Policy

- **Versioning:** Semver. While the crate is `0.x`, **minor bumps may include breaking changes** (per Cargo's interpretation of semver pre-1.0); patch bumps are bug fixes and additive-only.
- **MSRV:** Rust **1.85** (Rust 2024 edition, matches `edition = "2024"` in `Cargo.toml`). Bumping MSRV requires a minor version bump and a note in the release row below.
- **Dioxus pin:** `dioxus = "=0.7.3"` is exact-pinned for now because Dioxus 0.7 is still moving. Loosen to a caret range only after a Dioxus minor without breaking changes.
- **Changelog:** Each release adds a row to the table below at the time the version is tagged. No row, no release.
- **Pre-publish checklist (every release):** Run the following in order. If any step fails, fix or annotate before tagging.

  ```bash
  cargo fmt --check
  cargo clippy --all-targets --all-features -- -D warnings
  cargo build --features playground --release
  cargo test --all-features
  cargo test --doc
  cargo audit
  cargo deny check
  cargo semver-checks check-release
  ```

  One-time installs: `cargo install cargo-audit`, `cargo install cargo-deny`, `cargo install cargo-semver-checks`. Optional sanity check: `cargo +nightly miri test` if any `unsafe` code is added in the future.
- **Cut criteria for v0.5:** Smoke tests cover every registered component (or have a documented reason to skip); `eq_ui::prelude` module exists; integration guide (`docs/`) published; the pre-publish checklist above is green on the release commit; CHANGELOG row for v0.5.0 added. CI pipeline, axe-core a11y scanning, and EqCard macro unification slip to v0.5.x or v0.6.0.

## Release History

| Version | Date | Highlights |
|---------|------|------------|
| v0.5.0 | May 2026 | Mobile suite (EqDeviceFrame, EqToolbar, EqBottomNav, EqMobileAppShell), 11 new components total, Synthwave theme, `eq_ui::prelude`, smoke tests on every component, runnable doctests, 5-file integration guide in `docs/`, manual `PartialEq` on `ComponentDescriptor`, 26 themes, 45 components. Full notes: [release-notes-0.5.0.md](./releaseDocs/release-notes-0.5.0.md) |
| v0.4.0 | April 2026 | Full ARIA accessibility, playground refactoring, EqRadioGroup, EqSwitch, 21 themes. Full notes: [release-notes-0.4.0.md](./releaseDocs/release-notes-0.4.0.md) |
| v0.3.0 | — | EqButton, EqGrid virtualization/reorder/export, EqCheckbox, multi-sort, themes |
| v0.2.0 | — | Initial crates.io publish, EqGrid, mobile playground support, video component |
| v0.1.1 | — | Initial release |

---

## Contributing

To run the playground locally:
```bash
dx serve --example playground --platform web
```

To update the library in a consuming project:
```bash
rm -rf ~/.cargo/git/checkouts/eq_ui-*
rm -rf ~/.cargo/git/db/eq_ui-*
cargo update -p eq_ui
```
