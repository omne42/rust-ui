use super::*;
use crate::a11y::A11yDirection;

#[test]
fn checkbox_contract_exposes_typed_attrs_handlers_state_and_locale() {
    let (is_checked, set_checked) = signal(true);
    let checkbox = use_checkbox(CheckboxOptions {
        is_disabled: false,
        is_checked,
        on_press: None,
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(checkbox.attrs.role, "checkbox");
    assert_eq!(checkbox.attrs.tabindex, 0);
    assert_eq!(checkbox.attrs.aria_checked.get_untracked(), "true");
    assert_eq!(checkbox.attrs.aria_disabled, None);
    assert_eq!(checkbox.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(checkbox.attrs.dir, Some("rtl"));

    checkbox.handlers.hover.on_pointer_enter.run(());
    assert!(checkbox.state.is_hovered.get_untracked());
    checkbox.handlers.hover.on_pointer_leave.run(());
    assert!(!checkbox.state.is_hovered.get_untracked());

    checkbox.handlers.focus_ring.on_focus.run(());
    assert!(checkbox.state.is_focused.get_untracked());
    checkbox.handlers.focus_ring.on_blur.run(());
    assert!(!checkbox.state.is_focused.get_untracked());

    set_checked.set(false);
    assert_eq!(checkbox.attrs.aria_checked.get_untracked(), "false");
}

#[test]
fn checkbox_contract_normalizes_interaction_state_when_disabled() {
    let (is_checked, _set_checked) = signal(false);
    let checkbox = use_checkbox(CheckboxOptions {
        is_disabled: true,
        is_checked,
        on_press: None,
        lang: None,
        dir: None,
    });

    checkbox.handlers.press.on_pointer_down.run(());
    checkbox.handlers.hover.on_pointer_enter.run(());
    checkbox.handlers.focus_ring.on_focus.run(());

    assert!(!checkbox.state.is_pressed.get_untracked());
    assert!(!checkbox.state.is_hovered.get_untracked());
    assert!(!checkbox.state.is_focused.get_untracked());
    assert!(!checkbox.state.is_focus_visible.get_untracked());
    assert_eq!(checkbox.attrs.tabindex, -1);
    assert_eq!(checkbox.attrs.aria_disabled, Some("true"));
}
