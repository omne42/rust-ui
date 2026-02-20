use crate::{
    StatusLightRole, StatusLightVariant,
    logic::{self, StatusLightRootInput},
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, StatusLightOptions, use_status_light};

#[component]
pub fn StatusLight(
    #[prop(optional)] variant: Option<StatusLightVariant>,
    #[prop(optional)] role: Option<StatusLightRole>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let root = logic::normalize_root_state(StatusLightRootInput {
        variant,
        role,
        class_name,
    });
    let semantics = use_status_light(StatusLightOptions {
        state: root.state,
        lang,
        dir,
    });

    view! {
        <span
            class=root.class_name
            role=semantics.attrs.role
            aria-live=semantics.attrs.aria_live
            lang=semantics.attrs.lang
            dir=semantics.attrs.dir
            data-slot="status-light"
            data-variant=semantics.attrs.data_variant
            data-state=semantics.attrs.data_state
            data-live=semantics.attrs.data_live
            data-static=semantics.attrs.data_static
            data-role=semantics.attrs.data_role
            data-role-source=semantics.attrs.data_role_source
            data-custom-class=semantics.attrs.data_custom_class
            data-class-source=semantics.attrs.data_class_source
        >
            <span
                class="ui-status-light__dot"
                data-slot="status-light-indicator"
                data-variant=semantics.attrs.data_variant
                aria-hidden="true"
            ></span>
            <span class="ui-status-light__label" data-slot="status-light-label">
                {children()}
            </span>
        </span>
    }
}
