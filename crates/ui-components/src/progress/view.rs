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
    let motion = crate::progress::motion::sanitize_motion(motion);
    let class_name = logic::normalize_optional_text(class_name);
    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label);
    let (value_label, has_custom_value_label) = logic::resolve_value_label(value_label);

    let state = logic::resolve_state(logic::ProgressStateInput {
        has_custom_aria_label,
        has_custom_value_label,
        has_custom_motion: motion != ProgressMotion::default(),
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    let range = ProgressRange::sanitized(min, max);

    let clamped_value =
        Signal::derive(move || value.get().map(|value| logic::clamp_to_range(value, range)));
    let normalized_progress = Signal::derive(move || {
        clamped_value
            .get()
            .map(|value| logic::normalize_progress(value, range))
    });

    let is_indeterminate =
        Signal::derive(move || indeterminate || normalized_progress.get().is_none());
    let phase = Signal::derive(move || logic::resolve_phase(is_indeterminate.get()));

    let progress_value = Signal::derive(move || normalized_progress.get().unwrap_or(0.0));
    let indicator_ref = NodeRef::new();
    motion::attach_motion(indicator_ref, progress_value, motion);

    let value_label_override = StoredValue::new(value_label);
    let value_label_text = Signal::derive(move || {
        if is_indeterminate.get() {
            return None;
        }
        if let Some(value_label) = value_label_override.get_value() {
            return Some(value_label);
        }
        let progress = normalized_progress.get()?;
        Some(format!("{:.0}%", progress * 100.0))
    });

    let aria_value_now = Signal::derive(move || clamped_value.get().map(|value: f64| value));

    view! {
        <div
            class=class
            class:ui-progress--indeterminate=move || {
                phase.get() == logic::ProgressPhase::Indeterminate
            }
            class:ui-progress--state-indeterminate=move || {
                phase.get() == logic::ProgressPhase::Indeterminate
            }
            class:ui-progress--state-determinate=move || {
                phase.get() == logic::ProgressPhase::Determinate
            }
            data-slot="progress"
            data-state=move || phase.get().as_str()
            data-phase-class=move || phase.get().class_name()
            data-indeterminate=move || {
                (phase.get() == logic::ProgressPhase::Indeterminate).then_some("true")
            }
            data-determinate=move || {
                (phase.get() == logic::ProgressPhase::Determinate).then_some("true")
            }
            data-label-source=state.label_source_attr
            data-value-label-source=state.value_label_source_attr
            data-motion-source=state.motion_source_attr
            data-custom-aria-label=state.has_custom_aria_label.then_some("true")
            data-custom-value-label=state.has_custom_value_label.then_some("true")
            data-custom-motion=state.has_custom_motion.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
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
