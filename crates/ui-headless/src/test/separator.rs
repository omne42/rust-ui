use super::*;
use ui_state_primitives::separator::{
    SeparatorElementType, SeparatorOrientation, SeparatorStateInput, resolve_state,
};

#[test]
fn separator_contract_maps_semantic_a11y_attrs() {
    let state = resolve_state(SeparatorStateInput {
        orientation: SeparatorOrientation::Vertical,
        element_type: SeparatorElementType::Div,
        decorative: false,
        has_custom_class_name: false,
    });

    let separator = use_separator(SeparatorOptions {
        state,
        lang: Some("  en-US ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(separator.attrs.role, Some("separator"));
    assert_eq!(separator.attrs.aria_orientation, Some("vertical"));
    assert_eq!(separator.attrs.aria_hidden, None);
    assert_eq!(separator.attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(separator.attrs.dir, Some("rtl"));
    assert!(separator.state.is_semantic);
    assert!(!separator.state.is_decorative);
}

#[test]
fn separator_contract_maps_decorative_a11y_attrs() {
    let state = resolve_state(SeparatorStateInput {
        orientation: SeparatorOrientation::Horizontal,
        element_type: SeparatorElementType::Hr,
        decorative: true,
        has_custom_class_name: true,
    });

    let separator = use_separator(SeparatorOptions {
        state,
        lang: None,
        dir: None,
    });

    assert_eq!(separator.attrs.role, None);
    assert_eq!(separator.attrs.aria_orientation, None);
    assert_eq!(separator.attrs.aria_hidden, Some("true"));
    assert_eq!(separator.attrs.lang, None);
    assert_eq!(separator.attrs.dir, None);
    assert!(!separator.state.is_semantic);
    assert!(separator.state.is_decorative);
}
