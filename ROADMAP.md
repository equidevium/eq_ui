# eq_ui Roadmap

> A portable Dioxus 0.7 component library following atomic design principles.
> This roadmap is organized by priority: **Now**, **Next**, and **Later**.

---

## What We Have Today

### Atoms
- `EqText` — Text rendering with variants (H1, H2, H3, Body, Muted, Caption, Emphasis, Mono)
- `EqLabel` — Form labels
- `EqLink` — Styled anchor links
- `EqInput` — Text, email, password, textarea inputs
- `EqIcon` — Icon wrapper with size variants (Sm, Md, Lg)
- `EqImage` — Full-featured image atom (sizing, aspect ratios, object-fit, rounded corners)

### Molecules
- `EqCard` — Card with header, body, footer slots
- `EqImageCard` — Image card with caption modes (Below, Overlay)
- `EqCarousel` — Generic content carousel with arrow navigation and dot indicators

### Organisms
- `EqAppShell` — Full-page layout (header + main + footer)
- `EqHeader` — Sticky header with nav slot and backdrop blur
- `EqFooter` — Footer with link groups
- `EqNavbar` — Navigation bar
- `EqHeroShell` — Hero section with optional background image, overlay, and custom title/subtitle colors
- `EqPageSection` — Content section with title and description

### Theming
- `EqTheme` — Theme enum with 14 built-in themes + custom CSS support
- `EqThemeRenderer` — Runtime theme switcher using `document::Style`
- Theme context via `use_theme_provider()` / `use_theme()` / `set_theme()`
- Built-in themes: Unghosty, Burgundy, Gold, PurplePink, Monochrome, Watermelon, Sunset, Ocean, Spacetime, Gruvbox, Monokai, Hellas, Egypt, Dometrain

### Infrastructure
- Co-located `_styles.rs` pattern for all components
- `theme.rs` shared style tokens
- CSS variable system (`colors.css`, `buttons.css`, `index.css`)
- Tailwind CSS v4 with `@source` directives scanning `.rs` files
- Showcase example (`dx serve --example showcase --platform web`)

---

## Now - Active Development

These are the items currently being worked on or immediately planned.

### Components
- [ ] **EqAccordion** (Molecule) - Collapsible panels that accept any element as content. Supports single and multi-expand modes.
- [ ] **EqVideo** (Organism) - Video player organism. To be decomposed into smaller molecules/atoms (play button, progress bar, controls, thumbnail).
- [ ] **EqButton** (Atom) - Dedicated button component. Style tokens already exist in `theme.rs` (`BTN_BASE`, `BTN_PRIMARY`, `BTN_GHOST`, `BTN_DANGER`, size variants). Needs its own component file + `_styles.rs`.

### Theming & Customization
- [ ] **Tailwind Class Merger (`merge_classes`)** - A utility function in `theme.rs` (or a dedicated `utils.rs`) that lets consumers extend or fully replace a component's default Tailwind classes via a single `class_override` prop. This is the foundation for the style override system.

  **How it works:**
  ```rust
  // In theme.rs or utils.rs
  pub fn merge_classes(base: &str, override_class: &str) -> String {
      if override_class.is_empty() {
          base.to_string()
      } else if override_class.starts_with("!") {
          // "!" prefix = full replacement, discard defaults
          override_class[1..].to_string()
      } else {
          // Append to defaults
          format!("{base} {override_class}")
      }
  }
  ```

  **How components use it:**
  ```rust
  use crate::theme::merge_classes;

  #[component]
  pub fn EqCard(
      #[props(into, default)]
      class_override: String,
      children: Element,
  ) -> Element {
      rsx! {
          div { class: "{merge_classes(s::CARD_WRAPPER, &class_override)}",
              {children}
          }
      }
  }
  ```

  **How consumers use it:**
  ```rust
  // Extend - appends extra classes to the defaults
  EqCard { class_override: "shadow-xl border-2 border-red-500", ... }

  // Full replace - "!" prefix discards all default styles
  EqCard { class_override: "!my-custom-card p-8 bg-white rounded-none", ... }

  // No override - uses defaults as-is
  EqCard { ... }
  ```

  **Rollout plan:** Start with components that benefit most from customization (`EqCard`, `EqHeroShell`, `EqPageSection`), then expand to others incrementally. Not every component needs `class_override` - some should stay opinionated.

