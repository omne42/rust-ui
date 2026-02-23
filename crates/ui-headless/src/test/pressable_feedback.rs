use super::*;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn pressable_feedback_contract_maps_locale_and_button_attrs() {
    let contract = use_pressable_feedback_a11y(PressableFeedbackA11yOptions {
        is_disabled: false,
        on_press: None,
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, Some("button"));
    assert_eq!(contract.attrs.tabindex, Some(0));
    assert_eq!(contract.attrs.aria_disabled, None);
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert!(!contract.state.is_pressed.get_untracked());
}

#[test]
fn pressable_feedback_contract_disables_interaction_semantics() {
    let contract = use_pressable_feedback_a11y(PressableFeedbackA11yOptions {
        is_disabled: true,
        on_press: None,
        lang: None,
        dir: None,
    });

    assert_eq!(contract.attrs.role, Some("button"));
    assert_eq!(contract.attrs.tabindex, Some(-1));
    assert_eq!(contract.attrs.aria_disabled, Some("true"));
}

#[test]
fn pressable_feedback_contract_delegates_press_handlers() {
    let pressed_count = Arc::new(AtomicUsize::new(0));
    let pressed_count_for_handler = Arc::clone(&pressed_count);

    let contract = use_pressable_feedback_a11y(PressableFeedbackA11yOptions {
        is_disabled: false,
        on_press: Some(Callback::new(move |_| {
            pressed_count_for_handler.fetch_add(1, Ordering::SeqCst);
        })),
        lang: None,
        dir: None,
    });

    assert!(!contract.state.is_pressed.get_untracked());

    contract.handlers.button.press.on_pointer_down.run(());
    assert!(contract.state.is_pressed.get_untracked());

    contract.handlers.button.press.on_pointer_up.run(());
    assert!(!contract.state.is_pressed.get_untracked());
    assert_eq!(pressed_count.load(Ordering::SeqCst), 1);

    let prevented = contract
        .handlers
        .button
        .press
        .on_key_down
        .run("Enter".to_string());
    assert!(prevented);
    assert_eq!(pressed_count.load(Ordering::SeqCst), 2);
}
