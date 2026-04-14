# eq_ui

Dioxus 0.7 component library. Atomic design, pure Tailwind, 23 themes.

https://github.com/user-attachments/assets/4ea4f561-4581-481d-bc27-c2f5a2879998

## Components

<!-- COMPONENTS_START -->

| Component | Category | ARIA | Description |
|---|---|---|---|
| EqText | Atom | native | Semantic text with variant-based tag selection (h1-h3, body, caption, muted) |
| EqLabel | Atom | native | Form label with for_id binding |
| EqLink | Atom | native | Anchor link with color theming |
| EqInput | Atom | native | Input/textarea with kind variants (text, email, password, number, textarea) |
| EqIcon | Atom | full | Icon wrapper with size variants |
| EqImage | Atom | full | Image with sizing, aspect ratio, and object-fit control |
| EqCheckbox | Atom | full | Checkbox with checked/unchecked/indeterminate states |
| EqButton | Atom | native | Button with 5 variants, 3 sizes, gradient transitions |
| EqDivider | Atom | full | Separator with solid/dashed/dotted/spacer variants |
| EqScrollableSpace | Atom | full | Scrollable container with themed scrollbar |
| EqVideo | Atom | full | Video with poster overlay, autoplay, controls |
| EqProgress | Atom | full | Progress bar with determinate/indeterminate, 4 variants, gradient fill |
| EqTab | Atom | full | Tab bar with underline, pill, and card variants; badges; disabled state |
| EqRadioGroup | Atom | full | Radio button group with mutually exclusive selection, three sizes, vertical/horizontal layout |
| EqSwitch | Atom | full | Toggle switch with pill track and sliding thumb, three sizes |
| EqCard | Molecule | full | Card with header/body/footer slots |
| EqImageCard | Molecule | full | Image card with caption modes (below/overlay) |
| EqCarousel | Molecule | pending | Generic content carousel with arrows and dots |
| EqTree | Molecule | pending | Collapsible tree view with select and expand |
| EqAccordion | Molecule | pending | Collapsible panels with single/multi-expand modes |
| EqHeader | Organism | native | Sticky header with brand, nav, and backdrop blur |
| EqFooter | Organism | native | Footer with link groups and copyright |
| EqHeroShell | Organism | - | Hero banner with background image, overlay, custom colors |
| EqPageSection | Organism | native | Titled content section |
| EqAppShell | Organism | native | Full page layout (header + main + footer) |
| EqNavbar | Organism | native | Horizontal nav bar |
| EqGrid | Organism | pending | Data grid with sorting, filtering, pagination, virtualization, DnD, export |
| Getting Started Guide | Guide | - | In-app developer guide for the playground |
| Theme Showcase | Theming | - | Theme color and gradient swatch viewer |

ARIA legend: **full** = roles + attributes + keyboard, **native** = semantic HTML, **pending** = planned, **-** = non-interactive.

**Planned** (not yet built):

| Component | Category | ARIA | Notes |
|---|---|---|---|
| Skeleton | Atom | - | CSS keyframes |
| Slider | Atom | planned | Via dioxus-primitives |
| Calendar | Molecule | planned | Via dioxus-primitives |
| Pagination | Molecule | planned |  |
| ToastList | Molecule | planned | Via dioxus-primitives |
| Dialog | Molecule | planned | Via dioxus-primitives |
| Sheet / Drawer | Molecule | planned | Needs slide transition |
| Select | Atom | planned | Via dioxus-primitives, positioning via eval |
| ToolTip | Atom | planned | Via dioxus-primitives, positioning via eval |
| DropDownMenu | Molecule | planned | Via dioxus-primitives, positioning via eval |
| ContextMenu | Molecule | planned | Via dioxus-primitives, positioning via eval |
| HoverCard | Molecule | planned | Via dioxus-primitives, positioning via eval |
| DatePicker | Molecule | planned | Via dioxus-primitives, positioning via eval |
| VirtualList | Organism | - | Scroll position via eval |
| RichTextEditor | Organism | - | JS editor init via eval |
| Signature | Atom | - | Canvas drawing via eval |
| Babylon.js | Organism | - | JS engine init via eval |

### Blitz (native) readiness

Tier 1 = works as-is, Tier 2 = needs small fix, Tier 3 = needs significant work.

| Component | Tier | Needs eval | Notes |
|---|---|---|---|
| Getting Started Guide | 1 | no | Playground-only, feature-gated |
| EqText | 1 | no |  |
| EqLabel | 1 | no |  |
| EqLink | 1 | no |  |
| EqInput | 1 | no |  |
| EqIcon | 1 | no |  |
| EqImage | 1 | no | object-fit support on Blitz needs verification |
| EqCheckbox | 1 | no |  |
| EqButton | 2 | no | @property gradient transition needs Blitz fallback |
| EqDivider | 1 | no |  |
| EqScrollableSpace | 2 | no | Custom scrollbar CSS cosmetic-only on Blitz |
| EqVideo | 1 | no | Video playback on Blitz depends on media support |
| EqProgress | 1 | no | Indeterminate animation needs Blitz fallback |
| EqTab | 1 | no |  |
| EqCard | 1 | no |  |
| EqImageCard | 1 | no |  |
| EqCarousel | 2 | no | Slide transition needs Blitz fallback |
| EqTree | 1 | no | Tree ARIA (treeitem, group) planned |
| EqAccordion | 1 | no | ARIA planned via dioxus-primitives Phase 2 |
| EqHeader | 2 | no | backdrop-filter needs Blitz fallback |
| EqFooter | 1 | no |  |
| EqHeroShell | 2 | no | Overlay blend mode may need Blitz fallback |
| EqPageSection | 1 | no |  |
| EqAppShell | 1 | no |  |
| EqNavbar | 1 | no |  |
| EqGrid | 3 | yes | Clipboard via document::eval(), DnD and scroll measurement need Blitz testing |
| Theme Showcase | 1 | no | Playground-only, feature-gated |
| EqRadioGroup | 1 | no |  |
| EqSwitch | 2 | no | CSS transition degrades to instant toggle on Blitz |

