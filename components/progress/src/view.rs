use crate::{ProgressMotion, logic, motion};
use leptos::prelude::*;
use ui_headless::{
    A11yDirection, ProgressbarA11yOptions, progressbar_attrs, use_controllable_state,
};

#[component]
pub fn Progress(
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] value: Option<Signal<Option<f64>>>,
    #[prop(optional)] default_value: Option<f64>,
    #[prop(optional)] on_value_change: Option<Callback<Option<f64>>>,
    #[prop(optional, into)] min: Option<f64>,
    #[prop(optional, into)] max: Option<f64>,
    #[prop(optional)] is_indeterminate: bool,
    #[prop(optional, into)] value_label: Option<String>,
    #[prop(optional)] motion: ProgressMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let motion = crate::motion::sanitize_motion(motion);
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
    let render_state = Signal::derive(move || {
        logic::resolve_render_state(logic::ProgressRenderInput {
            clamped_value: clamped_value.get(),
            normalized_progress: normalized_progress.get(),
            mode,
            value_label_override: value_label_override.get_value(),
        })
    });
    let progress_value = Signal::derive(move || render_state.get().progress_value);
    let indicator_ref = NodeRef::new();
    motion::attach_motion(indicator_ref, progress_value, motion);

    let aria_label_text = StoredValue::new(aria_label);
    let locale_lang = StoredValue::new(lang);
    let locale_dir = StoredValue::new(dir);
    let a11y_contract = Signal::derive(move || {
        let render_state = render_state.get();
        progressbar_attrs(ProgressbarA11yOptions {
            aria_label: aria_label_text.get_value(),
            aria_valuemin: range.min,
            aria_valuemax: range.max,
            aria_valuenow: render_state.aria_value_now,
            aria_valuetext: render_state.value_label_text,
            is_indeterminate: render_state.is_indeterminate,
            lang: locale_lang.get_value(),
            dir: locale_dir.get_value(),
        })
    });

    view! {
        <div
            class=class
            class:ui-progress--indeterminate=move || {
                render_state.get().phase == logic::ProgressPhase::Indeterminate
            }
            class:ui-progress--state-indeterminate=move || {
                render_state.get().phase == logic::ProgressPhase::Indeterminate
            }
            class:ui-progress--state-determinate=move || {
                render_state.get().phase == logic::ProgressPhase::Determinate
            }
            data-slot="progress"
            data-state=move || a11y_contract.get().attrs.data_state
            data-phase-class=move || render_state.get().phase.class_name()
            data-status-mode=move || render_state.get().mode.as_str()
            data-indeterminate=move || a11y_contract.get().attrs.data_indeterminate
            data-determinate=move || a11y_contract.get().attrs.data_determinate
            data-label-source=state.label_source_attr
            data-value-label-source=state.value_label_source_attr
            data-motion-source=state.motion_source_attr
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
