use crate::button_group::ButtonGroupOrientation;
use leptos::prelude::*;

#[component]
pub fn ButtonGroup(
    #[prop(optional)] orientation: ButtonGroupOrientation,
    #[prop(optional)] attached: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Button group".to_string());

    let base_class = format!("ui-button-group {}", orientation.class_name());
    let base_class = if attached {
        format!("{base_class} ui-button-group--attached")
    } else {
        base_class
    };

    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <div class=class data-slot="button-group" role="group" aria-label=aria_label>
            {children()}
        </div>
    }
}
