use super::*;

#[test]
fn switch_contract_exposes_derived_state_and_locale() {
    let (is_checked, set_checked) = signal(true);
    let switch = use_switch(SwitchOptions {
        is_disabled: false,
        is_checked,
        on_press: None,
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(switch.attrs.role, "switch");
    assert_eq!(switch.attrs.tabindex, 0);
    assert_eq!(switch.attrs.aria_checked.get_untracked(), "true");
    assert_eq!(switch.attrs.aria_disabled, None);
    assert_eq!(switch.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(switch.attrs.dir, Some("rtl"));

    let resolved = switch.state.resolved.get_untracked();
    assert!(resolved.is_checked);
    assert_eq!(resolved.data_state(), "checked");

    set_checked.set(false);
    let resolved = switch.state.resolved.get_untracked();
    assert!(resolved.is_unchecked);
    assert_eq!(switch.attrs.aria_checked.get_untracked(), "false");
}

#[test]
fn switch_contract_normalizes_interaction_state_when_disabled() {
    let (is_checked, _) = signal(false);
    let switch = use_switch(SwitchOptions {
        is_disabled: true,
        is_checked,
        on_press: None,
        lang: None,
        dir: None,
    });

    switch.handlers.press.on_pointer_down.run(());
    switch.handlers.hover.on_pointer_enter.run(());
    switch.handlers.focus_ring.on_focus.run(());

    let resolved = switch.state.resolved.get_untracked();
    assert!(resolved.is_disabled);
    assert!(!resolved.is_enabled);
    assert!(!resolved.is_pressed);
    assert!(!resolved.is_hovered);
    assert!(!resolved.is_focused);
    assert!(!resolved.is_focus_visible);
    assert_eq!(switch.attrs.tabindex, -1);
    assert_eq!(switch.attrs.aria_disabled, Some("true"));
}
