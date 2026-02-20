use super::*;
use ui_state_primitives::spacer::{SpacerAxis, SpacerSize, SpacerStateInput, resolve_state};

#[test]
fn use_spacer_maps_locale_and_semantic_attrs() {
    let state = resolve_state(SpacerStateInput {
        axis: SpacerAxis::Horizontal,
        size: SpacerSize::Lg,
        has_custom_class_name: true,
    });

    let contract = use_spacer(SpacerOptions {
        state,
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "presentation");
    assert_eq!(contract.attrs.aria_hidden, "true");
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_slot, "spacer");
    assert_eq!(contract.attrs.data_axis, "horizontal");
    assert_eq!(contract.attrs.data_size, "lg");
    assert_eq!(contract.attrs.data_state, "horizontal");
    assert_eq!(contract.attrs.data_vertical, None);
    assert_eq!(contract.attrs.data_horizontal, Some("true"));
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.state.axis, "horizontal");
    assert_eq!(contract.state.size, "lg");
    assert!(contract.state.is_horizontal);
    assert!(!contract.state.is_vertical);
    assert!(contract.state.has_custom_class_name);
}

#[test]
fn use_spacer_omits_optional_markers_for_default_vertical_case() {
    let state = resolve_state(SpacerStateInput {
        axis: SpacerAxis::Vertical,
        size: SpacerSize::Md,
        has_custom_class_name: false,
    });

    let contract = use_spacer(SpacerOptions {
        state,
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.data_axis, "vertical");
    assert_eq!(contract.attrs.data_size, "md");
    assert_eq!(contract.attrs.data_vertical, Some("true"));
    assert_eq!(contract.attrs.data_horizontal, None);
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
}
