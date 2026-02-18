use crate::text_field::{TextFieldMotion, logic, motion};
use leptos::{html, prelude::*};
use ui_headless::A11yDirection;
use ui_headless::text_field::{
    TextFieldContractOptions, TextFieldOptions, use_text_field, use_text_field_contract,
};
use ui_headless::use_controllable_state;

const SLOT_ROOT: &str = "text-field";
const SLOT_LABEL: &str = "text-field-label";
const SLOT_INPUT: &str = "text-field-input";
const SLOT_DESCRIPTION: &str = "text-field-description";
const SLOT_ERROR: &str = "text-field-error";

const CLASS_LABEL: &str = "ui-text-field__label";
const CLASS_INPUT: &str = "ui-text-field__input";
const CLASS_DESCRIPTION: &str = "ui-text-field__description";
const CLASS_ERROR: &str = "ui-text-field__error";

const MOTION_SOURCE_CUSTOM: &str = "custom";
const MOTION_SOURCE_DEFAULT: &str = "default";

fn render_description(description: Option<String>, description_id: String) -> impl IntoView {
    description.map(|description| {
        view! {
            <div
                class=CLASS_DESCRIPTION
                id=description_id
                data-slot=SLOT_DESCRIPTION
            >
                {description}
            </div>
        }
    })
}

fn render_error(
    error: Option<String>,
    error_id: String,
    is_invalid: Signal<bool>,
) -> impl IntoView {
    error.map(|error| {
        let error_id = StoredValue::new(error_id);
        let error = StoredValue::new(error);
        view! {
            <Show when=move || is_invalid.get()>
                <div
                    class=CLASS_ERROR
                    id=move || error_id.get_value()
                    data-slot=SLOT_ERROR
                >
                    {move || error.get_value()}
                </div>
            </Show>
        }
    })
}

