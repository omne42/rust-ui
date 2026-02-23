use leptos::prelude::*;

pub use ui_state_primitives::text_area::{
    TextAreaAccessibilityStateInput as PrimitiveAccessibilityStateInput, TextAreaState,
    TextAreaStateInput, TextAreaValueAxisInput as PrimitiveValueAxisInput,
    normalize_default_value as primitive_normalize_default_value, normalize_optional_text,
    resolve_accessibility_state as primitive_resolve_accessibility_state,
    resolve_label_with_fallback, resolve_state,
    resolve_value_axis_state as primitive_resolve_value_axis_state,
};

pub struct ValueAxisInput {
    pub value: Option<Signal<String>>,
    pub default_value: Option<String>,
    pub on_value_change: Option<Callback<String>>,
}

pub struct ValueAxisState {
    pub value: Option<Signal<String>>,
    pub default_value: String,
    pub on_value_change: Option<Callback<String>>,
    pub is_controlled: bool,
    pub control_mode_attr: &'static str,
    pub default_value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
    pub has_value_change_handler: bool,
}

pub fn normalize_default_value(default_value: Option<String>) -> String {
    primitive_normalize_default_value(default_value)
}

pub fn normalize_on_value_change_handler(
    on_value_change: Option<Callback<String>>,
) -> Option<Callback<String>> {
    on_value_change
}

pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState {
    let has_default_value = input.default_value.is_some();
    let default_value = normalize_default_value(input.default_value);
    let on_value_change = normalize_on_value_change_handler(input.on_value_change);
    let markers = primitive_resolve_value_axis_state(PrimitiveValueAxisInput {
        is_controlled: input.value.is_some(),
        has_default_value,
        has_on_value_change: on_value_change.is_some(),
    });

    ValueAxisState {
        value: input.value,
        default_value,
        on_value_change,
        is_controlled: markers.is_controlled,
        control_mode_attr: markers.control_mode_attr,
        default_value_source_attr: markers.default_value_source_attr,
        value_change_source_attr: markers.value_change_source_attr,
        has_value_change_handler: markers.has_value_change_handler,
    }
}

pub struct AccessibilityStateInput {
    pub is_disabled: Option<bool>,
    pub is_read_only: Option<bool>,
    pub is_required: Option<Signal<bool>>,
    pub is_invalid: Option<Signal<bool>>,
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
}

pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState {
    let primitive = primitive_resolve_accessibility_state(PrimitiveAccessibilityStateInput {
        is_disabled: input.is_disabled,
        is_read_only: input.is_read_only,
    });
    let is_required = input
        .is_required
        .unwrap_or_else(|| Signal::derive(|| false));
    let is_invalid = input.is_invalid.unwrap_or_else(|| Signal::derive(|| false));

    AccessibilityState {
        is_disabled: primitive.is_disabled,
        is_read_only: primitive.is_read_only,
        is_required,
        is_invalid,
    }
}

pub struct ResolvedTextAreaPropsInput {
    pub label: String,
    pub fallback_label: String,
    pub description: Option<String>,
    pub error: Option<String>,
    pub placeholder: Option<String>,
    pub rows: Option<u32>,
    pub class_name: Option<String>,
}

pub struct ResolvedTextAreaProps {
    pub label: String,
    pub has_custom_label: bool,
    pub description: Option<String>,
    pub has_custom_description: bool,
    pub error: Option<String>,
    pub has_custom_error: bool,
    pub placeholder: Option<String>,
    pub has_custom_placeholder: bool,
    pub rows: Option<u32>,
    pub has_custom_rows: bool,
    pub class_name: Option<String>,
    pub has_custom_class_name: bool,
}

pub fn resolve_props(input: ResolvedTextAreaPropsInput) -> ResolvedTextAreaProps {
    let (label, has_custom_label) = resolve_label_with_fallback(input.label, &input.fallback_label);
    let description = normalize_optional_text(input.description);
    let has_custom_description = description.is_some();
    let error = normalize_optional_text(input.error);
    let has_custom_error = error.is_some();
    let placeholder = normalize_optional_text(input.placeholder);
    let has_custom_placeholder = placeholder.is_some();
    let rows = input.rows.filter(|rows| *rows > 0);
    let has_custom_rows = rows.is_some();
    let class_name = normalize_optional_text(input.class_name);
    let has_custom_class_name = class_name.is_some();

    ResolvedTextAreaProps {
        label,
        has_custom_label,
        description,
        has_custom_description,
        error,
        has_custom_error,
        placeholder,
        has_custom_placeholder,
        rows,
        has_custom_rows,
        class_name,
        has_custom_class_name,
    }
}

pub fn compose_class_name(class_name: Option<String>, state: TextAreaState) -> String {
    let mut classes = vec![
        "ui-text-area".to_string(),
        format!("ui-text-area--state-{}", state.state_attr),
        format!("ui-text-area--value-{}", state.value_attr),
        format!("ui-text-area--requirement-{}", state.requirement_attr),
    ];

    if state.has_custom_class_name {
        classes.push("ui-text-area--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/text_area/logic.rs"]
mod tests;
