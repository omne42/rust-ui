pub use crate::button::normalize_optional_text;

pub const DEFAULT_LABEL: &str = "Text field";
pub const DEFAULT_INPUT_TYPE: &str = "text";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextFieldStateInput<'a> {
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub value: &'a str,
    pub is_required: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextFieldState {
    pub state_attr: &'static str,
    pub value_attr: &'static str,
    pub requirement_attr: &'static str,
}

pub fn resolve_label(label: String) -> (String, &'static str) {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        (DEFAULT_LABEL.into(), "default")
    } else {
        (trimmed.into(), "custom")
    }
}

pub fn resolve_input_type(input_type: Option<&'static str>) -> (&'static str, &'static str) {
    match input_type.map(str::trim).filter(|value| !value.is_empty()) {
        Some(DEFAULT_INPUT_TYPE) => (DEFAULT_INPUT_TYPE, "default"),
        Some(value) => (value, "custom"),
        None => (DEFAULT_INPUT_TYPE, "default"),
    }
}

pub fn source_attr_from_presence(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: TextFieldStateInput<'_>) -> TextFieldState {
    TextFieldState {
        state_attr: resolve_state_attr(input.is_disabled, input.is_invalid, input.is_read_only),
        value_attr: resolve_value_attr(input.value),
        requirement_attr: resolve_requirement_attr(input.is_required),
    }
}

pub fn resolve_state_attr(is_disabled: bool, is_invalid: bool, is_read_only: bool) -> &'static str {
    if is_disabled {
        "disabled"
    } else if is_invalid {
        "invalid"
    } else if is_read_only {
        "readonly"
    } else {
        "ready"
    }
}

pub fn resolve_value_attr(value: &str) -> &'static str {
    if value.trim().is_empty() {
        "empty"
    } else {
        "filled"
    }
}

pub fn resolve_requirement_attr(is_required: bool) -> &'static str {
    if is_required { "required" } else { "optional" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("".to_string())), None);
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  hello  ".to_string())),
            Some("hello".to_string())
        );
    }

    #[test]
    fn resolve_label_falls_back_to_default_when_blank() {
        assert_eq!(
            resolve_label("".to_string()),
            (DEFAULT_LABEL.into(), "default")
        );
        assert_eq!(
            resolve_label("   ".to_string()),
            (DEFAULT_LABEL.into(), "default")
        );
        assert_eq!(
            resolve_label(" Name ".to_string()),
            ("Name".to_string(), "custom")
        );
    }

    #[test]
    fn resolve_input_type_tracks_default_and_custom() {
        assert_eq!(resolve_input_type(None), (DEFAULT_INPUT_TYPE, "default"));
        assert_eq!(
            resolve_input_type(Some("")),
            (DEFAULT_INPUT_TYPE, "default")
        );
        assert_eq!(
            resolve_input_type(Some("  ")),
            (DEFAULT_INPUT_TYPE, "default")
        );
        assert_eq!(
            resolve_input_type(Some("text")),
            (DEFAULT_INPUT_TYPE, "default")
        );
        assert_eq!(resolve_input_type(Some("email")), ("email", "custom"));
        assert_eq!(resolve_input_type(Some("  email  ")), ("email", "custom"));
    }

    #[test]
    fn source_attr_from_presence_tracks_custom_flag() {
        assert_eq!(source_attr_from_presence(false), "default");
        assert_eq!(source_attr_from_presence(true), "custom");
    }

    #[test]
    fn resolve_state_derives_state_value_and_requirement_attrs() {
        let state = resolve_state(TextFieldStateInput {
            is_disabled: false,
            is_invalid: true,
            is_read_only: false,
            value: "abc",
            is_required: true,
        });

        assert_eq!(state.state_attr, "invalid");
        assert_eq!(state.value_attr, "filled");
        assert_eq!(state.requirement_attr, "required");
    }

    #[test]
    fn state_attr_prioritizes_disabled_then_invalid_then_readonly() {
        assert_eq!(resolve_state_attr(false, false, false), "ready");
        assert_eq!(resolve_state_attr(false, false, true), "readonly");
        assert_eq!(resolve_state_attr(false, true, true), "invalid");
        assert_eq!(resolve_state_attr(true, true, true), "disabled");
    }

    #[test]
    fn value_and_requirement_attrs_map_to_closed_sets() {
        assert_eq!(resolve_value_attr(""), "empty");
        assert_eq!(resolve_value_attr("   "), "empty");
        assert_eq!(resolve_value_attr("value"), "filled");
        assert_eq!(resolve_requirement_attr(false), "optional");
        assert_eq!(resolve_requirement_attr(true), "required");
    }
}
