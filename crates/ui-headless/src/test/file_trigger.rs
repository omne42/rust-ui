use super::*;
use ui_state_primitives::file_trigger::{FileTriggerStateInput, resolve_state};

#[test]
fn use_file_trigger_maps_locale_and_hidden_input_contract() {
    let state = resolve_state(FileTriggerStateInput {
        disabled: false,
        has_custom_motion: false,
    });

    let contract = use_file_trigger(FileTriggerOptions {
        state,
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.data_state, "ready");
    assert_eq!(contract.attrs.data_disabled, None);
    assert_eq!(contract.attrs.data_enabled, Some("true"));
    assert_eq!(contract.attrs.input_tabindex, -1);
    assert_eq!(contract.attrs.input_aria_hidden, "true");
    assert_eq!(contract.state.state, "ready");
    assert!(!contract.state.is_disabled);
    assert!(contract.state.is_enabled);
}

#[test]
fn use_file_trigger_marks_disabled_state() {
    let state = resolve_state(FileTriggerStateInput {
        disabled: true,
        has_custom_motion: true,
    });

    let contract = use_file_trigger(FileTriggerOptions {
        state,
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.lang, None);
    assert_eq!(contract.attrs.dir, None);
    assert_eq!(contract.attrs.data_state, "disabled");
    assert_eq!(contract.attrs.data_disabled, Some("true"));
    assert_eq!(contract.attrs.data_enabled, None);
    assert_eq!(contract.state.state, "disabled");
    assert!(contract.state.is_disabled);
    assert!(!contract.state.is_enabled);
}
