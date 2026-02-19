use crate::slider::{
    logic::{self, SliderStateInput},
    motion::{self, SliderMotion},
};
use leptos::{ev, html, prelude::*};
use ui_headless::{A11yDirection, SliderAria, SliderOptions, use_slider};

const SLOT_SLIDER: &str = "slider";
const SLOT_SLIDER_LABEL: &str = "slider-label";
const SLOT_SLIDER_CONTROL: &str = "slider-control";
const SLOT_SLIDER_INPUT: &str = "slider-input";
const SLOT_SLIDER_TRACK: &str = "slider-track";
const SLOT_SLIDER_FILL: &str = "slider-fill";
const SLOT_SLIDER_THUMB: &str = "slider-thumb";
const CLASS_SLIDER_LABEL: &str = "ui-slider__label";
const CLASS_SLIDER_CONTROL: &str = "ui-slider__control";
const CLASS_SLIDER_INPUT: &str = "ui-slider__input";
const CLASS_SLIDER_TRACK: &str = "ui-slider__track";
const CLASS_SLIDER_FILL: &str = "ui-slider__fill";
const CLASS_SLIDER_THUMB: &str = "ui-slider__thumb";
const BOOL_TRUE: &str = "true";
const INPUT_TYPE_RANGE: &str = "range";

fn render_label(id: String, label: StoredValue<String>) -> impl IntoView {
    view! {
        <label class=CLASS_SLIDER_LABEL for=id data-slot=SLOT_SLIDER_LABEL>
            {move || label.with_value(|label| label.clone())}
        </label>
    }
}

struct SliderInputRenderInput {
    id: String,
    label: StoredValue<String>,
    is_disabled: bool,
    slider_aria: SliderAria,
}

fn render_input(input: SliderInputRenderInput) -> impl IntoView {
    let SliderInputRenderInput {
        id,
        label,
        is_disabled,
        slider_aria,
    } = input;

    view! {
        <input
            id=id
            class=CLASS_SLIDER_INPUT
            data-slot=SLOT_SLIDER_INPUT
            type=INPUT_TYPE_RANGE
            min=move || slider_aria.state.min
            max=move || slider_aria.state.max
            step=move || slider_aria.state.step
            prop:value=move || slider_aria.state.value.get()
            disabled=is_disabled
            role=slider_aria.input.role
            aria-label=move || label.with_value(|label| label.clone())
            aria-disabled=slider_aria.input.aria_disabled
            aria-valuemin=move || slider_aria.input.aria_valuemin.get()
            aria-valuemax=move || slider_aria.input.aria_valuemax.get()
            aria-valuenow=move || slider_aria.input.aria_valuenow.get()
            aria-valuetext=move || slider_aria.input.aria_valuetext.get()
            lang=move || slider_aria.input.lang.clone()
            dir=move || slider_aria.input.dir
            on:input=move |ev: ev::Event| {
                slider_aria.handlers.on_input.run(event_target_value(&ev));
            }
            on:pointerdown=move |_| slider_aria.handlers.on_pointer_down.run(())
            on:pointerup=move |_| slider_aria.handlers.on_pointer_up.run(())
            on:pointercancel=move |_| slider_aria.handlers.on_pointer_cancel.run(())
            on:pointerenter=move |_| slider_aria.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| slider_aria.handlers.on_pointer_leave.run(())
            on:focus=move |_| slider_aria.handlers.on_focus.run(())
            on:blur=move |_| slider_aria.handlers.on_blur.run(())
        />
    }
}

fn render_track() -> impl IntoView {
    view! {
        <div class=CLASS_SLIDER_TRACK data-slot=SLOT_SLIDER_TRACK aria-hidden=BOOL_TRUE>
            <div class=CLASS_SLIDER_FILL data-slot=SLOT_SLIDER_FILL></div>
            <div class=CLASS_SLIDER_THUMB data-slot=SLOT_SLIDER_THUMB></div>
        </div>
    }
}

fn render_control(input: SliderInputRenderInput) -> impl IntoView {
    let input_view = render_input(input);
    let track_view = render_track();

    view! {
        <div class=CLASS_SLIDER_CONTROL data-slot=SLOT_SLIDER_CONTROL>
            {input_view}
            {track_view}
        </div>
    }
}

