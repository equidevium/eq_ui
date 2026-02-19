# eq_ui

A portable component library for [Dioxus](https://dioxuslabs.com/) 0.7, built around atomic design. Drop it into any Dioxus project — web, desktop, or mobile — and get a consistent set of styled building blocks out of the box.

## What's in the box

The crate is organized into three layers, each building on the one below:

**Atoms** — the smallest pieces. A text element, a label, a link, an input, an icon wrapper.

**Molecules** — small compositions. Right now that's the card (`EqCard` with header/body/footer slots).

**Organisms** — page-level structures. Header, footer, hero section, page sections, the full app shell, and a navbar.

There's also a **theme** module with shared Tailwind utility constants for spacing, borders, surfaces, shadows, buttons, and more. Components pull from the theme internally, but you can use the constants directly in your own layouts too.

## Quick start

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
eq_ui = { path = "../eq_ui" }  # or wherever you put it
```

Then wire up the CSS assets in your root component:

```rust
use eq_ui::{UI_INDEX_CSS, UI_COLORS_CSS, UI_BUTTONS_CSS};

rsx! {
    document::Link { rel: "stylesheet", href: UI_INDEX_CSS }
    document::Link { rel: "stylesheet", href: UI_COLORS_CSS }
    document::Link { rel: "stylesheet", href: UI_BUTTONS_CSS }
    // ... your app
}
```

If you're using Tailwind, add a `@source` directive so it picks up the classes used inside eq_ui:

```css
@import "tailwindcss";
@source "../path/to/eq_ui/src/**/*.rs";
```

## Using the components

No all of the components are documented yet, but here's a quick taste of the API. The full docs are coming soon.:

```rust
use eq_ui::atoms::{EqText, TextVariant, EqInput, InputKind, EqLabel, EqLink, EqIcon, IconSize};
use eq_ui::molecules::{EqCard, EqCardHeader, EqCardBody, EqCardFooter};
use eq_ui::organisms::{EqAppShell, EqHeader, EqFooter, EqHeroShell, EqPageSection, EqNavbar};
use eq_ui::theme;  // shared constants like CONTAINER_LAYOUT, BTN_PRIMARY, etc.
```

### Atoms

```rust
// Text with semantic HTML — picks the right tag automatically
EqText { variant: TextVariant::H1, "Page title" }
EqText { "Just a paragraph." }  // defaults to Body

// Form label tied to an input
EqLabel { for_id: "email", "Email address" }

// Plain anchor link
EqLink { href: "/about".into(), "About us" }

// Input with kind variants
EqInput {
    kind: InputKind::Email,
    placeholder: "you@example.com",
    name: "email",
    oninput: move |e| { /* handle it */ },
}

// Icon wrapper — pass an SVG or image as children
EqIcon { size: IconSize::Lg, muted: true,
    // your svg or img here
}
```

### Molecules

```rust
EqCard {
    EqCardHeader { "Card title" }
    EqCardBody { "Some content here." }
    EqCardFooter { "Footer info" }
}
```

### Organisms

The organisms are designed to be **platform-agnostic**. They accept `Element` props instead of depending on any specific router, so you can use the same components across web, desktop, and mobile targets.

```rust
// App shell — pass your own header, footer, and page content
EqAppShell {
    header: rsx! { EqHeader { site_title: "My App", nav: rsx! { /* your nav items */ } } },
    footer: rsx! { EqFooter { copyright_holder: "Acme Inc", year: 2026 } },
    // children become the main content area
    p { "Hello world" }
}

// Hero section
EqHeroShell {
    title: "Welcome",
    subtitle: "Something cool goes here",
    actions: rsx! { button { "Get started" } },
}

// Page section with optional title/description
EqPageSection {
    id: "features",
    title: "Features",
    description: "Here's what we've got.",
    // children go in the body
}
```

For the header specifically, you provide the nav items as `<li>` elements and `EqHeader` wraps them in the right markup:

```rust
EqHeader {
    site_title: "My Site",
    nav: rsx! {
        li { a { href: "/", "Home" } }
        li { a { href: "/about", "About" } }
    },
}
```

## Project structure

```
src/
  lib.rs              — crate root, CSS asset exports
  theme.rs            — shared Tailwind class constants
  atoms/
    eq_text.rs        — text with semantic variants (h1-h3, body, muted, etc.)
    eq_label.rs       — form label
    eq_link.rs        — anchor link
    eq_input.rs       — input/textarea with kind variants
    eq_icon.rs        — icon wrapper with size variants
    *_styles.rs       — co-located style constants for each atom
  molecules/
    eq_card.rs        — card with header/body/footer slots
    eq_card_styles.rs
  organisms/
    eq_app_shell.rs   — full page layout (header + main + footer)
    eq_header.rs      — site header with brand + nav
    eq_footer.rs      — footer with link groups + copyright
    eq_hero_shell.rs  — hero banner with title/subtitle/actions
    eq_page_section.rs — titled content section
    eq_navbar.rs      — horizontal nav bar
    *_styles.rs       — co-located style constants for each organism
assets/
  theme/              — base CSS (colors, buttons, index)
  styling/            — component-specific CSS (navbar)
  tailwind.css        — Tailwind entry point with @source directives
```

## Style architecture

Each component keeps its Tailwind classes in a sibling `_styles.rs` file (e.g. `eq_text.rs` + `eq_text_styles.rs`). Constants that are shared across multiple components — spacing, borders, surfaces, button variants — live in `theme.rs`.

The theme uses CSS custom properties for colors (`--color-primary-dark`, `--color-label-primary`, etc.), defined in `assets/theme/colors.css`. This means you can swap the color palette by overriding those variables without touching any Rust code.

## Dependencies

Just one: `dioxus = "0.7.1"`. That's it. No other crates needed.

## Running the showcase

The crate includes a built-in showcase you can use to browse all components visually:

```bash
dx serve --example showcase --platform web
```

This opens a page showing every atom, molecule, and organism with live examples.

## Adding new components

Follow the pattern — every component gets two files:

1. `eq_whatever.rs` — the component itself
2. `eq_whatever_styles.rs` — its Tailwind class constants

Register both in the parent `mod.rs`, and re-export the component (plus any enums) from there. If the component introduces a variant enum (like `TextVariant` or `InputKind`), export it alongside the component so callers get everything from one import.


## Future additions

1. More components! Form controls (select, checkbox, radio), more molecules (media object, list group), more organisms (sidebars, dashboards, etc.)
2. Better documentation — full API docs with examples for every component, plus a style guide for how to use the theme constants in your own layouts.
3. Accessibility improvements — better ARIA attributes, keyboard navigation, etc.
4. Improved theming — maybe a dark mode toggle, or support for multiple themes out of the box.
5. Improved theming — allow to the developer to set the theming from outside the library.
