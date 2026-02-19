use crate::progress_bar::{ProgressBarSize, ProgressBarVariant, logic};
use leptos::prelude::*;

#[component]
pub fn ProgressBar(
    #[prop(optional)] variant: ProgressBarVariant,
    #[prop(optional)] size: ProgressBarSize,
    #[prop(optional)] value: Option<f64>,
    #[prop(optional, default = logic::DEFAULT_MAX)] max: f64,
    #[prop(optional)] indeterminate: bool,
    #[prop(optional, into, default = logic::DEFAULT_ARIA_LABEL.into())] aria_label: String,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label);

    let state = logic::resolve_state(logic::ProgressBarStateInput {
        variant,
        size,
        value,
        max,
        indeterminate,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);

    view! {
        <progress
            class=class
            data-slot="progress-bar"
            data-variant=state.variant_attr
            data-size=state.size_attr
            data-state=state.phase_attr
            data-indeterminate=state.is_indeterminate.then_some("true")
            data-determinate=state.is_determinate.then_some("true")
            data-has-value=state.has_value.then_some("true")
            data-label-source=state.label_source_attr
            data-custom-aria-label=state.has_custom_aria_label.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            aria-label=aria_label
            max=state.max.to_string()
            value=state.value.map(|value| value.to_string())
        ></progress>
    }
}
