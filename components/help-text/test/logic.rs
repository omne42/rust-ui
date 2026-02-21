use super::*;

#[test]
fn resolve_state_comes_from_state_primitives_contract() {
    let state = resolve_state(HelpTextStateInput {
        tone: HelpTextTone::Auto,
        invalid: true,
        disabled: false,
        show_error_icon: true,
        has_description: true,
        has_error_message: true,
        has_custom_aria_label: false,
        has_custom_error_message: false,
        has_custom_class_name: false,
    });

    assert_eq!(state.tone_attr, "negative");
    assert_eq!(state.message_kind, HelpTextMessageKind::Error);
    assert_eq!(state.data_state, HelpTextDataState::Error);
}

#[test]
fn locale_and_live_region_are_sourced_from_headless_contracts() {
    let locale = resolve_locale_attrs(Some("  zh-CN ".to_string()), Some(A11yDirection::Rtl));
    assert_eq!(locale.lang.as_deref(), Some("zh-CN"));
    assert_eq!(locale.dir, Some("rtl"));

    let region = resolve_error_live_region_attrs();
    assert_eq!(region.role, "alert");
    assert_eq!(region.aria_live, "assertive");
}

#[test]
fn compose_class_name_includes_custom_marker_and_user_class() {
    let state = resolve_state(HelpTextStateInput {
        tone: HelpTextTone::Neutral,
        invalid: false,
        disabled: true,
        show_error_icon: false,
        has_description: true,
        has_error_message: false,
        has_custom_aria_label: false,
        has_custom_error_message: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-help-text-custom".to_string()), state);

    for token in [
        "ui-help-text",
        "ui-help-text--tone-neutral",
        "ui-help-text--disabled",
        "ui-help-text--has-description",
        "ui-help-text--custom-class",
        "docs-help-text-custom",
    ] {
        assert!(
            class_name.contains(token),
            "composed class should include `{token}`"
        );
    }
}

#[test]
fn display_text_default_is_centralized_in_logic() {
    assert_eq!(
        resolve_display_text(Some("Resolved from logic".to_string())),
        "Resolved from logic"
    );
    assert_eq!(resolve_display_text(None), "");
}

#[test]
fn render_model_centralizes_input_normalization_and_state_derivation() {
    let render_model = resolve_render_model(HelpTextLogicInput {
        tone: HelpTextTone::Auto,
        is_invalid: true,
        is_disabled: false,
        is_error_icon_visible: true,
        description: Some("  ignored because invalid ".to_string()),
        error_message: None,
        aria_label: Some("  custom aria ".to_string()),
        class_name: Some("  docs-help-text-custom ".to_string()),
    });

    assert_eq!(render_model.aria_label, "custom aria");
    assert_eq!(render_model.error_message_text, DEFAULT_ERROR_MESSAGE);
    assert_eq!(render_model.description_text, "");
    assert_eq!(
        render_model.class_name.as_deref(),
        Some("docs-help-text-custom")
    );
    assert_eq!(render_model.state.message_kind, HelpTextMessageKind::Error);
    assert_eq!(
        render_model.state.error_source,
        HelpTextErrorSourceAttr::Default
    );
    assert_eq!(render_model.state.aria_source, HelpTextSourceAttr::Custom);
    assert!(render_model.state.show_error_icon);
}

#[test]
fn snapshot_mode_is_the_default_agent_contract_for_full_result_rendering() {
    let state = resolve_state(HelpTextStateInput {
        tone: HelpTextTone::Neutral,
        invalid: false,
        disabled: false,
        show_error_icon: false,
        has_description: true,
        has_error_message: false,
        has_custom_aria_label: false,
        has_custom_error_message: false,
        has_custom_class_name: false,
    });

    let contract = resolve_agent_contract_attrs(state);

    assert_eq!(contract.data_ui_stream_support, "optional");
    assert_eq!(contract.data_ui_stream_mode, "snapshot");
    assert_eq!(contract.data_ui_stream_fallback, "snapshot");
    assert_eq!(contract.data_ui_output_status, "verified");
}
