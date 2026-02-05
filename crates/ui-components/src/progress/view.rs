use crate::progress::{ProgressMotion, ProgressRange, logic, motion};
use leptos::prelude::*;

#[component]
pub fn Progress(
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] value: Signal<Option<f64>>,
    #[prop(optional, default = 0.0)] min: f64,
    #[prop(optional, default = 100.0)] max: f64,
    #[prop(optional)] indeterminate: bool,
    #[prop(optional, into)] value_label: Option<String>,
    #[prop(optional)] motion: ProgressMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let range = ProgressRange::sanitized(min, max);

    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Progress".to_string());

    let clamped_value =
        Signal::derive(move || value.get().map(|value| logic::clamp_to_range(value, range)));
    let normalized_progress = Signal::derive(move || {
        clamped_value
            .get()
            .map(|value| logic::normalize_progress(value, range))
    });

    let is_indeterminate =
        Signal::derive(move || indeterminate || normalized_progress.get().is_none());
    let progress_value = Signal::derive(move || normalized_progress.get().unwrap_or(0.0));

    let indicator_ref = NodeRef::new();
    motion::attach_motion(indicator_ref, progress_value, motion);

    let value_label_override = StoredValue::new(value_label);
    let value_label_text = Signal::derive(move || {
        if is_indeterminate.get() {
            return None;
        }
        if let Some(value_label) = value_label_override
            .get_value()
            .filter(|value_label| !value_label.trim().is_empty())
        {
            return Some(value_label);
        }
        let progress = normalized_progress.get()?;
        Some(format!("{:.0}%", progress * 100.0))
    });

    let base_class = "ui-progress".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let aria_value_now =
        Signal::derive(move || clamped_value.get().map(|value: f64| value.to_string()));

    view! {
        <div
            class=class
            class:ui-progress--indeterminate=move || is_indeterminate.get()
            data-slot="progress"
            role="progressbar"
            aria-label=aria_label
            aria-valuemin=range.min.to_string()
            aria-valuemax=range.max.to_string()
            aria-valuenow=move || if is_indeterminate.get() { None } else { aria_value_now.get() }
            aria-valuetext=move || value_label_text.get()
        >
            <div class="ui-progress__track" data-slot="progress-track">
                <div
                    class="ui-progress__indicator"
                    node_ref=indicator_ref
                    data-slot="progress-indicator"
                    aria-hidden="true"
                ></div>
            </div>
        </div>
    }
}
