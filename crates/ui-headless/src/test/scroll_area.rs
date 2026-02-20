use super::*;
use ui_state_primitives::scroll_area::{
    ScrollAreaOrientation, ScrollAreaStateInput, resolve_state,
};

#[test]
fn use_scroll_area_maps_region_locale_and_state_markers() {
    let state = resolve_state(ScrollAreaStateInput {
        orientation: ScrollAreaOrientation::Horizontal,
        disabled: true,
        max_height_px: Some(180),
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let contract = use_scroll_area(ScrollAreaOptions {
        state,
        aria_label: " Activity feed ".to_string(),
        lang: Some(" zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.root_attrs.role, "region");
    assert_eq!(contract.root_attrs.aria_label, " Activity feed ");
    assert_eq!(contract.root_attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.root_attrs.dir, Some("rtl"));
    assert_eq!(contract.root_attrs.data_orientation, "horizontal");
    assert_eq!(contract.root_attrs.data_disabled, Some("true"));
    assert_eq!(contract.root_attrs.data_max_height, "custom");
    assert_eq!(contract.root_attrs.data_aria_source, "custom");
    assert_eq!(contract.root_attrs.data_class_source, "custom");
    assert_eq!(contract.root_attrs.data_custom_class, Some("true"));

    assert_eq!(contract.viewport_attrs.tabindex, -1);
    assert_eq!(contract.viewport_attrs.aria_disabled, Some("true"));

    assert_eq!(contract.state.orientation, "horizontal");
    assert!(contract.state.is_disabled);
    assert!(contract.state.has_custom_max_height);
    assert_eq!(contract.state.max_height_source, "custom");
    assert_eq!(contract.state.aria_source, "custom");
    assert_eq!(contract.state.class_source, "custom");
    assert!(contract.state.has_custom_class_name);
}

#[test]
fn use_scroll_area_keeps_defaults_without_optional_markers() {
    let state = resolve_state(ScrollAreaStateInput {
        orientation: ScrollAreaOrientation::Vertical,
        disabled: false,
        max_height_px: None,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    let contract = use_scroll_area(ScrollAreaOptions {
        state,
        aria_label: "Scrollable region".to_string(),
        lang: None,
        dir: None,
    });

    assert_eq!(contract.root_attrs.data_orientation, "vertical");
    assert_eq!(contract.root_attrs.data_disabled, None);
    assert_eq!(contract.root_attrs.data_max_height, "default");
    assert_eq!(contract.root_attrs.data_aria_source, "default");
    assert_eq!(contract.root_attrs.data_class_source, "default");
    assert_eq!(contract.root_attrs.data_custom_class, None);
    assert_eq!(contract.root_attrs.lang, None);
    assert_eq!(contract.root_attrs.dir, None);

    assert_eq!(contract.viewport_attrs.tabindex, 0);
    assert_eq!(contract.viewport_attrs.aria_disabled, None);
    assert!(!contract.state.is_disabled);
    assert!(!contract.state.has_custom_max_height);
}
