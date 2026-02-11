use crate::meter::{MeterMotion, MeterSize, MeterVariant, logic, motion};
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
    #[prop(optional)] motion: MeterMotion,
    #[prop(optional, default = true)] show_value_label: bool,
    #[prop(optional, into)] value_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let motion = crate::meter::motion::sanitize_motion(motion);
    let class_name = logic::normalize_optional_text(class_name);
    let label = logic::normalize_optional_text(label);
    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label, label.clone());
    let (value_label, has_custom_value_label) = logic::resolve_value_label(value_label);

    let state = logic::resolve_state(logic::MeterStateInput {
        variant,
        size,
        has_custom_aria_label,
        has_custom_value_label,
        has_custom_motion: motion != MeterMotion::default(),
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    let range = logic::MeterRange::sanitized(min, max);

    let clamped_value =
        Signal::derive(move || value.get().map(|value| logic::clamp_to_range(value, range)));
    let normalized_progress = Signal::derive(move || {
        clamped_value
            .get()
            .map(|value| logic::normalize_progress(value, range))
    });

    let is_indeterminate = Signal::derive(move || normalized_progress.get().is_none());
    let phase = Signal::derive(move || logic::resolve_phase(is_indeterminate.get()));

    let progress_value = Signal::derive(move || normalized_progress.get().unwrap_or(0.0));
    let indicator_ref = NodeRef::new();
    motion::attach_motion(indicator_ref, progress_value, motion);

    let label_id = StoredValue::new(format!("{id}-label"));
    let label = StoredValue::new(label);
    let value_label_override = StoredValue::new(value_label);

    let value_label_text = Signal::derive(move || {
        if !show_value_label {
            return None;
        }
        if let Some(value_label) = value_label_override.get_value() {
            return Some(value_label);
        }
        let progress = normalized_progress.get()?;
        Some(format!("{:.0}%", progress * 100.0))
    });

    let aria_labelledby = label.get_value().map(|_| label_id.get_value());
    let aria_label = aria_labelledby.is_none().then_some(aria_label.clone());

    let aria_value_now = Signal::derive(move || {
        if is_indeterminate.get() {
            None
        } else {
            clamped_value.get().map(|value: f64| value.to_string())
        }
    });

    view! {
        <div
            class=class
            class:ui-meter--indeterminate=move || phase.get() == logic::MeterPhase::Indeterminate
            class:ui-meter--state-indeterminate=move || {
                phase.get() == logic::MeterPhase::Indeterminate
            }
            class:ui-meter--state-determinate=move || {
                phase.get() == logic::MeterPhase::Determinate
            }
            data-slot="meter"
            data-variant=state.variant_attr
            data-size=state.size_attr
            data-state=move || phase.get().as_str()
            data-phase-class=move || phase.get().class_name()
            data-indeterminate=move || {
                (phase.get() == logic::MeterPhase::Indeterminate).then_some("true")
            }
            data-determinate=move || {
                (phase.get() == logic::MeterPhase::Determinate).then_some("true")
            }
            data-label-source=state.label_source_attr
            data-value-label-source=state.value_label_source_attr
            data-motion-source=state.motion_source_attr
            data-custom-aria-label=state.has_custom_aria_label.then_some("true")
            data-custom-value-label=state.has_custom_value_label.then_some("true")
            data-custom-motion=state.has_custom_motion.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            role="meter"
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
