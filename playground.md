# EqPlayground - Architecture & Specification

> Interactive component playground for browsing, testing, and demonstrating every eq_ui component.
> The playground is a reusable, self-describing system driven by component descriptors. Any Dioxus
> project using eq_ui can embed the same playground to showcase both built-in and custom components.

---

## Core Idea

Every eq_ui component carries its own playground metadata: name, category, style tokens, usage
examples, interactive demo, and variant gallery. The playground reads this metadata at compile
time and renders a unified browsing experience. No hardcoded demo code lives in the playground
itself.

External users import eq_ui as a crate, write a `ComponentDescriptor` for their custom components,
and pass them alongside the built-in ones. Their components appear in the same tree, with the
same prop controls and code examples.

---

## Component Descriptor

Each component exposes a descriptor behind the `playground` feature flag. This is the contract
between the component and the playground.

```rust
use dioxus::prelude::*;

/// Which atomic design layer this component belongs to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ComponentCategory {
    /// Documentation and guides - always rendered first in the tree.
    Guide,
    Atom,
    Molecule,
    Organism,
    Theming,
}

/// A single usage example shown in the playground.
#[derive(Clone, PartialEq)]
pub struct UsageExample {
    /// Short label for the example (e.g. "Basic", "With icon", "Disabled state").
    pub label: &'static str,
    /// Rust code string displayed in the code block.
    pub code: String,
}

/// Complete playground metadata for a component.
#[derive(Clone, PartialEq)]
pub struct ComponentDescriptor {
    /// URL-safe identifier used for routing (e.g. "eq-button", "eq-progress").
    pub id: &'static str,
    /// Display name shown in the tree and header (e.g. "EqButton", "EqProgress").
    pub name: &'static str,
    /// Atomic design category - determines which tree group the component appears in.
    pub category: ComponentCategory,
    /// One-line description shown below the component name.
    pub description: &'static str,
    /// Returns the style token catalog from the component's `_styles.rs`.
    /// Each entry is (CONSTANT_NAME, "tailwind class string").
    pub style_tokens: fn() -> Vec<(&'static str, &'static str)>,
    /// Returns usage examples as displayable code blocks.
    pub usage_examples: fn() -> Vec<UsageExample>,
    /// Renders the interactive demo with prop controls and live preview.
    pub render_demo: fn() -> Element,
    /// Renders a static gallery of all variants for quick visual comparison.
    pub render_gallery: fn() -> Element,
}
```

### Category Sort Order

Categories appear in the sidebar tree in this order:

| Order | Category | Label | Description |
|-------|----------|-------|-------------|
| 0 | `Guide` | Guide | Documentation pages (Getting Started, etc.) |
| 1 | `Atom` | Atoms | Smallest building blocks |
| 2 | `Molecule` | Molecules | Composed from atoms |
| 3 | `Organism` | Organisms | Complex compositions |
| 4 | `Theming` | Theming | Theme showcase and CSS property viewer |

### Where It Lives

The types live in `src/playground/playground_types.rs` inside the feature-gated playground
module. They are re-exported from `lib.rs`:

```rust
#[cfg(feature = "playground")]
pub use playground::{ComponentDescriptor, ComponentCategory, UsageExample, EqPlayground};
```

---

## Feature Gate: `playground`

All demo rendering code is gated behind `#[cfg(feature = "playground")]`. This means:

- By default (`cargo add eq_ui`), zero demo code is compiled. No binary size impact.
- The playground example enables it: `dx serve --example playground --features playground`.
- External users enable it in their `Cargo.toml`:

```toml
[dependencies]
eq_ui = { version = "0.3.0", features = ["playground"] }
```

### What Gets Gated

Inside each component file (e.g. `src/atoms/eq_button.rs`):