#[component]
pub fn Slider(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, default = logic::DEFAULT_LABEL.into(), into)] label: String,
    #[prop(optional)] value: Option<Signal<f64>>,
    #[prop(optional)] default_value: Option<f64>,
    #[prop(optional)] on_value_change: Option<Callback<f64>>,
    #[prop(optional)] set_value: Option<WriteSignal<f64>>,
    #[prop(optional)] on_change: Option<Callback<f64>>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, default = logic::DEFAULT_MIN)] min: f64,
    #[prop(optional, default = logic::DEFAULT_MAX)] max: f64,
    #[prop(optional, default = logic::DEFAULT_STEP)] step: f64,
    #[prop(optional)] motion: SliderMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let id_state = logic::normalize_id(id.unwrap_or_default());
    let id = id_state.id;
    let id_source_attr = id_state.id_source_attr;
    let has_custom_id = id_state.has_custom_id;
    let (label, has_custom_label) = logic::resolve_label(label);
    let label = StoredValue::new(label);

    let value_axis = logic::normalize_value_axis(logic::ValueAxisInput {
        value,
        default_value,
        on_value_change,
        set_value,
        on_change,
    });
    let normalized_value = value_axis.value;
    let normalized_default_value = value_axis.default_value;
    let normalized_on_value_change = value_axis.on_value_change;
    let has_value_change_handler = normalized_on_value_change.is_some();
    let control_mode_attr = value_axis.control_mode_attr;
    let value_source_attr = value_axis.value_source_attr;
    let default_value_source_attr = value_axis.default_value_source_attr;
    let value_change_source_attr = value_axis.value_change_source_attr;
    let agent_contract = logic::resolve_agent_contract(has_value_change_handler);

    let accessibility_state =
        logic::normalize_accessibility_state(logic::AccessibilityStateInput {
            is_disabled,
            disabled,
        });
    let is_disabled = accessibility_state.is_disabled;
    let disabled_source_attr = accessibility_state.disabled_source_attr;

    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != SliderMotion::default();

    let slider_aria = use_slider(SliderOptions {
        is_disabled,
        value: normalized_value,
        default_value: Some(normalized_default_value),
        on_value_change: normalized_on_value_change,
        min,
        max,
        step,
        lang,
        dir,
    });

    let state = Signal::derive(move || {
        logic::resolve_state(SliderStateInput {
            value: slider_aria.state.value.get(),
            min: slider_aria.state.min,
            max: slider_aria.state.max,
            step: slider_aria.state.step,
            is_disabled,
            has_custom_motion,
            has_custom_class_name: class_name.get_value().is_some(),
            has_custom_label,
        })
    });
    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));
    let visual_percent = Signal::derive(move || state.get().value_percent);
    let ui_action = Signal::derive(move || {
        logic::resolve_ui_action(
            slider_aria.state.is_pressed.get(),
            slider_aria.state.is_focused.get(),
        )
    });

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, visual_percent, motion);
    let label_view = render_label(id.clone(), label);
    let control_view = render_control(SliderInputRenderInput {
        id,
        label,
        is_disabled,
        slider_aria: slider_aria.clone(),
    });

    view! {
        <div
            node_ref=root_ref
            class=move || class.get()
            data-slot=SLOT_SLIDER
            data-state=move || state.get().phase_attr
            data-enabled=move || state.get().is_enabled.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-pressed=move || slider_aria.state.is_pressed.get().then_some("true")
            data-hovered=move || slider_aria.state.is_hovered.get().then_some("true")
            data-focused=move || slider_aria.state.is_focused.get().then_some("true")
            data-focus-visible=move || slider_aria.state.is_focus_visible.get().then_some("true")
            data-motion-source=move || state.get().motion_source_attr
            data-label-source=move || state.get().label_source_attr
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            data-custom-label=move || state.get().has_custom_label.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-custom-id=has_custom_id.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-id-source=id_source_attr
            data-control-mode=control_mode_attr
            data-value-source=value_source_attr
            data-default-value-source=default_value_source_attr
            data-value-change-source=value_change_source_attr
            data-disabled-source=disabled_source_attr
            data-min=move || state.get().min
            data-max=move || state.get().max
            data-step=move || state.get().step
            data-value=move || state.get().value.to_string()
            data-value-percent=move || state.get().value_percent.to_string()
            data-ui-schema=agent_contract.schema_attr
            data-ui-stream-support=agent_contract.stream_support_attr
            data-ui-stream-fallback=agent_contract.stream_fallback_attr
            data-ui-stream-mode=agent_contract.stream_mode_attr
            data-ui-output-status=agent_contract.output_status_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action=move || ui_action.get().as_attr()
            data-ui-source=value_change_source_attr
        >
            {label_view}
            {control_view}
        </div>
    }
}
