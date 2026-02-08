use crate::tray::{TrayState, TrayStateInput};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_required_text(value: String, fallback: &'static str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn normalize_id_base(value: String) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        "ui-tray".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn resolve_state(input: TrayStateInput) -> TrayState {
    let (state_class, state_attr, description_attr) = if input.has_description {
        ("ui-tray--with-description", "with-description", "present")
    } else {
        ("ui-tray--title-only", "title-only", "absent")
    };

    let (close_button_class, close_button_attr) = if input.show_close_button {
        ("ui-tray--close-shown", "shown")
    } else {
        ("ui-tray--close-hidden", "hidden")
    };

    let (size_class, size_attr) = if input.is_fixed_height {
        ("ui-tray--fixed-height", "fixed")
    } else {
        ("ui-tray--auto-height", "auto")
    };

    TrayState {
        show_description: input.has_description,
        description_attr,
        show_footer: input.has_footer,
        footer_attr: if input.has_footer {
            "present"
        } else {
            "absent"
        },
        show_close_button: input.show_close_button,
        close_button_class,
        close_button_attr,
        is_fixed_height: input.is_fixed_height,
        size_class,
        size_attr,
        state_class,
        state_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: TrayState) -> String {
    let mut classes = vec![
        "ui-tray".to_string(),
        state.state_class.to_string(),
        state.close_button_class.to_string(),
        state.size_class.to_string(),
    ];

    if state.show_footer {
        classes.push("ui-tray--with-footer".to_string());
    } else {
        classes.push("ui-tray--no-footer".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-tray--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-tray ".to_string())),
            Some("docs-tray".to_string())
        );

        assert_eq!(
            normalize_required_text(" Tray ".to_string(), "Tray"),
            "Tray"
        );
        assert_eq!(normalize_required_text(" ".to_string(), "Tray"), "Tray");

        assert_eq!(normalize_id_base(" docs-tray ".to_string()), "docs-tray");
        assert_eq!(normalize_id_base(" ".to_string()), "ui-tray");
    }

    #[test]
    fn resolve_state_tracks_description_footer_close_and_size() {
        let state = resolve_state(TrayStateInput {
            has_description: true,
            has_footer: false,
            show_close_button: true,
            is_fixed_height: true,
            has_custom_class_name: true,
        });

        assert!(state.show_description);
        assert_eq!(state.state_class, "ui-tray--with-description");
        assert_eq!(state.state_attr, "with-description");
        assert_eq!(state.description_attr, "present");

        assert!(!state.show_footer);
        assert_eq!(state.footer_attr, "absent");

        assert!(state.show_close_button);
        assert_eq!(state.close_button_class, "ui-tray--close-shown");
        assert_eq!(state.close_button_attr, "shown");

        assert!(state.is_fixed_height);
        assert_eq!(state.size_class, "ui-tray--fixed-height");
        assert_eq!(state.size_attr, "fixed");
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(TrayStateInput {
            has_description: false,
            has_footer: true,
            show_close_button: false,
            is_fixed_height: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-tray".to_string()), state);
        assert!(class_name.contains("ui-tray"));
        assert!(class_name.contains("ui-tray--title-only"));
        assert!(class_name.contains("ui-tray--with-footer"));
        assert!(class_name.contains("ui-tray--close-hidden"));
        assert!(class_name.contains("ui-tray--auto-height"));
        assert!(class_name.contains("ui-tray--custom-class"));
        assert!(class_name.contains("docs-tray"));
    }
}