```rust
use super::eq_button_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

// Playground-specific imports - only compiled with feature
#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropSelect, PropToggle, PropInput, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// Component itself - ALWAYS compiled
#[component]
pub fn EqButton(/* ... */) -> Element { /* ... */ }

// Playground descriptor - only compiled with "playground"
#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-button",
        name: "EqButton",
        category: ComponentCategory::Atom,
        description: "Themed button with five variants, gradient transitions, and three sizes.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![/* ... */],
        render_demo: || rsx! { DemoEqButton {} },
        render_gallery: || rsx! { GalleryEqButton {} },
    }
}

// Interactive demo - only compiled with "playground"
#[cfg(feature = "playground")]
#[component]
fn DemoEqButton() -> Element {
    // signals, prop controls, live preview
}

// Variant gallery - only compiled with "playground"
#[cfg(feature = "playground")]
#[component]
fn GalleryEqButton() -> Element {
    // static grid of all variants
}
```

### Style Catalog Convention

Each `_styles.rs` file exports named Tailwind constants and a `catalog()` function:

```rust
// eq_button_styles.rs

pub const BASE: &str = "btn";
pub const PRIMARY: &str = "btn-primary";
pub const GHOST: &str = "btn-ghost";
// ...

/// All style tokens for playground introspection.
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("BASE", BASE),
        ("PRIMARY", PRIMARY),
        ("GHOST", GHOST),
        // ...
    ]
}
```

---

## Registry: Collecting All Descriptors

The library provides a convenience function that returns all 28 built-in descriptors:

```rust
// src/lib.rs

#[cfg(feature = "playground")]
pub fn all_component_descriptors() -> Vec<ComponentDescriptor> {
    vec![
        // Guide
        playground::playground_guide::descriptor(),
        // Atoms (13)
        atoms::eq_text::descriptor(),
        atoms::eq_label::descriptor(),
        atoms::eq_link::descriptor(),
        atoms::eq_input::descriptor(),
        atoms::eq_icon::descriptor(),
        atoms::eq_image::descriptor(),
        atoms::eq_checkbox::descriptor(),
        atoms::eq_button::descriptor(),
        atoms::eq_divider::descriptor(),
        atoms::eq_scrollable_space::descriptor(),
        atoms::eq_video::descriptor(),
        atoms::eq_progress::descriptor(),
        atoms::eq_tab::descriptor(),
        // Molecules (5)
        molecules::eq_card::descriptor(),
        molecules::eq_image_card::descriptor(),
        molecules::eq_carousel::descriptor(),
        molecules::eq_tree::descriptor(),
        molecules::eq_accordion::descriptor(),
        // Organisms (7)
        organisms::eq_header::descriptor(),
        organisms::eq_footer::descriptor(),
        organisms::eq_hero_shell::descriptor(),
        organisms::eq_page_section::descriptor(),
        organisms::eq_app_shell::descriptor(),
        organisms::eq_navbar::descriptor(),
        organisms::eq_grid::grid::descriptor(),
        // Theming
        playground::theme_showcase::descriptor(),
    ]
}
```

---

## EqPlayground Component

The playground is a self-contained organism in `src/playground/eq_playground.rs`. It includes
its own CSS links, theme provider, and theme renderer. The caller does not need to set up
anything else.

```rust
#[component]
pub fn EqPlayground(
    /// Component descriptors to render in the playground.
    descriptors: Vec<ComponentDescriptor>,
    /// Header brand text.
    #[props(default = "EqPlayground")]
    site_title: &'static str,
    /// Footer copyright holder.
    #[props(default = "Equidevium")]
    copyright_holder: &'static str,
) -> Element { /* ... */ }
```

### Internal Components

EqPlayground renders these internal components:

- `ThemeSwitcher` - dropdown to switch between all 21 built-in themes
- `EqThemeRenderer` - injects the active theme's CSS as an inline `<style>` element
- `build_tree_from_descriptors()` - groups descriptors by category into sidebar `TreeNode` vec
- `PreviewPanel` - routes the selected tree node ID to the matching descriptor's `render_demo`

### Self-Contained CSS

EqPlayground includes its own stylesheet links so consumers don't need to add them:

```rust
document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }
document::Link { rel: "stylesheet", href: UI_INDEX_CSS }
document::Link { rel: "stylesheet", href: UI_BUTTONS_CSS }
```

