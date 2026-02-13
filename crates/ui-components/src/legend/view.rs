use crate::legend::{
    LegendMotion, LegendStateInput,
    logic::{self, LegendTone},
    motion,
};
use leptos::prelude::*;

#[component]
pub fn Legend(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional)] tone: LegendTone,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: LegendMotion,
    #[prop(optional, into)] required_indicator: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (text, has_custom_text) = logic::normalize_text(text);
    let (required_indicator, has_custom_indicator) =
        logic::normalize_required_indicator(required_indicator);

    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != LegendMotion::default();

    let state = Signal::derive(move || {
        logic::resolve_state(LegendStateInput {
            tone,
            required,
            disabled,
            has_custom_text,
            has_custom_indicator,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <legend
            class=move || class.get()
            data-slot="legend"
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-tone=move || state.get().tone_attr
            data-state=move || if state.get().is_required { "required" } else { "optional" }
            data-required=move || state.get().is_required.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-text-source=move || state.get().text_source_attr
            data-indicator-source=move || state.get().indicator_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            aria-disabled=move || state.get().is_disabled.then_some("true")
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
