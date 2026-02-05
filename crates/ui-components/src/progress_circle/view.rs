use crate::progress_circle::{ProgressCircleMotion, ProgressCircleRange, logic, motion};
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
    let range = ProgressCircleRange::sanitized(min, max);
    let size_px = size_px.unwrap_or(24.0);
    let stroke_width_px = stroke_width_px.unwrap_or(3.0);
    let metrics = logic::resolve_metrics(size_px, stroke_width_px);

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

    let base_class = "ui-progress-circle".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let svg_class = "ui-progress-circle__svg".to_string();

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

    view! {
        <span
            class=class
            class:ui-progress-circle--indeterminate=move || is_indeterminate.get()
            data-slot="progress-circle"
            role="progressbar"
            aria-label=aria_label
            aria-valuemin=range.min.to_string()
            aria-valuemax=range.max.to_string()
            aria-valuenow=move || {
                if is_indeterminate.get() {
                    None
                } else {
                    clamped_value.get().map(|v: f64| v.to_string())
                }
            }
            aria-valuetext=move || value_label_text.get()
        >
            <svg
                class=svg_class
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
