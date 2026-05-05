# Getting started with eq_ui

This guide takes you from an empty folder to a running Dioxus app
that renders an eq_ui layout with a working theme switcher.

It assumes you already know Dioxus 0.7 RSX syntax and how the `dx`
CLI works. If you don't, read the
[Dioxus 0.7 quickstart](https://dioxuslabs.com/learn/0.7/) first;
this guide will not re-teach Dioxus.

> Throughout this file, blockquotes call out things that newcomers
> might want to chase up but that aren't strictly required to follow
> the walkthrough.

---

## What you need before starting

- **Rust 1.85 or newer.** eq_ui uses edition 2024 and won't compile
  on older toolchains. Check with `rustc --version`.
- **The `dx` CLI.** Install with `cargo install dioxus-cli`. If you
  don't already have it, this takes a few minutes.
- **A web target.** Add the wasm target if you don't have it:
  `rustup target add wasm32-unknown-unknown`.

> Desktop and mobile targets work too. Substitute `--platform desktop`
> or `--platform android` / `--platform ios` for the `web` flag in
> `dx serve` later in this guide. Cross-platform notes live in
> [troubleshooting.md](./troubleshooting.md).

---

## Step 1. Create the project

```bash
cargo new --bin my_app
cd my_app
```

Open `Cargo.toml` and add the dependencies:

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2024"

[dependencies]
dioxus = { version = "=0.7.3", features = ["web"] }
eq_ui = "0.5"
```

> The `=` in front of the Dioxus version is intentional. eq_ui pins
> against an exact Dioxus version because Dioxus 0.7 is still
> changing rapidly and minor bumps have introduced breaking changes
> in the past. Match the pin in your own `Cargo.toml` to avoid
> resolver fights.

If you want desktop or mobile, swap the Dioxus features:

```toml
# Desktop
dioxus = { version = "=0.7.3", features = ["desktop"] }

# Mobile (Android + iOS)
dioxus = { version = "=0.7.3", features = ["mobile"] }
```

> You can enable multiple features simultaneously
> (`features = ["web", "desktop"]`) but you still pick the platform
> at runtime via `dx serve --platform <name>`.

---

## Step 2. Set up Tailwind

eq_ui uses Tailwind v4 utility classes. You need a Tailwind setup
that knows about both your project's `.rs` files and eq_ui's source
files, otherwise the JIT will not generate the classes that eq_ui
emits.

Create `assets/tailwind.css` in your project:

```css
@import "tailwindcss";

@source "./src/**/*.rs";
@source "../eq_ui/src/**/*.rs";
```

The two `@source` directives tell Tailwind to scan both your code
and eq_ui's code for class names. If you skip the second one, eq_ui
components will render unstyled because Tailwind won't know which
classes to emit.

> The path in the second `@source` depends on where eq_ui lives. If
> you depend on it from crates.io, point at
> `~/.cargo/registry/src/index.crates.io-*/eq_ui-0.5.*/src/**/*.rs`.
> If you use a git or path dependency, point at the source folder of
> that checkout. The standalone Tailwind CLI handles globs in
> `@source` correctly.

Run the Tailwind compiler so the file builds at least once:

```bash
npx @tailwindcss/cli@4 -i assets/tailwind.css -o assets/tailwind-built.css --watch
```

> If you don't want to run a separate watcher, you can let `dx serve`
> handle the rebuild loop and just keep the input file as
> `assets/tailwind.css`. The `dx` build picks up the asset that eq_ui
> exports as `UI_TAILWIND_CSS` in the next step, so the
> hand-compiled `tailwind-built.css` is not strictly required if all
> you use is the eq_ui-bundled stylesheet.

---

## Step 3. Wire eq_ui's CSS into the page

eq_ui ships three CSS bundles as embedded assets. Reference them
once at the root of your app via Dioxus's `document::Link` so they
are emitted into the HTML head:

```rust
use eq_ui::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: UI_INDEX_CSS }
        document::Link { rel: "stylesheet", href: UI_BUTTONS_CSS }

        // Your layout goes here
        EqText { variant: TextVariant::H1, "Hello eq_ui" }
    }
}
```

What each one does:

| Asset | Provides |
|---|---|
| `UI_TAILWIND_CSS` | The compiled Tailwind output. Required. |
| `UI_INDEX_CSS` | Default fallback CSS variables (used until a theme is applied). |
| `UI_BUTTONS_CSS` | Button gradient definitions and `@property` declarations. |

All three are needed for a complete render. The order doesn't
matter; later sheets override earlier ones via CSS variable
specificity, and eq_ui ships its theming so the variables stay
consistent.

> `eq_ui::prelude::*` re-exports `dioxus::prelude::*` plus the
> CSS asset constants and the most-used components. If you'd rather
> import explicitly, the assets live at `eq_ui::UI_TAILWIND_CSS` etc.

---

## Step 4. Set up the theme

Themes in eq_ui are CSS files that override variables on `:root`.
The theme is selected at runtime via a Dioxus signal, and a small
runtime component injects the matching CSS into a `<style>` tag.

You wire two pieces in `App()`:

```rust
use eq_ui::prelude::*;

