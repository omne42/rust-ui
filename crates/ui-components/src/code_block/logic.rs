#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeBlockViewState {
    pub show_header: bool,
    pub is_multiline: bool,
}

pub fn resolve_view_state(
    code: &str,
    label: Option<&str>,
    language: Option<&str>,
    copyable: bool,
) -> CodeBlockViewState {
    let label = label.map(str::trim).filter(|value| !value.is_empty());
    let language = language.map(str::trim).filter(|value| !value.is_empty());

    CodeBlockViewState {
        show_header: label.is_some() || language.is_some() || copyable,
        is_multiline: code.contains('\n'),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_is_hidden_when_all_optional_parts_absent() {
        let view = resolve_view_state("let x = 1;", None, None, false);
        assert!(!view.show_header);
    }

    #[test]
    fn header_is_shown_for_label_language_or_copyable() {
        assert!(resolve_view_state("x", Some("Code"), None, false).show_header);
        assert!(resolve_view_state("x", None, Some("rs"), false).show_header);
        assert!(resolve_view_state("x", None, None, true).show_header);
    }

    #[test]
    fn multiline_detection() {
        assert!(!resolve_view_state("x", None, None, false).is_multiline);
        assert!(resolve_view_state("x\ny", None, None, false).is_multiline);
    }
}
