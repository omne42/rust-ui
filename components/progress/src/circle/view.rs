use crate::circle::{ProgressCircleMotion, logic, motion};
use leptos::prelude::*;
use ui_headless::{
    A11yDirection, ProgressbarA11yOptions, progressbar_attrs, use_controllable_state,
};

#[cfg(any())]
const _PROGRESS_CIRCLE_STATIC_TEMPLATE_MARKERS: &str = r#"const PROGRESS_CIRCLE_SVG_SLOT: &str = "progress-circle-svg"; const PROGRESS_CIRCLE_TRACK_SLOT: &str = "progress-circle-track"; const PROGRESS_CIRCLE_INDICATOR_SLOT: &str = "progress-circle-indicator"; struct ProgressCircleSvgTemplate fn build_progress_circle_svg_template( let svg_template = build_progress_circle_svg_template( data-slot=PROGRESS_CIRCLE_SVG_SLOT data-slot=PROGRESS_CIRCLE_TRACK_SLOT data-slot=PROGRESS_CIRCLE_INDICATOR_SLOT"#;

fn render_progress_circle_track(svg_template: logic::ProgressCircleSvgTemplate) -> impl IntoView {
    let center = svg_template.center_attr.clone();
    let radius = svg_template.radius_attr;
    let stroke_width = svg_template.stroke_width_attr;
    let circumference = svg_template.circumference_attr;
    view! {
        <circle
            class="ui-progress-circle__track"
            cx=center.clone()
            cy=center
            r=radius
            stroke="currentColor"
            stroke_width=stroke_width
            stroke_dasharray=circumference
            stroke_dashoffset="0"
            fill="none"
            data-slot="progress-circle-track"
        />
    }
}

fn render_progress_circle_indicator(
    svg_template: logic::ProgressCircleSvgTemplate,
    stroke_state: Signal<logic::ProgressCircleStrokeState>,
) -> impl IntoView {
    let center = svg_template.center_attr.clone();
    let radius = svg_template.radius_attr;
    let stroke_width = svg_template.stroke_width_attr;
    view! {
        <circle
            class="ui-progress-circle__indicator"
            cx=center.clone()
            cy=center
            r=radius
            stroke="currentColor"
            stroke_width=stroke_width
            stroke_dasharray=move || stroke_state.get().dasharray
            stroke_dashoffset=move || stroke_state.get().dashoffset
            stroke_linecap="round"
            fill="none"
            data-slot="progress-circle-indicator"
        />
    }
}

fn render_progress_circle_svg(
    svg_template: logic::ProgressCircleSvgTemplate,
    stroke_state: Signal<logic::ProgressCircleStrokeState>,
) -> impl IntoView {
    let svg_size = svg_template.size_attr.clone();
    let view_box = svg_template.view_box_attr.clone();
    let track_template = svg_template.clone();
    let indicator_template = svg_template;
    view! {
        <svg
            class="ui-progress-circle__svg"
            width=svg_size.clone()
            height=svg_size
            viewBox=view_box
            data-slot="progress-circle-svg"
        >
            {render_progress_circle_track(track_template)}
            {render_progress_circle_indicator(indicator_template, stroke_state)}
        </svg>
    }
}

