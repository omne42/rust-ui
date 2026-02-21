use crate::label::{
    LabelMotion,
    logic::{self, LabelEmphasis},
    motion,
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, locale_attrs};

fn render_required_indicator(required_indicator: String, is_required: bool) -> impl IntoView {
    view! {
        <Show when=move || is_required>
            <span class="ui-label__required" data-slot="label-required" aria-hidden="true">
                {required_indicator.clone()}
            </span>
        </Show>
    }
}

#[component]
pub fn Label(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] for_id: Option<String>,
    #[prop(optional)] is_required: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] emphasis: LabelEmphasis,
    #[prop(optional, into)] required_indicator: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: LabelMotion,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let normalized = logic::normalize_view_input(logic::LabelViewInput {
        text,
        for_id,
        required_indicator,
        class_name,
        lang,
    });
    let render_state = logic::derive_render_state(
        logic::LabelStateAxisInput {
            emphasis,
            is_required,
            is_disabled,
        },
        &normalized,
    );
    let locale = locale_attrs(normalized.lang.clone(), dir);
    let for_id = normalized.for_id;
    let text = normalized.text;
    let required_indicator = normalized.required_indicator;
    let state = render_state.state;
    let class_name = render_state.class_name;
    let motion_source = motion::motion_source_attr(motion);
    let agent_contract = logic::resolve_agent_contract_attrs(state, motion_source);
    let motion_style = Signal::derive(move || motion::attach_motion(None, motion));

    view! {
        <label
            class=class_name
            style=move || motion_style.get()
            for=for_id
            lang=locale.lang.clone()
            dir=locale.dir
            data-slot="label"
            data-emphasis=state.emphasis_attr
            data-state=if state.is_required { "required" } else { "optional" }
            data-required=state.is_required.then_some("true")
            data-disabled=state.is_disabled.then_some("true")
            data-has-for=state.has_for_id.then_some("true")
            data-label-source=state.label_source_attr
            data-indicator-source=state.indicator_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            data-motion-source=motion_source
            data-ui-schema=agent_contract.schema_attr
            data-ui-schema-version=agent_contract.schema_version_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action=agent_contract.action_attr
            data-ui-state=agent_contract.state_attr
            data-ui-source=agent_contract.source_attr
            data-ui-stream-support=agent_contract.stream_support_attr
            data-ui-stream-fallback=agent_contract.stream_fallback_attr
            data-ui-output-status=agent_contract.output_status_attr
            data-ui-label-source=agent_contract.label_source_attr
            data-ui-indicator-source=agent_contract.indicator_source_attr
            data-ui-class-source=agent_contract.class_source_attr
            data-ui-motion-source=agent_contract.motion_source_attr
            aria-disabled=state.is_disabled.then_some("true")
        >
            <span class="ui-label__text" data-slot="label-text">
                {text}
            </span>
            {render_required_indicator(required_indicator, state.is_required)}
        </label>
    }
}

#[cfg(all(test, not(feature = "component-label")))]
#[path = "../test/semantics.rs"]
mod semantics_tests;
