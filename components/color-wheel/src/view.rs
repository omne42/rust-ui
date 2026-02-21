use crate::{
    ColorWheelMotion, ColorWheelStateInput,
    logic::{self},
    motion,
};
use leptos::{ev, html, prelude::*};
use std::borrow::Cow;
use ui_headless::{self as overlay_open, A11yDirection};

const TRACE_COMPONENT: &str = "color-wheel";

type InteractionSource = logic::ColorWheelInteractionSource;

fn emit_trace_note(trace: Option<overlay_open::UiTrace>, message: String) {
    let Some(trace) = trace else {
        return;
    };
    trace.emit(
        TRACE_COMPONENT,
        overlay_open::UiTraceEventKind::Note { message },
    );
}

fn emit_value_transition(
    trace: Option<overlay_open::UiTrace>,
    event: &'static str,
    source: InteractionSource,
    before: f64,
    after: f64,
    step: f64,
) {
    emit_trace_note(
        trace,
        format!(
            "event={event} source={} before={before:.3} after={after:.3} step={step:.3}",
            source.as_attr()
        ),
    );
}

fn render_header_section(
    label_id: String,
    input_id_for_label: String,
    label: StoredValue<String>,
    value_id_for_output: StoredValue<String>,
    input_id_for_output: StoredValue<String>,
    state: Memo<logic::ColorWheelState>,
) -> impl IntoView {
    view! {
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
    }
}

fn render_static_track_visuals() -> impl IntoView {
    view! {
        <div class="ui-color-wheel__ring" data-slot="color-wheel-ring" aria-hidden="true"></div>
        <div class="ui-color-wheel__orbit" data-slot="color-wheel-orbit" aria-hidden="true">
            <div class="ui-color-wheel__thumb" data-slot="color-wheel-thumb"></div>
        </div>
    }
}

