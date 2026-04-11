# eq_ui

A portable component library for [Dioxus](https://dioxuslabs.com/) 0.7, built around atomic design. Drop it into any Dioxus project - web, desktop, or mobile - and get a consistent set of styled building blocks out of the box.

https://github.com/user-attachments/assets/4ea4f561-4581-481d-bc27-c2f5a2879998

## What's in the box

The crate is organized into three layers, each building on the one below:

**Atoms** — the smallest pieces. Text, labels, links, inputs, icons, images, checkboxes, buttons, scrollable containers, dividers, and video — each with variant enums for size, kind, and appearance.

**Molecules** — small compositions. Cards with header/body/footer slots, image cards with caption modes (below or overlay), a generic content carousel, collapsible tree views, and accordion panels.

**Organisms** — page-level structures. A sticky header with backdrop blur, footer with link groups, hero section with optional background images and overlay, page sections, the full app shell, a navbar, and a feature-rich data grid with sorting, filtering, pagination, virtualization, row selection, bulk actions, drag-and-drop, and export.

**Theming** — 21 built-in color themes plus custom CSS support. Switch themes at runtime with a single function call or let users pick from a dropdown.

The **theme** module provides shared Tailwind utility constants for spacing, borders, surfaces, shadows, buttons, and more. Components pull from the theme internally, but the constants are also available for use in your own layouts.

## Quick start

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
# From GitHub:
eq_ui = { git = "https://github.com/equidevium/eq_ui", branch = "main" }

# Or from a local path:
eq_ui = { path = "../eq_ui" }
```

Then wire up the CSS assets and theming in your root component:

```rust
use eq_ui::{UI_TAILWIND_CSS, UI_INDEX_CSS, UI_BUTTONS_CSS};
use eq_ui::eq_theme::EqTheme;

#[component]
fn App() -> Element {
    // Initialize the theme context
    let _theme = EqTheme::use_theme_provider();

    rsx! {
        document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: UI_INDEX_CSS }
        document::Link { rel: "stylesheet", href: UI_BUTTONS_CSS }

        // Injects the active theme's CSS at runtime
        EqThemeRenderer {}

        // ... your app layout
    }
}
```

If you're using Tailwind, add a `@source` directive so it picks up the classes used inside eq_ui:

```css
@import "tailwindcss";
@source "../path/to/eq_ui/src/**/*.rs";
```

### Updating the library

When you push changes to eq_ui and want your consuming project to pick them up:

```bash
rm -rf ~/.cargo/git/checkouts/eq_ui-*
rm -rf ~/.cargo/git/db/eq_ui-*
cargo update -p eq_ui
```

## Using the components

```rust
use eq_ui::atoms::{EqText, TextVariant, EqInput, InputKind, EqLabel, EqLink, EqIcon, IconSize, EqImage, AtomImageSize, AspectRatio, ObjectFit, EqCheckbox, CheckboxState, EqButton, ButtonVariant, ButtonSize, EqDivider, EqScrollableSpace, EqVideo};
use eq_ui::molecules::{EqCard, EqCardHeader, EqCardBody, EqCardFooter, EqImageCard, CaptionMode, EqCarousel, EqTree, TreeNode, EqAccordion, AccordionItem, AccordionMode};
use eq_ui::organisms::{EqAppShell, EqHeader, EqFooter, EqHeroShell, EqPageSection, EqNavbar, EqGrid, EqColumnDef, GridNavigation, GridDensity, RowSelection, ColumnAlign, ExportFormat, GridDragPayload};
use eq_ui::theme;  // shared constants like CONTAINER_LAYOUT, BTN_PRIMARY, etc.
```

### Atoms

```rust
// Text with semantic HTML - picks the right tag automatically
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

// Icon wrapper - pass an SVG or image as children
EqIcon { size: IconSize::Lg, muted: true,
    // your svg or img here
}

// Image with sizing, aspect ratio, and object-fit control
EqImage {
    src: "https://example.com/photo.jpg",
    alt: "A photo",
    size: AtomImageSize::Lg,
    aspect_ratio: AspectRatio::Ratio16_9,
    object_fit: ObjectFit::Cover,
    rounded: true,
}

// Checkbox with three visual states
EqCheckbox {
    state: CheckboxState::Checked,
    label: "I agree to the terms",
    on_change: move |next| agreed.set(next),
}

// Button with five variants and three sizes
EqButton {
    variant: ButtonVariant::Primary,
    size: ButtonSize::Lg,
    on_click: move |_| save(),
    "Save Changes"
}
EqButton { variant: ButtonVariant::Ghost, "Cancel" }
EqButton { variant: ButtonVariant::Danger, disabled: true, "Delete" }

// Solid (no gradient) with custom text color
EqButton { gradient: false, color: "#fbbf24", "Solid Button" }

// Divider with variants
EqDivider { variant: DividerVariant::Dashed }

