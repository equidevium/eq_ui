use dioxus::prelude::*;
use super::eq_hero_shell_styles::*;
use crate::theme::CONTAINER_LAYOUT;

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
) -> Element {
    rsx! {
        section { class: "{HERO_SHELL} {HERO_SHELL_RELATIVE}",
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
