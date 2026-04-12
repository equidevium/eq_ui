//! EqPlayground - interactive component showcase.
//!
//! Run with:
//!   dx serve --example playground --features playground
//!
//! To add custom components, push your own descriptors onto the vec:
//!
//! ```rust,ignore
//! let mut descs = eq_ui::all_component_descriptors();
//! descs.push(my_component::descriptor());
//! rsx! { EqPlayground { descriptors: descs } }
//! ```

use dioxus::prelude::*;
use eq_ui::{all_component_descriptors, EqPlayground};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        EqPlayground {
            descriptors: all_component_descriptors(),
        }
    }
}
