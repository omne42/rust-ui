use crate::ai_space::use_ai_space_state;
use crate::time_field::{
    TimeFieldMotion, TimeFieldStateInput, TimeFieldStrings,
    logic::{self, TimeFieldTone},
    motion,
};
use leptos::{ev, html, prelude::*};
use ui_headless as overlay_open;
use ui_headless::i18n;
use ui_headless::{
    A11yDirection, ButtonAria, ButtonOptions, HoverOptions, HoverState, TimeFieldOptions,
    use_button, use_hover, use_time_field,
};

const SLOT_TIME_FIELD: &str = "time-field";
const SLOT_TIME_FIELD_LABEL: &str = "time-field-label";
const SLOT_TIME_FIELD_CONTROL: &str = "time-field-control";
const SLOT_TIME_FIELD_HOUR: &str = "time-field-hour";
const SLOT_TIME_FIELD_SEPARATOR: &str = "time-field-separator";
const SLOT_TIME_FIELD_MINUTE: &str = "time-field-minute";
const SLOT_TIME_FIELD_CLEAR: &str = "time-field-clear";
const CLASS_TIME_FIELD_LABEL: &str = "ui-time-field__label";
const CLASS_TIME_FIELD_CONTROL: &str = "ui-time-field__control";
const CLASS_TIME_FIELD_INPUT: &str = "ui-time-field__input";
const CLASS_TIME_FIELD_SEPARATOR: &str = "ui-time-field__separator";
const CLASS_TIME_FIELD_CLEAR: &str = "ui-time-field__clear";
const BOOL_TRUE: &str = "true";
const INPUT_TYPE_NUMBER: &str = "number";
const TIME_SEPARATOR: &str = ":";

fn render_label(label_id: String, hour_id: String, label: StoredValue<String>) -> impl IntoView {
    view! {
        <label
            id=label_id
            class=CLASS_TIME_FIELD_LABEL
            data-slot=SLOT_TIME_FIELD_LABEL
            for=hour_id
        >
            {label.get_value()}
        </label>
    }
}

fn render_hour_input(
    hour_id: String,
    hour_placeholder: StoredValue<String>,
    hour_value: Memo<String>,
    is_disabled: bool,
    hour_aria_label: StoredValue<String>,
    agent_source: RwSignal<logic::TimeFieldAgentSource>,
    on_hour_input_handler: Callback<String>,
) -> impl IntoView {
    view! {
        <input
            id=hour_id
            class=CLASS_TIME_FIELD_INPUT
            data-slot=SLOT_TIME_FIELD_HOUR
            type=INPUT_TYPE_NUMBER
            min="0"
            max="23"
            step="1"
            placeholder=hour_placeholder.get_value()
            prop:value=move || hour_value.get()
            disabled=is_disabled
            aria-label=hour_aria_label.get_value()
            on:input=move |ev| {
                agent_source.set(logic::TimeFieldAgentSource::HourInput);
                on_hour_input_handler.run(event_target_value(&ev));
            }
        />
    }
}

fn render_separator() -> impl IntoView {
    view! {
        <span class=CLASS_TIME_FIELD_SEPARATOR data-slot=SLOT_TIME_FIELD_SEPARATOR aria-hidden=BOOL_TRUE>
            {TIME_SEPARATOR}
        </span>
    }
}

struct MinuteInputRenderInput {
    minute_id: String,
    minute_step: u8,
    minute_placeholder: StoredValue<String>,
    minute_value: Memo<String>,
    is_disabled: bool,
    minute_aria_label: StoredValue<String>,
    agent_source: RwSignal<logic::TimeFieldAgentSource>,
    on_minute_input_handler: Callback<String>,
}

