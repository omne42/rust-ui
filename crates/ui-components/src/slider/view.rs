use crate::slider::{
    logic::{self, SliderStateInput},
    motion::{self, SliderMotion},
};
use leptos::{ev, html, prelude::*};

#[component]
pub fn Slider(
    id: String,
    #[prop(default = logic::DEFAULT_LABEL.to_string(), into)] label: String,
    value: ReadSignal<f64>,
    set_value: WriteSignal<f64>,
    #[prop(default = logic::DEFAULT_MIN)] min: f64,
    #[prop(default = logic::DEFAULT_MAX)] max: f64,
    #[prop(default = logic::DEFAULT_STEP)] step: f64,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_change: Option<Callback<f64>>,
    #[prop(optional)] motion: SliderMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id = id.trim().to_string();
    let id = if id.is_empty() {
        "ui-slider".to_string()
    } else {
        id
    };

    let (label, has_custom_label) = logic::resolve_label(label);
    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let (min, max) = logic::sanitize_bounds(min, max);
    let step = logic::sanitize_step(step, min, max);

    let motion = motion::sanitize_motion(motion);
    let on_change = StoredValue::new(on_change);

    let state = Signal::derive(move || {
        logic::resolve_state(SliderStateInput {
            value: value.get(),
            min,
            max,
            step,
            disabled,
            has_custom_motion: motion != SliderMotion::default(),
            has_custom_class_name: class_name.get_value().is_some(),
            has_custom_label,
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let visual_percent = Signal::derive(move || state.get().value_percent);

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, visual_percent, motion);

    let on_input = move |ev: ev::Event| {
        if disabled {
            return;
        }

        let Some(parsed) = logic::parse_value(&event_target_value(&ev)) else {
            return;
        };

        let next = logic::sanitize_value(parsed, min, max, step);
        set_value.set(next);

        if let Some(on_change) = on_change.get_value() {
            on_change.run(next);
        }
    };

    view! {
        <div
            node_ref=root_ref
            class=move || class.get()
            data-slot="slider"
            data-state=move || state.get().phase_attr
            data-enabled=move || state.get().is_enabled.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-motion-source=move || state.get().motion_source_attr
            data-label-source=move || state.get().label_source_attr
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            data-custom-label=move || state.get().has_custom_label.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-min=move || state.get().min.to_string()
            data-max=move || state.get().max.to_string()
            data-step=move || state.get().step.to_string()
            data-value=move || state.get().value.to_string()
            data-value-percent=move || state.get().value_percent.to_string()
        >
            <label class="ui-slider__label" for=id.clone() data-slot="slider-label">
                {label.clone()}
            </label>

            <div class="ui-slider__control" data-slot="slider-control">
                <input
                    id=id
                    class="ui-slider__input"
                    data-slot="slider-input"
                    type="range"
                    min=move || state.get().min.to_string()
                    max=move || state.get().max.to_string()
                    step=move || state.get().step.to_string()
                    prop:value=move || state.get().value.to_string()
                    disabled=move || state.get().is_disabled
                    aria-label=label
                    aria-disabled=move || state.get().is_disabled.then_some("true")
                    aria-valuemin=move || state.get().min.to_string()
                    aria-valuemax=move || state.get().max.to_string()
                    aria-valuenow=move || state.get().value.to_string()
                    aria-valuetext=move || format!("{:.0}%", state.get().value_percent)
                    on:input=on_input
                />

                <div class="ui-slider__track" data-slot="slider-track" aria-hidden="true">
                    <div class="ui-slider__fill" data-slot="slider-fill"></div>
                    <div class="ui-slider__thumb" data-slot="slider-thumb"></div>
                </div>
            </div>
        </div>
    }
}
