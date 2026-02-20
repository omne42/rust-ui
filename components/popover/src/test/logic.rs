use super::*;

#[test]
fn state_and_modal_attrs_follow_contract() {
    assert_eq!(state_attr_for_open(true), "open");
    assert_eq!(state_attr_for_open(false), "closed");
    assert_eq!(modal_attr(true), "modal");
    assert_eq!(modal_attr(false), "non-modal");
}

#[test]
fn normalize_optional_text_trims_and_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  docs-popover  ".to_string())),
        Some("docs-popover".to_string())
    );
}

#[test]
fn resolve_state_tracks_source_markers() {
    let state = resolve_state(PopoverPartStateInput {
        slot: PopoverSlot::Root,
        open: true,
        is_modal: false,
        has_custom_class_name: true,
        has_custom_motion: true,
        has_custom_placement: true,
        has_on_exit_complete: true,
    });

    assert_eq!(state.slot_attr, "popover");
    assert_eq!(state.base_class, "ui-popover");
    assert_eq!(state.state_attr, "open");
    assert_eq!(state.modal_attr, "non-modal");
    assert_eq!(state.class_source_attr, "custom");
    assert_eq!(state.motion_source_attr, "custom");
    assert_eq!(state.placement_source_attr, "custom");
    assert_eq!(state.modal_source_attr, "custom");
    assert_eq!(state.exit_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_custom_markers() {
    let class_name = compose_class_name(
        Some("docs-popover-state".to_string()),
        resolve_state(PopoverPartStateInput {
            slot: PopoverSlot::Root,
            open: false,
            is_modal: false,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_custom_placement: true,
            has_on_exit_complete: true,
        }),
    );

    for token in [
        "ui-popover",
        "ui-popover--custom-motion",
        "ui-popover--custom-placement",
        "ui-popover--non-modal",
        "ui-popover--custom-modal",
        "ui-popover--custom-exit",
        "ui-popover--custom-class",
        "docs-popover-state",
    ] {
        assert!(
            class_name.contains(token),
            "popover class name should include `{token}`"
        );
    }
}

#[test]
fn compose_panel_vars_formats_css_custom_properties() {
    assert_eq!(
        compose_panel_vars(16.5, 24.0, 320.0),
        "--ui-popover-top: 16.5px; --ui-popover-left: 24px; --ui-popover-anchor-width: 320px;"
    );
}

#[test]
fn should_close_on_escape_requires_topmost_non_composing_non_prevented_escape() {
    assert!(should_close_on_escape("Escape", true, false, false));
    assert!(!should_close_on_escape("Enter", true, false, false));
    assert!(!should_close_on_escape("Escape", false, false, false));
    assert!(!should_close_on_escape("Escape", true, true, false));
    assert!(!should_close_on_escape("Escape", true, false, true));
}