// Scrollable container
EqScrollableSpace {
    div { class: "p-4",
        for item in items { p { "{item}" } }
    }
}
```

### Molecules

```rust
// Card with slots
EqCard {
    EqCardHeader { "Card title" }
    EqCardBody { "Some content here." }
    EqCardFooter { "Footer info" }
}

// Image card with caption below or as overlay
EqImageCard {
    src: "https://example.com/photo.jpg",
    alt: "Mountain landscape",
    mode: CaptionMode::Overlay,
    size: AtomImageSize::Lg,
    aspect_ratio: AspectRatio::Ratio16_9,
    rounded: true,
    title: "Alpine Meadow",
    description: "A serene landscape captured during the golden hour.",
}

// Generic carousel - pass any elements as slides
EqCarousel {
    slides: vec![
        rsx! { EqImageCard { src: "...", alt: "Slide 1", /* ... */ } },
        rsx! { EqImageCard { src: "...", alt: "Slide 2", /* ... */ } },
        rsx! { div { "Any content works as a slide" } },
    ],
}

// Collapsible tree view
EqTree {
    nodes: vec![
        TreeNode::new("id-1", "Item One"),
        TreeNode::new_with_children("id-2", "Parent", vec![
            TreeNode::new("id-3", "Child"),
        ]),
    ],
    selected: selected_id(),
    on_select: move |id: String| selected_id.set(Some(id)),
}

// Accordion with single-expand or multi-expand modes
EqAccordion {
    items: vec![
        AccordionItem::new("faq-1", "What is eq_ui?", rsx! { p { "A Dioxus component library." } }),
        AccordionItem::new("faq-2", "How many themes?", rsx! { p { "21 built-in themes." } }),
    ],
    mode: AccordionMode::Single,
}
```

### Organisms

The organisms are designed to be **platform-agnostic**. They accept `Element` props instead of depending on any specific router, so you can use the same components across web, desktop, and mobile targets.

```rust
// App shell - pass your own header, footer, and page content
EqAppShell {
    header: rsx! { EqHeader { site_title: "My App", nav: rsx! { /* your nav items */ } } },
    footer: rsx! { EqFooter {} },
    // children become the main content area
    p { "Hello world" }
}

// Hero section with background image and custom colors
EqHeroShell {
    title: "Welcome",
    subtitle: "Something cool goes here",
    title_color: "var(--my-custom-color)",       // optional color override
    subtitle_color: "#ff6b6b",                   // accepts any CSS color value
    background: rsx! {
        EqImage {
            src: "/assets/hero-bg.jpg",
            alt: "Hero background",
            size: AtomImageSize::Full,
            object_fit: ObjectFit::Cover,
        }
    },
    actions: rsx! { button { "Get started" } },
}

