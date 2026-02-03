use crate::status_light::{StatusLightRole, StatusLightVariant};
use leptos::prelude::*;

#[component]
pub fn StatusLight(
    #[prop(optional)] variant: StatusLightVariant,
    #[prop(optional)] role: Option<StatusLightRole>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let base_class = format!("ui-status-light {}", variant.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <span class=class data-slot="status-light" role=role.map(StatusLightRole::as_attr)>
            <span class="ui-status-light__dot" data-slot="status-light-indicator" aria-hidden="true"></span>
            <span class="ui-status-light__label" data-slot="status-light-label">
                {children()}
            </span>
        </span>
    }
}
