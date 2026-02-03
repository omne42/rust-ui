use crate::progress_bar::{ProgressBarSize, ProgressBarVariant};
use leptos::prelude::*;

#[component]
pub fn ProgressBar(
    #[prop(optional)] variant: ProgressBarVariant,
    #[prop(optional)] size: ProgressBarSize,
    #[prop(optional)] value: Option<f64>,
    #[prop(optional, default = 100.0)] max: f64,
    #[prop(optional)] indeterminate: bool,
    #[prop(optional, into, default = "Progress".to_string())] aria_label: String,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let max = if max.is_finite() && max > 0.0 {
        max
    } else {
        1.0
    };
    let is_indeterminate = indeterminate || value.is_none();
    let value = (!is_indeterminate)
        .then_some(value.unwrap_or_default())
        .filter(|value| value.is_finite())
        .map(|value| value.clamp(0.0, max));

    let value_attr = value.map(|value| value.to_string());
    let max_attr = max.to_string();

    let mut base_class = format!(
        "ui-progress-bar {} {}",
        variant.class_name(),
        size.class_name()
    );
    if is_indeterminate {
        base_class.push_str(" ui-progress-bar--indeterminate");
    }

    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <progress
            class=class
            data-slot="progress-bar"
            aria-label=aria_label
            max=max_attr
            value=value_attr
        ></progress>
    }
}
