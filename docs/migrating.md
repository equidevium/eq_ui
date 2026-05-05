# Adding eq_ui to an existing project

This guide is for the case where you already have a Dioxus or
Tailwind project running and want to bring eq_ui in without breaking
your existing UI. If you're starting from scratch, read
[getting-started.md](./getting-started.md) instead.

The friction points, in roughly the order people hit them:

1. Tailwind `@source` collisions
2. CSS variable conflicts (eq_ui owns the `--color-*` namespace)
3. Coexisting with components you already styled yourself
4. The incremental adoption order

---

## Audit checklist before you install

Before adding the dependency, take five minutes to check the
following in your existing project. Any one of these can cause
silent breakage:

- **Tailwind version.** eq_ui targets Tailwind v4. v3 will not
  produce the same output. Check `package.json` or your CSS imports
  for `@tailwindcss/cli@3.x` or `tailwindcss@3.x`.
- **CSS resets.** If you load Normalize, Reset, or another base
  stylesheet, eq_ui's `UI_INDEX_CSS` may conflict on form-element
  defaults (input border, button reset). Note which one wins.
- **CSS variable names.** eq_ui owns the `--color-*` namespace
  (about 60 variables). If your existing CSS uses any
  `--color-foo` variable, expect a clash.
- **Existing theme system.** If you already have a theme switcher
  that writes to `<style>` or `<html>` attributes, decide whether
  to keep yours or adopt `EqTheme`. They will fight each other.
- **Dioxus version.** eq_ui pins `dioxus = "=0.7.3"`. If you're on
  a different 0.7.x, Cargo's resolver will reject the install. If
  you're on 0.6.x, you cannot use eq_ui without upgrading.

---

## Tailwind setup

eq_ui ships its own compiled Tailwind output as `UI_TAILWIND_CSS`.
You have two options:

### Option A: use eq_ui's bundled CSS as-is

Simplest. Reference `UI_TAILWIND_CSS` once and forget about it. Your
own Tailwind config keeps building separately. The downside is that
your project ends up shipping two Tailwind bundles to the browser.
For small apps this is fine; for production you probably want
Option B.

```rust
document::Link { rel: "stylesheet", href: UI_TAILWIND_CSS }
document::Link { rel: "stylesheet", href: "/your/own/tailwind.css" }
```

### Option B: merge into one bundle

Recommended for production. Add a second `@source` directive to
your existing `tailwind.css` so your Tailwind compiler scans eq_ui
too:

```css
@import "tailwindcss";

@source "./src/**/*.rs";
@source "../path/to/eq_ui/src/**/*.rs";
```

Then drop the eq_ui `UI_TAILWIND_CSS` link and rely on your own
bundle. Make sure both are pointing at the same Tailwind v4. Mixing
v3 and v4 silently produces broken output.

If eq_ui is a crates.io dependency, the path is something like
`~/.cargo/registry/src/index.crates.io-*/eq_ui-0.5.*/src/**/*.rs`.
For a git or local-path dependency, use whatever path the checkout
lives at.

---

## CSS variable conflicts

eq_ui themes set ~60 CSS variables on `:root`. The full list is in
[theming.md](./theming.md), but the prefixes you should know about:

- `--color-primary-dark`, `--color-secondary-dark`, `--color-tertiary-dark`
- `--color-label-primary`, `--color-label-secondary`, `--color-label-bold`, `--color-label-muted`
- `--color-accent-primary`, `--color-accent-secondary`, `--color-accent-muted`
- `--color-card`, `--color-card-border`, `--color-card-shadow`
- `--color-error`, `--color-success`, `--color-warning`, `--color-info`
- `--color-input-*`, `--color-border-*`, `--color-surface-*`, `--color-code-*`
- `--gradient-primary-*`, `--btn-primary-*`, `--btn-ghost-*`, `--btn-outline-*`

If your existing CSS uses any of these names, the more recently
loaded sheet wins, which is usually eq_ui's. Two options:

- **Rename your variables.** Pick a project-specific prefix like
  `--myapp-color-*` and migrate. Cleanest long-term.
- **Override after eq_ui loads.** Keep your variables with their
  original names, but make sure your stylesheet is included after
  `EqThemeRenderer` in the rendered tree. Specificity wins by
  declaration order.

---

## Coexisting with components you already styled

You don't have to replace your existing components to use eq_ui.
The two design systems can coexist. Pragmatic rules:

- **Don't wrap eq_ui components in your own styled containers** if
  the container fights eq_ui's spacing or background. Either pick
  one styling system per region, or strip the wrapper to a layout
  primitive.
- **Don't use `class` overrides on eq_ui components to make them
  match your design language.** That fights the theme variables.
  If you need a different look, write a custom theme (see
  [theming.md](./theming.md)) so the variable contract stays
  consistent.
- **Page-level layout components are the most contagious.** If you
  adopt `EqAppShell` or `EqHeader`, you've effectively committed to
  eq_ui's spacing and theming for that page. Keep them at the
  outermost layer or don't use them.

---

## Incremental adoption order

If the project is large, bring eq_ui in bottom-up:

1. **Atoms first.** Replace one input or button with `EqInput` or
   `EqButton` on a single page. Verify theme switching still works.
   Atoms are the lowest-friction integration: they don't impose
   layout, they just style their own element.
2. **Molecules next.** `EqCard`, `EqAccordion`, `EqModal`. These
   bring layout opinions. Migrate one component at a time, not a
   whole page at once.
3. **Organisms last.** `EqAppShell`, `EqHeader`, `EqGrid`, the
   mobile shell. These take over their entire region. Adopt them
   only when you're committed to eq_ui's layout for that page.

This order minimizes the chance that you break a working layout
mid-migration. Atoms don't fight with your existing CSS; organisms
will.

---

## Replacing an existing styled component

When you swap one of your hand-rolled components for an eq_ui
equivalent, expect to:

- Delete the component's CSS file. eq_ui handles styling via theme
  variables.
- Update consumers if your old component had different prop names.
  eq_ui props are documented inline; match them up.
- Re-test theme switching. Your old component might have hard-coded
  colors; eq_ui's equivalent will follow the active theme.
- Re-test responsive behavior. eq_ui's breakpoints follow Tailwind
  defaults (`sm`, `md`, `lg`, `xl`, `2xl`). If your old component
  used different breakpoints, the layout will shift at different
  widths.

---

## When eq_ui isn't a fit

Some cases where you should not migrate:

- **Your existing design system is actively maintained and battle-
  tested.** eq_ui is a Dioxus-specific kit; if you already have one,
  the rewrite cost rarely pays back.
- **Your design language is dramatically different.** eq_ui's
  components are opinionated about spacing, typography, and shape.
  If your brand calls for square corners and tight density, you'll
  fight the theme variables on every page.
- **You need components eq_ui doesn't have yet.** Check the
  [ROADMAP](../ROADMAP.md) for what's planned. If your critical
  component (rich text editor, signature pad, full data grid with
  exotic features) isn't on the roadmap or in the catalog, eq_ui
  is missing pieces you'll have to build yourself.

---

## When something breaks during migration

See [troubleshooting.md](./troubleshooting.md) for failure modes
and fixes, especially:

- Components render unstyled (`@source` not finding eq_ui).
- Themes don't switch (missing `EqThemeRenderer` or provider).
- `cargo update` doesn't pick up local changes (cache cleanup).
- Visual regressions after migration (CSS variable conflicts).

If you hit something not covered there, file an issue at
[github.com/equidevium/eq_ui/issues](https://github.com/equidevium/eq_ui/issues)
with your previous setup, the migration step that broke, and the
visible symptom.
