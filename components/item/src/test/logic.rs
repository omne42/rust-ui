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

    assert_eq!(ItemStateSource::Default.as_attr(), "default");
    assert_eq!(ItemStateSource::Prop.as_attr(), "prop");
}

#[test]
fn normalize_defaults_are_owned_by_logic_layer() {
    assert_eq!(normalize_item_variant(None), ItemVariant::Default);
    assert_eq!(
        normalize_item_variant(Some(ItemVariant::Outline)),
        ItemVariant::Outline
    );

    assert_eq!(normalize_item_size(None), ItemSize::Default);
    assert_eq!(normalize_item_size(Some(ItemSize::Sm)), ItemSize::Sm);

    assert_eq!(
        normalize_item_media_variant(None),
        ItemMediaVariant::Default
    );
    assert_eq!(
        normalize_item_media_variant(Some(ItemMediaVariant::Image)),
        ItemMediaVariant::Image
    );
}

#[test]
fn derive_render_state_outputs_stable_semantic_attrs() {
    let default_state = derive_item_render_state(None, None);
    assert_eq!(default_state.variant_attr, "default");
    assert_eq!(default_state.size_attr, "default");
    assert_eq!(default_state.variant_source_attr, "default");
    assert_eq!(default_state.size_source_attr, "default");

    let outline_sm_state = derive_item_render_state(Some(ItemVariant::Outline), Some(ItemSize::Sm));
    assert_eq!(outline_sm_state.variant_attr, "outline");
    assert_eq!(outline_sm_state.size_attr, "sm");
    assert_eq!(outline_sm_state.variant_source_attr, "prop");
    assert_eq!(outline_sm_state.size_source_attr, "prop");

    let media_default = derive_item_media_render_state(None);
    assert_eq!(media_default.variant_attr, "default");
    assert_eq!(media_default.variant_source_attr, "default");

    let media_icon = derive_item_media_render_state(Some(ItemMediaVariant::Icon));
    assert_eq!(media_icon.variant_attr, "icon");
    assert_eq!(media_icon.variant_source_attr, "prop");
}

#[test]
fn locale_attrs_are_normalized_via_headless_contract() {
    let locale = resolve_locale_attrs(Some("  en-US  ".to_string()), Some(A11yDirection::Rtl));
    assert_eq!(locale.lang, Some("en-US".to_string()));
    assert_eq!(locale.dir, Some("rtl"));

    let locale = resolve_locale_attrs(Some("   ".to_string()), None);
    assert_eq!(locale.lang, None);
    assert_eq!(locale.dir, None);
}
