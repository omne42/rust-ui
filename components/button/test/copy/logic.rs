use super::*;

#[test]
fn normalize_optional_text_trims_and_drops_blank_values() {
    assert_eq!(
        normalize_optional_text(Some("  Copy now  ".to_string())),
        Some("Copy now".to_string())
    );
    assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
    assert_eq!(normalize_optional_text(None), None);
}

#[test]
fn resolve_text_contract_uses_defaults_when_values_missing() {
    let contract = resolve_text_contract(None, None, None);

    assert_eq!(contract.label, DEFAULT_COPY_LABEL);
    assert_eq!(contract.copied_label, DEFAULT_COPIED_LABEL);
    assert_eq!(contract.aria_label, DEFAULT_COPY_LABEL);
}

#[test]
fn resolve_text_contract_prefers_custom_values_when_present() {
    let contract = resolve_text_contract(
        Some("  Copy URL  ".to_string()),
        Some("  URL copied  ".to_string()),
        Some("  Copy URL to clipboard  ".to_string()),
    );

    assert_eq!(contract.label, "Copy URL");
    assert_eq!(contract.copied_label, "URL copied");
    assert_eq!(contract.aria_label, "Copy URL to clipboard");
}

#[test]
fn resolve_text_contract_falls_back_aria_to_resolved_label() {
    let contract = resolve_text_contract(Some("  Install  ".to_string()), None, None);

    assert_eq!(contract.label, "Install");
    assert_eq!(contract.copied_label, DEFAULT_COPIED_LABEL);
    assert_eq!(contract.aria_label, "Install");
}

#[test]
fn button_copy_mode_contract_exposes_expected_flags() {
    let text_only = resolve_view_state(
        "",
        false,
        ButtonCopyMode::TextOnly,
        false,
        false,
        false,
        false,
    );
    assert_eq!(text_only.mode_attr, "text-only");
    assert!(text_only.shows_text);
    assert!(!text_only.shows_icon);
    assert!(!text_only.is_icon_only);

    let icon_only = resolve_view_state(
        "",
        false,
        ButtonCopyMode::IconOnly,
        false,
        false,
        false,
        false,
    );
    assert_eq!(icon_only.mode_attr, "icon-only");
    assert!(!icon_only.shows_text);
    assert!(icon_only.shows_icon);
    assert!(icon_only.is_icon_only);
}

#[test]
fn empty_text_is_not_copyable() {
    assert!(
        !resolve_view_state(
            "",
            false,
            ButtonCopyMode::IconAndText,
            false,
            false,
            false,
            false
        )
        .is_copyable
    );
    assert!(
        !resolve_view_state(
            "   ",
            false,
            ButtonCopyMode::IconAndText,
            false,
            false,
            false,
            false
        )
        .is_copyable
    );
}

#[test]
fn disabled_is_not_copyable_even_when_text_present() {
    assert!(
        !resolve_view_state(
            "hello",
            true,
            ButtonCopyMode::IconAndText,
            false,
            false,
            false,
            false
        )
        .is_copyable
    );
}

#[test]
fn enabled_with_text_is_copyable() {
    assert!(
        resolve_view_state(
            "hello",
            false,
            ButtonCopyMode::IconAndText,
            false,
            false,
            false,
            false
        )
        .is_copyable
    );
}

#[test]
fn resolve_view_state_tracks_metadata_flags() {
    let state = resolve_view_state(
        "hello",
        false,
        ButtonCopyMode::IconAndText,
        true,
        true,
        true,
        true,
    );
    assert!(state.is_copyable);
    assert!(!state.is_disabled);
    assert!(state.is_enabled);
    assert!(state.has_text);
    assert_eq!(state.state_attr, "copyable");
    assert_eq!(state.mode_attr, "icon-and-text");
    assert!(state.shows_text);
    assert!(state.shows_icon);
    assert!(!state.is_icon_only);
    assert!(state.has_custom_label);
    assert!(state.has_custom_copied_label);
    assert!(state.has_custom_aria_label);
    assert!(state.has_custom_class_name);
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("custom".to_string()),
        resolve_view_state(
            "hello",
            false,
            ButtonCopyMode::IconAndText,
            true,
            true,
            false,
            true,
        ),
    );

    for token in [
        "ui-button-copy",
        "ui-button-copy--copyable",
        "ui-button-copy--custom-label",
        "ui-button-copy--custom-copied-label",
        "ui-button-copy--icon-and-text",
        "custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class name should include `{token}`"
        );
    }
}

#[test]
fn button_copy_agent_contract_is_schema_typed_and_stateful() {
    let ready_state = resolve_view_state(
        "copy me",
        false,
        ButtonCopyMode::IconAndText,
        false,
        false,
        false,
        false,
    );
    let contract = resolve_agent_contract(ready_state);

    assert_eq!(contract.schema_name, "ui.button-copy.agent-contract");
    assert_eq!(contract.schema_version.as_str(), "1");
    assert_eq!(contract.intent.as_str(), "clipboard-copy");
    assert_eq!(contract.action.as_str(), "copy");
    assert_eq!(contract.state.as_str(), "ready");
    assert!(contract.capabilities.can_copy);
    assert!(contract.capabilities.can_visual_feedback);
    assert!(contract.capabilities.can_announce_feedback);

    let disabled_state = resolve_view_state(
        "copy me",
        true,
        ButtonCopyMode::IconAndText,
        false,
        false,
        false,
        false,
    );
    assert_eq!(
        resolve_agent_contract(disabled_state).state.as_str(),
        "disabled"
    );

    let empty_state = resolve_view_state(
        "   ",
        false,
        ButtonCopyMode::IconAndText,
        false,
        false,
        false,
        false,
    );
    assert_eq!(resolve_agent_contract(empty_state).state.as_str(), "empty");
}

#[test]
fn button_copy_agent_output_status_prioritizes_loading_then_error_then_copied() {
    assert_eq!(
        resolve_agent_output_status(true, true, true).as_str(),
        "loading"
    );
    assert_eq!(
        resolve_agent_output_status(false, true, true).as_str(),
        "error"
    );
    assert_eq!(
        resolve_agent_output_status(false, false, true).as_str(),
        "copied"
    );
    assert_eq!(
        resolve_agent_output_status(false, false, false).as_str(),
        "idle"
    );
    assert_eq!(
        resolve_agent_output_status_attr(true, false, false),
        "loading"
    );
    assert_eq!(
        resolve_agent_output_status_attr(false, true, false),
        "error"
    );
    assert_eq!(
        resolve_agent_output_status_attr(false, false, true),
        "copied"
    );
    assert_eq!(
        resolve_agent_output_status_attr(false, false, false),
        "idle"
    );
}
