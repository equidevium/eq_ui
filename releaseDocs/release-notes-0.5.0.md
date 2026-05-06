# eq_ui v0.5.0

Mobile suite, eleven new components, runnable docs, smoke tests on every component, the first integration guide, and a developer prelude. The release jumps directly from v0.4.0; v0.4.1 and v0.4.2 were never tagged or published.

## New components (11)

### Atoms (4)

**EqAvatar** - User avatar with image, initials fallback, icon fallback, four sizes (Sm, Md, Lg, Xl), online/offline/busy status dot, selection ring.

**EqSlider** - Range slider built on the native `<input type="range">`, accent-color theming, three sizes (Sm, Md, Lg), optional value label, disabled state.

**EqTooltip** - Hover/focus tooltip with four positions (Top, Bottom, Left, Right), pure CSS positioning, ARIA describedby, keyboard accessible. Note: hover-only; not usable on touch devices. See [docs/troubleshooting.md](../docs/troubleshooting.md).

**EqSelect** - Themed dropdown with search, placeholder, disabled options, keyboard navigation, WAI-ARIA combobox pattern, check mark on selected option.

### Molecules (5)

**EqCta** - Call-to-action section with title, description, action slot, and two layout modes (Inline, Centered).

**EqModal** - Modal dialog with backdrop, five size presets, close-on-backdrop and close-on-Escape, focus management, WAI-ARIA dialog pattern.

**EqToastList** - Toast notification stack with four severity levels, auto-dismiss via `setTimeout`, six position anchors, manual close, WAI-ARIA status/alert pattern.

**EqDropdown** - Dropdown menu with selectable items, separators, disabled state, keyboard navigation (arrows / Enter / Escape), two positions, close-on-outside-click.

**EqDatePicker** - Date picker with calendar popup, month navigation, today highlight, formatted display, pure-Rust date math, WAI-ARIA dialog pattern.

**EqCalendar** - Standalone calendar with month and week views, event dots, timed events, month/year picker drill-down, WAI-ARIA grid pattern.

**EqVirtualList** - High-performance windowed list rendering only visible items. Fixed-size rows, overscan buffer, scroll-to-index, sticky section headers, vertical and horizontal modes.

**EqDeviceFrame** - Static iPhone 16 / 16 Pro chrome with Dynamic Island, status bar, home indicator, painted side buttons. Pure presentation wrapper for showcasing mobile components in the playground; no event callbacks. Tier 1 Blitz-ready.

### Organisms (4)

**EqDrawer** - Slide-in panel from any screen edge (Left, Right, Top, Bottom), four size presets, header/body/footer slots, backdrop overlay, close-on-Escape, WAI-ARIA dialog.

**EqFilePicker** - File and folder picker with drag-and-drop zone, single/multiple/folder modes, file-type filter, max-size validation, image thumbnails, upload progress, abstracted `FilePickerBackend` trait (web + native).

**EqToolbar** - Mobile header with start / title / end slots and an optional secondary row for search, segmented controls, or progress bars. All slots optional. Pure layout; consumer drives slot content.

**EqBottomNav** - Bottom-anchored mobile tab bar with icon + label items, count or dot badges, disabled state, WAI-ARIA tablist pattern. Active state owned by the consumer via `String` ids (stable across reordering).

**EqMobileAppShell** - Three-region mobile layout: fixed toolbar, scrollable body, fixed bottom nav. Both fixed slots optional. Safe-area padding via `env(safe-area-inset-*)`. Distinct from `EqAppShell` because the structural concerns differ.

Total component count: **45** (19 atoms + 14 molecules + 12 organisms).

## Themes

One new built-in theme: **Synthwave**. Violet base with magenta primary, cyan complement, sunset highlights, 80s tricolor gradients, pill corners (matches the Limbotron / Cloud structure).

Total theme count: **26**.

## Mobile-friendly playground tree

`ComponentDescriptor` gained a `mobile_friendly: bool` field. Set it via the `#[playground(mobile_friendly, ...)]` attribute. Components with the flag set render with a small phone icon next to the name in the playground sidebar tree, so consumers can find mobile-tested components at a glance. Currently set on `EqDeviceFrame`, `EqToolbar`, `EqBottomNav`, and `EqMobileAppShell`.

## Developer experience

