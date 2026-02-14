pub const DEFAULT_LABEL: &str = "Text field";

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_label(label: String) -> (String, &'static str) {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        (DEFAULT_LABEL.to_string(), "default")
    } else {
        (trimmed.to_string(), "custom")
    }
}

pub fn resolve_input_type(input_type: Option<&'static str>) -> (&'static str, &'static str) {
    match input_type.map(str::trim).filter(|value| !value.is_empty()) {
        Some("text") => ("text", "default"),
        Some(value) => (value, "custom"),
        None => ("text", "default"),
    }
}

pub fn source_attr_from_presence(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
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
            (DEFAULT_LABEL.to_string(), "default")
        );
        assert_eq!(
            resolve_label("   ".to_string()),
            (DEFAULT_LABEL.to_string(), "default")
        );
        assert_eq!(
            resolve_label(" Name ".to_string()),
            ("Name".to_string(), "custom")
        );
    }

    #[test]
    fn resolve_input_type_tracks_default_and_custom() {
        assert_eq!(resolve_input_type(None), ("text", "default"));
        assert_eq!(resolve_input_type(Some("")), ("text", "default"));
        assert_eq!(resolve_input_type(Some("  ")), ("text", "default"));
        assert_eq!(resolve_input_type(Some("text")), ("text", "default"));
        assert_eq!(resolve_input_type(Some("email")), ("email", "custom"));
        assert_eq!(resolve_input_type(Some("  email  ")), ("email", "custom"));
    }

    #[test]
    fn source_attr_from_presence_tracks_custom_flag() {
        assert_eq!(source_attr_from_presence(false), "default");
        assert_eq!(source_attr_from_presence(true), "custom");
    }
}
