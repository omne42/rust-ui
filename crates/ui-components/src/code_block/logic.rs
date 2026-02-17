pub use ui_state_primitives::code_block::{
    CodeBlockStateInput, CodeBlockViewState, normalize_optional_text, resolve_state,
    resolve_view_state,
};

pub fn compose_class_name(base_class_name: Option<String>, state: CodeBlockViewState) -> String {
    let mut classes = vec![
        "ui-code-block".to_string(),
        state.state_class.to_string(),
        state.header_class.to_string(),
        state.motion_source_class.to_string(),
    ];

    if state.copyable {
        classes.push("ui-code-block--copyable".to_string());
    }
    if state.has_label {
        classes.push("ui-code-block--with-label".to_string());
    }
    if state.has_language {
        classes.push("ui-code-block--with-language".to_string());
    }
    if state.is_empty {
        classes.push("ui-code-block--empty".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-code-block--custom-class".to_string());
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
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  rust  ".to_string())),
            Some("rust".to_string())
        );
    }

    #[test]
    fn resolve_state_is_consumed_from_primitives() {
        let state = resolve_state(CodeBlockStateInput {
            is_multiline: true,
            is_empty: false,
            has_label: true,
            has_language: true,
            copyable: true,
            has_custom_class_name: true,
            has_custom_motion: true,
        });

        assert!(state.show_header);
        assert_eq!(state.state_class, "ui-code-block--state-multiline");
        assert_eq!(state.header_class, "ui-code-block--header-visible");
        assert_eq!(state.motion_source_class, "ui-code-block--motion-custom");
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("docs-code-block".to_string()),
            resolve_state(CodeBlockStateInput {
                is_multiline: true,
                is_empty: true,
                has_label: true,
                has_language: false,
                copyable: true,
                has_custom_class_name: true,
                has_custom_motion: false,
            }),
        );

        for token in [
            "ui-code-block",
            "ui-code-block--state-multiline",
            "ui-code-block--header-visible",
            "ui-code-block--motion-default",
            "ui-code-block--copyable",
            "ui-code-block--with-label",
            "ui-code-block--empty",
            "ui-code-block--custom-class",
            "docs-code-block",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
