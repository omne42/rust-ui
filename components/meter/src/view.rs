use crate::{MeterMotion, MeterSize, MeterVariant, logic, motion, protocol};
use leptos::prelude::*;
use ui_headless::{A11yDirection, i18n, locale_attrs};

fn render_meter_header(
    label: StoredValue<Option<String>>,
    label_id: StoredValue<String>,
    render_state: Signal<logic::MeterRenderState>,
) -> impl IntoView {
    view! {
        <Show
            when=move || {
                label.get_value().is_some() || render_state.get().value_label_text.is_some()
            }
        >
            <div class="ui-meter__header" data-slot="meter-header">
                {label.get_value().map(|label| view! {
                    <div class="ui-meter__label" data-slot="meter-label" id=label_id.get_value()>
                        {label}
                    </div>
                })}
                {move || render_state.get().value_label_text.map(|value_label| view! {
                    <div class="ui-meter__value-label" data-slot="meter-value-label">
                        {value_label}
                    </div>
                })}
            </div>
        </Show>
    }
}

fn render_meter_track(indicator_ref: NodeRef<leptos::html::Div>) -> impl IntoView {
    view! {
        <div class="ui-meter__track" data-slot="meter-track">
            <div
                class="ui-meter__indicator"
                node_ref=indicator_ref
                data-slot="meter-indicator"
                aria-hidden="true"
            ></div>
        </div>
    }
}

#[component]
pub fn Meter(
    id: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] value: Signal<Option<f64>>,
    #[prop(optional, into)] min: Option<f64>,
    #[prop(optional, into)] max: Option<f64>,
    #[prop(optional)] size: MeterSize,
    #[prop(optional)] variant: MeterVariant,
    #[prop(optional)] motion: MeterMotion,
    #[prop(optional, into)] is_value_label_visible: Option<bool>,
    // Backward-compatible alias. `is_value_label_visible` has precedence.
    #[prop(optional, into)] show_value_label: Option<bool>,
    #[prop(optional, into)] value_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<logic::MeterStrings>();
    let locale = locale_attrs(lang, dir);
    let motion = crate::motion::sanitize_motion(motion);
    let normalized = logic::normalize_inputs(logic::MeterInputNormalizationInput {
        label,
        aria_label,
        default_aria_label: Some(strings.aria_label.clone()),
        min,
        max,
        is_value_label_visible,
        show_value_label,
        value_label,
        class_name,
    });
    let logic::MeterInputNormalization {
        label,
        aria_label,
        has_custom_aria_label,
        range,
        is_value_label_visible,
        value_label,
        has_custom_value_label,
        class_name,
        has_custom_class_name,
    } = normalized;

    let state = logic::resolve_state(logic::MeterStateInput {
        variant,
        size,
        has_custom_aria_label,
        has_custom_value_label,
        has_custom_motion: motion != MeterMotion::default(),
        has_custom_class_name,
    });
    let class = logic::compose_class_name(class_name, state);

    let label_id = StoredValue::new(format!("{id}-label"));
    let label = StoredValue::new(label);
    let value_label_override = StoredValue::new(value_label);
    let render_state = Signal::derive(move || {
        logic::derive_render_state(logic::MeterRenderStateInput {
            value: value.get(),
            range,
            is_value_label_visible,
            value_label: value_label_override.get_value(),
        })
    });
    let progress_value =
        Signal::derive(move || render_state.get().normalized_progress.unwrap_or(0.0));
    let indicator_ref = NodeRef::new();
    motion::attach_motion(indicator_ref, progress_value, motion);
    let agent_attrs =
        Signal::derive(move || protocol::agent_data_attrs(state, render_state.get().phase));

    let aria_labelledby = label.get_value().map(|_| label_id.get_value());
    let aria_label = aria_labelledby.is_none().then(|| aria_label.to_string());

    view! {
        <div
            class=class
            class:ui-meter--indeterminate=move || {
                render_state.get().phase == logic::MeterPhase::Indeterminate
            }
            class:ui-meter--state-indeterminate=move || {
                render_state.get().phase == logic::MeterPhase::Indeterminate
            }
            class:ui-meter--state-determinate=move || {
                render_state.get().phase == logic::MeterPhase::Determinate
            }
            data-slot="meter"
            data-variant=state.variant_attr
            data-size=state.size_attr
            data-state=move || render_state.get().phase.as_str()
            data-phase-class=move || render_state.get().phase.class_name()
            data-indeterminate=move || {
                (render_state.get().phase == logic::MeterPhase::Indeterminate).then_some("true")
            }
            data-determinate=move || {
                (render_state.get().phase == logic::MeterPhase::Determinate).then_some("true")
            }
            data-label-source=state.label_source_attr
            data-value-label-source=state.value_label_source_attr
            data-motion-source=state.motion_source_attr
            data-custom-aria-label=state.has_custom_aria_label.then_some("true")
            data-custom-value-label=state.has_custom_value_label.then_some("true")
            data-custom-motion=state.has_custom_motion.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            data-ui-schema=move || agent_attrs.get().schema
            data-ui-intent=move || agent_attrs.get().intent
            data-ui-action=move || agent_attrs.get().action
            data-ui-stream-mode=move || agent_attrs.get().stream_mode
            data-ui-output-mode=move || agent_attrs.get().output_mode
            data-ui-output-status=move || agent_attrs.get().output_status
            data-ui-state-phase=move || agent_attrs.get().state_phase
            data-ui-state-variant=move || agent_attrs.get().state_variant
            data-ui-state-size=move || agent_attrs.get().state_size
            data-ui-source-label=move || agent_attrs.get().source_label
            data-ui-source-value-label=move || agent_attrs.get().source_value_label
            data-ui-source-motion=move || agent_attrs.get().source_motion
            data-ui-source-class=move || agent_attrs.get().source_class
            role="meter"
            aria-label=aria_label
            aria-labelledby=aria_labelledby
            aria-valuemin=range.min.to_string()
            aria-valuemax=range.max.to_string()
            aria-valuenow=move || render_state.get().aria_value_now
            aria-valuetext=move || render_state.get().value_label_text
            lang=locale.lang
            dir=locale.dir
        >
            {render_meter_header(label, label_id, render_state)}
            {render_meter_track(indicator_ref)}
        </div>
    }
}
