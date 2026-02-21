use super::{SearchFieldMotion, logic, motion};
use crate::clear_button::{ClearButton, ClearButtonFocusMode};
use leptos::{ev, html, prelude::*};
use ui_headless::{
    A11yDirection, CommonStrings, FocusWithinOptions, HoverOptions, PressOptions,
    SearchFieldKeyDownResult, TextFieldOptions, use_controllable_state, use_focus_visible,
    use_focus_within, use_hover, use_press, use_search_field as use_search_field_contract,
    use_text_field, use_ui_i18n,
};

fn render_description(description: Option<String>, description_id: String) -> impl IntoView {
    description.map(|description| {
        view! {
            <div
                class="ui-search-field__description"
                id=description_id
                data-slot="search-field-description"
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
                    class="ui-search-field__error"
                    id=move || error_id.get_value()
                    data-slot="search-field-error"
                >
                    {move || error.get_value()}
                </div>
            </Show>
        }
    })
}

fn render_search_icon() -> impl IntoView {
    view! {
        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
            <circle cx="9" cy="9" r="6" stroke="currentColor" stroke_width="2" />
            <path
                d="M13.5 13.5l3 3"
                stroke="currentColor"
                stroke_width="2"
                stroke_linecap="round"
            />
        </svg>
    }
}

fn render_clear_icon() -> impl IntoView {
    view! {
        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
            <path
                d="M6 6l8 8M14 6l-8 8"
                stroke="currentColor"
                stroke_width="2"
                stroke_linecap="round"
            />
        </svg>
    }
}

