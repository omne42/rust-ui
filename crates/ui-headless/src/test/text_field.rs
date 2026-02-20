use super::*;

#[test]
fn text_field_contract_exposes_locale_and_derived_state() {
    let (value, set_value) = signal("  ".to_string());
    let (is_invalid, set_invalid) = signal(false);
    let (is_required, set_required) = signal(false);

    let contract = use_text_field_contract(TextFieldContractOptions {
        is_disabled: false,
        is_read_only: false,
        value: value.into(),
        on_value_change: Callback::new(move |next| set_value.set(next)),
        is_invalid: is_invalid.into(),
        is_required: is_required.into(),
        lang: Some("  en-US ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    let state = contract.state.resolved.get_untracked();
    assert_eq!(state.state_attr, "ready");
    assert_eq!(state.value_attr, "empty");
    assert_eq!(state.requirement_attr, "optional");

    contract.handlers.on_input.run("hello".to_string());
    set_invalid.set(true);
    set_required.set(true);

    let state = contract.state.resolved.get_untracked();
    assert_eq!(state.state_attr, "invalid");
    assert_eq!(state.value_attr, "filled");
    assert_eq!(state.requirement_attr, "required");
}

#[test]
fn text_field_contract_focus_handlers_respect_disabled() {
    let (value, _) = signal("value".to_string());
    let (is_invalid, _) = signal(false);
    let (is_required, _) = signal(false);

    let enabled = use_text_field_contract(TextFieldContractOptions {
        is_disabled: false,
        is_read_only: false,
        value: value.into(),
        on_value_change: Callback::new(move |_| {}),
        is_invalid: is_invalid.into(),
        is_required: is_required.into(),
        lang: None,
        dir: None,
    });

    enabled.handlers.focus_ring.on_focus.run(());
    assert!(enabled.state.is_focused.get_untracked());
    assert_eq!(enabled.state.resolved.get_untracked().state_attr, "ready");

    let (value, _) = signal("value".to_string());
    let (is_invalid, _) = signal(false);
    let (is_required, _) = signal(false);
    let disabled = use_text_field_contract(TextFieldContractOptions {
        is_disabled: true,
        is_read_only: false,
        value: value.into(),
        on_value_change: Callback::new(move |_| {}),
        is_invalid: is_invalid.into(),
        is_required: is_required.into(),
        lang: None,
        dir: None,
    });

    disabled.handlers.focus_ring.on_focus.run(());
    assert!(!disabled.state.is_focused.get_untracked());
    assert_eq!(
        disabled.state.resolved.get_untracked().state_attr,
        "disabled"
    );
}

#[test]
fn text_field_contract_input_handler_respects_disabled_and_read_only() {
    let (value, set_value) = signal("before".to_string());
    let (is_invalid, _) = signal(false);
    let (is_required, _) = signal(false);
    let enabled = use_text_field_contract(TextFieldContractOptions {
        is_disabled: false,
        is_read_only: false,
        value: value.into(),
        on_value_change: Callback::new(move |next| set_value.set(next)),
        is_invalid: is_invalid.into(),
        is_required: is_required.into(),
        lang: None,
        dir: None,
    });

    enabled.handlers.on_input.run("after".to_string());
    assert_eq!(enabled.state.resolved.get_untracked().value_attr, "filled");

    let (disabled_value, set_disabled_value) = signal("locked".to_string());
    let (is_invalid, _) = signal(false);
    let (is_required, _) = signal(false);
    let disabled = use_text_field_contract(TextFieldContractOptions {
        is_disabled: true,
        is_read_only: false,
        value: disabled_value.into(),
        on_value_change: Callback::new(move |next| set_disabled_value.set(next)),
        is_invalid: is_invalid.into(),
        is_required: is_required.into(),
        lang: None,
        dir: None,
    });
    disabled.handlers.on_input.run("ignored".to_string());
    assert_eq!(disabled.state.resolved.get_untracked().value_attr, "filled");

    let (readonly_value, set_readonly_value) = signal("readonly".to_string());
    let (is_invalid, _) = signal(false);
    let (is_required, _) = signal(false);
    let read_only = use_text_field_contract(TextFieldContractOptions {
        is_disabled: false,
        is_read_only: true,
        value: readonly_value.into(),
        on_value_change: Callback::new(move |next| set_readonly_value.set(next)),
        is_invalid: is_invalid.into(),
        is_required: is_required.into(),
        lang: None,
        dir: None,
    });
    read_only.handlers.on_input.run("ignored".to_string());
    assert_eq!(
        read_only.state.resolved.get_untracked().value_attr,
        "filled"
    );
}
