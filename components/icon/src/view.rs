use crate::{
    IconSize, IconStateInput, IconTone,
    logic::{self},
    protocol,
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn Icon(
    #[prop(optional)] size: IconSize,
    #[prop(optional)] tone: IconTone,
    #[prop(optional)] is_disabled: bool,
    #[prop(default = true)] is_decorative: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] slot: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let slot = logic::normalize_optional_text(slot);
    let slot_kind = logic::resolve_slot_kind(slot.as_deref());
    let has_named_slot = slot.is_some();
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);

    let state = logic::resolve_state(IconStateInput {
        size,
        tone,
        disabled: is_disabled,
        decorative: is_decorative,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        slot_kind,
        has_named_slot,
    });

    let class = logic::compose_class_name(class_name, state);
    let agent_data = protocol::resolve_agent_data_attrs(protocol::IconAgentInput {
        intent: protocol::IconAgentIntent::IconRender,
        state_attr: state.data_state_attr,
        source_attr: state.aria_source_attr,
    });
    let output_data = protocol::resolve_output_data_attrs();

    view! {
        <span
            class=class
            lang=locale.lang
            dir=locale.dir
            role=(!state.is_decorative).then_some("img")
            aria-label=state.has_accessible_name.then_some(aria_label)
            aria-hidden=state.is_decorative.then_some("true")
            data-slot="icon"
            slot=slot.clone()
            data-slot-name=slot.clone()
            data-slot-kind=state.slot_kind.as_attr()
            data-has-slot=state.has_named_slot.then_some("true")
            data-size=state.size_attr
            data-tone=state.tone_attr
            data-state=state.data_state_attr
            data-disabled=state.is_disabled.then_some("true")
            data-decorative=state.is_decorative.then_some("true")
            data-has-label=state.has_accessible_name.then_some("true")
            data-aria-source=state.aria_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            data-ui-schema=agent_data.schema_name
            data-ui-schema-version=agent_data.schema_version.as_attr()
            data-ui-intent=agent_data.intent.as_attr()
            data-ui-action=agent_data.action.as_attr()
            data-ui-state=agent_data.state.as_attr()
            data-ui-source=agent_data.source.as_attr()
            data-ui-streaming=output_data.streaming.as_attr()
            data-ui-streaming-fallback=output_data.fallback.as_attr()
            data-ui-output-mode=output_data.mode.as_attr()
            data-ui-output-status=output_data.status.as_attr()
        >
            <span class="ui-icon__glyph" data-slot="icon-glyph" aria-hidden="true">
                {children()}
            </span>
        </span>
    }
}
