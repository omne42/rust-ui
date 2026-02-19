use crate::color::slider::{
    ColorSliderChannel, ColorSliderMotion, ColorSliderStateInput,
    logic::{self},
    motion,
};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, SliderOptions, use_slider};

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
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: ColorSliderMotion,
    #[prop(optional, default = true)] show_value_label: bool,
    #[prop(optional, into)] track_start_color: Option<String>,
    #[prop(optional, into)] track_end_color: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let has_external_value = value.is_some();
    let has_default_value = default_value.is_some();
    let has_on_value_change = on_value_change.is_some();
    let control_mode_attr = if has_external_value {
        logic::ColorSliderControlMode::Controlled.as_attr()
    } else {
        logic::ColorSliderControlMode::Uncontrolled.as_attr()
    };
    let value_source_attr = if has_external_value {
        logic::ColorSliderValueSource::External.as_attr()
    } else {
        logic::ColorSliderValueSource::DefaultValue.as_attr()
    };
    let value_change_source_attr = if has_on_value_change {
        logic::ColorSliderValueChangeSource::OnValueChange.as_attr()
    } else {
        logic::ColorSliderValueChangeSource::None.as_attr()
    };
    let default_value_source_attr = logic::source_attr_from_presence(has_default_value);
    let agent_contract = logic::resolve_agent_contract(has_on_value_change);

    let accessibility_state = logic::normalize_accessibility_state(is_disabled, disabled);
    let is_disabled = accessibility_state.is_disabled;
    let disabled_source_attr = accessibility_state.disabled_source_attr;

    let id_base = logic::normalize_optional_text(Some(id_base))
        .unwrap_or_else(|| "ui-color-slider".to_string());
    let input_id = format!("{id_base}-input");
    let input_id_for_label = input_id.clone();
    let input_id_for_output = StoredValue::new(input_id.clone());

    let label_id = format!("{id_base}-label");
    let label_id_for_root = label_id.clone();
    let label_id_for_input = label_id.clone();
    let label_id_for_label = label_id.clone();

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

    let (label, has_custom_label) = logic::normalize_label(label, channel);
    let label = StoredValue::new(label);

    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, &label.get_value(), channel);
    let aria_label = StoredValue::new(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let track_start_color = logic::sanitize_track_color(track_start_color);
    let track_end_color = logic::sanitize_track_color(track_end_color);
    let has_custom_track = track_start_color.is_some() || track_end_color.is_some();
    let inline_style =
        logic::compose_inline_style(track_start_color.as_deref(), track_end_color.as_deref());
    let inline_style = StoredValue::new(inline_style);

    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != ColorSliderMotion::default();

    let lang = logic::normalize_optional_text(lang);
    let slider_aria = use_slider(SliderOptions {
        is_disabled,
        value,
        default_value: Some(default_value),
        on_value_change,
        min,
        max,
        step,
        lang,
        dir,
    });

    let state = Memo::new(move |_| {
        logic::resolve_state(ColorSliderStateInput {
            disabled: is_disabled,
            channel,
            value: slider_aria.state.value.get(),
            min: slider_aria.state.min,
            max: slider_aria.state.max,
            step: slider_aria.state.step,
            show_value_label,
            has_custom_motion,
            has_custom_label,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_track,
        })
    });
    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let ui_action = Signal::derive(move || {
        logic::resolve_ui_action(
            slider_aria.state.is_pressed.get(),
            slider_aria.state.is_focused.get(),
        )
    });

    let visual_percent = Signal::derive(move || state.get().value_percent);
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, visual_percent, motion);
    let locale_lang = StoredValue::new(slider_aria.input.lang.clone());
    let locale_dir = slider_aria.input.dir;

    view! {
        <div
            id=id_base
            node_ref=root_ref
            class=move || class.get()
            style=inline_style.get_value().unwrap_or_default()
            role="group"
            lang=move || locale_lang.get_value()
            dir=locale_dir
            aria-labelledby=label_id_for_root
            data-slot="color-slider"
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-pressed=move || slider_aria.state.is_pressed.get().then_some("true")
            data-hovered=move || slider_aria.state.is_hovered.get().then_some("true")
            data-focused=move || slider_aria.state.is_focused.get().then_some("true")
            data-focus-visible=move || slider_aria.state.is_focus_visible.get().then_some("true")
            data-channel=move || state.get().channel_attr
            data-min=move || state.get().min
            data-max=move || state.get().max
            data-step=move || state.get().step
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
            data-control-mode=control_mode_attr
            data-value-source=value_source_attr
            data-default-value-source=default_value_source_attr
            data-value-change-source=value_change_source_attr
            data-disabled-source=disabled_source_attr
            data-ui-schema=agent_contract.schema_attr
            data-ui-schema-version=agent_contract.schema_version_attr
            data-ui-stream-support=agent_contract.stream_support_attr
            data-ui-stream-fallback=agent_contract.stream_fallback_attr
            data-ui-stream-mode=agent_contract.stream_mode_attr
            data-ui-output-status=agent_contract.output_status_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action=move || ui_action.get().as_attr()
            data-ui-source=value_change_source_attr
            data-ui-state=move || state.get().data_state_attr
        >
            <div class="ui-color-slider__header" data-slot="color-slider-header">
                <label
                    id=label_id_for_label
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
                    min=move || state.get().min
                    max=move || state.get().max
                    step=move || state.get().step
                    prop:value=move || state.get().value
                    disabled=move || state.get().is_disabled
                    role=slider_aria.input.role
                    lang=move || locale_lang.get_value()
                    dir=locale_dir
                    aria-label=aria_label.get_value()
                    aria-labelledby=label_id_for_input
                    aria-describedby=move || {
                        state
                            .get()
                            .show_value_label
                            .then_some(value_id_for_input.get_value())
                    }
                    aria-disabled=slider_aria.input.aria_disabled
                    aria-valuemin=move || slider_aria.input.aria_valuemin.get()
                    aria-valuemax=move || slider_aria.input.aria_valuemax.get()
                    aria-valuenow=move || slider_aria.input.aria_valuenow.get()
                    aria-valuetext=move || logic::format_channel_value(channel, state.get().value)
                    on:input=move |ev| {
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

                <div class="ui-color-slider__track" data-slot="color-slider-track" aria-hidden="true">
                    <div class="ui-color-slider__fill" data-slot="color-slider-fill"></div>
                    <div class="ui-color-slider__thumb" data-slot="color-slider-thumb"></div>
                </div>
            </div>
        </div>
    }
}
