use crate::badge::{
    BadgeVariant,
    logic::{self, BadgeStateInput},
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn Badge(
    #[prop(optional)] variant: BadgeVariant,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(BadgeStateInput {
        variant,
        has_custom_class_name: class_name.is_some(),
    });
    let locale = locale_attrs(lang, dir);
    let agent_contract = logic::resolve_agent_contract(state);
    let class = logic::compose_class_name(class_name, state);

    view! {
        <span
            class=class
            lang=locale.lang
            dir=locale.dir
            data-slot="badge"
            data-variant=state.variant_attr
            data-fill=state.fill_attr
            data-state=state.fill_attr
            data-solid=state.is_solid.then_some("true")
            data-outline=state.is_outline.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=agent_contract.class_source_attr
            data-ui-schema=agent_contract.schema_attr
            data-ui-schema-version=agent_contract.schema_version_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action=agent_contract.action_attr
            data-ui-state=agent_contract.state_attr
            data-ui-source=agent_contract.source_attr
            data-ui-stream-support=agent_contract.stream_support_attr
            data-ui-stream-fallback=agent_contract.stream_fallback_attr
            data-ui-stream-mode=agent_contract.stream_mode_attr
            data-ui-output-status=agent_contract.output_status_attr
        >
            {children()}
        </span>
    }
}
