use super::{
    FormFieldIndicatorPlacement, FormFieldIndicatorVariant, FormFieldStateInput, FormFieldTone,
    logic,
};
use leptos::prelude::*;
use ui_checkbox::Checkbox;
use ui_headless::{
    A11yDirection, OnPress, SwitchOptions, locale_attrs, use_controllable_state, use_switch,
    use_ui_id_provider,
};

fn render_switch_view(
    checked: ReadSignal<bool>,
    on_checked_change: Callback<bool>,
    disabled: bool,
    aria_label: String,
    class_name: String,
    label: String,
) -> impl IntoView {
    let toggle: OnPress = Callback::new(move |_| {
        let next = !checked.get_untracked();
        on_checked_change.run(next);
    });

    let aria = use_switch(SwitchOptions {
        is_disabled: disabled,
        is_checked: checked,
        on_press: Some(toggle),
        lang: None,
        dir: None,
    });

    let class = if class_name.trim().is_empty() {
        "ui-switch".to_owned()
    } else {
        format!("ui-switch {class_name}")
    };

    view! {
        <button
            type="button"
            class=class
            disabled=disabled
            data-slot="switch"
            data-state=move || aria.state.resolved.get().data_state()
            data-checked=move || aria.state.resolved.get().is_checked.then_some("true")
            data-unchecked=move || aria.state.resolved.get().is_unchecked.then_some("true")
            data-disabled=move || aria.state.resolved.get().is_disabled.then_some("true")
            data-enabled=move || aria.state.resolved.get().is_enabled.then_some("true")
            data-pressed=move || aria.state.resolved.get().is_pressed.then_some("true")
            data-hovered=move || aria.state.resolved.get().is_hovered.then_some("true")
            data-focused=move || aria.state.resolved.get().is_focused.then_some("true")
            data-focus-visible=move || aria.state.resolved.get().is_focus_visible.then_some("true")
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-checked=move || aria.attrs.aria_checked.get()
            aria-label=aria_label
            lang=move || aria.attrs.lang.clone()
            dir=move || aria.attrs.dir
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
            on:pointerenter=move |_| aria.handlers.hover.on_pointer_enter.run(())
            on:pointerleave=move |_| aria.handlers.hover.on_pointer_leave.run(())
            on:click=move |_| aria.handlers.press.on_click.run(())
            on:keydown=move |ev| {
                let key = ev.key();
                if aria.handlers.press.on_key_down.run(key) {
                    ev.prevent_default();
                }
            }
            on:keyup=move |ev| {
                let key = ev.key();
                if aria.handlers.press.on_key_up.run(key) {
                    ev.prevent_default();
                }
            }
            on:focus=move |_| aria.handlers.focus_ring.on_focus.run(())
            on:blur=move |_| {
                aria.handlers.press.on_blur.run(());
                aria.handlers.focus_ring.on_blur.run(());
            }
        >
            <span class="ui-switch__track" data-slot="switch-track">
                <span class="ui-switch__thumb" data-slot="switch-thumb"></span>
            </span>
            <span class="ui-switch__label" data-slot="switch-label">
                {label}
            </span>
        </button>
    }
}

struct IndicatorRenderInput {
    indicator_variant: FormFieldIndicatorVariant,
    selected: ReadSignal<bool>,
    on_selected_change: Callback<bool>,
    is_disabled: bool,
    checkbox_variant: ui_checkbox::CheckboxVariant,
    control_aria_label: StoredValue<String>,
    control_class: StoredValue<String>,
    label: StoredValue<String>,
}

fn render_indicator_view(input: IndicatorRenderInput) -> impl IntoView {
    let IndicatorRenderInput {
        indicator_variant,
        selected,
        on_selected_change,
        is_disabled,
        checkbox_variant,
        control_aria_label,
        control_class,
        label,
    } = input;

    view! {
        <div class="ui-form-field__indicator" data-slot="form-field-indicator">
            {match indicator_variant {
                FormFieldIndicatorVariant::Switch => {
                    view! {
                        {render_switch_view(
                            selected,
                            on_selected_change,
                            is_disabled,
                            control_aria_label.get_value(),
                            control_class.get_value(),
                            label.get_value(),
                        )}
                    }
                    .into_any()
                }
                FormFieldIndicatorVariant::Checkbox => {
                    view! {
                        <Checkbox
                            checked=selected
                            on_change=on_selected_change
                            is_disabled=is_disabled
                            variant=checkbox_variant
                            aria_label=control_aria_label.get_value()
                            class_name=control_class.get_value()
                        >
                            {move || label.get_value()}
                        </Checkbox>
                    }
                    .into_any()
                }
            }}
        </div>
    }
}

