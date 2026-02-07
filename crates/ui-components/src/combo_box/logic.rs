pub fn normalize_label(label: String) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        "Options".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_placeholder(placeholder: Option<String>) -> String {
    normalize_optional_text(placeholder).unwrap_or_else(|| "Select…".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_label_trims_and_defaults() {
        assert_eq!(normalize_label("  Language  ".to_string()), "Language");
        assert_eq!(normalize_label("   ".to_string()), "Options");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Pick one  ".to_string())),
            Some("Pick one".to_string())
        );
    }

    #[test]
    fn resolve_placeholder_uses_fallback() {
        assert_eq!(
            resolve_placeholder(Some("  Choose  ".to_string())),
            "Choose"
        );
        assert_eq!(resolve_placeholder(Some("   ".to_string())), "Select…");
        assert_eq!(resolve_placeholder(None), "Select…");
    }
}
