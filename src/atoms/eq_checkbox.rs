//! EqCheckbox — themed checkbox atom with checked, unchecked, and
//! indeterminate states. Renders Phosphor square icons internally.

use super::eq_checkbox_styles as s;
use super::eq_icon_paths;
use super::{EqIcon, IconSize};
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// Visual state of the checkbox.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CheckboxState {
    /// Empty square.
    #[default]
    Unchecked,
    /// Square with checkmark.
    Checked,
    /// Square with a horizontal dash (partial selection).
    Indeterminate,
}

/// Themed checkbox with three visual states.
///
/// Can be used standalone or composed inside other components (e.g. EqGrid
/// row selection). The component renders an icon-based checkbox — no native
/// `<input type="checkbox">` — for full theme control.
#[component]
pub fn EqCheckbox(
    /// Current visual state.
    #[props(default)]
    state: CheckboxState,
    /// Fired when the user clicks the checkbox. Receives the *new* state
    /// that would result from a toggle (Unchecked ↔ Checked). Indeterminate
    /// always transitions to Checked on click.
    #[props(default)]
    on_change: Option<EventHandler<CheckboxState>>,
    /// Disables interaction and dims the visual.
    #[props(default = false)]
    disabled: bool,
    /// Icon size override.
    #[props(default = IconSize::Sm)]
    size: IconSize,
    /// Optional label text rendered beside the checkbox.
    #[props(into, default)]
    label: String,
    /// Optional class override on the wrapper element.
    #[props(into, default)]
    class: String,
) -> Element {
    let (icon_path, icon_cls) = match state {
        CheckboxState::Unchecked => (eq_icon_paths::SQUARE, s::ICON),
        CheckboxState::Checked => (eq_icon_paths::CHECK_SQUARE, s::ICON_ACTIVE),
        CheckboxState::Indeterminate => (eq_icon_paths::MINUS_SQUARE, s::ICON_ACTIVE),
    };

    let wrapper_base = if disabled { s::WRAPPER_DISABLED } else { s::WRAPPER };
    let wrapper_cls = merge_classes(wrapper_base, &class);

    rsx! {
        span {
            class: "{wrapper_cls}",
            onclick: move |evt| {
                evt.stop_propagation();
                if !disabled {
                    if let Some(ref handler) = on_change {
                        let next = match state {
                            CheckboxState::Checked => CheckboxState::Unchecked,
                            _ => CheckboxState::Checked,
                        };
                        handler.call(next);
                    }
                }
            },
            EqIcon { path: icon_path, size: size, class: icon_cls }
            if !label.is_empty() {
                span { class: s::LABEL, "{label}" }
            }
        }
    }
}