#[component]
fn App() -> Element {
    // 1. Provide the theme context. Default is Limbotron.
    let _theme = EqTheme::use_theme_provider();

    rsx! {
        document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: UI_INDEX_CSS }
        document::Link { rel: "stylesheet", href: UI_BUTTONS_CSS }

        // 2. Render the theme's CSS as a <style> tag. Without this,
        //    every theme variable falls back to whatever UI_INDEX_CSS
        //    set, and theme switching does nothing.
        EqThemeRenderer {}

        EqText { variant: TextVariant::H1, "Hello eq_ui" }
    }
}
```

`EqTheme::use_theme_provider()` must be called at the root of your
app. It uses Dioxus's context system, so any descendant component
that calls `EqTheme::use_theme()` or `EqTheme::set_theme(...)` walks
up the tree to find this provider. Calling the provider lower down
will scope the theme to that subtree, which is rarely what you want.

`EqThemeRenderer` is a tiny component that watches the theme signal
and emits a `<style>` element with the active theme's CSS. It must
live in the rendered tree (typically right after the asset links).
If you remove it, theme switching becomes a no-op.

> The 26 built-in themes live in `eq_ui::eq_theme::EqTheme`. Defaults
> to `Limbotron`. Custom CSS themes are also supported via
> `EqTheme::set_custom_theme(css_string)`. Full theme reference is in
> [theming.md](./theming.md).

---

## Step 5. Render your first component

eq_ui follows atomic design: atoms (text, buttons, inputs),
molecules (cards, modals, dropdowns), organisms (page shells,
headers, grids).

The fastest way to a working layout is `EqAppShell`, which wraps a
header, page body, and footer:

```rust
use eq_ui::prelude::*;

#[component]
fn App() -> Element {
    let _theme = EqTheme::use_theme_provider();

    rsx! {
        document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: UI_INDEX_CSS }
        document::Link { rel: "stylesheet", href: UI_BUTTONS_CSS }
        EqThemeRenderer {}

        EqAppShell {
            header: rsx! {
                EqHeader {
                    site_title: "My App",
                    nav: rsx! {
                        li { a { href: "/", "Home" } }
                        li { a { href: "/about", "About" } }
                    },
                }
            },
            footer: rsx! { EqFooter {} },

            // Page content as children
            EqPageSection {
                id: "intro",
                title: "Welcome",
                description: "This is a starter page rendered by eq_ui.",
                EqText {
                    "Replace this with whatever your app actually does. "
                    "See "
                    a { href: "https://github.com/equidevium/eq_ui", "the eq_ui repo" }
                    " for the full component catalog."
                }
            }
        }
    }
}
```

The header takes `site_title` plus a `nav` slot containing `<li>`
elements. The footer renders a default link group layout. Page
content goes as children of `EqAppShell` and is wrapped in a
constrained-width container automatically.

> The full prop API for each component is in the playground. Run
> `dx serve --example playground --features playground --platform web`
> from inside the eq_ui repo to browse it interactively.

---

## Step 6. Add a theme switcher

Switching themes at runtime is one signal write away. A minimal
switcher renders a `<select>` populated from
`EqTheme::built_in_variants()`:

```rust
use eq_ui::prelude::*;

