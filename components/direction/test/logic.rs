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
    let default_class = compose_class_name(None);
    assert!(matches!(
        default_class,
        std::borrow::Cow::Borrowed("ui-direction-provider")
    ));
    assert_eq!(default_class.as_ref(), "ui-direction-provider");

    let custom_class = compose_class_name(Some(" docs-direction-rtl ".to_string()));
    assert!(matches!(custom_class, std::borrow::Cow::Owned(_)));
    assert_eq!(
        custom_class.as_ref(),
        "ui-direction-provider docs-direction-rtl"
    );
}

#[test]
fn resolve_direction_prefers_primary_direction_prop() {
    let (direction, source) = resolve_direction(Some(DirectionMode::Ltr), Some(DirectionMode::Rtl));
    assert_eq!(direction, DirectionMode::Ltr);
    assert_eq!(source, DirectionPropSource::Direction);
    assert_eq!(source.as_attr(), "direction");
}

#[test]
fn resolve_direction_uses_dir_alias_or_default_as_fallback() {
    let (direction, source) = resolve_direction(None, Some(DirectionMode::Rtl));
    assert_eq!(direction, DirectionMode::Rtl);
    assert_eq!(source, DirectionPropSource::DirAlias);
    assert_eq!(source.as_attr(), "dir-alias");

    let (direction, source) = resolve_direction(None, None);
    assert_eq!(direction, DirectionMode::Ltr);
    assert_eq!(source, DirectionPropSource::Default);
    assert_eq!(source.as_attr(), "default");
}
