use super::*;
use ui_state_primitives::keyboard::{KeyboardStateInput, KeyboardTone, resolve_state};

#[test]
fn use_keyboard_maps_locale_and_semantic_attrs() {
    let state = resolve_state(KeyboardStateInput {
        tone: KeyboardTone::Muted,
        compact: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    let contract = use_keyboard(KeyboardOptions {
        state,
        aria_label: "Keyboard Shortcut".to_string(),
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.aria_label, "Keyboard Shortcut");
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_slot, "keyboard");
    assert_eq!(contract.attrs.data_tone, "muted");
    assert_eq!(contract.attrs.data_state, "compact");
    assert_eq!(contract.attrs.data_compact, Some("true"));
    assert_eq!(contract.attrs.data_aria_source, "custom");
    assert_eq!(contract.attrs.data_custom_class, Some("true"));
    assert_eq!(contract.attrs.data_class_source, "custom");
    assert_eq!(contract.attrs.data_ui_schema, KEYBOARD_AGENT_SCHEMA);
    assert_eq!(contract.attrs.data_ui_schema_version, "v1");
    assert_eq!(contract.attrs.data_ui_intent, "display.keyboard.render");
    assert_eq!(contract.attrs.data_ui_action, "render");
    assert_eq!(contract.attrs.data_ui_state, "compact");
    assert_eq!(contract.attrs.data_ui_source, "custom");
    assert_eq!(contract.attrs.data_ui_output_status, "verified");
    assert_eq!(contract.state.tone, "muted");
    assert_eq!(contract.state.state, "compact");
    assert!(contract.state.is_compact);
    assert_eq!(contract.state.aria_source, "custom");
    assert_eq!(contract.state.class_source, "custom");
    assert!(contract.state.has_custom_class_name);
    assert_eq!(contract.state.intent, "display.keyboard.render");
    assert_eq!(contract.state.action, "render");
    assert_eq!(contract.state.agent_state, "compact");
    assert_eq!(contract.state.source, "custom");
    assert_eq!(contract.state.output_status, "verified");
}

#[test]
fn use_keyboard_omits_optional_markers_in_default_case() {
    let state = resolve_state(KeyboardStateInput {
        tone: KeyboardTone::Default,
        compact: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });

    let contract = use_keyboard(KeyboardOptions {
        state,
        aria_label: "Keyboard".to_string(),
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.aria_label, "Keyboard");
    assert_eq!(contract.attrs.data_tone, "default");
    assert_eq!(contract.attrs.data_state, "default");
    assert_eq!(contract.attrs.data_compact, None);
    assert_eq!(contract.attrs.data_aria_source, "default");
    assert_eq!(contract.attrs.data_custom_class, None);
    assert_eq!(contract.attrs.data_class_source, "default");
    assert_eq!(contract.attrs.data_ui_schema, KEYBOARD_AGENT_SCHEMA);
    assert_eq!(contract.attrs.data_ui_schema_version, "v1");
    assert_eq!(contract.attrs.data_ui_intent, "display.keyboard.render");
    assert_eq!(contract.attrs.data_ui_action, "render");
    assert_eq!(contract.attrs.data_ui_state, "default");
    assert_eq!(contract.attrs.data_ui_source, "default");
    assert_eq!(contract.attrs.data_ui_output_status, "verified");
    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
    assert_eq!(contract.state.output_status, "verified");
}