### Usage in eq_ui's Own Example

```rust
// examples/playground.rs
use dioxus::prelude::*;
use eq_ui::{all_component_descriptors, EqPlayground};

fn main() { dioxus::launch(App); }

#[component]
fn App() -> Element {
    rsx! {
        EqPlayground {
            descriptors: all_component_descriptors(),
        }
    }
}
```

### Usage by External Users (crates.io)

```rust
use dioxus::prelude::*;
use eq_ui::{all_component_descriptors, EqPlayground};

mod my_components;

fn App() -> Element {
    let mut descs = all_component_descriptors();
    descs.push(my_components::rating_stars::descriptor());
    descs.push(my_components::color_picker::descriptor());

    rsx! {
        EqPlayground {
            descriptors: descs,
            site_title: "My Design System",
            copyright_holder: "My Company",
        }
    }
}
```

External users can also skip the built-in components entirely and only show their own:

```rust
let descs = vec![
    my_components::rating_stars::descriptor(),
    my_components::color_picker::descriptor(),
];

rsx! { EqPlayground { descriptors: descs } }
```

---

## Playground Module Structure

The playground lives in its own feature-gated module at `src/playground/`:

```
src/playground/
  mod.rs                  - module root, re-exports all public types
  playground_types.rs     - ComponentDescriptor, ComponentCategory, UsageExample
  playground_helpers.rs   - DemoSection, CodeBlock, StyleInfo, PropSelect, PropInput, PropToggle
  eq_playground.rs        - EqPlayground organism + ThemeSwitcher, PreviewPanel, tree builder
  eq_playground_styles.rs - Tailwind constants for the playground layout
  playground_guide.rs     - In-app "Getting Started" guide (Guide category)
  theme_showcase.rs       - Theme color/gradient swatch viewer (Theming category)
```

### Shared Demo Helpers

These helper components are exported from `playground_helpers.rs` for use in all component demos:

| Helper | Purpose |
|--------|---------|
| `DemoSection` | Titled wrapper for each demo panel |
| `CodeBlock` | Gruvbox-themed Rust syntax-highlighted code block |
| `StyleInfo` | Collapsible style token viewer with Tailwind highlighting |
| `PropSelect` | Dropdown control for enum prop selection |
| `PropInput` | Text input for string/numeric props |
| `PropToggle` | Boolean toggle switch |
| `format_catalog()` | Converts `Vec<(&str, &str)>` to display string for StyleInfo |
| `highlight_rust()` | Gruvbox syntax highlighting for Rust code |
| `highlight_styles()` | Gruvbox syntax highlighting for Tailwind token catalogs |

Layout constants for prop controls:

| Constant | Value |
|----------|-------|
| `PROP_ROW` | Row layout for a single prop control |
| `PROP_LABEL` | Label styling for prop names |
| `PROP_CONTROL` | Input/select styling for prop values |

---

## Complete File Structure

