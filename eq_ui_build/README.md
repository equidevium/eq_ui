# eq_ui_build

Build-time utility for [eq_ui](https://crates.io/crates/eq_ui). Scans a folder of SVG files and generates a Rust source file with `pub const` path data entries, ready for use with `EqIcon`.

Zero dependencies — only uses `std`.

## Setup

```toml
[dependencies]
eq_ui = "0.2"

[build-dependencies]
eq_ui_build = "0.1"
```

## Usage

Create a `build.rs` at your crate root:

```rust
fn main() {
    eq_ui_build::generate_icon_paths(
        "assets/icons",         // folder containing your .svg files
        "my_icon_paths.rs",     // output file name (written to OUT_DIR)
    );
}
```

Include the generated file in your source:

```rust
// src/icons.rs
include!(concat!(env!("OUT_DIR"), "/my_icon_paths.rs"));
```

Constant names derive from file names: `arrow-right.svg` becomes `ARROW_RIGHT`, `star.svg` becomes `STAR`.

## How it works

The function reads every `.svg` file in the given folder, extracts the `d` attribute from the first `<path>` element, and writes a `pub const` entry for each one. The build script reruns automatically when the folder contents change. If the folder does not exist, an empty file is generated.

## License

MIT OR Apache-2.0