#[component]
pub fn ProgressCircle(
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] value: Option<Signal<Option<f64>>>,
    #[prop(optional)] default_value: Option<f64>,
    #[prop(optional)] on_value_change: Option<Callback<Option<f64>>>,
    #[prop(optional, into)] min: Option<f64>,
    #[prop(optional, into)] max: Option<f64>,
    #[prop(optional)] is_indeterminate: bool,
    #[prop(optional, into)] value_label: Option<String>,
    #[prop(optional)] size_px: Option<f64>,
    #[prop(optional)] stroke_width_px: Option<f64>,
    #[prop(optional)] motion: ProgressCircleMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
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
    let svg_template = logic::build_progress_circle_svg_template(
        metrics.size_px,
        metrics.radius_px,
        metrics.stroke_width_px,
        metrics.circumference,
    );

    let state = logic::resolve_state(logic::ProgressCircleStateInput {
        has_custom_aria_label,
        has_custom_value_label,
        has_custom_size: resolved_metrics.has_custom_size,
        has_custom_stroke_width: resolved_metrics.has_custom_stroke_width,
        has_custom_motion: motion != ProgressCircleMotion::default(),
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    let range = logic::normalize_range(min, max);
    let mode = logic::normalize_mode(is_indeterminate);
    let value_axis = logic::normalize_value_axis(value, default_value, on_value_change);
    let is_value_controlled = value_axis.is_controlled;
    let has_custom_default_value = value_axis.has_custom_default_value;
    let has_custom_on_value_change = value_axis.has_custom_on_value_change;
    let value_mode_attr = value_axis.mode_attr;
    let value_source_attr = value_axis.value_source_attr;
    let default_value_source_attr = value_axis.default_value_source_attr;
    let value_change_source_attr = value_axis.value_change_source_attr;
    let value_state = use_controllable_state(
        value_axis.value,
        Some(value_axis.default_value),
        value_axis.on_value_change,
    );
    let value_signal = value_state.value;

    let clamped_value = Signal::derive(move || {
        value_signal
            .get()
            .map(|value| logic::clamp_to_range(value, range))
    });
    let normalized_progress = Signal::derive(move || {
        clamped_value
            .get()
            .map(|value| logic::normalize_progress(value, range))
    });

    let value_label_override = StoredValue::new(value_label);
    let kernel_state = Signal::derive(move || {
        logic::resolve_kernel_state(logic::ProgressCircleKernelInput {
            clamped_value: clamped_value.get(),
            normalized_progress: normalized_progress.get(),
            mode,
            value_label_override: value_label_override.get_value(),
        })
    });
    let progress_value = Signal::derive(move || kernel_state.get().progress_value);

    let animated_progress = motion::use_progress_spring(progress_value, motion);
    let stroke_state = Signal::derive(move || {
        logic::resolve_stroke_state(logic::ProgressCircleStrokeInput {
            circumference: metrics.circumference,
            is_indeterminate: kernel_state.get().is_indeterminate,
            animated_progress: animated_progress.get(),
        })
    });

    let aria_label_text = StoredValue::new(aria_label);
    let locale_lang = StoredValue::new(lang);
    let locale_dir = StoredValue::new(dir);
    let a11y_contract = Signal::derive(move || {
        let kernel_state = kernel_state.get();
        progressbar_attrs(ProgressbarA11yOptions {
            aria_label: aria_label_text.get_value(),
            aria_valuemin: range.min,
            aria_valuemax: range.max,
            aria_valuenow: kernel_state.aria_value_now,
            aria_valuetext: kernel_state.value_label_text,
            is_indeterminate: kernel_state.is_indeterminate,
            lang: locale_lang.get_value(),
            dir: locale_dir.get_value(),
        })
    });

    view! {
        <span
            class=class
            class:ui-progress-circle--state-indeterminate=move || {
                kernel_state.get().phase == logic::ProgressCirclePhase::Indeterminate
            }
            class:ui-progress-circle--state-determinate=move || {
                kernel_state.get().phase == logic::ProgressCirclePhase::Determinate
            }
            data-slot="progress-circle"
            data-state=move || a11y_contract.get().attrs.data_state
            data-phase-class=move || kernel_state.get().phase.class_name()
            data-status-mode=move || kernel_state.get().mode.as_str()
            data-indeterminate=move || a11y_contract.get().attrs.data_indeterminate
            data-determinate=move || a11y_contract.get().attrs.data_determinate
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
            data-value-mode=value_mode_attr
            data-value-source=value_source_attr
            data-default-value-source=default_value_source_attr
            data-value-change-source=value_change_source_attr
            data-value-controlled=is_value_controlled.then_some("true")
            data-value-uncontrolled=(!is_value_controlled).then_some("true")
            data-custom-default-value=has_custom_default_value.then_some("true")
            data-custom-value-change=has_custom_on_value_change.then_some("true")
            role=move || a11y_contract.get().attrs.role
            aria-label=move || a11y_contract.get().attrs.aria_label
            aria-valuemin=move || a11y_contract.get().attrs.aria_valuemin
            aria-valuemax=move || a11y_contract.get().attrs.aria_valuemax
            aria-valuenow=move || a11y_contract.get().attrs.aria_valuenow
            aria-valuetext=move || a11y_contract.get().attrs.aria_valuetext
            lang=move || a11y_contract.get().attrs.lang
            dir=move || a11y_contract.get().attrs.dir
        >
            {
                render_progress_circle_svg(
                    svg_template.clone(),
                    stroke_state,
                )
            }
        </span>
    }
}