```
eq_ui/
  Cargo.toml                          - playground feature flag
  Dioxus.toml                         - example = "playground", default features = ["playground"]
  src/
    lib.rs                            - re-exports, all_component_descriptors()
    theme.rs                          - merge_classes(), shared layout constants
    eq_theme.rs                       - EqTheme enum, 21 themes, CSS content
    playground/
      mod.rs                          - module root
      playground_types.rs             - ComponentDescriptor, ComponentCategory, UsageExample
      playground_helpers.rs           - DemoSection, CodeBlock, StyleInfo, prop controls
      eq_playground.rs                - EqPlayground organism
      eq_playground_styles.rs         - playground layout Tailwind constants
      playground_guide.rs             - Getting Started guide descriptor
      theme_showcase.rs               - Theme showcase descriptor
    atoms/
      mod.rs                          - module declarations + pub use exports
      eq_text.rs / eq_text_styles.rs
      eq_label.rs / eq_label_styles.rs
      eq_link.rs / eq_link_styles.rs
      eq_input.rs / eq_input_styles.rs
      eq_icon.rs / eq_icon_styles.rs / eq_icon_paths.rs
      eq_image.rs / eq_image_styles.rs
      eq_checkbox.rs / eq_checkbox_styles.rs
      eq_button.rs / eq_button_styles.rs
      eq_divider.rs / eq_divider_styles.rs
      eq_scrollable_space.rs / eq_scrollable_space_styles.rs
      eq_video.rs / eq_video_styles.rs
      eq_progress.rs / eq_progress_styles.rs
      eq_tab.rs / eq_tab_styles.rs
    molecules/
      mod.rs
      eq_card.rs / eq_card_styles.rs
      eq_image_card.rs / eq_image_card_styles.rs
      eq_carousel.rs / eq_carousel_styles.rs
      eq_tree.rs / eq_tree_styles.rs
      eq_accordion.rs / eq_accordion_styles.rs
    organisms/
      mod.rs
      eq_header.rs / eq_header_styles.rs
      eq_footer.rs / eq_footer_styles.rs
      eq_hero_shell.rs / eq_hero_shell_styles.rs
      eq_page_section.rs / eq_page_section_styles.rs
      eq_app_shell.rs
      eq_navbar.rs
      eq_grid/
        mod.rs
        grid.rs                       - EqGrid component + descriptor + tabbed demo
        styles.rs                     - grid Tailwind constants + catalog()
        types.rs                      - GridNavigation, RowSelection, GridDensity, etc.
        column_def.rs                 - EqColumnDef builder
        header.rs                     - grid header rendering
        body.rs                       - grid body rendering
        pagination.rs                 - pagination controls
        quick_filter.rs               - search/filter bar
        bulk_actions.rs               - multi-row action toolbar
        export.rs                     - CSV/JSON/TXT/ODS export
  examples/
    playground.rs                     - minimal entry point (28 lines)
    playground_legacy.rs              - backup of original 3700-line monolithic playground
```

---

## UI/UX Specification

### Layout

```
+--------------------------------------------------+
|  EqHeader  [site_title]         [ThemeSwitcher]  |
+------------+-------------------------------------+
|            |                                     |
|  Sidebar   |         Preview Panel               |
|  Tree      |                                     |
|            |  +-----------------------------+    |
|  > Guide   |  |  Tab bar (if applicable)    |    |
|    Getting  |  +-----------------------------+    |
|    Started  |  |  Props controls             |    |
|  > Atoms   |  +-----------------------------+    |
|    EqText  |  |  Live preview               |    |
|    EqLabel |  +-----------------------------+    |
|    EqTab   |  |  Style tokens               |    |
|    ...     |  +-----------------------------+    |
|  > Molecul |  |  Code block                 |    |
|  > Organis |  +-----------------------------+    |
|  > Theming |  |                             |    |
|            |                                     |
+------------+-------------------------------------+
|  EqFooter  [copyright_holder]                    |
+--------------------------------------------------+
```

### Preview Panel Rendering Order

For a selected `ComponentDescriptor`, the playground calls `(descriptor.render_demo)()`.
Each demo component is responsible for its own layout using the shared helpers. The typical
structure inside a demo is:

1. `DemoSection { title: "ComponentName" }` wrapper
2. Props controls panel (PropSelect, PropToggle, PropInput)
3. Live preview area (the actual component with current prop values)
4. `StyleInfo` (collapsible style token viewer)
5. `CodeBlock` (usage code example)

### EqGrid Tabbed Demo

The EqGrid demo uses the EqTab component to offer three sub-demos under one sidebar entry:

| Tab | Content |
|-----|---------|
| Data Grid | Full interactive demo with prop controls (navigation, density, selection, pagination, export, clipboard, bulk actions) |
| Drag & Drop | Two side-by-side grids (Team A / Team B) with bidirectional drag-and-drop |
| Reorder | Single grid with grip-handle row reordering |

### Responsive Behavior

- Desktop (md+): sidebar always visible, static position
- Mobile (<md): sidebar hidden, hamburger toggle in header, overlay with backdrop
- Selecting a component on mobile closes the sidebar

### Theme Switcher

