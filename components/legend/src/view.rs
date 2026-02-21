use crate::{
    LegendMotion,
    logic::{self, LegendTone},
    motion,
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, LegendOptions, use_legend};

fn required_indicator_view(is_required: bool, required_indicator: String) -> impl IntoView {
    view! {
        <Show when=move || is_required>
            <span class="ui-legend__required" data-slot="legend-required" aria-hidden="true">
                {required_indicator.clone()}
            </span>
        </Show>
    }
}

#[component]
pub fn Legend(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional)] tone: LegendTone,
    #[prop(optional)] is_required: Option<bool>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] motion: LegendMotion,
    #[prop(optional, into)] required_indicator: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let normalized = logic::normalize_component_state(logic::LegendNormalizeInput {
        tone,
        is_required,
        is_disabled,
        text,
        required_indicator,
        class_name,
    });
    let required_state = normalized.required_state;
    let accessibility_state = normalized.accessibility_state;
    let state = normalized.state;
    let class = logic::compose_class_name(normalized.class_name, state);
    let text = normalized.text;
    let required_indicator = normalized.required_indicator;

    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != LegendMotion::default();
    let motion_style = motion::attach_motion(motion);

    let semantics = use_legend(LegendOptions { state, lang, dir });
    let attrs = semantics.attrs;
    let legend_lang = attrs.lang;
    let legend_dir = attrs.dir;
    let legend_aria_disabled = attrs.aria_disabled;
    let legend_data_tone = attrs.data_tone;
    let legend_data_state = attrs.data_state;
    let legend_data_required = attrs.data_required;
    let legend_data_disabled = attrs.data_disabled;
    let legend_data_text_source = attrs.data_text_source;
    let legend_data_indicator_source = attrs.data_indicator_source;
    let legend_data_custom_class = attrs.data_custom_class;
    let legend_data_class_source = attrs.data_class_source;
    let agent_contract = logic::resolve_agent_contract();

    view! {
        <legend
            class=class
            style=motion_style.clone()
            lang=legend_lang
            dir=legend_dir
            data-slot="legend"
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-tone=legend_data_tone
            data-state=legend_data_state
            data-required=legend_data_required
            data-disabled=legend_data_disabled
            data-text-source=legend_data_text_source
            data-indicator-source=legend_data_indicator_source
            data-custom-class=legend_data_custom_class
            data-class-source=legend_data_class_source
            data-required-source=required_state.required_source_attr
            data-disabled-source=accessibility_state.disabled_source_attr
            data-ui-schema=agent_contract.schema_attr
            data-ui-schema-version=agent_contract.schema_version_attr
            data-ui-stream-support=agent_contract.stream_support_attr
            data-ui-stream-fallback=agent_contract.stream_fallback_attr
            data-ui-stream-mode=agent_contract.stream_mode_attr
            data-ui-output-status=agent_contract.output_status_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action=logic::LegendUiAction::Idle.as_attr()
            data-ui-source=logic::LegendUiSource::Component.as_attr()
            data-ui-state=legend_data_state
            aria-disabled=legend_aria_disabled
        >
            <span class="ui-legend__text" data-slot="legend-text">
                {text}
            </span>

            {required_indicator_view(state.is_required, required_indicator)}
        </legend>
    }
}
