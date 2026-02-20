use super::*;

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("".to_string())), None);
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  hello  ".to_string())),
        Some("hello".to_string())
    );
}

#[test]
fn compose_class_appends_custom_class_when_present() {
    assert_eq!(compose_class("base", None), "base");
    assert_eq!(compose_class("base", Some("".to_string())), "base");
    assert_eq!(
        compose_class("base", Some("  extra  ".to_string())),
        "base extra"
    );
}

#[test]
fn item_attrs_match_variants() {
    assert_eq!(ItemVariant::Default.as_attr(), "default");
    assert_eq!(ItemVariant::Outline.as_attr(), "outline");
    assert_eq!(ItemVariant::Muted.as_attr(), "muted");

    assert_eq!(ItemSize::Default.as_attr(), "default");
    assert_eq!(ItemSize::Sm.as_attr(), "sm");

    assert_eq!(ItemMediaVariant::Default.as_attr(), "default");
    assert_eq!(ItemMediaVariant::Icon.as_attr(), "icon");
    assert_eq!(ItemMediaVariant::Image.as_attr(), "image");
}