- `<select>` dropdown in the header, lists all 21 built-in themes
- Changing the theme updates the global context, all components re-render with new CSS variables

---

## How to Add a New Component Demo

### For eq_ui contributors (built-in components)

**Step 1:** Create `eq_my_widget_styles.rs` with Tailwind constants and `catalog()`:

```rust
pub const BASE: &str = "relative flex items-center ...";
pub const ACTIVE: &str = "bg-[var(--color-accent-primary)] ...";
pub const SM: &str = "px-2 py-1 text-xs";
pub const MD: &str = "px-4 py-2 text-sm";

pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![("BASE", BASE), ("ACTIVE", ACTIVE), ("SM", SM), ("MD", MD)]
}
```

**Step 2:** In `eq_my_widget.rs`, add the component (always compiled) plus feature-gated
descriptor, demo, and gallery:

```rust
use super::eq_my_widget_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, StyleInfo, PropSelect, PropToggle, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// Component - always compiled
#[component]
pub fn EqMyWidget(/* props */) -> Element { /* ... */ }

// Descriptor - playground only
#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-my-widget",
        name: "EqMyWidget",
        category: ComponentCategory::Atom,
        description: "Short description of what this component does.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample { label: "Basic", code: "EqMyWidget { value: 42 }".into() },
        ],
        render_demo: || rsx! { DemoEqMyWidget {} },
        render_gallery: || rsx! { GalleryEqMyWidget {} },
    }
}

#[cfg(feature = "playground")]
#[component]
fn DemoEqMyWidget() -> Element {
    // signals, prop controls, live preview, StyleInfo, CodeBlock
}

#[cfg(feature = "playground")]
#[component]
fn GalleryEqMyWidget() -> Element {
    // all variants side by side
}
```

**Step 3:** Register the module in `atoms/mod.rs`:

```rust
pub mod eq_my_widget;
pub mod eq_my_widget_styles;

pub use eq_my_widget::{EqMyWidget, MyWidgetVariant};
```

**Step 4:** Register the descriptor in `lib.rs` -> `all_component_descriptors()`:

```rust
atoms::eq_my_widget::descriptor(),
```

**Step 5:** Run and verify:

```bash
dx serve --example playground --features playground
```

### For external users (crates.io consumers)

1. Add eq_ui with the playground feature: `eq_ui = { version = "0.3.0", features = ["playground"] }`
2. Write your component in your own project
3. Write a `descriptor()` function returning a `ComponentDescriptor`
4. Write `DemoYourComponent` and `GalleryYourComponent`
5. Push your descriptor into the vec and pass to `EqPlayground`:

```rust
let mut descs = eq_ui::all_component_descriptors();
descs.push(my_widget::descriptor());

rsx! { EqPlayground { descriptors: descs } }
```

Your component appears in the sidebar under whichever `ComponentCategory` you chose.

### Feature gating for external users

External users should gate their playground code the same way:

```toml
# In your Cargo.toml
[features]
default = []
playground = ["eq_ui/playground"]
```

```rust
// Your component file
#[component]
pub fn MyWidget(/* ... */) -> Element { /* always compiled */ }

#[cfg(feature = "playground")]
pub fn descriptor() -> eq_ui::ComponentDescriptor { /* ... */ }

#[cfg(feature = "playground")]
#[component]
fn DemoMyWidget() -> Element { /* ... */ }
```

---

## CSS Dependencies

EqPlayground bundles three CSS assets internally:

| Asset | Purpose |
|-------|---------|
| `UI_TAILWIND_CSS` | Tailwind base with `@source` directives |
| `UI_INDEX_CSS` | Global styles, shared keyframes |
| `UI_BUTTONS_CSS` | Button `@property` declarations and gradient animation |

New components must use pure Tailwind in `_styles.rs`. CSS files are only for what Tailwind
cannot express (e.g. `@property` for animatable custom properties).

---

## Component Inventory (28 descriptors)

### Guide (1)

