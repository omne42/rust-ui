use super::*;

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some(" docs-bottom-sheet ".to_string())),
        Some("docs-bottom-sheet".to_string())
    );

    assert_eq!(
        normalize_required_text(" Bottom sheet ".to_string(), "Bottom sheet"),
        "Bottom sheet"
    );
    assert_eq!(
        normalize_required_text(" ".to_string(), "Bottom sheet"),
        "Bottom sheet"
    );

    assert_eq!(
        normalize_id_base(" docs-bottom-sheet ".to_string()),
        "docs-bottom-sheet"
    );
    assert_eq!(normalize_id_base(" ".to_string()), "ui-bottom-sheet");

    assert_eq!(normalize_bottom_inset_px(-12.0), 0.0);
    assert_eq!(normalize_bottom_inset_px(999.0), 240.0);
    assert_eq!(normalize_bottom_inset_px(18.5), 18.5);
}

#[test]
fn resolve_state_tracks_description_footer_handle_close_detached_and_inset() {
    let state = resolve_state(BottomSheetStateInput {
        has_description: true,
        has_footer: false,
        show_handle: true,
        show_close_button: false,
        detached: true,
        bottom_inset_px: 17.0,
        has_custom_class_name: true,
    });

    assert!(state.show_description);
    assert_eq!(state.state_class, "ui-bottom-sheet--with-description");
    assert_eq!(state.state_attr, "with-description");
    assert_eq!(state.description_attr, "present");

    assert!(!state.show_footer);
    assert_eq!(state.footer_attr, "absent");

    assert!(state.show_handle);
    assert_eq!(state.handle_class, "ui-bottom-sheet--handle-shown");
    assert_eq!(state.handle_attr, "shown");

    assert!(!state.show_close_button);
    assert_eq!(state.close_button_class, "ui-bottom-sheet--close-hidden");
    assert_eq!(state.close_button_attr, "hidden");

    assert!(state.detached);
    assert_eq!(state.detached_class, "ui-bottom-sheet--detached");
    assert_eq!(state.detached_attr, "true");
    assert_eq!(state.inset_class, "ui-bottom-sheet--inset-md");
    assert_eq!(state.inset_attr, "md");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn resolve_state_maps_attached_to_no_inset() {
    let state = resolve_state(BottomSheetStateInput {
        has_description: false,
        has_footer: false,
        show_handle: false,
        show_close_button: true,
        detached: false,
        bottom_inset_px: 240.0,
        has_custom_class_name: false,
    });

    assert_eq!(state.inset_class, "ui-bottom-sheet--inset-none");
    assert_eq!(state.inset_attr, "none");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(BottomSheetStateInput {
        has_description: false,
        has_footer: true,
        show_handle: false,
        show_close_button: true,
        detached: false,
        bottom_inset_px: 24.0,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-bottom-sheet".to_string()), state);

    for token in [
        "ui-bottom-sheet",
        "ui-bottom-sheet--title-only",
        "ui-bottom-sheet--with-footer",
        "ui-bottom-sheet--handle-hidden",
        "ui-bottom-sheet--close-shown",
        "ui-bottom-sheet--attached",
        "ui-bottom-sheet--inset-none",
        "ui-bottom-sheet--custom-class",
        "docs-bottom-sheet",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn agent_contract_is_stable_and_machine_readable() {
    let contract = agent_contract();

    assert_eq!(contract.schema_attr, "bottom-sheet.v1");
    assert_eq!(contract.intent_attr, "overlay");
    assert_eq!(contract.action_attr, "dismiss");
    assert_eq!(
        contract.state_axis_attr,
        "visibility|description|footer|detached|inset"
    );
    assert_eq!(contract.source_axis_attr, "default|custom");
    assert_eq!(contract.render_mode_attr, "snapshot");
    assert_eq!(contract.streaming_attr, "optional");
    assert_eq!(contract.fallback_attr, "snapshot");
    assert_eq!(contract.output_status_attr, "verified");
}
