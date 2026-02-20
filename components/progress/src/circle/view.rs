use crate::circle::{ProgressCircleMotion, ProgressCircleRange, logic, motion};
use leptos::prelude::*;

#[component]
pub fn ProgressCircle(
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] value: Signal<Option<f64>>,
    #[prop(optional, default = 0.0)] min: f64,
    #[prop(optional, default = 100.0)] max: f64,
    #[prop(optional)] indeterminate: bool,
    #[prop(optional, into)] value_label: Option<String>,
    #[prop(optional)] size_px: Option<f64>,
    #[prop(optional)] stroke_width_px: Option<f64>,
    #[prop(optional)] motion: ProgressCircleMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let motion = crate::circle::motion::sanitize_motion(motion);
    let class_name = logic::normalize_optional_text(class_name);
    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label);
    let (value_label, has_custom_value_label) = logic::resolve_value_label(value_label);

    let resolved_metrics = logic::resolve_metrics(logic::ProgressCircleMetricsInput {
        size_px,
        stroke_width_px,
    });
    let metrics = resolved_metrics.metrics;

    let state = logic::resolve_state(logic::ProgressCircleStateInput {
        has_custom_aria_label,
        has_custom_value_label,
        has_custom_size: resolved_metrics.has_custom_size,
        has_custom_stroke_width: resolved_metrics.has_custom_stroke_width,
        has_custom_motion: motion != ProgressCircleMotion::default(),
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    let range = ProgressCircleRange::sanitized(min, max);
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

    let animated_progress = motion::use_progress_spring(progress_value, motion);
    let dashoffset = Signal::derive(move || {
        let progress = if is_indeterminate.get() {
            0.25
        } else {
            animated_progress.get()
        };
        (metrics.circumference * (1.0 - progress)).to_string()
    });

    let dasharray = Signal::derive(move || {
        if is_indeterminate.get() {
            (metrics.circumference * 0.25).to_string()
        } else {
            metrics.circumference.to_string()
        }
    });

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
        <span
            class=class
            class:ui-progress-circle--state-indeterminate=move || {
                logic::resolve_phase(is_indeterminate.get()) == logic::ProgressCirclePhase::Indeterminate
            }
            class:ui-progress-circle--state-determinate=move || {
                logic::resolve_phase(is_indeterminate.get()) == logic::ProgressCirclePhase::Determinate
            }
            data-slot="progress-circle"
            data-state=move || logic::resolve_phase(is_indeterminate.get()).as_str()
            data-phase-class=move || logic::resolve_phase(is_indeterminate.get()).class_name()
            data-indeterminate=move || is_indeterminate.get().then_some("true")
            data-determinate=move || (!is_indeterminate.get()).then_some("true")
            data-size=state.has_custom_size.then_some("custom")
            data-stroke=state.has_custom_stroke_width.then_some("custom")
            data-size-source=state.size_source_attr
            data-stroke-source=state.stroke_source_attr
            data-label-source=state.label_source_attr
            data-value-label-source=state.value_label_source_attr
            data-motion-source=state.motion_source_attr
            data-custom-size=state.has_custom_size.then_some("true")
            data-custom-stroke=state.has_custom_stroke_width.then_some("true")
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
            <svg
                class="ui-progress-circle__svg"
                width=metrics.size_px.to_string()
                height=metrics.size_px.to_string()
                viewBox=format!("0 0 {} {}", metrics.size_px, metrics.size_px)
                data-slot="progress-circle-svg"
            >
                <circle
                    class="ui-progress-circle__track"
                    cx=(metrics.size_px / 2.0).to_string()
                    cy=(metrics.size_px / 2.0).to_string()
                    r=metrics.radius_px.to_string()
                    stroke="currentColor"
                    stroke_width=metrics.stroke_width_px.to_string()
                    stroke_dasharray=metrics.circumference.to_string()
                    stroke_dashoffset="0"
                    fill="none"
                    data-slot="progress-circle-track"
                />
                <circle
                    class="ui-progress-circle__indicator"
                    cx=(metrics.size_px / 2.0).to_string()
                    cy=(metrics.size_px / 2.0).to_string()
                    r=metrics.radius_px.to_string()
                    stroke="currentColor"
                    stroke_width=metrics.stroke_width_px.to_string()
                    stroke_dasharray=move || dasharray.get()
                    stroke_dashoffset=move || dashoffset.get()
                    stroke_linecap="round"
                    fill="none"
                    data-slot="progress-circle-indicator"
                />
            </svg>
        </span>
    }
}
