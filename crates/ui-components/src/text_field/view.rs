use leptos::{html, prelude::*};
use ui_headless::{FocusRingOptions, TextFieldOptions, use_focus_ring, use_text_field};

const DEFAULT_LABEL: &str = "Text field";

#[component]
pub fn TextField(
    id: String,
    label: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] read_only: bool,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] input_type: Option<&'static str>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });

    let (label, label_source_attr) = {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            (DEFAULT_LABEL.to_string(), "default")
        } else {
            (trimmed.to_string(), "custom")
        }
    };

    let description = description.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    });
    let error = error.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    });
    let placeholder = placeholder.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    });

    let class_name = class_name.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    });
    let has_custom_class_name = class_name.is_some();

    let base_class = "ui-text-field".to_string();
    let class = class_name
        .as_ref()
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let (input_type, type_source_attr) =
        match input_type.map(str::trim).filter(|value| !value.is_empty()) {
            Some("text") => ("text", "default"),
            Some(value) => (value, "custom"),
            None => ("text", "default"),
        };

    let description_source_attr = if description.is_some() {
        "custom"
    } else {
        "default"
    };
    let error_source_attr = if error.is_some() { "custom" } else { "default" };
    let placeholder_source_attr = if placeholder.is_some() {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let aria = use_text_field(TextFieldOptions {
        id: id.clone(),
        has_description: description.is_some(),
        has_error: error.is_some(),
        aria_describedby,
        is_invalid: invalid,
        is_required: required,
    });

    let data_state = Signal::derive(move || {
        if disabled {
            "disabled"
        } else if invalid.get() {
            "invalid"
        } else if read_only {
            "readonly"
        } else {
            "ready"
        }
    });

    let data_value = Signal::derive(move || {
        if value.get().trim().is_empty() {
            "empty"
        } else {
            "filled"
        }
    });

    let data_requirement = Signal::derive(move || {
        if required.get() {
            "required"
        } else {
            "optional"
        }
    });

    view! {
        <div
            class=class
            class:ui-text-field--focus-visible=move || focus_ring.is_focus_visible.get()
            class:ui-text-field--invalid=move || invalid.get()
            class:ui-text-field--disabled=disabled
            class:ui-text-field--custom-class=has_custom_class_name
            data-slot="text-field"
            data-state=move || data_state.get()
            data-value=move || data_value.get()
            data-requirement=move || data_requirement.get()
            data-label-source=label_source_attr
            data-description-source=description_source_attr
            data-error-source=error_source_attr
            data-placeholder-source=placeholder_source_attr
            data-type-source=type_source_attr
            data-class-source=class_source_attr
            data-custom-class=has_custom_class_name.then_some("true")
            data-focused=move || focus_ring.is_focused.get().then_some("true")
            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
            data-invalid=move || invalid.get().then_some("true")
            data-disabled=disabled.then_some("true")
            data-read-only=read_only.then_some("true")
            data-required=move || required.get().then_some("true")
        >
            <label
                class="ui-text-field__label"
                for=aria.label.for_attr.clone()
                data-slot="text-field-label"
            >
                {label}
            </label>

            <input
                class="ui-text-field__input"
                data-slot="text-field-input"
                node_ref=node_ref
                id=aria.input.id.clone()
                type=input_type
                placeholder=placeholder
                prop:value=move || value.get()
                disabled=disabled
                readonly=read_only
                required=move || required.get()
                aria-describedby=move || aria.input.aria_describedby.get()
                aria-invalid=move || aria.input.aria_invalid.get()
                aria-required=move || aria.input.aria_required.get()
                on:input=move |ev| set_value.set(event_target_value(&ev))
                on:focus=move |_| focus_ring.handlers.on_focus.run(())
                on:blur=move |_| focus_ring.handlers.on_blur.run(())
            />

            {description.map(|description| {
                let description_id = aria.description.id.clone();
                view! {
                    <div
                        class="ui-text-field__description"
                        id=description_id
                        data-slot="text-field-description"
                    >
                        {description}
                    </div>
                }
            })}

            {error.map(|error| {
                let error_id = aria.error.id.clone();
                let error_id = StoredValue::new(error_id);
                let error = StoredValue::new(error);
                view! {
                    <Show when=move || invalid.get()>
                        <div
                            class="ui-text-field__error"
                            id=move || error_id.get_value()
                            data-slot="text-field-error"
                        >
                            {move || error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </div>
    }
}
