use super::*;

#[test]
fn direction_mode_attr_contract_is_stable() {
    assert_eq!(DirectionMode::Ltr.as_attr(), "ltr");
    assert_eq!(DirectionMode::Rtl.as_attr(), "rtl");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-direction ".to_string())),
        Some("docs-direction".to_string())
    );
}

#[test]
fn compose_class_name_merges_base_and_custom_class() {
    assert_eq!(
        compose_class_name(None),
        "ui-direction-provider".to_string()
    );
    assert_eq!(
        compose_class_name(Some(" docs-direction-rtl ".to_string())),
        "ui-direction-provider docs-direction-rtl".to_string()
    );
}
