# Theming

eq_ui themes are CSS files. Each theme defines a set of CSS variables
on `:root`, and the runtime switches themes by injecting the matching
CSS into a `<style>` tag. There are 26 built-in themes plus an API
for runtime-loaded custom CSS.

This guide covers the variable contract, the built-in themes, how to
author a custom theme, and how to override colors per component
instance. Setup (`use_theme_provider`, `EqThemeRenderer`) is in
[getting-started.md](./getting-started.md).

---

## The variable contract

Every theme defines roughly 60 variables on `:root`. They group as
follows. The Limbotron source file
(`assets/theme/limbotron.css`) is the canonical reference if you
need an exact, complete list with values.

### Surfaces and backgrounds

```css
--color-primary-dark    /* main app background */
--color-secondary-dark  /* secondary surface (rarely used directly) */
--color-tertiary-dark   /* card / elevated surface */
--color-card            /* card body fill */
--color-card-border     /* default border color on cards and dividers */
--color-card-shadow     /* glow / drop-shadow color */
--color-background      /* deepest backdrop for full-bleed sections */
```

### Text and labels

```css
--color-label-primary    /* primary heading and body text */
--color-label-secondary  /* secondary / muted text */
--color-label-bold       /* emphasis color (often the accent) */
--color-label-muted      /* dimmer secondary text */
--color-label-disabled   /* disabled-state text */
```

### Accent colors

```css
--color-accent-primary   /* main accent (CTA buttons, active state) */
--color-accent-secondary /* supporting accent */
--color-accent-muted     /* desaturated accent for hovers / focus */
```

### Gradients

```css
--color-gradient-default-start
--color-gradient-default-mid
--color-gradient-default-end
--color-btn-hover-start
--color-btn-hover-mid
--color-btn-hover-end
--color-btn-default-start
--color-btn-default-mid
--color-btn-default-end

--gradient-primary-tricolor       /* assembled from the three default colors */
--gradient-primary-tricolor-hover /* assembled from the three hover colors */
--gradient-primary-duocolor       /* end -> start, two-stop */
--gradient-background             /* full-page diagonal */
```

### State and feedback

```css
--color-success
--color-warning
--color-error
--color-info
```

### Borders and inputs

```css
--color-border-default
--color-border-subtle
--color-border-active
--color-input-bg
--color-input-border
--color-input-focus
--color-input-placeholder
```

### Overlays and tooltips

```css
--color-surface-elevated
--color-surface-overlay   /* modal / dropdown backdrop, usually translucent black */
--color-surface-tooltip
```

### Code blocks

```css
--color-code-bg
--color-code-text
--color-code-comment
--color-code-keyword
--color-code-string
```

### Buttons

```css
--btn-primary-bg
--btn-primary-bg-hover
--btn-primary-bg-solid
--btn-primary-hover
--btn-primary-text
--btn-ghost-hover
--btn-outline-border
--btn-outline-hover-border
--btn-outline-hover-bg
--btn-danger-bg
```

### Interactive

```css
--color-card-border-bright
--color-focus-ring
--color-shadow-glow
--color-primary
```

### Scrollbar and grid

```css
--color-scrollbar-thumb
--color-scrollbar-track

--color-grid-header-bg
--color-grid-header-text
--color-grid-border
```

### Transitions and corner tokens

```css
--transition-fast
--transition-normal

--radius-pill       /* used by themes that opt into pill corners */
--radius-container
--radius-textarea
```

The pill / radius tokens are present in every theme file but only
matter for themes that use them. Limbotron, Cloud, and Synthwave
all apply pill corners via a `!important` override block at the end
of their CSS file. Other themes ignore the tokens and leave the
default Tailwind rounding.

---

## Switching themes

Setup lives in your root component (see
[getting-started.md](./getting-started.md)). Once `EqTheme::use_theme_provider()`
is called and `EqThemeRenderer {}` is in the rendered tree, theme
changes propagate automatically.

### From UI

Read the theme list and write the signal:

