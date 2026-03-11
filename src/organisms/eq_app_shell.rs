use dioxus::prelude::*;
use crate::theme::{merge_classes, APP, CONTAINER_LAYOUT, MAIN_CONTENT, MAIN_INNER};

/// Generic app shell layout.
/// The platform crate passes its own header, footer, and main content
/// (typically `Outlet::<Route>`) as Element props.
#[component]
pub fn EqAppShell(
    header: Element,
    footer: Element,
    children: Element,
    /// Optional class override — extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let cls = merge_classes(APP, &class);
    rsx! {
        div { id: "app", class: "{cls}",
            {header}

            main { class: "{MAIN_CONTENT} {MAIN_INNER}",
                div { class: CONTAINER_LAYOUT,
                    {children}
                }
            }

            {footer}
        }
    }
}