#[cfg(target_arch = "wasm32")]
fn focus_input(input_ref: &NodeRef<html::Input>) {
    let Some(el) = input_ref.get_untracked() else {
        return;
    };
    ui_observability::observe_js_result!(el.focus());
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_input(_input_ref: &NodeRef<html::Input>) {}

#[component]
pub fn SearchField(
    id: String,
    label: String,
    #[prop(optional, into)] value: Option<Signal<String>>,
    #[prop(optional, into)] default_value: Option<String>,
    #[prop(optional)] on_value_change: Option<Callback<String>>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] is_read_only: Option<bool>,
    #[prop(optional, into)] is_required: Option<Signal<bool>>,
    #[prop(optional, into)] is_invalid: Option<Signal<bool>>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] on_submit: Option<Callback<String>>,
    #[prop(optional)] on_clear: Option<Callback<()>>,
    #[prop(optional, into)] clear_button_aria_label: Option<String>,
    #[prop(optional)] motion: SearchFieldMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();

    let value_axis = logic::normalize_value_axis(logic::ValueAxisInput {
        value,
        default_value,
        on_value_change,
    });
    let controlled_value = value_axis.value;
    let controlled_default_value = value_axis.default_value.clone();
    let controlled_on_value_change = value_axis.on_value_change;
    let value_control_mode_attr = value_axis.control_mode_attr;
    let value_is_controlled = value_axis.is_controlled;
    let default_value_source_attr = value_axis.default_value_source_attr;
    let value_change_source_attr = value_axis.value_change_source_attr;
    let has_value_change_handler = value_axis.has_value_change_handler;
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
            is_read_only,
            is_required,
            is_invalid,
        });
    let is_disabled = accessibility_state.is_disabled;
    let is_read_only = accessibility_state.is_read_only;
    let is_required = accessibility_state.is_required;
    let is_invalid = accessibility_state.is_invalid;

    let state = logic::use_search_field(value, is_disabled, is_read_only, is_invalid, is_required);

    let focus_within = use_focus_within(FocusWithinOptions { is_disabled });
    let global_focus_visible = use_focus_visible()
        .map(|state| state.is_focus_visible())
        .unwrap_or_else(|| signal(false).0);
    let is_focus_visible =
        Memo::new(move |_| focus_within.is_focus_within.get() && global_focus_visible.get());

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

    let search_field_contract = use_search_field_contract(ui_headless::SearchFieldOptions {
        is_disabled,
        is_read_only,
        value,
        on_value_change: request_value_change,
        on_submit,
        on_clear,
        lang,
        dir,
    });

    let class = logic::resolve_root_class(class_name);
    let class_name = class.class;
    let has_custom_class_name = class.has_custom_class_name;
    let class_source_attr = class.class_source_attr;
    let agent_contract = logic::search_field_agent_contract();
    let clear_label_state = logic::resolve_clear_button_label(logic::ClearButtonLabelInput {
        aria_label: clear_button_aria_label,
        i18n_clear_aria_label: Some(common.clear_aria_label.as_ref().into()),
    });
    let clear_button_aria_label = clear_label_state.aria_label;
    let clear_label_source_attr = clear_label_state.source_attr;

    let clear_hover = use_hover(HoverOptions {
        is_disabled: is_disabled || is_read_only,
    });

    let clear_press = use_press(PressOptions {
        is_disabled: is_disabled || is_read_only,
        ..Default::default()
    });

    let is_clear_visible = Signal::derive(move || search_field_contract.state.can_clear.get());
    let input_ref: NodeRef<html::Input> = NodeRef::new();
    let clear_button_ref: NodeRef<html::Button> = NodeRef::new();
    motion::attach_clear_motion(
        clear_button_ref,
        is_clear_visible,
        clear_hover.is_hovered,
        clear_press.is_pressed,
        motion,
    );

    let on_clear = Callback::new(move |_: ()| search_field_contract.handlers.on_clear.run(()));
    let clear_pointer_down_handler = clear_press.handlers.on_pointer_down;
    let clear_pointer_up_handler = clear_press.handlers.on_pointer_up;
    let clear_pointer_cancel_handler = clear_press.handlers.on_pointer_cancel;
    let clear_click_handler = clear_press.handlers.on_click;
    let clear_key_down_handler = clear_press.handlers.on_key_down;
    let clear_key_up_handler = clear_press.handlers.on_key_up;
    let clear_blur_handler = clear_press.handlers.on_blur;
    let clear_hover_enter_handler = clear_hover.handlers.on_pointer_enter;
    let clear_hover_leave_handler = clear_hover.handlers.on_pointer_leave;
    let input_ref_for_clear = input_ref;
    let on_clear_pointer_down = Callback::new(move |ev: ev::PointerEvent| {
        ev.prevent_default();
        focus_input(&input_ref_for_clear);
        clear_pointer_down_handler.run(());
    });

    let on_input_key_down = move |ev: ev::KeyboardEvent| {
        let key = ev.key();
        match search_field_contract.handlers.on_key_down.run(key) {
            SearchFieldKeyDownResult::Ignored => {}
            SearchFieldKeyDownResult::Submitted => {
                ev.prevent_default();
            }
            SearchFieldKeyDownResult::Cleared => {
                ev.stop_propagation();
                ev.prevent_default();
            }
        }
    };

    let description_view = render_description(description, aria.description.id.clone());
    let error_view = render_error(error, aria.error.id.clone(), is_invalid);

    view! {
        <div
            class=class_name
            class:ui-search-field--focus-visible=move || is_focus_visible.get()
            class:ui-search-field--invalid=move || is_invalid.get()
            class:ui-search-field--disabled=is_disabled
            class:ui-search-field--readonly=is_read_only
            class:ui-search-field--custom-class=has_custom_class_name
            data-slot="search-field"
            data-state=move || state.semantic.get().state_attr
            data-value=move || state.semantic.get().value_attr
            data-requirement=move || state.semantic.get().requirement_attr
            data-ui-schema=agent_contract.schema_attr
            data-ui-schema-version=agent_contract.schema_version_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action-model=agent_contract.action_model_attr
            data-ui-state-axis=agent_contract.state_axis_attr
            data-ui-source-axis=agent_contract.source_axis_attr
            data-value-control-mode=value_control_mode_attr
            data-value-controlled=value_is_controlled.then_some("true")
            data-value-uncontrolled=(!value_is_controlled).then_some("true")
            data-default-value-source=default_value_source_attr
            data-value-change-source=value_change_source_attr
            data-has-value-change=has_value_change_handler.then_some("true")
            data-clear-label-source=clear_label_source_attr
            data-class-source=class_source_attr
            data-custom-class=has_custom_class_name.then_some("true")
            data-focused=move || focus_within.is_focus_within.get().then_some("true")
            data-focus-visible=move || is_focus_visible.get().then_some("true")
            data-invalid=move || is_invalid.get().then_some("true")
            data-disabled=is_disabled.then_some("true")
            data-read-only=is_read_only.then_some("true")
            data-required=move || is_required.get().then_some("true")
            data-empty=move || search_field_contract.state.is_empty.get().then_some("true")
            data-readonly=is_read_only.then_some("true")
            lang=move || search_field_contract.attrs.lang.clone()
            dir=move || search_field_contract.attrs.dir
        >
            <label
                class="ui-search-field__label"
                for=aria.label.for_attr.clone()
                data-slot="search-field-label"
            >
                {label}
            </label>

            <div
                class="ui-search-field__control"
                data-slot="search-field-control"
                on:focusin=move |_| focus_within.handlers.on_focus_in.run(())
                on:focusout=move |_| focus_within.handlers.on_focus_out.run(())
            >
                <span
                    class="ui-search-field__icon"
                    data-slot="search-field-icon"
                    aria-hidden="true"
                >
                    {render_search_icon()}
                </span>

                <input
                    class="ui-search-field__input"
                    data-slot="search-field-input"
                    node_ref=input_ref
                    id=aria.input.id.clone()
                    type="search"
                    placeholder=placeholder
                    prop:value=move || value.get()
                    disabled=is_disabled
                    readonly=is_read_only
                    required=move || is_required.get()
                    aria-describedby=move || aria.input.aria_describedby.get()
                    aria-invalid=move || aria.input.aria_invalid.get()
                    aria-required=move || aria.input.aria_required.get()
                    aria-keyshortcuts=move || search_field_contract.attrs.aria_keyshortcuts.get()
                    on:input=move |ev| {
                        search_field_contract
                            .handlers
                            .on_input
                            .run(event_target_value(&ev))
                    }
                    on:keydown=on_input_key_down
                />

                <ClearButton
                    slot_name="search-field-clear"
                    class_name="ui-search-field__clear".to_string()
                    aria_label=clear_button_aria_label
                    node_ref=clear_button_ref
                    on_press=on_clear
                    is_visible=Signal::derive(move || search_field_contract.state.can_clear.get())
                    is_disabled_signal=
                        Signal::derive(move || {
                            is_disabled || is_read_only || !search_field_contract.state.can_clear.get()
                        })
                    aria_hidden_when_invisible=true
                    focus_mode=ClearButtonFocusMode::ExcludeTab
                    on_pointer_down=on_clear_pointer_down
                    on_pointer_enter=Callback::new(move |_| clear_hover_enter_handler.run(()))
                    on_pointer_leave=Callback::new(move |_| clear_hover_leave_handler.run(()))
                    on_pointer_up=Callback::new(move |_| clear_pointer_up_handler.run(()))
                    on_pointer_cancel=Callback::new(move |_| clear_pointer_cancel_handler.run(()))
                    on_click=Callback::new(move |_| clear_click_handler.run(()))
                    on_key_down=Callback::new(move |key: String| clear_key_down_handler.run(key))
                    on_key_up=Callback::new(move |key: String| clear_key_up_handler.run(key))
                    on_blur=Callback::new(move |_| clear_blur_handler.run(()))
                >
                    {render_clear_icon()}
                </ClearButton>
            </div>

            {description_view}
            {error_view}
        </div>
    }
}
