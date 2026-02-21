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
