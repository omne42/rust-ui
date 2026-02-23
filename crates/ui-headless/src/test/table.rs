use super::*;
use ui_state_primitives::table::{
    TableDensity, TableLayout, TableStateInput, TableVariant, resolve_state,
};

#[test]
fn use_table_a11y_maps_locale_and_semantic_attrs() {
    let state = resolve_state(TableStateInput {
        variant: TableVariant::Outline,
        density: TableDensity::Compact,
        layout: TableLayout::Fixed,
        striped: true,
        sticky_header: true,
        has_caption: true,
        row_count: 3,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let contract = use_table_a11y(TableA11yOptions {
        state,
        aria_label: "  Service table  ".to_string(),
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "region");
    assert_eq!(contract.attrs.aria_label, "Service table");
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_variant, "outline");
    assert_eq!(contract.attrs.data_density, "compact");
    assert_eq!(contract.attrs.data_layout, "fixed");
    assert_eq!(contract.attrs.data_state, "data");
    assert_eq!(contract.attrs.data_striped, Some("true"));
    assert_eq!(contract.attrs.data_sticky_header, Some("true"));
    assert_eq!(contract.attrs.data_has_caption, Some("true"));
    assert_eq!(contract.attrs.data_row_count, "3");
    assert_eq!(contract.attrs.data_aria_source, "custom");
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.attrs.data_class_source, "custom");
}

#[test]
fn use_table_a11y_handles_default_and_empty_state() {
    let state = resolve_state(TableStateInput {
        variant: TableVariant::Default,
        density: TableDensity::Comfortable,
        layout: TableLayout::Auto,
        striped: false,
        sticky_header: false,
        has_caption: false,
        row_count: 0,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    let contract = use_table_a11y(TableA11yOptions {
        state,
        aria_label: "   ".to_string(),
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.aria_label, "Data table");
    assert_eq!(contract.attrs.data_state, "empty");
    assert_eq!(contract.attrs.data_striped, None);
    assert_eq!(contract.attrs.data_sticky_header, None);
    assert_eq!(contract.attrs.data_has_caption, None);
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.attrs.data_aria_source, "default");
    assert_eq!(contract.attrs.data_class_source, "default");
    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
    assert_eq!(contract.state.state, "empty");
    assert!(contract.state.is_empty);
    assert!(!contract.state.has_custom_aria_label);
    assert!(!contract.state.has_custom_class_name);
}