#[component]
fn ThemeSwitcher() -> Element {
    let mut theme = EqTheme::use_theme();

    rsx! {
        select {
            onchange: move |evt: Event<FormData>| {
                let name = evt.value();
                if let Some((_, variant)) = EqTheme::built_in_variants()
                    .into_iter()
                    .find(|(n, _)| *n == name)
                {
                    theme.set(variant);
                }
            },
            for (name, _variant) in EqTheme::built_in_variants() {
                option { value: "{name}", "{name}" }
            }
        }
    }
}
```

Drop `ThemeSwitcher {}` somewhere in your header or settings panel
and the dropdown will flip the active theme as the user selects.

> The full theming model (custom CSS themes, per-instance overrides,
> the variable contract) is in [theming.md](./theming.md). This
> guide only covers the built-in switch case.

---

## Step 7. Build and run

For web:

```bash
dx serve --platform web
```

The `dx` CLI compiles to wasm, serves the bundle, and opens a
browser at `http://localhost:8080`. Hot reload picks up `.rs`
changes; CSS changes need the Tailwind watcher running in another
terminal (or a full restart).

For desktop:

```bash
dx serve --platform desktop
```

This produces a native window via Wry (the same webview Tauri uses),
not a browser. The same eq_ui components render unchanged because
Wry is a webview internally.

> If you've installed `deno` globally and `dx serve` errors on
> startup, the eq_ui repo notes a workaround: invoke `dx` with its
> full path: `~/.cargo/bin/dx serve --platform web --port 3030`.
> The conflict is between Deno's `dx` shim and the Dioxus CLI of
> the same name.

---

## Complete starter listing

Drop these two files in a fresh project and `dx serve --platform web`
will give you a running eq_ui app with a theme switcher.

`Cargo.toml`:

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2024"

[dependencies]
dioxus = { version = "=0.7.3", features = ["web"] }
eq_ui = "0.5"
```

`src/main.rs`:

```rust
use eq_ui::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let _theme = EqTheme::use_theme_provider();

    rsx! {
        document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: UI_INDEX_CSS }
        document::Link { rel: "stylesheet", href: UI_BUTTONS_CSS }
        EqThemeRenderer {}

        EqAppShell {
            header: rsx! {
                EqHeader {
                    site_title: "My App",
                    nav: rsx! {
                        li { ThemeSwitcher {} }
                    },
                }
            },
            footer: rsx! { EqFooter {} },

            EqPageSection {
                id: "intro",
                title: "Welcome",
                description: "Starter page rendered with eq_ui.",
                EqText {
                    "This is the body. Replace with your real content."
                }
            }
        }
    }
}

#[component]
fn ThemeSwitcher() -> Element {
    let mut theme = EqTheme::use_theme();

    rsx! {
        select {
            onchange: move |evt: Event<FormData>| {
                let name = evt.value();
                if let Some((_, variant)) = EqTheme::built_in_variants()
                    .into_iter()
                    .find(|(n, _)| *n == name)
                {
                    theme.set(variant);
                }
            },
            for (name, _variant) in EqTheme::built_in_variants() {
                option { value: "{name}", "{name}" }
            }
        }
    }
}
```

`assets/tailwind.css`:

```css
@import "tailwindcss";

@source "./src/**/*.rs";
@source "../eq_ui/src/**/*.rs";
```

Run it:

```bash
dx serve --platform web
```

You should see a header with site title and theme dropdown, a body
with a section title and paragraph, and a footer. Pick any theme
from the dropdown and the colors update immediately.

---

## Where to go next

- **Adding eq_ui to a project that already uses Dioxus and/or
  Tailwind?** Read [migrating.md](./migrating.md). Covers `@source`
  collisions, CSS variable conflicts, and incremental adoption.
- **Want to author a custom theme?** [theming.md](./theming.md) has
  the full variable contract, the 26 built-in themes, and the
  custom-CSS API.
- **Something broke?** [troubleshooting.md](./troubleshooting.md) is
  a list of failure modes and fixes.
- **Browse the components.** Clone the eq_ui repo and run
  `dx serve --example playground --features playground --platform web`.
  Every component has interactive prop controls, source examples,
  and a theme picker.

If something in this guide didn't work, please file an issue at
[github.com/equidevium/eq_ui/issues](https://github.com/equidevium/eq_ui/issues)
with the eq_ui version, your `Cargo.toml`, and the error you saw.
The integration story is still maturing and friction reports help us
prioritize.