#[component]
pub fn TextField(
    id: String,
    label: String,
    #[prop(optional, into)] value: Option<Signal<String>>,
    #[prop(optional, into)] default_value: Option<String>,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
    #[prop(optional)] set_value: Option<WriteSignal<String>>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] is_read_only: Option<bool>,
    #[prop(optional)] read_only: bool,
    #[prop(optional, into)] is_required: Option<Signal<bool>>,
    #[prop(optional, into)] required: Option<Signal<bool>>,
    #[prop(optional, into)] is_invalid: Option<Signal<bool>>,
    #[prop(optional, into)] invalid: Option<Signal<bool>>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] input_type: Option<&'static str>,
    #[prop(optional)] motion: TextFieldMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let value_axis = logic::normalize_value_axis(logic::ValueAxisInput {
        value,
        default_value,
        on_value_change,
        set_value,
    });
    let controlled_value = value_axis.value;
    let controlled_default_value = value_axis.default_value.clone();
    let controlled_on_value_change = value_axis.on_value_change;
    let value_state = use_controllable_state(
        controlled_value,
        Some(controlled_default_value),
        controlled_on_value_change,
    );
    let value = value_state.value;
    let request_value_change = value_state.request_change;

    let accessibility_state =
        logic::normalize_accessibility_state(logic::AccessibilityStateInput {
            is_disabled,
            disabled,
            is_read_only,
            read_only,
            is_required,
            required,
            is_invalid,
            invalid,
        });
    let is_disabled = accessibility_state.is_disabled;
    let is_read_only = accessibility_state.is_read_only;
    let is_required = accessibility_state.is_required;
    let is_invalid = accessibility_state.is_invalid;

    let resolved_props = logic::resolve_props(
        label,
        description,
        error,
        placeholder,
        input_type,
        class_name,
    );
    let logic::TextFieldResolvedProps {
        label,
        label_source_attr,
        description,
        error,
        placeholder,
        input_type,
        type_source_attr,
        class,
        has_custom_class_name,
        description_source_attr,
        error_source_attr,
        placeholder_source_attr,
        class_source_attr,
    } = resolved_props;
    let has_description = description.is_some();
    let has_error = error.is_some();

    let aria = use_text_field(TextFieldOptions {
        id: id.clone(),
        has_description,
        has_error,
        aria_describedby,
        is_invalid,
        is_required,
    });

    let contract = use_text_field_contract(TextFieldContractOptions {
        is_disabled,
        is_read_only,
        value,
        on_value_change: request_value_change,
        is_invalid,
        is_required,
        lang,
        dir,
    });
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != TextFieldMotion::default();
    let inline_style = StoredValue::new(Some(motion::motion_style_vars(motion)));
    let agent_contract = logic::text_field_agent_contract();
    let contract_for_active = contract.clone();
    let is_active = Signal::derive(move || {
        contract_for_active.state.is_focus_visible.get() || is_invalid.get()
    });
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, is_active, motion);
    let description_view = render_description(description, aria.description.id.clone());
    let error_view = render_error(error, aria.error.id.clone(), is_invalid);

    view! {
        <div
            node_ref=root_ref
            class=class
            style=inline_style.get_value().unwrap_or_default()
            lang=move || contract.attrs.lang.clone()
            dir=move || contract.attrs.dir
            class:ui-text-field--focus-visible=move || contract.state.is_focus_visible.get()
            class:ui-text-field--invalid=move || is_invalid.get()
            class:ui-text-field--disabled=is_disabled
            class:ui-text-field--custom-class=has_custom_class_name
            data-slot=SLOT_ROOT
            data-ui-schema=agent_contract.schema_attr
            data-ui-schema-version=agent_contract.schema_version_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action-model=agent_contract.action_model_attr
            data-ui-state-axis=agent_contract.state_axis_attr
            data-ui-source-axis=agent_contract.source_axis_attr
            data-motion-source=if has_custom_motion {
                MOTION_SOURCE_CUSTOM
            } else {
                MOTION_SOURCE_DEFAULT
            }
            data-custom-motion=has_custom_motion.then_some("true")
            data-value-control-mode=value_axis.control_mode_attr
            data-value-controlled=value_axis.is_controlled.then_some("true")
            data-value-uncontrolled=(!value_axis.is_controlled).then_some("true")
            data-default-value-source=value_axis.default_value_source_attr
            data-value-change-source=value_axis.value_change_source_attr
            data-has-value-change=value_axis.has_value_change_handler.then_some("true")
            data-state=move || contract.state.resolved.get().state_attr
            data-value=move || contract.state.resolved.get().value_attr
            data-requirement=move || contract.state.resolved.get().requirement_attr
            data-label-source=label_source_attr
            data-description-source=description_source_attr
            data-error-source=error_source_attr
            data-placeholder-source=placeholder_source_attr
            data-type-source=type_source_attr
            data-class-source=class_source_attr
            data-custom-class=has_custom_class_name.then_some("true")
            data-focused=move || contract.state.is_focused.get().then_some("true")
            data-focus-visible=move || contract.state.is_focus_visible.get().then_some("true")
            data-invalid=move || is_invalid.get().then_some("true")
            data-disabled=is_disabled.then_some("true")
            data-read-only=is_read_only.then_some("true")
            data-required=move || is_required.get().then_some("true")
        >
            <label
                class=CLASS_LABEL
                for=aria.label.for_attr.clone()
                data-slot=SLOT_LABEL
            >
                {label}
            </label>

            <input
                class=CLASS_INPUT
                data-slot=SLOT_INPUT
                node_ref=node_ref
                id=aria.input.id.clone()
                type=input_type
                placeholder=placeholder
                prop:value=move || value.get()
                disabled=is_disabled
                readonly=is_read_only
                required=move || is_required.get()
                aria-describedby=move || aria.input.aria_describedby.get()
                aria-invalid=move || aria.input.aria_invalid.get()
                aria-required=move || aria.input.aria_required.get()
                on:input=move |ev| contract.handlers.on_input.run(event_target_value(&ev))
                on:focus=move |_| contract.handlers.focus_ring.on_focus.run(())
                on:blur=move |_| contract.handlers.focus_ring.on_blur.run(())
            />

            {description_view}
            {error_view}
        </div>
    }
}
