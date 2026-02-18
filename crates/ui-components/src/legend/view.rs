use crate::legend::{
    LegendMotion, LegendStateInput,
    logic::{self, LegendTone},
    motion,
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, LegendOptions, use_legend};

#[component]
pub fn Legend(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional)] tone: LegendTone,
    #[prop(optional)] is_required: Option<bool>,
    #[prop(optional)] required: bool,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: LegendMotion,
    #[prop(optional, into)] required_indicator: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let required_state = logic::normalize_required_state(is_required, required);
    let accessibility_state = logic::normalize_accessibility_state(is_disabled, disabled);

    let (text, has_custom_text) = logic::normalize_text(text);
    let (required_indicator, has_custom_indicator) =
        logic::normalize_required_indicator(required_indicator);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != LegendMotion::default();
    let motion_style = motion::attach_motion(motion);

    let state = Signal::derive(move || {
        logic::resolve_state(LegendStateInput {
            tone,
            is_required: required_state.is_required,
            is_disabled: accessibility_state.is_disabled,
            has_custom_text,
            has_custom_indicator,
            has_custom_class_name,
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let semantics = use_legend(LegendOptions {
        state: state.get_untracked(),
        lang: logic::normalize_optional_text(lang),
        dir,
    });
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
            class=move || class.get()
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
            data-ui-source="component"
            data-ui-state=legend_data_state
            aria-disabled=legend_aria_disabled
        >
            <span class="ui-legend__text" data-slot="legend-text">
                {text}
            </span>

            <Show when=move || state.get().is_required>
                <span class="ui-legend__required" data-slot="legend-required" aria-hidden="true">
                    {required_indicator.clone()}
                </span>
            </Show>
        </legend>
    }
}
