use dioxus::prelude::*;
use crate::theme::{APP, CONTAINER_LAYOUT, MAIN_CONTENT, MAIN_INNER};

/// Generic app shell layout.
/// The platform crate passes its own header, footer, and main content
/// (typically `Outlet::<Route>`) as Element props.
#[component]
pub fn EqAppShell(
    header: Element,
    footer: Element,
    children: Element,
) -> Element {
    rsx! {
        div { id: "app", class: APP,
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
