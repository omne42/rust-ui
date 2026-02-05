use crate::meter::{
    MeterRange, MeterSize, MeterVariant, clamp_to_range, motion, normalize_progress,
};
use leptos::prelude::*;

#[component]
pub fn Meter(
    id: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] value: Signal<Option<f64>>,
    #[prop(optional, default = 0.0)] min: f64,
    #[prop(optional, default = 100.0)] max: f64,
    #[prop(optional)] size: MeterSize,
    #[prop(optional)] variant: MeterVariant,
    #[prop(optional)] motion: crate::meter::MeterMotion,
    #[prop(optional, default = true)] show_value_label: bool,
    #[prop(optional, into)] value_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let range = MeterRange::sanitized(min, max);

    let label_id = format!("{id}-label");
    let label_id = StoredValue::new(label_id);
    let label = StoredValue::new(label);

    let resolved_aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .or_else(|| label.get_value())
        .unwrap_or_else(|| "Meter".to_string());

    let clamped_value =
        Signal::derive(move || value.get().map(|value| clamp_to_range(value, range)));
    let normalized_progress = Signal::derive(move || {
        clamped_value
            .get()
            .map(|value| normalize_progress(value, range))
    });

    let is_indeterminate = Signal::derive(move || normalized_progress.get().is_none());
    let progress_value = Signal::derive(move || normalized_progress.get().unwrap_or(0.0));

    let indicator_ref = NodeRef::new();
    motion::attach_motion(indicator_ref, progress_value, motion);

    let value_label_override = StoredValue::new(value_label);
    let value_label_text = Signal::derive(move || {
        if !show_value_label {
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

    let base_class = format!("ui-meter {} {}", variant.class_name(), size.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let aria_labelledby = label.get_value().map(|_| label_id.get_value());
    let aria_label = aria_labelledby
        .is_none()
        .then_some(resolved_aria_label.clone());

    let aria_value_now = Signal::derive(move || clamped_value.get().map(|value| value.to_string()));

    view! {
        <div
            class=class
            class:ui-meter--indeterminate=move || is_indeterminate.get()
            data-slot="meter"
            role="meter progressbar"
            aria-label=aria_label
            aria-labelledby=aria_labelledby
            aria-valuemin=range.min.to_string()
            aria-valuemax=range.max.to_string()
            aria-valuenow=move || aria_value_now.get()
            aria-valuetext=move || value_label_text.get()
        >
            <Show when=move || label.get_value().is_some() || value_label_text.get().is_some()>
                <div class="ui-meter__header" data-slot="meter-header">
                    {label.get_value().map(|label| view! {
                        <div
                            class="ui-meter__label"
                            data-slot="meter-label"
                            id=label_id.get_value()
                        >
                            {label}
                        </div>
                    })}
                    {move || value_label_text.get().map(|value_label| view! {
                        <div class="ui-meter__value-label" data-slot="meter-value-label">
                            {value_label}
                        </div>
                    })}
                </div>
            </Show>

            <div class="ui-meter__track" data-slot="meter-track">
                <div
                    class="ui-meter__indicator"
                    node_ref=indicator_ref
                    data-slot="meter-indicator"
                    aria-hidden="true"
                ></div>
            </div>
        </div>
    }
}
