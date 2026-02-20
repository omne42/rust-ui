use super::*;
use ui_state_primitives::divider::{DividerOrientation, DividerStateInput, resolve_state};

#[test]
fn use_divider_maps_locale_and_semantic_attrs() {
    let state = resolve_state(DividerStateInput {
        orientation: DividerOrientation::Vertical,
        has_custom_class_name: true,
    });

    let contract = use_divider(DividerOptions {
        state,
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "separator");
    assert_eq!(contract.attrs.aria_orientation, Some("vertical"));
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_orientation, "vertical");
    assert_eq!(contract.attrs.data_state, "vertical");
    assert_eq!(contract.attrs.data_horizontal, None);
    assert_eq!(contract.attrs.data_vertical, Some("true"));
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
}

#[test]
fn use_divider_omits_vertical_marker_for_horizontal_case() {
    let state = resolve_state(DividerStateInput {
        orientation: DividerOrientation::Horizontal,
        has_custom_class_name: false,
    });

    let contract = use_divider(DividerOptions {
        state,
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.aria_orientation, None);
    assert_eq!(contract.attrs.data_orientation, "horizontal");
    assert_eq!(contract.attrs.data_horizontal, Some("true"));
    assert_eq!(contract.attrs.data_vertical, None);
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
}