**`eq_ui::prelude`**. New module re-exporting `dioxus::prelude::*` plus the most-used eq_ui types (CSS asset constants, `EqTheme`, common atoms / molecules / organisms). One-line import for the typical consumer:

```rust
use eq_ui::prelude::*;
```

**Smoke tests**. Every registered component now has at least one `#[cfg(test)] mod tests` smoke test using `VirtualDom::new(...).rebuild_in_place()`. Run via `cargo test --lib`. Components with builders, formatters, or pure helpers also got tests for those.

**Doctests**. 21 examples in `///` and `//!` doc comments are now runnable as `,no_run` doctests, using the new prelude where possible. Run via `cargo test --doc`. Catches API drift in documentation.

**Integration guide**. Five-file guide under `docs/`:

- `getting-started.md` - walkthrough from `cargo new` to a running app with a theme switcher.
- `migrating.md` - adding eq_ui to an existing Dioxus or Tailwind project.
- `theming.md` - the 60-variable contract, all 26 built-in themes, custom-theme authoring.
- `troubleshooting.md` - common failure modes (Tailwind `@source`, theme switching, mobile limitations, Wry quirks) with cause and fix.
- `README.md` - index.

**Release-policy section in ROADMAP**. Documents semver discipline for 0.x, the MSRV (Rust 1.85, edition 2024), the Dioxus exact-pin, the changelog convention, and the pre-publish checklist.

## Breaking changes

These break consumer code that depended on the previous public API. Pre-1.0 minor bumps are allowed to break, but worth being explicit:

**`ComponentDescriptor` gained a `mobile_friendly: bool` field.** Anyone constructing the struct as a literal in their own crate must add the field. The `#[playground(...)]` macro path is unaffected.

**`ComponentDescriptor` no longer derives `PartialEq` automatically.** A manual `impl PartialEq` was added that compares only the data fields. The function-pointer fields (`style_tokens`, `usage_examples`, `render_demo`, `render_gallery`) are now intentionally excluded. Two descriptors with the same id, name, category, description, and mobile-friendly flag but different render functions are now considered equal. In practice nobody relied on the old behavior because function-pointer comparisons were unreliable across codegen units.

**`EqTheme` gained a `Synthwave` variant.** Exhaustive `match` statements on `EqTheme` in consumer code break. Add a `Synthwave` arm or a wildcard.

## Quality and infrastructure

- All 95 clippy lints across the codebase fixed. `cargo clippy --features playground -- -D warnings` is now green.
- `cargo-semver-checks` documented in the pre-publish checklist.
- Removed unused imports, redundant closures, redundant locals, and other accumulated cruft.
- `EqGrid` private internal helpers carry `#[allow(clippy::too_many_arguments)]` with a one-line rationale; refactoring those to a struct is deferred.

## Documentation

- `README.md` Testing section added, explaining `cargo test`, `cargo test --lib`, `cargo test --tests`, `cargo test --doc`.
- `ROADMAP.md` Quality section moved test/CI work to **Now**.
- ROADMAP gains a Release Policy section with versioning, MSRV, Dioxus pin rationale, and cut criteria for v0.5.

## Known limitations (mobile)

Documented in `docs/troubleshooting.md`. Not blockers, not bugs. Tracked for v0.5.x or v0.6:

- `EqCarousel` has no swipe gesture; advances via arrow buttons only.
- `EqDropdown` and `EqDatePicker` popups have no viewport-aware positioning; can clip on narrow viewports.
- `EqHeader` and `EqNavbar` don't collapse to a hamburger; horizontal nav overflows at mobile widths. Use `EqToolbar` inside `EqMobileAppShell` instead.
- `EqGrid` doesn't fit at mobile widths; use a custom card-list rendering.
- `EqDeviceFrame` is presentation-only; does not simulate touch events or safe-area insets.

## Migration

For most consumers, the upgrade is a `cargo update -p eq_ui` and adding the `mobile_friendly` field to any custom `ComponentDescriptor` literals.

If you author custom themes via `set_custom_theme(...)`, no changes needed. If you exhaustively `match` on `EqTheme`, add a `Synthwave` arm.

If you've been depending on `PartialEq` on `ComponentDescriptor` to compare render functions, that comparison is now data-only. The old behavior was unreliable anyway.

Full integration walkthrough in [docs/getting-started.md](../docs/getting-started.md). Migration scenarios in [docs/migrating.md](../docs/migrating.md).
