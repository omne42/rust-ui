use crate::color_slider::{
    ColorSliderChannel, ColorSliderMotion, ColorSliderStateInput,
    logic::{self},
};
use crate::{overlay_open, slider::motion as slider_motion};
use leptos::{ev, html, prelude::*};

#[component]
pub fn ColorSlider(
    id_base: String,
    #[prop(optional)] channel: ColorSliderChannel,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] value: Option<Signal<f64>>,
    #[prop(optional)] default_value: Option<f64>,
    #[prop(optional)] on_value_change: Option<Callback<f64>>,
    #[prop(optional, default = f64::NAN)] min: f64,
    #[prop(optional, default = f64::NAN)] max: f64,
    #[prop(optional, default = f64::NAN)] step: f64,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: ColorSliderMotion,
    #[prop(optional, default = true)] show_value_label: bool,
    #[prop(optional, into)] track_start_color: Option<String>,
    #[prop(optional, into)] track_end_color: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_optional_text(Some(id_base))
        .unwrap_or_else(|| "ui-color-slider".to_string());

    let input_id = format!("{id_base}-input");
    let input_id_for_label = input_id.clone();
    let input_id_for_output = StoredValue::new(input_id.clone());

    let label_id = format!("{id_base}-label");
    let label_id_for_root = label_id.clone();

    let value_id = format!("{id_base}-value");
    let value_id_for_output = StoredValue::new(value_id.clone());
    let value_id_for_input = StoredValue::new(value_id.clone());

    let (min, max) = logic::sanitize_bounds(channel, min, max);
    let step = logic::sanitize_step(channel, step, min, max);

    let default_value = logic::sanitize_value(
        channel,
        default_value.unwrap_or_else(|| channel.default_value()),
        min,
        max,
        step,
    );

    let value_state =
        overlay_open::use_controllable_state(value, Some(default_value), on_value_change);
    let value = value_state.value;
    let request_value_change = value_state.request_change;

    let (label, has_custom_label) = logic::normalize_label(label, channel);
    let label = StoredValue::new(label);

    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, &label.get_value(), channel);
    let aria_label = StoredValue::new(aria_label);

    let motion = slider_motion::sanitize_motion(motion);
    let has_custom_motion = motion != ColorSliderMotion::default();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let track_start_color = logic::sanitize_track_color(track_start_color);
    let track_end_color = logic::sanitize_track_color(track_end_color);
    let has_custom_track = track_start_color.is_some() || track_end_color.is_some();

    let inline_style =
        logic::compose_inline_style(track_start_color.as_deref(), track_end_color.as_deref());
    let inline_style = StoredValue::new(inline_style);

    let state = Memo::new(move |_| {
        logic::resolve_state(ColorSliderStateInput {
            disabled,
            channel,
            value: value.get(),
            min,
            max,
            step,
            show_value_label,
            has_custom_motion,
            has_custom_label,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_track,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let visual_percent = Signal::derive(move || state.get().value_percent);
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    slider_motion::attach_motion(root_ref, visual_percent, motion);

    let on_input = move |ev: ev::Event| {
        if disabled {
            return;
        }

        let Some(parsed) = logic::parse_value(&event_target_value(&ev)) else {
            return;
        };

        let next = logic::sanitize_value(channel, parsed, min, max, step);
        request_value_change.run(next);
    };

    view! {
        <div
            id=id_base
            node_ref=root_ref
            class=move || class.get()
            style=inline_style.get_value().unwrap_or_default()
            role="group"
            aria-labelledby=label_id_for_root
            data-slot="color-slider"
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-channel=move || state.get().channel_attr
            data-min=move || state.get().min.to_string()
            data-max=move || state.get().max.to_string()
            data-step=move || state.get().step.to_string()
            data-value=move || state.get().value.to_string()
            data-value-percent=move || state.get().value_percent.to_string()
            data-has-value-label=move || state.get().show_value_label.then_some("true")
            data-motion-source=move || state.get().motion_source_attr
            data-label-source=move || state.get().label_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-track-source=move || state.get().track_source_attr
            data-custom-track=move || state.get().has_custom_track.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
        >
            <div class="ui-color-slider__header" data-slot="color-slider-header">
                <label
                    id=label_id.clone()
                    class="ui-color-slider__label"
                    for=input_id_for_label
                    data-slot="color-slider-label"
                >
                    {label.get_value()}
                </label>

                <Show when=move || state.get().show_value_label>
                    <output
                        id=move || value_id_for_output.get_value()
                        class="ui-color-slider__value"
                        for=move || input_id_for_output.get_value()
                        data-slot="color-slider-value"
                        aria-live="polite"
                    >
                        {move || logic::format_channel_value(channel, state.get().value)}
                    </output>
                </Show>
            </div>

            <div class="ui-color-slider__control" data-slot="color-slider-control">
                <input
                    id=input_id
                    class="ui-color-slider__input"
                    data-slot="color-slider-input"
                    type="range"
                    min=move || state.get().min.to_string()
                    max=move || state.get().max.to_string()
                    step=move || state.get().step.to_string()
                    prop:value=move || state.get().value.to_string()
                    disabled=move || state.get().is_disabled
                    aria-label=aria_label.get_value()
                    aria-labelledby=label_id
                    aria-describedby=move || {
                        state
                            .get()
                            .show_value_label
                            .then_some(value_id_for_input.get_value())
                    }
                    aria-disabled=move || state.get().is_disabled.then_some("true")
                    aria-valuemin=move || state.get().min.to_string()
                    aria-valuemax=move || state.get().max.to_string()
                    aria-valuenow=move || state.get().value.to_string()
                    aria-valuetext=move || logic::format_channel_value(channel, state.get().value)
                    on:input=on_input
                />

                <div class="ui-color-slider__track" data-slot="color-slider-track" aria-hidden="true">
                    <div class="ui-color-slider__fill" data-slot="color-slider-fill"></div>
                    <div class="ui-color-slider__thumb" data-slot="color-slider-thumb"></div>
                </div>
            </div>
        </div>
    }
}
