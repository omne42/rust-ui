use crate::help_text::*;

#[test]
fn help_text_tone_contract_is_stable() {
    assert_eq!(HelpTextTone::Auto.class_name(), "ui-help-text--tone-auto");
    assert_eq!(
        HelpTextTone::Neutral.class_name(),
        "ui-help-text--tone-neutral"
    );
    assert_eq!(
        HelpTextTone::Negative.class_name(),
        "ui-help-text--tone-negative"
    );

    assert_eq!(HelpTextTone::Auto.as_attr(), "auto");
    assert_eq!(HelpTextTone::Neutral.as_attr(), "neutral");
    assert_eq!(HelpTextTone::Negative.as_attr(), "negative");
}

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  hint text  ".to_string())),
        Some("hint text".to_string())
    );

    let (aria, custom_aria) = normalize_aria_label(Some("  Email help  ".to_string()));
    assert_eq!(aria, "Email help");
    assert!(custom_aria);

    let (aria, custom_aria) = normalize_aria_label(None);
    assert_eq!(aria, DEFAULT_ARIA_LABEL);
    assert!(!custom_aria);

    let (error, custom_error) = normalize_error_message(Some("  Required  ".to_string()), true);
    assert_eq!(error, Some("Required".to_string()));
    assert!(custom_error);

    let (error, custom_error) = normalize_error_message(None, true);
    assert_eq!(error, Some(DEFAULT_ERROR_MESSAGE.into()));
    assert!(!custom_error);

    let (error, custom_error) = normalize_error_message(Some("ignored".to_string()), false);
    assert_eq!(error, None);
    assert!(!custom_error);
}

#[test]
fn resolve_effective_tone_matches_auto_error_semantics() {
    assert_eq!(
        resolve_effective_tone(HelpTextTone::Auto, true, true),
        HelpTextTone::Negative
    );
    assert_eq!(
        resolve_effective_tone(HelpTextTone::Auto, false, false),
        HelpTextTone::Neutral
    );
    assert_eq!(
        resolve_effective_tone(HelpTextTone::Neutral, true, true),
        HelpTextTone::Neutral
    );
}

#[test]
fn resolve_state_tracks_message_kind_and_sources() {
    let state = resolve_state(HelpTextStateInput {
        tone: HelpTextTone::Auto,
        invalid: true,
        disabled: false,
        show_error_icon: true,
        has_description: true,
        has_error_message: true,
        has_custom_aria_label: true,
        has_custom_error_message: false,
        has_custom_class_name: false,
    });

    assert_eq!(state.message_kind, HelpTextMessageKind::Error);
    assert_eq!(state.tone_attr, "negative");
    assert!(state.show_error_icon);
    assert_eq!(state.data_state, HelpTextDataState::Error);
    assert_eq!(state.aria_source, HelpTextSourceAttr::Custom);
    assert_eq!(state.error_source, HelpTextErrorSourceAttr::Default);
    assert_eq!(state.class_source, HelpTextSourceAttr::Default);
}