// Page section with optional title/description
EqPageSection {
    id: "features",
    title: "Features",
    description: "Everything you need to build fast.",
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

### Data Grid

EqGrid is a feature-rich, type-safe data grid organism. It handles sorting, filtering, pagination, row virtualization, row selection, bulk actions (delete, export, status change, clipboard), column resizing, drag-and-drop between grids, and full theme integration.

```rust
#[derive(Clone, PartialEq)]
struct Employee {
    name: String,
    role: String,
    salary: f64,
}

let columns = vec![
    EqColumnDef::new("name", "Name", |e: &Employee| e.name.clone())
        .filterable(true),
    EqColumnDef::new("role", "Role", |e: &Employee| e.role.clone())
        .filterable(true),
    EqColumnDef::new("salary", "Salary", |e: &Employee| e.salary.to_string())
        .with_formatter(|e: &Employee| format!("${:.0}", e.salary))
        .align(ColumnAlign::Right)
        .comparator(|a: &Employee, b: &Employee| {
            a.salary.partial_cmp(&b.salary).unwrap_or(std::cmp::Ordering::Equal)
        }),
];

EqGrid {
    data: employees,
    columns: columns,
    navigation: GridNavigation::Paginate,  // or Standard, Virtualize
    page_size: 10,
    row_selection: RowSelection::Multi,
    density: GridDensity::Normal,
    quick_filter: true,
    striped: true,
    export: true,
    on_selection_change: move |rows| { /* handle selection */ },
    on_delete: move |rows| { /* handle delete */ },
}
```

Virtualization mode renders only the visible rows plus a small buffer, supporting datasets of thousands of rows with measured row heights and a synced split-table layout. See the [EqGrid README](./src/organisms/eq_grid/README.md) for full documentation.

## Theming

eq_ui ships with 21 built-in themes and supports custom CSS themes at runtime.

**Built-in themes:** Unghosty (default), Burgundy, Gold, PurplePink, Monochrome, Watermelon, Sunset, Ocean, Spacetime, Gruvbox, Monokai, Hellas, Egypt, Dometrain, Catppuccin, Dracula, Nord, OneDark, RosePine, SolarizedDark, TokyoNight.

### Setting up theming

1. Call `EqTheme::use_theme_provider()` in your root `App` component.
2. Place `EqThemeRenderer {}` before your layout - this replaces the static `UI_COLORS_CSS` link.
3. Optionally add a theme switcher UI for user-facing selection.

### Switching themes programmatically

```rust
use eq_ui::eq_theme::EqTheme;

// Set a built-in theme
EqTheme::set_theme(EqTheme::Ocean);

// Load a custom theme from a CSS string
EqTheme::set_custom_theme(r#"
    :root {
        --color-primary-dark: #1a0a0a;
        --color-accent-primary: #ff6600;
        /* ... all your CSS variables ... */
    }
"#.to_string());
```

### Creating a theme switcher

```rust
use eq_ui::eq_theme::EqTheme;

#[component]
fn ThemeSwitcher() -> Element {
    let mut theme = EqTheme::use_theme();

    rsx! {
        select {
            onchange: move |evt: Event<FormData>| {
                let new_theme = match evt.value().as_str() {
                    "Ocean" => EqTheme::Ocean,
                    "Burgundy" => EqTheme::Burgundy,
                    // ... other themes
                    _ => EqTheme::Unghosty,
                };
                theme.set(new_theme);
            },
            for (name, _variant) in EqTheme::built_in_variants() {
                option { value: "{name}", "{name}" }
            }
        }
    }
}
```

## Project structure

```
src/
  lib.rs              - crate root, CSS asset exports
  theme.rs            - shared Tailwind class constants
  eq_theme.rs         - theme enum, context, and runtime switching
  atoms/
    eq_text.rs        - text with semantic variants (h1-h3, body, muted, etc.)
    eq_label.rs       - form label
    eq_link.rs        - anchor link
    eq_input.rs       - input/textarea with kind variants
    eq_icon.rs        - icon wrapper with size variants
    eq_icon_paths.rs  - SVG path data constants (Phosphor icons)
    eq_image.rs       - image with sizing, aspect ratio, object-fit
    eq_checkbox.rs    - checkbox with checked/unchecked/indeterminate states
    eq_scrollable_space.rs - scrollable container with themed scrollbar
    eq_divider.rs     - separator with solid/dashed/dotted/spacer variants
    eq_video.rs       - video with poster overlay, autoplay, controls
    *_styles.rs       - co-located style constants for each atom
  molecules/
    eq_card.rs        - card with header/body/footer slots
    eq_image_card.rs  - image card with caption modes (below/overlay)
    eq_carousel.rs    - generic content carousel with arrows and dots
    eq_tree.rs        - collapsible tree view with select and expand
    eq_accordion.rs   - collapsible panels with single/multi-expand modes
    *_styles.rs       - co-located style constants for each molecule
  organisms/
    eq_app_shell.rs   - full page layout (header + main + footer)
    eq_header.rs      - sticky site header with brand + nav + backdrop blur
    eq_footer.rs      - footer with link groups + copyright
    eq_hero_shell.rs  - hero banner with background image, overlay, custom colors
    eq_page_section.rs - titled content section
    eq_navbar.rs      - horizontal nav bar
    eq_grid/          - feature-rich data grid organism
      grid.rs         - main orchestration component
      types.rs        - shared enums (GridNavigation, RowSelection, GridDensity, etc.)
      column_def.rs   - column definition builder
      header.rs       - sortable header with column filters and resize handles
      body.rs         - row rendering with selection and drag support
      pagination.rs   - page controls
      quick_filter.rs - global search bar
      bulk_actions.rs - selection toolbar (delete, export, status, clipboard)
      export.rs       - CSV, JSON, TXT, ODS export
      styles.rs       - co-located style constants
    *_styles.rs       - co-located style constants for each organism
assets/
  icons/              - Phosphor SVG icons (square, check-square, etc.)
  theme/              - base CSS + 21 theme color files
  styling/            - component-specific CSS (navbar)
  tailwind.css        - Tailwind entry point with @source directives
```

## Style architecture

Each component keeps its Tailwind classes in a sibling `_styles.rs` file (e.g. `eq_text.rs` + `eq_text_styles.rs`). Constants that are shared across multiple components - spacing, borders, surfaces, button variants - live in `theme.rs`.

The theme uses CSS custom properties for colors (`--color-primary-dark`, `--color-label-primary`, etc.), defined in theme CSS files under `assets/theme/`. This means you can swap the entire color palette by switching themes at runtime or by providing your own CSS variables.

Components that benefit from per-instance customization accept optional override props (like `title_color` on `EqHeroShell`) that take any CSS color value - hex, `rgb()`, or `var(--your-variable)`.

## Dependencies

Just one: `dioxus = "0.7.3"`. That's it. No other crates needed.

## Running the Playground

The crate includes an interactive component playground for browsing and testing all components:

```bash
dx serve --example playground --platform web

#in the rare case that deno interferes with dx command
~/.cargo/bin/dx serve --example playground --platform web --port 3030
```

This opens a two-panel environment with a collapsible component tree on the left and an isolated preview panel on the right. Switch between all 21 built-in themes from the header dropdown.

## Roadmap

See [ROADMAP.md](./ROADMAP.md) for the full prioritized development roadmap, organized into Now, Next, and Later priorities.
