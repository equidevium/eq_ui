# Troubleshooting

Common failure modes when working with eq_ui. Each section lists
the symptom, the cause, and the fix.

If you hit something not covered here, file an issue at
[github.com/equidevium/eq_ui/issues](https://github.com/equidevium/eq_ui/issues)
with your eq_ui version, your `Cargo.toml`, and the visible
symptom.

---

## Components render unstyled

**Symptom.** eq_ui components show up in the DOM but with no
styling. Buttons look like plain HTML buttons, cards have no
background, the layout collapses.

**Cause.** Tailwind's JIT did not generate the utility classes that
eq_ui's source uses, usually because your Tailwind setup is not
scanning the eq_ui source files.

**Fix.** Make sure your `assets/tailwind.css` has a `@source`
directive that points at eq_ui's source folder:

```css
@import "tailwindcss";
@source "./src/**/*.rs";
@source "../path/to/eq_ui/src/**/*.rs";
```

If you depend on eq_ui from crates.io, the path is
`~/.cargo/registry/src/index.crates.io-*/eq_ui-0.5.*/src/**/*.rs`.
For a git or path dependency, use whatever the checkout location is.

If you are using eq_ui's bundled Tailwind output via
`UI_TAILWIND_CSS` and still see this, confirm the
`document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }` is
rendered before the components mount. A typo in the prop name will
not error.

---

## Theme doesn't switch when the dropdown changes

**Symptom.** `EqTheme::set_theme(...)` runs without error, the
dropdown shows the new selection, but the colors don't update.

**Cause.** One of three things:

- `EqTheme::use_theme_provider()` was not called at the root, so
  there is no theme context to write to.
- `EqThemeRenderer {}` is not in the rendered tree, so even though
  the signal updates, no `<style>` tag gets injected.
- `UI_INDEX_CSS` is missing, so the fallback variables don't load
  and `EqThemeRenderer`'s output has nothing to override.

**Fix.** Verify all three pieces in your root component, in order:

```rust,no_run
use eq_ui::prelude::*;

#[component]
fn App() -> Element {
    let _theme = EqTheme::use_theme_provider();   // <- must be present

    rsx! {
        document::Link { rel: "stylesheet", href: UI_INDEX_CSS }   // <- needed for fallback
        document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: UI_BUTTONS_CSS }

        EqThemeRenderer {}   // <- must be in tree

        // your layout
    }
}
```

---

## `cargo update` doesn't pick up changes from a git or path dependency

**Symptom.** You push a change to the eq_ui repo and your consuming
project still uses the old code after `cargo update`.

**Cause.** Cargo caches git checkouts and won't refresh them just
because the upstream branch moved.

**Fix.**

```bash
rm -rf ~/.cargo/git/checkouts/eq_ui-*
rm -rf ~/.cargo/git/db/eq_ui-*
cargo update -p eq_ui
```

For a path dependency, `cargo update` should pick up changes
automatically. If it doesn't, run `cargo clean` in the consuming
project and rebuild.

---

## `dx serve` errors on startup

**Symptom 1.** `dx serve` runs but the wrong tool starts (often a
Deno-related shim) or the command fails immediately.

**Cause.** A globally installed `deno` binary ships its own `dx`
shim that takes precedence in `PATH`. The Dioxus CLI is at
`~/.cargo/bin/dx`.

**Fix.** Invoke Dioxus's CLI by absolute path, or fix your `PATH`
order:

```bash
~/.cargo/bin/dx serve --platform web --port 3030
```

**Symptom 2.** `dx serve` reports a port already in use.

**Fix.** Pass `--port <number>` with a free port. The default is
8080.

**Symptom 3.** WASM bundle doesn't update on save.

**Fix.** The `dx` watcher only picks up `.rs` changes. CSS edits in
`assets/tailwind.css` need either a separate Tailwind watcher or a
full `dx serve` restart.

---

## Desktop platform: Wry quirks

eq_ui works on `dx serve --platform desktop` because Wry is a
webview internally. The same components render, but a few things
behave differently from web:

- **`document::eval()` runs in the webview.** Most components that
  use eval (file inputs, scroll-to-index, focus management) work,
  but some webview versions don't expose `clipboard.writeText`. If
  copy buttons silently no-op on desktop, this is why.
- **No `web_sys` access.** eq_ui never imports `web_sys` in
  components, but if your own code does, it won't compile for
  desktop. Use `document::eval()` or feature-gate per platform.
- **File pickers use the OS dialog.** `EqFilePicker`'s drag-and-drop
  zone works on web but does nothing on desktop where Wry doesn't
  forward drag events. Use the click-to-open fallback.
- **Window chrome.** `dx serve --platform desktop` opens a borderless
  Wry window by default. Add window config in your Dioxus
  `LaunchBuilder` if you need title bars or specific dimensions.

---

## Mobile platform: known limitations

eq_ui targets mobile via Dioxus mobile (Android + iOS, Wry-based).
The components render, but specific behaviors are missing:

- **`EqCarousel` has no swipe gesture.** It only advances via the
  arrow buttons. Touch users will not get the expected swipe-to-
  advance behavior. A fix is on the roadmap.
- **`EqTooltip` is hover-triggered.** Touch devices don't have hover.
  Tooltips will appear stuck or not at all on mobile. Avoid
  `EqTooltip` for content that mobile users need to read; consider
  inline help text instead.
- **`EqDropdown` and `EqDatePicker` popups can clip the viewport.**
  Neither has viewport-aware positioning. On a 393px-wide viewport,
  a dropdown anchored near the right edge will cut off horizontally.
  A fix is on the roadmap.
- **`EqHeader` and `EqNavbar` don't collapse to a hamburger.** Their
  horizontal nav overflows on mobile. For mobile, use `EqToolbar`
  inside `EqMobileAppShell` instead.
- **`EqGrid` doesn't fit at mobile widths.** Data tables are
  desktop-first. Use a custom card-list rendering for mobile.
- **`EqDeviceFrame` is presentation-only.** It does not simulate
  touch events, safe-area insets, or any iOS API. It's a visual
  preview wrapper for the playground, not a testing harness.

These are known limitations, not bugs to file. When the fixes
land, the ROADMAP will note them.

---

## Component compiles but doesn't behave

A few component-level gotchas that aren't obvious from the type
signatures:

- **`EqInput` requires `oninput`.** The handler is non-optional. If
  you don't actually need to react to input, pass
  `oninput: move |_| {}` rather than skipping the prop.
- **`EqTab` uses index-based active state.** `active: usize` is the
  index into the items vector. Reorder items and your active
  pointer points at a different tab.
- **`EqBottomNav` uses id-based active state.** `active: String` is
  the id of the active item, not the index. Stable across reorders.
- **`EqModal` and `EqDrawer` need their `open` signal in the
  consumer.** They don't own state; close-on-Escape calls
  `on_close` and the consumer is responsible for setting `open` to
  `false`.
- **`EqGrid` is generic over the row type.** Type inference works
  through the column-def vec, so you usually don't need turbofish.
  If inference fails, use `EqGrid::<MyRowType> { ... }`.
- **`EqDeviceFrame` constrains its inner content to a fixed size.**
  If a component you put inside expects to fill the viewport, it
  will fill the device frame's screen area instead. That's the
  point.

---

## Visual regressions after upgrading eq_ui

**Symptom.** You bumped eq_ui's version and a layout that previously
looked correct is now broken.

**Cause.** Either a CSS variable name changed, a component prop
changed, or a default value changed.

**Fix.**

1. Read the [ROADMAP.md](../ROADMAP.md) release table for the
   version range you bumped across. Breaking changes are noted there.
2. Run `cargo semver-checks check-release` (install with
   `cargo install cargo-semver-checks`) to detect API changes that
   could explain the regression.
3. Compare the variable list in [theming.md](./theming.md) against
   any custom theme CSS you wrote; renamed variables silently fall
   back to defaults.

---

## When to file an issue vs check this guide

File an issue when:

- A component panics at runtime.
- A documented prop or API doesn't behave as documented.
- Theme switching produces visibly broken output for a built-in
  theme.
- A test in the smoke-test suite passes locally but fails for you.

Check this guide first when:

- Components render but look wrong.
- Build fails with a Tailwind / CSS / asset error.
- A mobile gesture or interaction doesn't work (most of those are
  known limitations above).
- `cargo update` does something surprising.
