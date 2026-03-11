use dioxus::prelude::*;
use super::eq_hero_shell_styles::*;
use crate::theme::{merge_classes, CONTAINER_LAYOUT};

#[component]
pub fn EqHeroShell(
    #[props(into)]
    title: String,
    #[props(into)]
    subtitle: Option<String>,
    #[props(into)]
    title_color: Option<String>,
    #[props(into)]
    subtitle_color: Option<String>,
    actions: Option<Element>,
    background: Option<Element>,
    /// Optional class override — extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    let base = format!("{HERO_SHELL} {HERO_SHELL_RELATIVE}");
    let cls = merge_classes(&base, &class);
    rsx! {
        section { class: "{cls}",
            if let Some(bg) = background {
                div { class: HERO_BG,
                    {bg}
                }
                div { class: HERO_OVERLAY }
            }
            div { class: "{CONTAINER_LAYOUT} {HERO_CONTENT}",
                h1 {
                    class: HERO_TITLE,
                    style: if let Some(ref c) = title_color { format!("color: {c}") } else { String::new() },
                    "{title}"
                }

                if let Some(subtitle) = subtitle {
                    p {
                        class: HERO_SUBTITLE,
                        style: if let Some(ref c) = subtitle_color { format!("color: {c}") } else { String::new() },
                        "{subtitle}"
                    }
                }

                if let Some(actions) = actions {
                    div { class: HERO_ACTIONS,
                        {actions}
                    }
                }
            }
        }
    }
}