fn render_content_view(
    label: StoredValue<String>,
    description: StoredValue<Option<String>>,
    error_message: StoredValue<Option<String>>,
    description_id: Memo<String>,
    error_id: Memo<String>,
    is_description_visible: Signal<bool>,
    is_error_visible: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class="ui-form-field__content" data-slot="form-field-content">
            <p class="ui-form-field__label" data-slot="form-field-label">
                {move || label.get_value()}
            </p>

            <Show when=move || is_description_visible.get()>
                <p
                    id=move || description_id.get()
                    class="ui-form-field__description"
                    data-slot="form-field-description"
                >
                    {move || description.get_value().unwrap_or_default()}
                </p>
            </Show>

            <Show when=move || is_error_visible.get()>
                <p
                    id=move || error_id.get()
                    class="ui-form-field__error"
                    data-slot="form-field-error"
                    role="alert"
                >
                    {move || error_message.get_value().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}

#[component]
pub fn FormField(
    #[prop(optional, into)] is_selected: Option<Signal<bool>>,
    #[prop(optional)] default_selected: Option<bool>,
    #[prop(optional)] on_selected_change: Option<Callback<bool>>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_invalid: bool,
    #[prop(optional)] tone: FormFieldTone,
    #[prop(optional)] indicator_variant: FormFieldIndicatorVariant,
    #[prop(optional)] indicator_placement: FormFieldIndicatorPlacement,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error_message: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let selected_axis = logic::normalize_selected_axis(logic::FormFieldSelectedAxisInput {
        is_selected,
        default_selected,
        on_selected_change,
    });
    let is_controlled_selected = selected_axis.is_controlled;
    let selected_control_mode_attr = selected_axis.control_mode_attr;
    let default_selected_source_attr = selected_axis.default_selected_source_attr;
    let selected_change_source_attr = selected_axis.selected_change_source_attr;
    let selected_state = use_controllable_state(
        selected_axis.controlled_selected,
        Some(selected_axis.default_selected),
        selected_axis.on_selected_change,
    );
    let (selected, set_selected) = signal(selected_state.value.get_untracked());
    Effect::new(move |_| {
        set_selected.set(selected_state.value.get());
    });
    let request_selected_change = StoredValue::new(selected_state.request_change);

    let id_base = id_base.or_else(|| {
        use_ui_id_provider().map(|id_provider| id_provider.next_prefixed_id(logic::DEFAULT_ID_BASE))
    });
    let id_base = StoredValue::new(logic::normalize_id_base(id_base));

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let description = logic::normalize_optional_text(description);
    let has_description = description.is_some();
    let description = StoredValue::new(description);

    let (error_message, has_custom_error_message) =
        logic::normalize_error_message(error_message, is_invalid);
    let has_error_message = error_message.is_some();
    let error_message = StoredValue::new(error_message);

    let (control_aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, &label.get_value());
    let control_aria_label = StoredValue::new(control_aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(FormFieldStateInput {
            is_selected: selected.get(),
            is_disabled,
            is_invalid,
            tone,
            indicator_variant,
            indicator_placement,
            has_description,
            has_error_message,
            has_custom_label,
            has_custom_aria_label,
            has_custom_error_message,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let description_id = Memo::new(move |_| format!("{}-description", id_base.get_value()));
    let error_id = Memo::new(move |_| format!("{}-error", id_base.get_value()));

    let describedby = Signal::derive(move || {
        let state = state.get();
        logic::compose_describedby(
            state.has_description,
            state.shows_error,
            description_id.get(),
            error_id.get(),
        )
    });

    let control_class = StoredValue::new("ui-form-field__control".into());
    let checkbox_variant = logic::resolve_checkbox_variant(is_invalid);
    let locale = locale_attrs(lang, dir);
    let is_description_visible =
        Signal::derive(move || state.get().message_kind_attr == "description");
    let is_error_visible = Signal::derive(move || state.get().message_kind_attr == "error");
    let agent_contract = Memo::new(move |_| {
        logic::resolve_agent_contract_attrs(state.get(), selected_control_mode_attr)
    });

    view! {
        <div
            id=move || id_base.get_value()
            class=move || class.get()
            data-slot="form-field"
            data-state=move || state.get().state_attr
            data-tone=move || state.get().tone_attr
            data-indicator-variant=move || state.get().indicator_variant_attr
            data-indicator-placement=move || state.get().indicator_placement_attr
            data-selected=move || state.get().is_selected.then_some("true")
            data-unselected=move || state.get().is_unselected.then_some("true")
            data-selected-control-mode=selected_control_mode_attr
            data-selected-controlled=is_controlled_selected.then_some("true")
            data-selected-uncontrolled=(!is_controlled_selected).then_some("true")
            data-default-selected-source=default_selected_source_attr
            data-selected-change-source=selected_change_source_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-message-kind=move || state.get().message_kind_attr
            data-label-source=move || state.get().label_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-error-source=move || state.get().error_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version
            data-ui-intent=move || agent_contract.get().intent_attr
            data-ui-action=move || agent_contract.get().action_attr
            data-ui-state=move || agent_contract.get().state_attr
            data-ui-source=move || agent_contract.get().source_attr
            data-ui-stream-support=move || agent_contract.get().stream_support_attr
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr
            data-ui-output-status=move || agent_contract.get().output_status_attr
            role="group"
            aria-label=move || control_aria_label.get_value()
            aria-describedby=move || describedby.get()
            aria-disabled=move || state.get().is_disabled.then_some("true")
            aria-invalid=move || state.get().is_invalid.then_some("true")
            lang=locale.lang.clone()
            dir=locale.dir
        >
            <Show when=move || state.get().indicator_placement == FormFieldIndicatorPlacement::Start>
                {render_indicator_view(IndicatorRenderInput {
                    indicator_variant,
                    selected,
                    on_selected_change: request_selected_change.get_value(),
                    is_disabled,
                    checkbox_variant,
                    control_aria_label,
                    control_class,
                    label,
                })}
            </Show>

            {render_content_view(
                label,
                description,
                error_message,
                description_id,
                error_id,
                is_description_visible,
                is_error_visible,
            )}

            <Show when=move || state.get().indicator_placement == FormFieldIndicatorPlacement::End>
                {render_indicator_view(IndicatorRenderInput {
                    indicator_variant,
                    selected,
                    on_selected_change: request_selected_change.get_value(),
                    is_disabled,
                    checkbox_variant,
                    control_aria_label,
                    control_class,
                    label,
                })}
            </Show>
        </div>
    }
}
