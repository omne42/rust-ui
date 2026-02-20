use super::{
    TextareaMotion,
    logic::{self},
    motion,
};
use leptos::{html, prelude::*};
use ui_headless::{
    A11yDirection, CommonStrings, FocusRingOptions, TextFieldOptions, locale_attrs,
    use_controllable_state, use_focus_ring, use_text_field, use_ui_i18n,
};

fn render_description_block(description: Option<String>, description_id: String) -> AnyView {
    match description {
        Some(description) => view! {
            <div
                class="ui-textarea__description"
                id=description_id
                data-slot="textarea-description"
            >
                {description}
            </div>
        }
        .into_any(),
        None => ().into_any(),
    }
}

fn render_error_block(
    error: Option<String>,
    error_id: String,
    is_invalid: Signal<bool>,
) -> AnyView {
    match error {
        Some(error) => {
            let error_id = StoredValue::new(error_id);
            let error = StoredValue::new(error);
            view! {
                <Show when=move || is_invalid.get()>
                    <div
                        class="ui-textarea__error"
                        id=move || error_id.get_value()
                        data-slot="textarea-error"
                    >
                        {move || error.get_value()}
                    </div>
                </Show>
            }
            .into_any()
        }
        None => ().into_any(),
    }
}

#[component]
pub fn Textarea(
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
    #[prop(optional)] rows: Option<u32>,
    #[prop(optional)] motion: TextareaMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] node_ref: NodeRef<html::Textarea>,
) -> impl IntoView {
    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let value_axis = logic::normalize_value_axis(logic::ValueAxisInput {
        has_controlled_value: value.is_some(),
        default_value,
        has_on_value_change: on_value_change.is_some(),
    });
    let value_state = use_controllable_state(
        value,
        Some(value_axis.default_value.clone()),
        on_value_change,
    );
    let value = value_state.value;
    let request_value_change = value_state.request_change;

    let accessibility_state =
        logic::normalize_accessibility_state(logic::AccessibilityStateInput {
            is_disabled,
            is_read_only,
        });
    let is_disabled = accessibility_state.is_disabled;
    let is_read_only = accessibility_state.is_read_only;
    let is_required_input = is_required;
    let is_invalid_input = is_invalid;
    let is_required = Signal::derive(move || match is_required_input {
        Some(required) => required.get(),
        None => false,
    });
    let is_invalid = Signal::derive(move || match is_invalid_input {
        Some(invalid) => invalid.get(),
        None => false,
    });

    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });

    let (label, has_custom_label) =
        logic::resolve_label_with_fallback(label, common.textarea_label.as_ref());

    let description = logic::normalize_optional_text(description);
    let has_custom_description = description.is_some();

    let error = logic::normalize_optional_text(error);
    let has_custom_error = error.is_some();

    let placeholder = logic::normalize_optional_text(placeholder);
    let has_custom_placeholder = placeholder.is_some();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let rows = rows.filter(|rows| *rows > 0);
    let has_custom_rows = rows.is_some();

    let aria = use_text_field(TextFieldOptions {
        id: id.clone(),
        has_description: description.is_some(),
        has_error: error.is_some(),
        aria_describedby,
        is_invalid,
        is_required,
    });
    let description_view = render_description_block(description, aria.description.id.clone());
    let error_view = render_error_block(error, aria.error.id.clone(), is_invalid);

    let state = Signal::derive(move || {
        logic::resolve_state(logic::TextareaStateInput {
            disabled: is_disabled,
            read_only: is_read_only,
            required: is_required.get(),
            invalid: is_invalid.get(),
            has_value: !value.get().is_empty(),
            has_custom_label,
            has_custom_description,
            has_custom_error,
            has_custom_placeholder,
            has_custom_rows,
            has_custom_class_name,
        })
    });

    let class = Signal::derive(move || logic::compose_class_name(class_name.clone(), state.get()));
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != TextareaMotion::default();
    let inline_style = StoredValue::new(Some(motion::motion_style_vars(motion)));
    let is_active = Signal::derive(move || focus_ring.is_focus_visible.get() || is_invalid.get());
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, is_active, motion);

    let locale = locale_attrs(lang, dir);

    view! {
        <div
            node_ref=root_ref
            class=move || class.get()
            style=inline_style.get_value().unwrap_or_default()
            lang=locale.lang.clone()
            dir=locale.dir
            class:ui-textarea--focus-visible=move || focus_ring.is_focus_visible.get()
            class:ui-textarea--invalid=move || is_invalid.get()
            class:ui-textarea--disabled=is_disabled
            data-slot="textarea"
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-state=move || state.get().state_attr.as_str()
            data-value=move || state.get().value_attr.as_str()
            data-requirement=move || state.get().requirement_attr.as_str()
            data-label-source=move || state.get().label_source_attr.as_str()
            data-description-source=move || state.get().description_source_attr.as_str()
            data-error-source=move || state.get().error_source_attr.as_str()
            data-placeholder-source=move || state.get().placeholder_source_attr.as_str()
            data-rows-source=move || state.get().rows_source_attr.as_str()
            data-class-source=move || state.get().class_source_attr.as_str()
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-focused=move || focus_ring.is_focused.get().then_some("true")
            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
            data-invalid=move || is_invalid.get().then_some("true")
            data-value-control-mode=value_axis.control_mode_attr.as_str()
            data-value-controlled=value_axis.is_controlled.then_some("true")
            data-value-uncontrolled=(!value_axis.is_controlled).then_some("true")
            data-default-value-source=value_axis.default_value_source_attr.as_str()
            data-value-change-source=value_axis.value_change_source_attr.as_str()
            data-has-value-change=value_axis.has_value_change_handler.then_some("true")
            data-disabled=is_disabled.then_some("true")
            data-read-only=is_read_only.then_some("true")
            data-required=move || is_required.get().then_some("true")
        >
            <label
                class="ui-textarea__label"
                for=aria.label.for_attr.clone()
                data-slot="textarea-label"
            >
                {label}
            </label>

            <textarea
                class="ui-textarea__textarea"
                data-slot="textarea-input"
                node_ref=node_ref
                id=aria.input.id.clone()
                rows=rows
                placeholder=placeholder
                prop:value=move || value.get()
                disabled=is_disabled
                readonly=is_read_only
                required=move || is_required.get()
                aria-describedby=move || aria.input.aria_describedby.get()
                aria-invalid=move || aria.input.aria_invalid.get()
                aria-required=move || aria.input.aria_required.get()
                on:input=move |ev| request_value_change.run(event_target_value(&ev))
                on:focus=move |_| focus_ring.handlers.on_focus.run(())
                on:blur=move |_| focus_ring.handlers.on_blur.run(())
            ></textarea>

            {description_view}
            {error_view}
        </div>
    }
}
