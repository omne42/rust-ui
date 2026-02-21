use super::*;

#[test]
fn checkbox_group_contract_exposes_attrs_state_and_locale() {
    let (aria_describedby, set_aria_describedby) = signal(Some(" external-id ".to_string()));
    let (is_invalid, set_invalid) = signal(false);
    let (is_required, set_required) = signal(false);

    let contract = use_checkbox_group(CheckboxGroupOptions {
        id: "prefs".to_string(),
        is_disabled: false,
        has_description: true,
        has_error: true,
        aria_describedby: aria_describedby.into(),
        is_invalid: is_invalid.into(),
        is_required: is_required.into(),
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.fieldset.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.fieldset.dir, Some("rtl"));
    assert_eq!(contract.attrs.description.id, "prefs-description");
    assert_eq!(contract.attrs.error.id, "prefs-error");
    assert_eq!(
        contract
            .attrs
            .fieldset
            .aria_describedby
            .get_untracked()
            .as_deref(),
        Some("prefs-description external-id")
    );

    let state = contract.state.resolved.get_untracked();
    assert!(state.is_enabled);
    assert!(state.has_description);
    assert!(state.has_error);
    assert!(!state.shows_error);
    assert!(!state.is_required);

    set_invalid.set(true);
    set_required.set(true);
    set_aria_describedby.set(Some("ext-a ext-b".to_string()));

    assert_eq!(
        contract.attrs.fieldset.aria_invalid.get_untracked(),
        Some("true")
    );
    assert_eq!(
        contract.attrs.fieldset.aria_required.get_untracked(),
        Some("true")
    );
    assert_eq!(
        contract
            .attrs
            .fieldset
            .aria_describedby
            .get_untracked()
            .as_deref(),
        Some("prefs-description prefs-error ext-a ext-b")
    );

    let state = contract.state.resolved.get_untracked();
    assert!(state.is_invalid);
    assert!(state.shows_error);
    assert!(state.has_messages);
    assert!(state.is_required);
}

#[test]
fn checkbox_group_contract_disables_semantic_state() {
    let (aria_describedby, _) = signal(None::<String>);
    let (is_invalid, _) = signal(false);
    let (is_required, _) = signal(false);

    let contract = use_checkbox_group(CheckboxGroupOptions {
        id: "disabled".to_string(),
        is_disabled: true,
        has_description: false,
        has_error: false,
        aria_describedby: aria_describedby.into(),
        is_invalid: is_invalid.into(),
        is_required: is_required.into(),
        lang: None,
        dir: None,
    });

    let state = contract.state.resolved.get_untracked();
    assert!(state.is_disabled);
    assert!(!state.is_enabled);
    assert!(!state.has_messages);
}
