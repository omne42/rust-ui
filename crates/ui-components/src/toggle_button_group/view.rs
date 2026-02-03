use crate::toggle_button_group::ToggleButtonGroupOrientation;
use leptos::prelude::*;

#[component]
pub fn ToggleButtonGroup(
    #[prop(optional)] orientation: ToggleButtonGroupOrientation,
    #[prop(optional)] attached: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Toggle group".to_string());

    let base_class = format!("ui-toggle-button-group {}", orientation.class_name());
    let base_class = if attached {
        format!("{base_class} ui-toggle-button-group--attached")
    } else {
        base_class
    };

    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <div
            class=class
            data-slot="toggle-button-group"
            role="group"
            aria-label=aria_label
        >
            {children()}
        </div>
    }
}