#[component]
pub fn ColorWheel(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] value: Option<Signal<f64>>,
    #[prop(optional)] default_value: Option<f64>,
    #[prop(optional)] on_value_change: Option<Callback<f64>>,
    #[prop(optional, default = logic::DEFAULT_STEP)] step: f64,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: ColorWheelMotion,
    #[prop(optional)] is_value_label_visible: Option<bool>,
    #[prop(optional, default = true)] show_value_label: bool,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let is_controlled = value.is_some();
    let control_mode_attr = if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    };
    let value_source_attr = if is_controlled { "external" } else { "default" };

    let normalized_inputs = logic::normalize_state_inputs(
        is_disabled,
        disabled,
        is_value_label_visible,
        show_value_label,
    );
    let is_disabled = normalized_inputs.is_disabled();
    let is_value_label_visible = normalized_inputs.is_value_label_visible();

    let id_base: Cow<'static, str> = logic::normalize_optional_text(Some(id_base))
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("ui-color-wheel"));
    let id_base = id_base.into_owned();

    let input_id = format!("{id_base}-input");
    let input_id_for_label = input_id.clone();
    let input_id_for_output = StoredValue::new(input_id.clone());

    let label_id = format!("{id_base}-label");
    let label_id_for_input = label_id.clone();

    let value_id = format!("{id_base}-value");
    let value_id_for_output = StoredValue::new(value_id.clone());
    let value_id_for_input = StoredValue::new(value_id.clone());

    let step = logic::sanitize_step(step);

    let default_value = logic::resolve_default_value(default_value, step);
    let has_value_change_handler = on_value_change.is_some();
    let value_state =
        overlay_open::use_controllable_state(value, Some(default_value), on_value_change);
    let value = value_state.value;
    let request_value_change = value_state.request_change;
    let agent_contract = logic::resolve_agent_contract(has_value_change_handler);

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
            status: normalized_inputs.status,
            value: value.get(),
            step,
            value_label_mode: normalized_inputs.value_label_mode,
            motion_source: logic::source_from_custom_flag(has_custom_motion),
            label_source: logic::source_from_custom_flag(has_custom_label),
            aria_source: logic::source_from_custom_flag(has_custom_aria_label),
            class_source: logic::source_from_custom_flag(has_custom_class_name),
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let trace = overlay_open::use_ui_trace();

    let semantics = overlay_open::use_color_wheel(overlay_open::ColorWheelOptions {
        is_disabled,
        value,
        step,
        aria_label: aria_label.get_value(),
        label_id: label_id_for_input.clone(),
        value_id: is_value_label_visible.then_some(value_id_for_input.get_value()),
        lang,
        dir,
    });
    let root_attrs = semantics.root_attrs.clone();
    let input_attrs = semantics.input_attrs.clone();
    let track_attrs = semantics.track_attrs.clone();

    let track_ref: NodeRef<html::Div> = NodeRef::new();
    #[cfg(target_arch = "wasm32")]
    let track_ref_for_pointer = track_ref;
    let on_input_handler = semantics.handlers.on_input;
    let on_key_down_handler = semantics.handlers.on_key_down;
    let on_track_pointer_down_handler = semantics.handlers.on_track_pointer_down;
    let on_track_pointer_move_handler = semantics.handlers.on_track_pointer_move;
    let on_track_pointer_up_handler = semantics.handlers.on_track_pointer_up;
    let on_track_pointer_cancel_handler = semantics.handlers.on_track_pointer_cancel;
    let on_track_pointer_leave_handler = semantics.handlers.on_track_pointer_leave;
    let root_aria_labelledby = StoredValue::new(root_attrs.aria_labelledby.clone());
    let root_lang = StoredValue::new(root_attrs.lang.clone());
    let root_role = root_attrs.role;
    let root_dir = root_attrs.dir;
    let input_aria_label = StoredValue::new(input_attrs.aria_label.clone());
    let input_aria_labelledby = StoredValue::new(input_attrs.aria_labelledby.clone());
    let input_aria_describedby = StoredValue::new(input_attrs.aria_describedby.clone());
    let input_aria_valuemin = StoredValue::new(input_attrs.aria_valuemin.clone());
    let input_aria_valuemax = StoredValue::new(input_attrs.aria_valuemax.clone());
    let input_role = input_attrs.role;
    let input_aria_disabled = input_attrs.aria_disabled;
    let input_aria_valuenow = input_attrs.aria_valuenow;
    let input_aria_valuetext = input_attrs.aria_valuetext;
    let track_data_dragging = track_attrs.data_dragging;
    let track_data_dragging_for_action = track_data_dragging;
    let (drag_preview_value, set_drag_preview_value) = signal(None::<f64>);
    let (drag_preview_percent, set_drag_preview_percent) = signal(None::<f64>);
    let (interaction_source, set_interaction_source) = signal(InteractionSource::None);
    let ui_action = Memo::new(move |_| {
        logic::resolve_ui_action(
            track_data_dragging_for_action.get().is_some(),
            interaction_source.get(),
        )
    });
    let ui_state = Memo::new(move |_| logic::resolve_ui_state(state.get().is_disabled));

    let visual_percent = Signal::derive(move || {
        drag_preview_percent
            .get()
            .unwrap_or(state.get().value_percent)
    });
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, visual_percent, motion);

    let commit_drag_end = Callback::new(move |_| {
        let before = value.get_untracked();
        let maybe_preview_value = drag_preview_value.get_untracked();
        let resolved_value = maybe_preview_value
            .map(|next| logic::resolve_action(logic::Action::DragEnd { value: next, step }));

        set_drag_preview_value.set(None);
        set_drag_preview_percent.set(None);

        if let Some(next) = resolved_value {
            emit_value_transition(
                trace,
                "drag_end_commit",
                InteractionSource::Pointer,
                before,
                next,
                step,
            );
            set_interaction_source.set(InteractionSource::Pointer);
            request_value_change.run(next);
        }
    });
    let commit_drag_end_on_up = commit_drag_end;
    let commit_drag_end_on_cancel = commit_drag_end;
    let commit_drag_end_on_leave = commit_drag_end;

    let on_pointer_down = move |ev: ev::PointerEvent| {
        ev.prevent_default();

        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;

            let Some(track) = track_ref_for_pointer.get_untracked() else {
                return;
            };

            let track: leptos::web_sys::Element = track.unchecked_into();
            let rect = track.get_bounding_client_rect();
            let next = logic::pointer_to_hue_angle(
                ev.client_x() as f64,
                ev.client_y() as f64,
                rect.left(),
                rect.top(),
                rect.width(),
                rect.height(),
            );

            let Some(next) = on_track_pointer_down_handler.run(next) else {
                return;
            };
            let before = drag_preview_value
                .get_untracked()
                .unwrap_or_else(|| value.get_untracked());

            emit_value_transition(
                trace,
                "pointer_down_preview",
                InteractionSource::Pointer,
                before,
                next,
                step,
            );
            set_interaction_source.set(InteractionSource::Pointer);
            set_drag_preview_value.set(Some(next));
            set_drag_preview_percent.set(Some((next / 360.0 * 100.0).clamp(0.0, 100.0)));
        }
    };

    let on_pointer_move = move |_ev: ev::PointerEvent| {
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;

            let Some(track) = track_ref_for_pointer.get_untracked() else {
                return;
            };

            let track: leptos::web_sys::Element = track.unchecked_into();
            let rect = track.get_bounding_client_rect();
            let next = logic::pointer_to_hue_angle(
                _ev.client_x() as f64,
                _ev.client_y() as f64,
                rect.left(),
                rect.top(),
                rect.width(),
                rect.height(),
            );

            let Some(next) = on_track_pointer_move_handler.run(next) else {
                return;
            };
            let before = drag_preview_value
                .get_untracked()
                .unwrap_or_else(|| value.get_untracked());
            emit_value_transition(
                trace,
                "pointer_move_preview",
                InteractionSource::Pointer,
                before,
                next,
                step,
            );
            set_interaction_source.set(InteractionSource::Pointer);
            set_drag_preview_value.set(Some(next));
            set_drag_preview_percent.set(Some((next / 360.0 * 100.0).clamp(0.0, 100.0)));
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            drop(_ev);
        }
    };

    let on_input = move |ev: ev::Event| {
        let Some(next) = on_input_handler.run(event_target_value(&ev)) else {
            return;
        };
        let before = value.get_untracked();

        emit_value_transition(
            trace,
            "input_commit",
            InteractionSource::Input,
            before,
            next,
            step,
        );
        set_interaction_source.set(InteractionSource::Input);
        request_value_change.run(next);
    };

    let on_key_down = move |ev: ev::KeyboardEvent| {
        let Some(result) = on_key_down_handler.run(ev.key()) else {
            return;
        };
        let before = value.get_untracked();

        if result.prevent_default {
            ev.prevent_default();
        }
        emit_value_transition(
            trace,
            "keyboard_commit",
            InteractionSource::Keyboard,
            before,
            result.next_value,
            step,
        );
        set_interaction_source.set(InteractionSource::Keyboard);
        request_value_change.run(result.next_value);
    };

    let render_track_section = move || {
        view! {
            <div
                class="ui-color-wheel__track"
                data-slot="color-wheel-track"
                node_ref=track_ref
                data-dragging=move || track_data_dragging.get()
                on:pointerdown=on_pointer_down
                on:pointermove=on_pointer_move
                on:pointerup=move |_| {
                    on_track_pointer_up_handler.run(());
                    commit_drag_end_on_up.run(());
                }
                on:pointercancel=move |_| {
                    on_track_pointer_cancel_handler.run(());
                    commit_drag_end_on_cancel.run(());
                }
                on:pointerleave=move |_| {
                    on_track_pointer_leave_handler.run(());
                    commit_drag_end_on_leave.run(());
                }
            >
                {render_static_track_visuals()}

                <input
                    id=input_id
                    class="ui-color-wheel__input"
                    data-slot="color-wheel-input"
                    type="range"
                    min=logic::MIN_VALUE.to_string()
                    max=logic::MAX_VALUE.to_string()
                    step=move || state.get().step
                    prop:value=move || state.get().value
                    disabled=move || state.get().is_disabled
                    role=input_role
                    aria-label=move || input_aria_label.get_value()
                    aria-labelledby=move || input_aria_labelledby.get_value()
                    aria-describedby=move || input_aria_describedby.get_value()
                    aria-disabled=input_aria_disabled
                    aria-valuemin=move || input_aria_valuemin.get_value()
                    aria-valuemax=move || input_aria_valuemax.get_value()
                    aria-valuenow=move || input_aria_valuenow.get()
                    aria-valuetext=move || input_aria_valuetext.get()
                    on:input=on_input
                    on:keydown=on_key_down
                />
            </div>
        }
    };

    view! {
        <div
            id=id_base
            node_ref=root_ref
            class=move || class.get()
            role=root_role
            aria-labelledby=move || root_aria_labelledby.get_value()
            lang=move || root_lang.get_value()
            dir=root_dir
            data-slot="color-wheel"
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-control-mode=control_mode_attr
            data-value-source=value_source_attr
            data-interaction-source=move || interaction_source.get().as_attr()
            data-value=move || state.get().value.to_string()
            data-step=move || state.get().step
            data-value-percent=move || state.get().value_percent.to_string()
            data-has-value-label=move || state.get().show_value_label.then_some("true")
            data-motion-source=move || state.get().motion_source_attr
            data-label-source=move || state.get().label_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-ui-schema=agent_contract.schema_attr
            data-ui-schema-version=agent_contract.schema_version_attr
            data-ui-stream-support=agent_contract.stream_support_attr
            data-ui-stream-fallback=agent_contract.stream_fallback_attr
            data-ui-stream-mode=agent_contract.stream_mode_attr
            data-ui-output-status=agent_contract.output_status_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action=move || ui_action.get().as_attr()
            data-ui-source=agent_contract.source_attr
            data-ui-state=move || ui_state.get().as_attr()
        >
            {render_header_section(
                label_id.clone(),
                input_id_for_label.clone(),
                label,
                value_id_for_output,
                input_id_for_output,
                state,
            )}
            {render_track_section()}
        </div>
    }
}