```rust,no_run
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

`EqTheme::built_in_variants()` returns `Vec<(&'static str, EqTheme)>`
ordered the same as the `EqTheme` enum.

### Programmatically

```rust,no_run
use eq_ui::prelude::*;

EqTheme::set_theme(EqTheme::Ocean);
```

`set_theme` looks up the provider in the current Dioxus context and
writes the signal. It must be called from inside a component or hook,
not from a free function.

---

## The 26 built-in themes

Quick character notes. Run the playground locally for a visual
preview of each.

| Theme | Character |
|---|---|
| Limbotron (default) | Dark plum base, hot rose accent, mint borders, pill corners |
| Cloud | Light theme, white surfaces, blue accents, pill corners |
| Unghosty | Neutral dark, subtle accents, rounded standard corners |
| Burgundy | Deep wine red, warm brown surfaces |
| Gold | Warm gold and amber, dark base |
| PurplePink | Saturated purple primary, pink highlights |
| Monochrome | Pure black and white, single grayscale palette |
| Watermelon | Pink and green pairing, summer-feel |
| Sunset | Orange-pink gradients, warm dark base |
| Ocean | Deep blue base, cyan and teal accents |
| Spacetime | Cosmic blue-purple, star-field feel |
| Gruvbox | Warm earth tones, classic editor palette |
| Monokai | Classic Sublime/TextMate green-orange-pink |
| Hellas | Mediterranean blue and white |
| Egypt | Sand, gold, deep blue |
| Dometrain | Brand colors for the Dometrain training platform |
| Catppuccin | Pastel mocha-flavor palette |
| Dracula | Purple-pink editor theme |
| Nord | Cool blue-gray Scandinavian palette |
| OneDark | Atom editor's signature dark theme |
| RosePine | Soft pink-purple, low-contrast |
| SolarizedDark | Solarized-base dark variant |
| TokyoNight | Purple-blue night feel |
| Warcraft | Deep red and gold fantasy palette |
| SweetRush | Vivid candy colors, high saturation |
| Synthwave | Violet base, magenta primary, cyan complement, 80s gradients, pill corners |

`Limbotron` is the default if you do not call `set_theme`. To change
the default for your app, call `set_theme` once during startup.

---

## Authoring a custom theme

Two paths.

### Path A: ship a CSS file, add a variant

For a theme you want to use across the app and possibly contribute
back to eq_ui:

1. Copy `assets/theme/limbotron.css` (or whichever theme is closest
   to what you want) and rename it.
2. Edit the variable values. Keep the variable names and structure
   identical. Skipped variables fall back to whatever
   `UI_INDEX_CSS` defines, which may not match your other choices.
3. If you want pill corners, copy the override block at the bottom
   of `limbotron.css`. If you don't, drop it.
4. To make it part of the EqTheme enum, add a variant in
   `src/eq_theme.rs` and wire it in `css_content()` and
   `build_in_variants()`.

For external use, you can keep the CSS file outside the eq_ui crate
and load it via Path B below.

### Path B: load custom CSS at runtime

For one-off themes or user-customizable themes:

```rust,no_run
use eq_ui::prelude::*;

let css = r#"
:root {
    --color-primary-dark: #0a0518;
    --color-card: #1a0d2e;
    --color-card-border: #ff2e88;
    --color-accent-primary: #ff2e88;
    --color-accent-secondary: #22d3ee;
    --color-label-primary: #f5f0ff;
    /* ... define every variable from the contract above ... */
}
"#;

EqTheme::set_custom_theme(css.to_string());
```

`set_custom_theme` writes the variant `EqTheme::Custom(String)` into
the signal. `EqThemeRenderer` then injects the raw CSS instead of one
of the bundled files.

### Common mistakes

- **Defining only some variables.** Skipped variables fall back to
  `UI_INDEX_CSS` defaults, which may not match the rest of your
  palette. Define the full set.
- **Mixing themes via concatenation.** If you load custom CSS that
  imports another theme, browser specificity rules will produce
  surprising results. Pick one base.
- **Forgetting `EqThemeRenderer`.** A theme change without the
  renderer in the tree updates the signal but never injects CSS.

---

## Per-instance color overrides

A few components accept color props that override theme variables
at the instance level. The most common:

- `EqHeroShell` has `title_color` and `subtitle_color` for one-off
  brand colors on a hero.
- `EqDeviceFrame` has `shell_color` to override the device shell.
- `EqButton` has `color` for solid (non-gradient) variants.

When you reach for these, ask whether the override should really
live in the theme. If five buttons all need the same one-off color,
that's a theme variable, not five hard-coded overrides. If a hero
truly needs a brand color that doesn't exist anywhere else in the
design, the per-instance override is correct.

---

## Roadmap: dark and light pairing

Currently themes are independent. There is no built-in concept of a
"light variant of TokyoNight" or "dark variant of Cloud". This is
roadmapped for v0.6 or later. Until it ships, themes that want a
light/dark pair need to define two separate themes and switch
between them based on `prefers-color-scheme` in app code:

```rust,no_run
use eq_ui::prelude::*;

let prefers_dark = /* read from web-sys / wry / your platform */;
let theme = if prefers_dark { EqTheme::Limbotron } else { EqTheme::Cloud };
EqTheme::set_theme(theme);
```

Track [ROADMAP.md](../ROADMAP.md) for when this becomes a first-class
feature.
