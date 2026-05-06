# eq_ui docs

Pick the file that matches what you're trying to do.

---

### [getting-started.md](./getting-started.md)

You're starting a new project and want to add eq_ui from scratch.
Walks you from `cargo new` to a running app with a working theme
switcher. Assumes Dioxus 0.7 familiarity; not a Dioxus tutorial.

### [migrating.md](./migrating.md)

You already have a Dioxus or Tailwind project and want to bring
eq_ui in without breaking what you have. Covers `@source`
collisions, CSS variable conflicts, and incremental adoption order.

### [theming.md](./theming.md)

Reference for the theme system. The 60-variable contract, the 26
built-in themes, how to author a custom theme, per-instance color
overrides. Read this when you want to make eq_ui look different.

### [troubleshooting.md](./troubleshooting.md)

Something broke. Lists the common failure modes (components
unstyled, theme not switching, cargo cache issues, mobile platform
limitations) with cause and fix for each. Check here before filing
an issue.

---

### Other useful resources

- **The playground.** Clone the eq_ui repo and run
  `dx serve --example playground --features playground --platform web`.
  Every component has live prop controls, source examples, and a
  theme picker. Components verified inside `EqDeviceFrame` carry a
  phone icon (📱) in the tree so you can find mobile-tested ones at
  a glance.
- **[ROADMAP.md](../ROADMAP.md).** What's coming, what's deferred,
  and the cut criteria for each release.
- **[The repo](https://github.com/equidevium/eq_ui).** Source,
  issues, contributions.

If something in this guide is wrong or unclear, file an issue with
the page name and the problem you saw.