| Component | ID | Description |
|-----------|----|-------------|
| Getting Started | `playground-guide` | In-app developer documentation covering playground navigation, the descriptor pattern, step-by-step instructions for adding components, feature gating, and the style catalog convention |

### Atoms (13)

| Component | ID | Key Features |
|-----------|----|-------------|
| EqText | `eq-text` | 8 variants (H1-H4, Body, Emphasis, Muted, Caption) |
| EqLabel | `eq-label` | Form label with required indicator |
| EqLink | `eq-link` | Themed anchor with hover effects |
| EqInput | `eq-input` | Text, password, email, search, textarea |
| EqIcon | `eq-icon` | Phosphor icons, 4 sizes |
| EqImage | `eq-image` | Aspect ratio, object fit, lazy loading |
| EqCheckbox | `eq-checkbox` | Three states (checked, unchecked, indeterminate) |
| EqButton | `eq-button` | 5 variants (Primary, Ghost, Outline, Card, Danger), 3 sizes, gradient animation |
| EqDivider | `eq-divider` | Horizontal/vertical, 3 weights, 3 spacings |
| EqScrollableSpace | `eq-scrollable-space` | Scrollable container with fade edges |
| EqVideo | `eq-video` | Themed video player |
| EqProgress | `eq-progress` | Determinate/indeterminate, 4 variants, 3 sizes, gradient fill |
| EqTab | `eq-tab` | 3 variants (Underline, Pill, Card), 3 sizes, badges, disabled state |

### Molecules (5)

| Component | ID | Key Features |
|-----------|----|-------------|
| EqCard | `eq-card` | Content container with border and hover effects |
| EqImageCard | `eq-image-card` | Card with image, caption, attribution |
| EqCarousel | `eq-carousel` | Image/content carousel with navigation |
| EqTree | `eq-tree` | Expandable tree with selection and item counts |
| EqAccordion | `eq-accordion` | Collapsible sections with animation |

### Organisms (7)

| Component | ID | Key Features |
|-----------|----|-------------|
| EqHeader | `eq-header` | Site header with brand, navigation slots |
| EqFooter | `eq-footer` | Site footer with copyright |
| EqHeroShell | `eq-hero-shell` | Full-width hero section |
| EqPageSection | `eq-page-section` | Content section with title and layout |
| EqAppShell | `eq-app-shell` | Page shell with header/footer/content slots |
| EqNavbar | `eq-navbar` | Navigation bar with links |
| EqGrid | `eq-grid` | Feature-rich data grid with sorting, filtering, pagination, virtualization, row selection, drag-and-drop, reorder, export, bulk actions. Tabbed demo (Data Grid / Drag & Drop / Reorder) |

### Theming (1)

| Component | ID | Description |
|-----------|----|-------------|
| Theme Showcase | `theme-showcase` | Displays all CSS custom properties grouped by category with live color swatches, gradient previews, transition demos, and interactive card demo |

---

## Running

```bash
# Development (eq_ui repo)
dx serve --example playground --features playground --platform web

# Specify port if needed
dx serve --example playground --features playground --platform web --port 3030
```

---

## Resolved Decisions

Items that were previously in the parking lot, now resolved:

- **Theme Showcase**: Lives as a descriptor with `category: Theming` in `playground/theme_showcase.rs`. Includes ColorSwatch, GradientSwatch, ShowcaseSection, TransitionDemo, InteractiveCardDemo.

- **Grid demo data**: DemoEmployee, demo_employees, demo_columns, DndPerson, team data - all live inside `organisms/eq_grid/grid.rs` behind `#[cfg(feature = "playground")]`.

- **Grid sub-demos**: Resolved as tabs inside one descriptor. DemoEqGrid uses EqTab (Card variant) with three tabs: Data Grid, Drag & Drop, and Reorder. All under a single sidebar entry.

- **Playground location**: The playground module lives at `src/playground/` (not inside organisms/). It is a meta-level tool, not a UI component.

- **Getting Started guide**: Lives as a descriptor with `category: Guide` in `playground/playground_guide.rs`. Renders a rich documentation page with step-by-step instructions, code examples, and a quick reference table.
