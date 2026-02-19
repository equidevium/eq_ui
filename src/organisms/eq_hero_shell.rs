use dioxus::prelude::*;
use super::eq_hero_shell_styles::*;
use crate::theme::CONTAINER_LAYOUT;

#[component]
pub fn EqHeroShell(
    #[props(into)]
    title: String,
    #[props(into)]
    subtitle: Option<String>,
    actions: Option<Element>,
) -> Element {
    rsx! {
        section { class: HERO_SHELL,
            div { class: CONTAINER_LAYOUT,
                h1 { class: HERO_TITLE, "{title}" }

                if let Some(subtitle) = subtitle {
                    p { class: HERO_SUBTITLE, "{subtitle}" }
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
