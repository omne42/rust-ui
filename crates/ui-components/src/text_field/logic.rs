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
