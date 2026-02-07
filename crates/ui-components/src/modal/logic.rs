#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModalStateInput {
    pub has_description: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModalState {
    pub show_description: bool,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub description_attr: &'static str,
    pub has_custom_class_name: bool,
}

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
        "ui-modal".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn resolve_state(input: ModalStateInput) -> ModalState {
    let (state_class, state_attr, description_attr) = if input.has_description {
        ("ui-modal--with-description", "with-description", "present")
    } else {
        ("ui-modal--title-only", "title-only", "absent")
    };

    ModalState {
        show_description: input.has_description,
        state_class,
        state_attr,
        description_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ModalState) -> String {
    let mut classes = vec!["ui-modal".to_string(), state.state_class.to_string()];

    if state.has_custom_class_name {
        classes.push("ui-modal--custom-class".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-modal ".to_string())),
            Some("docs-modal".to_string())
        );
    }

    #[test]
    fn normalize_required_text_falls_back_for_blank_values() {
        assert_eq!(
            normalize_required_text(" Confirm ".to_string(), "Modal"),
            "Confirm"
        );
        assert_eq!(normalize_required_text(" ".to_string(), "Modal"), "Modal");
    }

    #[test]
    fn normalize_id_base_uses_default_for_blank_values() {
        assert_eq!(normalize_id_base(" docs-modal ".to_string()), "docs-modal");
        assert_eq!(normalize_id_base("  ".to_string()), "ui-modal");
    }

    #[test]
    fn resolve_state_tracks_description_and_custom_class_flags() {
        let state = resolve_state(ModalStateInput {
            has_description: true,
            has_custom_class_name: true,
        });

        assert!(state.show_description);
        assert_eq!(state.state_class, "ui-modal--with-description");
        assert_eq!(state.state_attr, "with-description");
        assert_eq!(state.description_attr, "present");
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-modal".to_string()),
            resolve_state(ModalStateInput {
                has_description: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-modal",
            "ui-modal--title-only",
            "ui-modal--custom-class",
            "docs-modal",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
