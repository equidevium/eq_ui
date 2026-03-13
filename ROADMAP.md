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
- `EqScrollableSpace` — Scrollable container with themed scrollbar
- `EqDivider` — Separator with variants (Solid, Dashed, Dotted, Spacer), weights, and spacing

### Molecules
- `EqCard` — Card with header, body, footer slots
- `EqImageCard` — Image card with caption modes (Below, Overlay)
- `EqCarousel` — Generic content carousel with Default and Peek modes
- `EqTree` — Collapsible tree view with select, expand/collapse, and child count
- `EqAccordion` — Collapsible panels with single-expand and multi-expand modes, smooth CSS grid animation, element headers

### Organisms
- `EqAppShell` — Full-page layout (header + main + footer)
- `EqHeader` — Sticky header with nav slot and backdrop blur
- `EqFooter` — Footer with link groups
- `EqNavbar` — Navigation bar
- `EqHeroShell` — Hero section with optional background image, overlay, and custom title/subtitle colors
- `EqPageSection` — Content section with title and description

### Theming
- `EqTheme` — Theme enum with custom CSS support
- `EqThemeRenderer` — Runtime theme switcher using `document::Style`
- Theme context via `use_theme_provider()` / `use_theme()` / `set_theme()`
- Built-in themes (21): Unghosty, Burgundy, Gold, PurplePink, Monochrome, Watermelon, Sunset, Ocean, Spacetime, Gruvbox, Monokai, Hellas, Egypt, Dometrain, Catppuccin, Dracula, Nord, OneDark, RosePine, SolarizedDark, TokyoNight

### Infrastructure
- Co-located `_styles.rs` pattern for all components
- `theme.rs` shared style tokens + `merge_classes()` utility
- CSS variable system (`colors.css`, `buttons.css`, `index.css`)
- Tailwind CSS v4 with `@source` directives scanning `.rs` files
- `class` prop on every component for style overrides via `merge_classes()`
- EqPlayground — interactive component playground with prop controls, variant galleries, CSS documentation, and usage examples

---

## Now - Active Development

These are the items currently being worked on or immediately planned.

### Components
- [ ] **EqVideo** (Organism) - Video player organism. To be decomposed into smaller molecules/atoms (play button, progress bar, controls, thumbnail).
- [ ] **EqButton** (Atom) - Dedicated button component. Style tokens already exist in `theme.rs` (`BTN_BASE`, `BTN_PRIMARY`, `BTN_GHOST`, `BTN_DANGER`, size variants). Needs its own component file + `_styles.rs`.

### Theming & Customization
- [ ] **Custom theme loading** - Finalize `set_custom_theme()` for loading user-provided CSS strings at runtime.

### Refactoring
- [ ] **EqCard macro unification** - The four EqCard sub-components (`EqCard`, `EqCardHeader`, `EqCardBody`, `EqCardFooter`) are structurally identical wrappers. Unify via a declarative macro to reduce duplication.

### Documentation & Tooling
- [ ] **Integration guide** - Step-by-step instructions for adding eq_ui to a consuming project (git dependency, Tailwind `@source`, theme setup, `EqThemeRenderer` wiring).
- [ ] **Playground improvements** - Continue refining interactive demos, adding new component demos as they ship.

---

## Next - Short-Term Priorities

Items to tackle once the "Now" batch stabilizes.

### Components
- [ ] **EqModal** (Molecule) - Modal dialog with backdrop, configurable size, and close behavior.
- [ ] **EqToast / EqNotification** (Molecule) - Toast notifications with auto-dismiss, severity levels (info, success, warning, error).
- [ ] **EqTabs** (Molecule) - Tabbed content panels.
- [ ] **EqBadge** (Atom) - Small status indicator/label.
- [ ] **EqAvatar** (Atom) - User avatar with image, initials fallback, and size variants. Note: EqImage currently forces aspect ratios that don't work well for a circular profile crop — EqAvatar will need either a dedicated circular image mode or a raw `img` element with `rounded-full` + `object-cover` instead of going through EqImage.
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
- [ ] **EqPlayground public version** - Host EqPlayground as a public site where users can explore components, tweak props, and preview themes without cloning the repo.

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
