# eq_ui v0.4.0

Accessibility-first release. Every implemented component now carries full or native ARIA support, the playground has been rebuilt from the ground up, and two new interactive atoms join the library.

## New Components

**EqRadioGroup** - Radio button group with mutually exclusive selection. Three sizes (Sm, Md, Lg), vertical and horizontal layout, disabled state, full WAI-ARIA radiogroup pattern with roving tabindex keyboard navigation (arrow keys, Home, End).

**EqSwitch** - Toggle switch with a pill track and sliding thumb. Three sizes matching EqRadioGroup, on/off state, disabled state, smooth CSS transition on toggle. Full ARIA switch role with aria-checked.

## Playground Refactoring

The playground has been completely restructured for maintainability and extensibility.

- Extracted the ComponentDescriptor pattern into a dedicated module, making it straightforward for consuming projects to register their own components alongside eq_ui's built-in set.
- Split playground logic into focused modules: playground_helpers.rs for shared rendering utilities, playground_types.rs for type definitions, and playground_guide.rs for the new in-app getting started guide.
- Added a Theme Showcase panel that displays all CSS variables for the active theme with live color swatches, gradient previews, button variant demos, and interactive card samples.
- Two new built-in themes: total theme count is now 21.
- Progress bar CSS updated for better visual consistency across themes.

## ARIA Accessibility

This is the headline change. Every implemented component has been audited and updated with appropriate ARIA attributes, keyboard navigation, and screen reader support.

**Atoms with full ARIA:**
- EqCheckbox: role checkbox, aria-checked with tristate (true/false/mixed), aria-label
- EqDivider: role separator, aria-orientation
- EqIcon: aria-hidden on decorative icons, aria-label on meaningful ones
- EqImage: alt text enforcement, role img where appropriate
- EqRadioGroup: role radiogroup, role radio on options, aria-checked, roving tabindex with arrow key navigation via document::eval()
- EqSwitch: role switch, aria-checked, aria-label
- EqVideo: aria-label on container, alt text on poster image
- EqScrollableSpace: role region, aria-label, tabindex for keyboard scrolling
- EqTab: role tablist/tab/tabpanel, aria-selected, aria-controls/aria-labelledby linking, roving tabindex with left/right arrow navigation, Home/End support
- EqProgress: role progressbar, aria-valuenow/min/max, aria-label

**Molecules with full ARIA:**
- EqCard: role and aria-label support, aria-hidden on decorative elements
- EqImageCard: inherits EqCard ARIA, proper alt text on card image
- EqCarousel: role region, aria-roledescription carousel, aria-label on slides, aria-live polite for slide changes
- EqTree: Full WAI-ARIA tree view pattern with role tree/treeitem/group, aria-expanded, aria-selected, aria-level, aria-setsize, aria-posinset, shared expansion state via Signal<HashSet<String>>, roving tabindex with full keyboard navigation (Up/Down/Left/Right/Home/End/Enter/Space)
- EqAccordion: Full WAI-ARIA accordion pattern with aria-expanded, aria-controls/aria-labelledby linking between headers and panels, role region on panels, keyboard navigation (Up/Down/Home/End) via document::eval()

**Organisms with full ARIA:**
- EqHeroShell: aria-labelledby auto-linked to h1 title, aria-hidden on decorative background and overlay divs, optional role prop (e.g. banner)
- EqGrid: Comprehensive table ARIA including aria-sort on sortable column headers (ascending/descending/none), scope=col on all header cells, aria-selected on data rows, aria-rowcount and aria-colcount on the table element, aria-busy during loading, aria-live polite on pagination info, aria-label on filter inputs and navigation buttons, aria-current page on active page button, aria-hidden on virtualisation spacer rows, optional ARIA live region for drag-and-drop reorder announcements

**Components using native HTML semantics (no extra ARIA needed):**
- EqText, EqLabel, EqLink, EqInput, EqButton, EqHeader, EqFooter, EqNavbar, EqAppShell, EqPageSection

## Bug Fixes

- Fixed alt string being incorrectly borrowed in EqImage, causing a compile error in certain usage patterns.
- Updated progress bar CSS for more consistent rendering across all 21 themes.

## Infrastructure

- README updated to reflect current ARIA coverage across all components.

## Breaking Changes

None. All new ARIA attributes are additive. Existing component APIs remain unchanged. New components (EqRadioGroup, EqSwitch) are additions only.

## What's Next

- Blitz renderer compatibility testing (CSS gap analysis filed upstream)
- dioxus-primitives integration for positioning-dependent components (Select, Tooltip, Dropdown)
- New components: Skeleton, Slider, Dialog, ToastList
