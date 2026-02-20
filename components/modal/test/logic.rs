use super::*;

#[test]
fn state_and_description_attrs_follow_contract() {
    assert_eq!(state_attr(true), "with-description");
    assert_eq!(state_attr(false), "title-only");
    assert_eq!(description_attr(true), "present");
    assert_eq!(description_attr(false), "absent");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-modal ".to_string())),
        Some("docs-modal".to_string())
    );
}

#[test]
fn normalize_required_text_falls_back_for_blank_values() {
    assert_eq!(
        normalize_required_text(" Confirm ".to_string(), DEFAULT_TITLE),
        "Confirm"
    );
    assert_eq!(
        normalize_required_text(" ".to_string(), DEFAULT_TITLE),
        DEFAULT_TITLE
    );
}

#[test]
fn normalize_id_base_uses_default_for_blank_values() {
    assert_eq!(normalize_id_base(" docs-modal ".to_string()), "docs-modal");
    assert_eq!(normalize_id_base("  ".to_string()), DEFAULT_ID_BASE);
}

#[test]
fn resolve_state_tracks_source_markers() {
    let state = resolve_state(ModalPartStateInput {
        slot: ModalSlot::Root,
        has_description: true,
        has_custom_id_base: true,
        has_custom_title: true,
        has_custom_description: true,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_on_exit_complete: true,
    });

    assert_eq!(state.slot_attr, "modal");
    assert_eq!(state.base_class, "ui-modal");
    assert_eq!(state.state_attr, "with-description");
    assert_eq!(state.description_attr, "present");
    assert_eq!(state.id_source_attr, "custom");
    assert_eq!(state.title_source_attr, "custom");
    assert_eq!(state.description_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.exit_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-modal".to_string()),
        resolve_state(ModalPartStateInput {
            slot: ModalSlot::Root,
            has_description: true,
            has_custom_id_base: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_on_exit_complete: true,
        }),
    );

    for token in [
        "ui-modal",
        "ui-modal--with-description",
        "ui-modal--custom-id",
        "ui-modal--custom-title",
        "ui-modal--custom-description",
        "ui-modal--custom-motion",
        "ui-modal--custom-exit",
        "ui-modal--custom-class",
        "docs-modal",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}