fn render_minute_input(input: MinuteInputRenderInput) -> impl IntoView {
    let MinuteInputRenderInput {
        minute_id,
        minute_step,
        minute_placeholder,
        minute_value,
        is_disabled,
        minute_aria_label,
        agent_source,
        on_minute_input_handler,
    } = input;

    view! {
        <input
            id=minute_id
            class=CLASS_TIME_FIELD_INPUT
            data-slot=SLOT_TIME_FIELD_MINUTE
            type=INPUT_TYPE_NUMBER
            min="0"
            max="59"
            step=minute_step.to_string()
            placeholder=minute_placeholder.get_value()
            prop:value=move || minute_value.get()
            disabled=is_disabled
            aria-label=minute_aria_label.get_value()
            on:input=move |ev| {
                agent_source.set(logic::TimeFieldAgentSource::MinuteInput);
                on_minute_input_handler.run(event_target_value(&ev));
            }
        />
    }
}

fn render_clear_button(
    state: Memo<logic::TimeFieldState>,
    clear_hover: HoverState,
    clear_button: ButtonAria,
    is_disabled: bool,
    clear_aria_label: StoredValue<String>,
    clear_label: StoredValue<String>,
    clear_ref: NodeRef<html::Button>,
) -> impl IntoView {
    let clear_is_hovered = clear_hover.is_hovered;
    let clear_hover_handlers = clear_hover.handlers;
    let clear_is_pressed = clear_button.is_pressed;
    let clear_press_handlers = clear_button.handlers.press;
    let on_pointer_enter = clear_hover_handlers.on_pointer_enter;
    let on_pointer_leave = clear_hover_handlers.on_pointer_leave;
    let on_pointer_down = clear_press_handlers.on_pointer_down;
    let on_pointer_up = clear_press_handlers.on_pointer_up;
    let on_pointer_cancel = clear_press_handlers.on_pointer_cancel;
    let on_click = clear_press_handlers.on_click;
    let on_key_down = clear_press_handlers.on_key_down;
    let on_key_up = clear_press_handlers.on_key_up;
    let on_blur = clear_press_handlers.on_blur;
    let clear_role = clear_button.attrs.role;
    let clear_tabindex = clear_button.attrs.tabindex;
    let clear_aria_disabled = clear_button.attrs.aria_disabled;

    view! {
        <button
            type="button"
            class=CLASS_TIME_FIELD_CLEAR
            data-slot=SLOT_TIME_FIELD_CLEAR
            data-visible=move || state.get().has_value.then_some(BOOL_TRUE)
            data-hovered=move || clear_is_hovered.get().then_some(BOOL_TRUE)
            data-pressed=move || clear_is_pressed.get().then_some(BOOL_TRUE)
            role=clear_role
            tabindex=move || {
                if state.get().has_value {
                    clear_tabindex
                } else {
                    Some(-1)
                }
            }
            aria-disabled=clear_aria_disabled
            aria-hidden=move || (!state.get().has_value).then_some(BOOL_TRUE)
            disabled=move || is_disabled || !state.get().has_value
            aria-label=clear_aria_label.get_value()
            node_ref=clear_ref
            on:pointerdown=move |_| on_pointer_down.run(())
            on:pointerup=move |_| on_pointer_up.run(())
            on:pointercancel=move |_| on_pointer_cancel.run(())
            on:pointerenter=move |_| on_pointer_enter.run(())
            on:pointerleave=move |_| on_pointer_leave.run(())
            on:click=move |_| on_click.run(())
            on:keydown=move |ev: ev::KeyboardEvent| {
                let key = ev.key();
                if on_key_down.run(key) {
                    ev.prevent_default();
                }
            }
            on:keyup=move |ev: ev::KeyboardEvent| {
                let key = ev.key();
                if on_key_up.run(key) {
                    ev.prevent_default();
                }
            }
            on:blur=move |_| on_blur.run(())
        >
            {clear_label.get_value()}
        </button>
    }
}

