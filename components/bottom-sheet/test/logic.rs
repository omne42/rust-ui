use super::*;
use leptos::prelude::Callable;

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
fn resolve_defaults_are_single_source_and_explicit() {
    assert_eq!(resolve_title(" ".to_string()), DEFAULT_TITLE);
    assert_eq!(
        resolve_title("  Bottom sheet title  ".to_string()),
        "Bottom sheet title"
    );

    assert_eq!(resolve_close_label(None), DEFAULT_CLOSE_LABEL);
    assert_eq!(resolve_close_label(Some("  ")), DEFAULT_CLOSE_LABEL);
    assert_eq!(resolve_close_label(Some("Dismiss")), "Dismiss");
    assert_eq!(resolve_description_text(None), "");
    assert_eq!(
        resolve_description_text(Some("Description".to_string())),
        "Description"
    );

    assert_eq!(
        resolve_handle_visibility(None, None),
        BottomSheetVisibility::Visible
    );
    assert_eq!(
        resolve_handle_visibility(Some(false), Some(true)),
        BottomSheetVisibility::Hidden
    );
    assert_eq!(
        resolve_handle_visibility(None, Some(false)),
        BottomSheetVisibility::Hidden
    );

    assert_eq!(
        resolve_close_button_visibility(None, None),
        BottomSheetVisibility::Visible
    );
    assert_eq!(
        resolve_close_button_visibility(Some(false), Some(true)),
        BottomSheetVisibility::Hidden
    );
    assert_eq!(
        resolve_close_button_visibility(None, Some(false)),
        BottomSheetVisibility::Hidden
    );

    assert_eq!(
        resolve_attachment(None, None),
        BottomSheetAttachment::Attached
    );
    assert_eq!(
        resolve_attachment(Some(true), Some(false)),
        BottomSheetAttachment::Detached
    );
    assert_eq!(
        resolve_attachment(None, Some(true)),
        BottomSheetAttachment::Detached
    );
    assert_eq!(
        resolve_detached(Some(true), Some(false)),
        BottomSheetAttachment::Detached
    );

    assert_eq!(resolve_bottom_inset_px(None), DEFAULT_BOTTOM_INSET_PX);
    assert_eq!(resolve_bottom_inset_px(Some(999.0)), 240.0);

    assert!(resolve_dismissable(None));
    assert!(!resolve_dismissable(Some(false)));
    assert!(!resolve_keyboard_dismiss_disabled(None));
    assert!(resolve_keyboard_dismiss_disabled(Some(true)));
}

#[test]
fn resolve_on_exit_complete_returns_noop_when_absent() {
    let callback = resolve_on_exit_complete(None);
    callback.run(());
}

#[test]
fn derive_view_state_centralizes_state_and_motion_markers() {
    let derived = derive_view_state(BottomSheetDeriveInput {
        has_description: true,
        has_footer: false,
        handle_visibility: BottomSheetVisibility::Visible,
        close_button_visibility: BottomSheetVisibility::Hidden,
        attachment: BottomSheetAttachment::Detached,
        bottom_inset_px: 18.0,
        has_custom_class_name: true,
        has_custom_motion: true,
    });

    assert_eq!(derived.motion_source_attr, CUSTOM_MOTION_SOURCE_ATTR);
    assert!(derived.has_custom_motion);
    assert_eq!(derived.state.state_attr, "with-description");
    assert_eq!(derived.state.footer_attr, "absent");
    assert_eq!(derived.state.handle_attr, "shown");
    assert_eq!(derived.state.close_button_attr, "hidden");
    assert_eq!(derived.state.detached_attr, "true");
}

#[test]
fn has_slot_is_generic_and_reports_option_presence() {
    assert!(has_slot(&Some("slot")));
    assert!(!has_slot::<i32>(&None));
}

#[test]
fn agent_contract_is_stable_and_machine_readable() {
    let contract = resolve_agent_contract(BottomSheetAgentContractInput {
        is_open: false,
        show_description: false,
        show_footer: false,
        detached: false,
        is_dismissable: true,
        is_keyboard_dismiss_disabled: false,
        motion_source_attr: DEFAULT_MOTION_SOURCE_ATTR,
    });

    assert_eq!(contract.schema_name, "ui.bottom-sheet.agent-contract");
    assert_eq!(contract.schema_version, BottomSheetAgentSchemaVersion::V1);
    assert_eq!(contract.intent, BottomSheetAgentIntent::OverlayBottomSheet);
    assert_eq!(contract.action, BottomSheetAgentAction::DismissAnyInput);
    assert_eq!(contract.state, BottomSheetAgentStateAxis::Closed);
    assert_eq!(
        contract.source,
        BottomSheetAgentSourceAxis::StatePrimitivesDefaultMotion
    );
    assert_eq!(
        contract.output_status,
        BottomSheetAgentOutputStatus::Verified
    );
    assert_eq!(
        contract.stream_support,
        BottomSheetAgentStreamSupport::Optional
    );
    assert_eq!(contract.stream_mode, BottomSheetAgentStreamMode::Snapshot);
    assert_eq!(
        contract.stream_fallback,
        BottomSheetAgentStreamFallback::Snapshot
    );
    assert_eq!(
        contract.render_policy,
        BottomSheetAgentRenderPolicy::TypedOnly
    );
}
