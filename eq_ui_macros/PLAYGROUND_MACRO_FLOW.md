# The `#[playground]` Macro: How It Works

## The Problem

Every component in eq_ui needs playground support, which means an interactive demo where you can tweak props and see the result live. Writing that by hand means around 200 lines of boilerplate per component: signal declarations, prop controls, a live preview, a gallery, a descriptor. It's the same pattern every time. Only the prop names and types change.

The `#[playground]` macro reads your component's signature and writes all of that for you.

---

## The Two Macros

There are two macros that work together. One is for enums, the other is for components.

### `#[derive(PlaygroundEnum)]`: Teaching Enums to Describe Themselves

When you write a prop enum like `SwitchSize { Sm, Md, Lg }`, the macro system needs to know what variants exist so it can build a dropdown control. But a proc macro can only see the code it's attached to. It can't peek into other files.

`PlaygroundEnum` solves this by making each enum carry its own description. When the compiler sees `#[derive(PlaygroundEnum)]` on an enum, it generates three functions:

- **`variant_names()`** returns `["Sm", "Md", "Lg"]` so the demo can populate a dropdown.
- **`from_name("Lg")`** converts the selected dropdown string back into `SwitchSize::Lg`.
- **`default_name()`** returns `"Md"` (whichever variant has `#[default]`) so the demo starts with a sensible value.

That's it. A self-description card stapled to the enum at compile time.

### `#[playground(...)]`: The Main Event

This is the attribute you place above `#[component]`. It does four things in sequence.

---

## The Flow, Step by Step

### Step 1: Parse the Attribute

The macro reads the arguments you passed:

```rust
#[playground(
    category = Atom,
    description = "Toggle switch with pill track.",
    examples = [("Basic", "EqSwitch { checked: true }")],
)]
```

It pulls out:

- **category**: which playground section this lives in (Atom, Molecule, Organism).
- **description**: the one-liner shown in the component list.
- **examples**: code snippets displayed at the bottom of the demo page.
- **flags**: `custom_demo`, `custom_gallery`, `no_styles` if you need escape hatches.

### Step 2: Parse the Props

The macro walks through each parameter in the function signature, looks at its type, and decides what kind of control it should get in the demo panel.

It sees `checked` and `disabled` are booleans, so it knows those should be on/off toggle switches. The user clicks the toggle, the component re-renders with the new value.

It sees `size` is a `SwitchSize`, which is not a primitive type. It recognizes this as a custom enum and decides to make a dropdown menu for it. The dropdown options come from the `PlaygroundEnumInfo` trait that `#[derive(PlaygroundEnum)]` generated earlier. So the dropdown automatically shows "Sm", "Md", "Lg" without the macro ever seeing the enum definition.

It sees `label` and `description` are strings, so it creates text input fields for them. The user types something in, the component updates live.

Then it hits `on_change`, which is an `Option<EventHandler>`. Event handlers are callbacks that the parent component provides at runtime. There's no sensible way to let a user "type in" a callback from a demo panel, so the macro skips it entirely.

Finally it sees `class`, which is the standard style override prop. Every component has one and it's meant for the consumer to pass custom CSS classes. It's not something you'd demo, so the macro skips that too.

The same logic applies to `children` and `Element` props. These represent nested content that gets passed into the component, and there's no way to represent that as a simple control.

It also reads `#[props(default = false)]` to know what initial value each signal should start with.

### Step 3: Generate the Code

This is where `codegen.rs` does its work. It takes the classified prop list and stamps out three things:

**The descriptor function.** A small function that returns metadata:

- An ID derived from the component name. `EqSwitch` becomes `"eq-switch"`.
- The category, description, and examples you provided.
- A pointer to the style catalog: `s::catalog()`.
- Pointers to the generated demo and gallery components.

**The demo component.** This is the big one. For each non-skipped prop, it generates:

- A `use_signal` declaration with the right default value. `use_signal(|| false)` for a bool, `use_signal(|| "Md".to_string())` for an enum.
- A prop control in the UI panel. `PropToggle` for bools, `PropSelect` for enums (populated via `PlaygroundEnumInfo::variant_names()`), `PropInput` for strings.
- A line in the live preview that reads from the signal. `checked: sig_checked()`, or `size: SwitchSize::from_name(&sig_size())` for enums.

All of this gets wrapped in the standard demo layout: a controls panel at the top, a live preview in the middle, style info at the bottom.

**The gallery component.** A compact showcase. If the component has an enum prop, it renders one instance per variant side by side. If not, it renders a single default instance.

### Step 4: Emit Everything

The macro outputs:

1. **Your original component function**, completely unchanged, byte for byte.
2. **The descriptor, demo, and gallery**, all wrapped in `#[cfg(feature = "playground")]` so they vanish in production builds.

The playground infrastructure doesn't know or care whether a descriptor came from a macro or was written by hand. They're the same type. So you can migrate components one at a time.

---

## The Escape Hatches

For most components, the auto-generated demo is perfect. For the rest (components with unusual interactivity like a carousel or a recursive tree) you have options:

- **`custom_demo`**: the macro skips demo generation. You write `DemoEqTree` by hand, but still get the free descriptor and gallery.
- **`custom_gallery`**: same idea, but for the gallery.
- **`#[playground(skip)]`** on a prop: excludes a specific prop from the controls panel.
- **`no_styles`**: for components that don't have a co-located `_styles.rs` file.

---

## What Changes in Practice

**Before**, adding a new component to the playground:

1. Write the component (~60 lines)
2. Write the descriptor function (~30 lines)
3. Write the demo component with signals, controls, preview (~100 lines)
4. Write the gallery component (~40 lines)
5. Add playground imports with feature gates (~10 lines)
6. Register in `lib.rs` (1 line)

**After**, with the macro:

1. Write the component (~60 lines)
2. Add `#[derive(PlaygroundEnum)]` to any prop enums (0 extra lines)
3. Add `#[playground(...)]` above `#[component]` (~5 lines)
4. Register in `lib.rs` (1 line)

The ~180 lines of boilerplate per component disappear. Across 20+ components, that's thousands of lines you never have to write, read, or maintain.