- [ ] **Style override system** - Apply the `merge_classes` pattern to components. Started with `title_color`/`subtitle_color` on `EqHeroShell` for CSS color overrides. The `class_override` prop (powered by `merge_classes`) handles Tailwind class overrides.
- [ ] **EqHeroShell overlay customizability** - Make the overlay opacity/color configurable instead of hardcoded `bg-black/50`. Can use `class_override` on the overlay div or a dedicated `overlay_class` prop.
- [ ] **Custom theme loading** - Finalize `set_custom_theme()` for loading user-provided CSS strings at runtime.

### Documentation
- [ ] **Integration guide** - Step-by-step instructions for adding eq_ui to a consuming project (git dependency, Tailwind `@source`, theme setup, `EqThemeRenderer` wiring).
- [ ] **Showcase improvements** - Add theming section to showcase. Document all component prop variations.

---

## Next - Short-Term Priorities

Items to tackle once the "Now" batch stabilizes.

### Components
- [ ] **EqTree** (Molecule/Organism) - Hierarchical tree component. Each branch can contain sub-branches and leaf items. Supports expand/collapse.
- [ ] **EqModal** (Molecule) - Modal dialog with backdrop, configurable size, and close behavior.
- [ ] **EqToast / EqNotification** (Molecule) - Toast notifications with auto-dismiss, severity levels (info, success, warning, error).
- [ ] **EqTabs** (Molecule) - Tabbed content panels.
- [ ] **EqBadge** (Atom) - Small status indicator/label.
- [ ] **EqAvatar** (Atom) - User avatar with image, initials fallback, and size variants.
- [ ] **EqDivider** (Atom) - Horizontal/vertical separator.
- [ ] **EqTooltip** (Atom/Molecule) - Hover tooltip.
- [ ] **EqDropdown** (Molecule) - Dropdown menu with selectable items.

### Theming & Customization
- [ ] **Dark/light mode toggle** - Allow themes to define both dark and light variants with a toggle mechanism.
- [ ] **Theme creation guide** - Document how to create new built-in themes (CSS variable structure, naming conventions).

### Infrastructure
- [ ] **Automated testing** - Component snapshot or visual regression tests.
- [ ] **CI/CD pipeline** - GitHub Actions for build verification on push/PR.
- [ ] **Crate publishing** - Evaluate publishing to crates.io for easier dependency management (vs git dependency).

---

## Later - Future Vision

Longer-term ideas and aspirations.

### Components
- [ ] **EqTable** (Organism) - Data table with sorting, filtering, pagination.
- [ ] **EqForm** (Organism) - Form builder with validation, field groups, and submission handling.
- [ ] **EqSidebar** (Organism) - Collapsible sidebar navigation.
- [ ] **EqBreadcrumb** (Molecule) - Navigation breadcrumb trail.
- [ ] **EqPagination** (Molecule) - Page navigation controls.
- [ ] **EqStepper** (Molecule) - Multi-step progress indicator.
- [ ] **EqDrawer** (Organism) - Slide-in panel from edge of screen.
- [ ] **EqCommandPalette** (Organism) - Keyboard-driven command palette (Cmd+K style).

### Platform
- [ ] **Desktop-specific components** - Components optimized for Dioxus desktop (native menus, system tray integration).
- [ ] **Mobile-responsive patterns** - Ensure all components work well on mobile viewports. Add mobile-specific organisms (bottom sheet, swipe gestures).
- [ ] **Accessibility audit** - ARIA roles, keyboard navigation, screen reader support across all components.
- [ ] **Animation system** - Shared transition/animation utilities for component enter/exit states.

### Ecosystem
- [ ] **Documentation site** - Dedicated docs site built with eq_ui itself (dogfooding).
- [ ] **Starter templates** - Project templates for web, desktop, and fullstack Dioxus apps using eq_ui.
- [ ] **Community themes** - Allow third-party theme contributions with a standard format.
- [ ] **Component playground** - Interactive editor where users can tweak props and see live previews.

---

## Contributing


To run the showcase locally:
```bash
dx serve --example showcase --platform web
```

To update the library in a consuming project:
```bash
rm -rf ~/.cargo/git/checkouts/eq_ui-*
rm -rf ~/.cargo/git/db/eq_ui-*
cargo update -p eq_ui
```
