use crate::color_wheel::{
    ColorWheelMotion, ColorWheelStateInput,
    logic::{self},
    motion,
};
use leptos::{ev, html, prelude::*};
use ui_headless as overlay_open;

#[component]
pub fn ColorWheel(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] value: Option<Signal<f64>>,
    #[prop(optional)] default_value: Option<f64>,
    #[prop(optional)] on_value_change: Option<Callback<f64>>,
    #[prop(optional, default = logic::DEFAULT_STEP)] step: f64,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: ColorWheelMotion,
    #[prop(optional, default = true)] show_value_label: bool,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_optional_text(Some(id_base))
        .unwrap_or_else(|| "ui-color-wheel".to_string());

    let input_id = format!("{id_base}-input");
    let input_id_for_label = input_id.clone();
    let input_id_for_output = StoredValue::new(input_id.clone());

    let label_id = format!("{id_base}-label");
    let label_id_for_root = label_id.clone();
    let label_id_for_input = label_id.clone();

    let value_id = format!("{id_base}-value");
    let value_id_for_output = StoredValue::new(value_id.clone());
    let value_id_for_input = StoredValue::new(value_id.clone());

    let step = logic::sanitize_step(step);

    let default_value = logic::sanitize_value(default_value.unwrap_or(logic::MIN_VALUE), step);
    let value_state =
        overlay_open::use_controllable_state(value, Some(default_value), on_value_change);
    let value = value_state.value;
    let request_value_change = value_state.request_change;

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, &label.get_value());
    let aria_label = StoredValue::new(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != ColorWheelMotion::default();

    let state = Memo::new(move |_| {
        logic::resolve_state(ColorWheelStateInput {
            disabled,
            value: value.get(),
            step,
            show_value_label,
            has_custom_motion,
            has_custom_label,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let visual_percent = Signal::derive(move || state.get().value_percent);
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, visual_percent, motion);

    let track_ref: NodeRef<html::Div> = NodeRef::new();
    #[cfg(target_arch = "wasm32")]
    let track_ref_for_pointer = track_ref;
    let (is_dragging, set_dragging) = signal(false);

    let on_pointer_down = move |ev: ev::PointerEvent| {
        if disabled {
            return;
        }

        ev.prevent_default();
        set_dragging.set(true);

        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;

            let Some(track) = track_ref_for_pointer.get_untracked() else {
                return;
            };

            let track: leptos::web_sys::Element = track.unchecked_into();
            let Some(next) = logic::hue_from_pointer_event(&track, &ev) else {
                return;
            };

            let next = logic::sanitize_value(next, step);
            request_value_change.run(next);
        }
    };

    let on_pointer_move = move |_ev: ev::PointerEvent| {
        if disabled || !is_dragging.get() {
            return;
        }

        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;

            let Some(track) = track_ref_for_pointer.get_untracked() else {
                return;
            };

            let track: leptos::web_sys::Element = track.unchecked_into();
            let Some(next) = logic::hue_from_pointer_event(&track, &_ev) else {
                return;
            };

            let next = logic::sanitize_value(next, step);
            request_value_change.run(next);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = _ev;
        }
    };

    let on_input = move |ev: ev::Event| {
        if disabled {
            return;
        }

        let Some(parsed) = logic::parse_value(&event_target_value(&ev)) else {
            return;
        };

        let next = logic::sanitize_value(parsed, step);
        request_value_change.run(next);
    };

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if disabled {
            return;
        }

        let key = ev.key();
        let current = state.get().value;
        let next = match key.as_str() {
            "ArrowRight" | "ArrowUp" => Some(logic::move_value_by_delta(current, step, step)),
            "ArrowLeft" | "ArrowDown" => Some(logic::move_value_by_delta(current, -step, step)),
            "PageUp" => Some(logic::move_value_by_delta(
                current,
                logic::page_step(step),
                step,
            )),
            "PageDown" => Some(logic::move_value_by_delta(
                current,
                -logic::page_step(step),
                step,
            )),
            "Home" => Some(logic::MIN_VALUE),
            "End" => Some(logic::MAX_VALUE),
            _ => None,
        };

        let Some(next) = next else {
            return;
        };

        ev.prevent_default();
        request_value_change.run(next);
    };

    view! {
        <div
            id=id_base
            node_ref=root_ref
            class=move || class.get()
            role="group"
            aria-labelledby=label_id_for_root
            data-slot="color-wheel"
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-value=move || state.get().value.to_string()
            data-step=move || state.get().step.to_string()
            data-value-percent=move || state.get().value_percent.to_string()
            data-has-value-label=move || state.get().show_value_label.then_some("true")
            data-motion-source=move || state.get().motion_source_attr
            data-label-source=move || state.get().label_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
        >
            <div class="ui-color-wheel__header" data-slot="color-wheel-header">
                <label
                    id=label_id
                    class="ui-color-wheel__label"
                    for=input_id_for_label
                    data-slot="color-wheel-label"
                >
                    {label.get_value()}
                </label>

                <Show when=move || state.get().show_value_label>
                    <output
                        id=move || value_id_for_output.get_value()
                        class="ui-color-wheel__value"
                        for=move || input_id_for_output.get_value()
                        data-slot="color-wheel-value"
                        aria-live="polite"
                    >
                        {move || logic::format_value_text(state.get().value)}
                    </output>
                </Show>
            </div>

            <div
                class="ui-color-wheel__track"
                data-slot="color-wheel-track"
                node_ref=track_ref
                data-dragging=move || is_dragging.get().then_some("true")
                on:pointerdown=on_pointer_down
                on:pointermove=on_pointer_move
                on:pointerup=move |_| set_dragging.set(false)
                on:pointercancel=move |_| set_dragging.set(false)
                on:pointerleave=move |_| set_dragging.set(false)
            >
                <div class="ui-color-wheel__ring" data-slot="color-wheel-ring" aria-hidden="true"></div>
                <div class="ui-color-wheel__orbit" data-slot="color-wheel-orbit" aria-hidden="true">
                    <div class="ui-color-wheel__thumb" data-slot="color-wheel-thumb"></div>
                </div>

                <input
                    id=input_id
                    class="ui-color-wheel__input"
                    data-slot="color-wheel-input"
                    type="range"
                    min=logic::MIN_VALUE.to_string()
                    max=logic::MAX_VALUE.to_string()
                    step=move || state.get().step.to_string()
                    prop:value=move || state.get().value.to_string()
                    disabled=move || state.get().is_disabled
                    aria-label=aria_label.get_value()
                    aria-labelledby=label_id_for_input
                    aria-describedby=move || {
                        state
                            .get()
                            .show_value_label
                            .then_some(value_id_for_input.get_value())
                    }
                    aria-disabled=move || state.get().is_disabled.then_some("true")
                    aria-valuemin=logic::MIN_VALUE.to_string()
                    aria-valuemax=logic::MAX_VALUE.to_string()
                    aria-valuenow=move || state.get().value.round().to_string()
                    aria-valuetext=move || logic::format_value_text(state.get().value)
                    on:input=on_input
                    on:keydown=on_key_down
                />
            </div>
        </div>
    }
}
