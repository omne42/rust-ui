use crate::circular_progress::logic::{self, CircularProgressStateInput};
use leptos::prelude::*;

#[component]
pub fn CircularProgress(
    #[prop(optional, into, default = logic::DEFAULT_ARIA_LABEL.to_string())] aria_label: String,
    #[prop(optional)] size_px: Option<f64>,
    #[prop(optional)] thickness_px: Option<f64>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label);

    let state = logic::resolve_state(CircularProgressStateInput {
        size_px,
        thickness_px,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, &state);

    view! {
        <span
            class=class
            style=state.style_vars
            data-slot="circular-progress"
            data-state="indeterminate"
            data-motion="spin"
            data-size=state.has_custom_size.then_some("custom")
            data-thickness=state.has_custom_thickness.then_some("custom")
            data-size-source=state.size_source_attr
            data-thickness-source=state.thickness_source_attr
            data-label-source=state.label_source_attr
            data-custom-size=state.has_custom_size.then_some("true")
            data-custom-thickness=state.has_custom_thickness.then_some("true")
            data-custom-aria-label=state.has_custom_aria_label.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            role="progressbar"
            aria-label=aria_label
            aria-valuemin="0"
            aria-valuemax="100"
        ></span>
    }
}
