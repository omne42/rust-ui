use crate::status_light::{
    StatusLightRole, StatusLightVariant,
    logic::{self, StatusLightStateInput},
};
use leptos::prelude::*;

#[component]
pub fn StatusLight(
    #[prop(optional)] variant: StatusLightVariant,
    #[prop(optional)] role: Option<StatusLightRole>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(StatusLightStateInput {
        variant,
        role,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    view! {
        <span
            class=class
            role=state.role_attr
            data-slot="status-light"
            data-variant=state.variant_attr
            data-state=if state.is_live { "live" } else { "static" }
            data-live=state.is_live.then_some("true")
            data-static=(!state.is_live).then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-role=state.role_attr
        >
            <span
                class="ui-status-light__dot"
                data-slot="status-light-indicator"
                data-variant=state.variant_attr
                aria-hidden="true"
            ></span>
            <span class="ui-status-light__label" data-slot="status-light-label">
                {children()}
            </span>
        </span>
    }
}
