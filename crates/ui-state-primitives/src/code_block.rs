pub use crate::button::normalize_optional_text;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeBlockStateInput {
    pub is_multiline: bool,
    pub is_empty: bool,
    pub has_label: bool,
    pub has_language: bool,
    pub copyable: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeBlockViewState {
    pub show_header: bool,
    pub is_multiline: bool,
    pub is_empty: bool,
    pub has_label: bool,
    pub has_language: bool,
    pub copyable: bool,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub header_class: &'static str,
    pub header_attr: &'static str,
    pub motion_source_class: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

fn has_non_empty_text(value: Option<&str>) -> bool {
    value.map(str::trim).is_some_and(|value| !value.is_empty())
}

pub fn resolve_state(input: CodeBlockStateInput) -> CodeBlockViewState {
    let show_header = input.has_label || input.has_language || input.copyable;
    let (state_class, state_attr) = if input.is_multiline {
        ("ui-code-block--state-multiline", "multiline")
    } else {
        ("ui-code-block--state-single-line", "single-line")
    };

    let (header_class, header_attr) = if show_header {
        ("ui-code-block--header-visible", "visible")
    } else {
        ("ui-code-block--header-hidden", "hidden")
    };

    let (motion_source_class, motion_source_attr) = if input.has_custom_motion {
        ("ui-code-block--motion-custom", "custom")
    } else {
        ("ui-code-block--motion-default", "default")
    };

    CodeBlockViewState {
        show_header,
        is_multiline: input.is_multiline,
        is_empty: input.is_empty,
        has_label: input.has_label,
        has_language: input.has_language,
        copyable: input.copyable,
        state_class,
        state_attr,
        header_class,
        header_attr,
        motion_source_class,
        motion_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn resolve_view_state(
    code: &str,
    label: Option<&str>,
    language: Option<&str>,
    copyable: bool,
) -> CodeBlockViewState {
    resolve_state(CodeBlockStateInput {
        is_multiline: code.contains('\n'),
        is_empty: code.trim().is_empty(),
        has_label: has_non_empty_text(label),
        has_language: has_non_empty_text(language),
        copyable,
        has_custom_class_name: false,
        has_custom_motion: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_is_hidden_when_all_optional_parts_absent() {
        let view = resolve_view_state("let x = 1;", None, None, false);
        assert!(!view.show_header);
        assert_eq!(view.header_attr, "hidden");
    }

    #[test]
    fn header_is_shown_for_label_language_or_copyable() {
        assert!(resolve_view_state("x", Some("Code"), None, false).show_header);
        assert!(resolve_view_state("x", None, Some("rs"), false).show_header);
        assert!(resolve_view_state("x", None, None, true).show_header);
    }

    #[test]
    fn multiline_detection() {
        assert_eq!(
            resolve_view_state("x", None, None, false).state_attr,
            "single-line"
        );
        assert_eq!(
            resolve_view_state("x\ny", None, None, false).state_attr,
            "multiline"
        );
    }

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
    fn resolve_state_tracks_custom_sources_and_flags() {
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
}