<!-- COMPONENTS_END -->

**Theming** - 23 built-in themes, custom CSS, runtime switching. The `theme` module also exports shared Tailwind constants you can use in your own layouts.

## Quick start

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
# From crates.io:
eq_ui = "0.3.0"

# Or from GitHub:
eq_ui = { git = "https://github.com/equidevium/eq_ui", branch = "main" }

# Or from a local path:
eq_ui = { path = "../eq_ui" }
```

Wire up CSS and theming in your root component:

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

If you're using Tailwind in your own project, add a `@source` so it picks up eq_ui's classes:

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
use eq_ui::atoms::{EqText, TextVariant, EqInput, InputKind, EqLabel, EqLink, EqIcon, IconSize, EqImage, AtomImageSize, AspectRatio, ObjectFit, EqCheckbox, CheckboxState, EqButton, ButtonVariant, ButtonSize, EqDivider, EqScrollableSpace, EqVideo, EqProgress, ProgressVariant, ProgressSize, EqTab, TabItem, TabVariant, TabSize};
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

// Progress bar - determinate, indeterminate, variants
EqProgress { value: 0.65 }
EqProgress { value: 0.3, variant: ProgressVariant::Warning, label: true }
EqProgress { indeterminate: true, size: ProgressSize::Lg }

// Tab bar - three visual variants, badges, disabled tabs
let mut active = use_signal(|| 0usize);

EqTab {
    tabs: vec![
        TabItem::new("Overview"),
        TabItem::new("Inbox").badge(12),
        TabItem::new("Settings"),
        TabItem::new("Archived").disabled(true),
    ],
    variant: TabVariant::Underline,  // or Pill, Card
    size: TabSize::Md,
    active: active(),
    on_change: move |idx| active.set(idx),
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
        AccordionItem::new("faq-2", "How many themes?", rsx! { p { "23 built-in themes." } }),
    ],
    mode: AccordionMode::Single,
}
```

### Organisms

Organisms take `Element` props instead of depending on a specific router, so they work across web, desktop, and mobile.

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

EqGrid handles sorting, filtering, pagination, virtualization, row selection, bulk actions, column resizing, drag-and-drop, and export.

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

Virtualization renders only visible rows plus a small buffer (split-table layout with measured row heights). See the [EqGrid README](./src/organisms/eq_grid/README.md) for details.

## Theming

23 built-in themes, custom CSS themes at runtime.

**Built-in:** Unghosty (default), Burgundy, Gold, PurplePink, Monochrome, Watermelon, Sunset, Ocean, Spacetime, Gruvbox, Monokai, Hellas, Egypt, Dometrain, Catppuccin, Dracula, Nord, OneDark, RosePine, SolarizedDark, TokyoNight, Warcraft, SweetRush.

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
  playground/         - feature-gated interactive component showcase
    eq_playground.rs  - EqPlayground organism (self-contained with CSS/theme)
    playground_types.rs - ComponentDescriptor, ComponentCategory, UsageExample
    playground_helpers.rs - DemoSection, CodeBlock, StyleInfo, prop controls
    playground_guide.rs - Getting Started in-app guide
    theme_showcase.rs - theme color/gradient swatch viewer
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
    eq_progress.rs    - progress bar with determinate/indeterminate modes
    eq_tab.rs         - tab bar with underline, pill, and card variants
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

Each component keeps its Tailwind classes in a sibling `_styles.rs` file (e.g. `eq_text.rs` + `eq_text_styles.rs`). Shared constants (spacing, borders, surfaces, button variants) live in `theme.rs`.

Colors use CSS custom properties (`--color-primary-dark`, `--color-label-primary`, etc.) defined in `assets/theme/`. Swap the palette by switching themes or providing your own CSS variables. Some components also accept per-instance color overrides (e.g. `title_color` on `EqHeroShell`) - any CSS color value works.

## Dependencies

`dioxus = "0.7.3"`. Nothing else.

## Platform Compatibility

Dioxus desktop uses Wry (webview) for rendering, but compiles to a native binary - not WASM. `web_sys`/`wasm-bindgen` are only available on `wasm32`.

Where browser APIs are needed (element measurement, focus, scroll position), we use `document::eval()` instead of `web_sys`. This works on both web and desktop via Wry's webview, no `wasm-bindgen` required.

All existing components work cross-platform: web, desktop (Wry), mobile. See the Blitz readiness table in the Components section for native renderer status. See the Planned table for components not yet built.

## Running the Playground

Interactive playground for browsing and testing all 28 components. Enable with the `playground` feature:

```bash
dx serve --example playground --features playground --platform web

# if deno interferes with dx command
~/.cargo/bin/dx serve --example playground --features playground --platform web --port 3030
```

Two-panel layout: component tree on the left, live demo on the right. Prop controls, style tokens, code examples, and a theme switcher in the header.

The playground is self-contained (bundles its own CSS/theme). You can inject your own components by pushing `ComponentDescriptor` entries:

```rust
use eq_ui::{all_component_descriptors, EqPlayground};

let mut descs = all_component_descriptors();
descs.push(my_component::descriptor());

rsx! { EqPlayground { descriptors: descs } }
```

See [playground.md](./playground.md) for the full architecture specification.

## Roadmap

See [ROADMAP.md](./ROADMAP.md) for what's coming next.
