pub use ui_state_primitives::color_field::{
    ColorFieldDerivedStateInput, ColorFieldState, ColorFieldStateInput, compose_class_name,
    is_invalid_state, normalize_aria_label, normalize_color_value, normalize_label,
    normalize_optional_text, normalize_placeholder, resolve_derived_state, sanitize_preview_color,
};

pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {
    is_disabled.or(disabled).unwrap_or(false)
}

pub fn resolve_is_preview_visible(
    is_preview_visible: Option<bool>,
    show_preview: Option<bool>,
) -> bool {
    is_preview_visible.or(show_preview).unwrap_or(true)
}

pub fn resolve_input_value(value: Option<String>) -> String {
    value.unwrap_or_default()
}

pub fn resolve_preview_color(value: Option<String>) -> Option<String> {
    sanitize_preview_color(value)
}

pub fn resolve_next_value(raw_value: String) -> Option<String> {
    normalize_color_value(Some(raw_value))
}
