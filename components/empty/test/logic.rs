use super::*;

#[test]
fn media_variant_attr_contract_is_stable() {
    assert_eq!(EmptyMediaVariant::Default.as_attr(), "default");
    assert_eq!(EmptyMediaVariant::Icon.as_attr(), "icon");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-empty  ".to_string())),
        Some("docs-empty".to_string())
    );
}

#[test]
fn resolve_state_tracks_slot_and_source_markers() {
    let state = resolve_state(EmptyPartStateInput {
        slot: EmptySlot::Media,
        media_variant: EmptyMediaVariant::Icon,
        has_custom_class_name: true,
    });

    assert_eq!(state.slot_attr, "empty-icon");
    assert_eq!(state.base_class, "ui-empty__media");
    assert_eq!(state.state_attr, "media");
    assert_eq!(state.media_variant_attr, "icon");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.variant_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_and_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-empty-custom".to_string()),
        resolve_state(EmptyPartStateInput {
            slot: EmptySlot::Media,
            media_variant: EmptyMediaVariant::Icon,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-empty__media",
        "ui-empty__media--icon",
        "ui-empty--custom-class",
        "docs-empty-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn normalize_part_centralizes_default_variant_in_logic_layer() {
    let (class_name, state) =
        normalize_part(EmptySlot::Header, Some("  docs-empty  ".to_string()), None);
    assert_eq!(state.slot, EmptySlot::Header);
    assert_eq!(state.media_variant, EmptyMediaVariant::Default);
    assert_eq!(state.variant_source_attr, "default");
    assert!(class_name.contains("ui-empty__header"));
    assert!(class_name.contains("docs-empty"));
}