#[component]
pub fn TimeField(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] tone: TimeFieldTone,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] value: Option<Signal<Option<String>>>,
    #[prop(optional)] default_value: Option<String>,
    #[prop(optional)] on_value_change: Option<Callback<Option<String>>>,
    #[prop(optional, default = 1)] minute_step: u8,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] hour_aria_label: Option<String>,
    #[prop(optional, into)] minute_aria_label: Option<String>,
    #[prop(optional, into)] clear_label: Option<String>,
    #[prop(optional, into)] clear_aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: TimeFieldMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<TimeFieldStrings>();
    let minute_step = logic::normalize_minute_step(minute_step);
    let is_disabled = logic::normalize_disabled_state(logic::DisabledStateInput {
        is_disabled,
        disabled,
    });
    let normalized_value_state = logic::normalize_value_state(logic::ValueStateInput {
        value,
        default_value,
        on_value_change,
        minute_step,
    });
    let is_controlled = normalized_value_state.is_controlled;
    let has_default_value = normalized_value_state.has_default_value;
    let has_value_change_handler = normalized_value_state.has_value_change_handler;
    let controlled_value = normalized_value_state.value;
    let controlled_default_value = normalized_value_state.default_value;
    let controlled_on_value_change = normalized_value_state.on_value_change;

    let value_state = overlay_open::use_controllable_state(
        controlled_value,
        Some(controlled_default_value),
        controlled_on_value_change,
    );
    let value = value_state.value;
    let request_value_change = value_state.request_change;

    let (label, has_custom_label) = logic::normalize_label(label, strings.label.as_ref());
    let label = StoredValue::new(label);
    let (placeholder, has_custom_placeholder) =
        logic::normalize_placeholder(placeholder, strings.placeholder.as_ref());
    let placeholder = StoredValue::new(placeholder);
    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, strings.aria_label.as_ref());
    let (hour_aria_label, _) =
        logic::normalize_hour_aria_label(hour_aria_label, strings.hour_aria_label.as_ref());
    let (minute_aria_label, _) =
        logic::normalize_minute_aria_label(minute_aria_label, strings.minute_aria_label.as_ref());
    let (clear_label, _) = logic::normalize_clear_label(clear_label, strings.clear_label.as_ref());
    let clear_label = StoredValue::new(clear_label);
    let (clear_aria_label, _) =
        logic::normalize_clear_aria_label(clear_aria_label, strings.clear_aria_label.as_ref());

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != TimeFieldMotion::default();

    let ids = logic::resolve_ids(&id_base);
    let root_id = ids.root_id;
    let label_id = ids.label_id;
    let hour_id = ids.hour_id;
    let minute_id = ids.minute_id;

    let time_field = use_time_field(TimeFieldOptions {
        is_disabled,
        value,
        on_value_change: request_value_change,
        minute_step,
        aria_label,
        lang,
        dir,
        hour_aria_label,
        minute_aria_label,
        clear_aria_label,
    });
    let group_role = time_field.attrs.role;
    let group_aria_label = StoredValue::new(time_field.attrs.aria_label.clone());
    let group_lang = StoredValue::new(time_field.attrs.lang.clone());
    let group_dir = time_field.attrs.dir;
    let hour_aria_label = StoredValue::new(time_field.attrs.hour_aria_label.clone());
    let minute_aria_label = StoredValue::new(time_field.attrs.minute_aria_label.clone());
    let clear_aria_label = StoredValue::new(time_field.attrs.clear_aria_label.clone());
    let on_hour_input_handler = time_field.handlers.on_hour_input;
    let on_minute_input_handler = time_field.handlers.on_minute_input;
    let on_clear_handler = time_field.handlers.on_clear;
    let parts = time_field.state.parts;
    let has_value = time_field.state.has_value;
    let resolved_minute_step = time_field.state.minute_step;
    let state = Memo::new(move |_| {
        logic::resolve_state(TimeFieldStateInput {
            tone,
            disabled: is_disabled,
            is_controlled,
            has_default_value,
            has_value_change_handler,
            has_value: has_value.get(),
            minute_step: resolved_minute_step,
            has_custom_label,
            has_custom_placeholder,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_motion,
        })
    });
    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let input_placeholders = logic::resolve_input_placeholders(&placeholder.get_value());
    let hour_placeholder = StoredValue::new(input_placeholders.0);
    let minute_placeholder = StoredValue::new(input_placeholders.1);

    let hour_value = Memo::new(move |_| {
        let (hour, _, has_value) = parts.get();
        if has_value {
            format!("{hour:02}")
        } else {
            String::new()
        }
    });
    let minute_value = Memo::new(move |_| {
        let (_, minute, has_value) = parts.get();
        if has_value {
            format!("{minute:02}")
        } else {
            String::new()
        }
    });
    let agent_source = RwSignal::new(logic::TimeFieldAgentSource::Init);
    let agent_contract =
        Signal::derive(move || logic::resolve_agent_contract(state.get(), agent_source.get()));
    let ai_space_state = StoredValue::new(use_ai_space_state());

    let clear_hover = use_hover(HoverOptions { is_disabled });
    let agent_source_for_clear = agent_source;
    let clear_button = use_button(ButtonOptions {
        is_disabled,
        on_press: Some(Callback::new(move |_| {
            agent_source_for_clear.set(logic::TimeFieldAgentSource::ClearPress);
            on_clear_handler.run(());
        })),
        ..Default::default()
    });
    let clear_ref: NodeRef<html::Button> = NodeRef::new();
    motion::attach_clear_button_motion(
        clear_ref,
        Signal::derive(move || state.get().has_value),
        clear_hover.is_hovered,
        clear_button.is_pressed,
        is_disabled,
        motion,
    );
    let label_view = render_label(label_id.clone(), hour_id.clone(), label);
    let hour_input_view = render_hour_input(
        hour_id,
        hour_placeholder,
        hour_value,
        is_disabled,
        hour_aria_label,
        agent_source,
        on_hour_input_handler,
    );
    let separator_view = render_separator();
    let minute_input_view = render_minute_input(MinuteInputRenderInput {
        minute_id,
        minute_step,
        minute_placeholder,
        minute_value,
        is_disabled,
        minute_aria_label,
        agent_source,
        on_minute_input_handler,
    });
    let clear_button_view = render_clear_button(
        state,
        clear_hover,
        clear_button,
        is_disabled,
        clear_aria_label,
        clear_label,
        clear_ref,
    );

    view! {
        <div
            id=root_id
            class=move || class.get()
            data-slot=SLOT_TIME_FIELD
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some(BOOL_TRUE)
            data-has-value=move || state.get().has_value.then_some(BOOL_TRUE)
            data-minute-step=move || state.get().minute_step.to_string()
            data-control-mode=move || state.get().control_mode_attr
            data-controlled=move || state.get().is_controlled.then_some(BOOL_TRUE)
            data-uncontrolled=move || state.get().is_uncontrolled.then_some(BOOL_TRUE)
            data-value-source=move || state.get().value_source_attr
            data-default-value-source=move || state.get().default_value_source_attr
            data-value-change-source=move || state.get().value_change_source_attr
            data-label-source=move || state.get().label_source_attr
            data-placeholder-source=move || state.get().placeholder_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some(BOOL_TRUE)
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-custom-motion=has_custom_motion.then_some(BOOL_TRUE)
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-capability-edit=move || {
                agent_contract.get().capabilities.can_edit.then_some(BOOL_TRUE)
            }
            data-ui-capability-clear=move || {
                agent_contract.get().capabilities.can_clear.then_some(BOOL_TRUE)
            }
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode=move || {
                ai_space_state
                    .get_value()
                    .map(|state| state.get().mode.as_str())
                    .unwrap_or("snapshot")
            }
            data-ui-output-status=move || {
                ai_space_state
                    .get_value()
                    .map(|state| state.get().output_status.as_str())
                    .unwrap_or(agent_contract.get().output_status.as_str())
            }
            role=group_role
            aria-label=group_aria_label.get_value()
            aria-labelledby=label_id.clone()
            lang=group_lang.get_value()
            dir=group_dir
        >
            {label_view}

            <div class=CLASS_TIME_FIELD_CONTROL data-slot=SLOT_TIME_FIELD_CONTROL>
                {hour_input_view}
                {separator_view}
                {minute_input_view}
                {clear_button_view}
            </div>
        </div>
    }
}
