use super::{FormLabelAlign, FormLabelPosition, logic};
use leptos::prelude::*;
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn Form(
    children: Children,
    #[prop(optional, into)] is_disabled: Option<bool>,
    #[prop(optional, into)] is_read_only: Option<bool>,
    #[prop(optional, into)] is_required: Option<bool>,
    #[prop(optional)] label_position: Option<FormLabelPosition>,
    #[prop(optional)] label_align: Option<FormLabelAlign>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let resolved = logic::resolve_props(
        is_disabled,
        is_read_only,
        is_required,
        label_position,
        label_align,
        class_name,
    );

    provide_context(logic::FormContextValue {
        disabled: resolved.disabled,
        read_only: resolved.read_only,
        required: resolved.required,
        label_position: resolved.label_position,
        label_align: resolved.label_align,
    });

    let view_state = logic::resolve_view_state(&resolved);
    let agent_contract = logic::resolve_agent_contract_attrs(&view_state);
    let locale = locale_attrs(lang, dir);

    view! {
        <form
            class=resolved.class_name
            data-slot="form"
            data-disabled=view_state.data_disabled
            data-readonly=view_state.data_read_only
            data-required=view_state.data_required
            data-label-position=view_state.label_position
            data-label-align=view_state.label_align
            data-state-source=view_state.state_source
            data-ui-schema=agent_contract.schema
            data-ui-schema-version=agent_contract.schema_version
            data-ui-intent=agent_contract.intent
            data-ui-action=agent_contract.action
            data-ui-stream-mode=agent_contract.stream_mode
            data-ui-streaming-policy=agent_contract.streaming_policy
            data-ui-streaming-fallback=agent_contract.streaming_fallback
            data-ui-output-status=agent_contract.output_status
            data-ui-state-disabled=agent_contract.state_disabled
            data-ui-state-readonly=agent_contract.state_read_only
            data-ui-state-required=agent_contract.state_required
            data-ui-source=agent_contract.source
            aria-disabled=view_state.aria_disabled
            lang=locale.lang
            dir=locale.dir
        >
            {children()}
        </form>
    }
}

#[cfg(all(test, not(feature = "component-form")))]
#[path = "../test/semantics.rs"]
mod semantics_tests;
