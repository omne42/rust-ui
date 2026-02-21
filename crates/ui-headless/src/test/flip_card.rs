use super::*;
use crate::A11yDirection;

#[test]
fn key_down_contract_only_toggles_for_enter_or_space_when_allowed() {
    assert_eq!(
        resolve_flip_card_key_down("Enter", false, false),
        FlipCardKeyDownResult::ToggleAndPreventDefault
    );
    assert_eq!(
        resolve_flip_card_key_down(" ", false, false),
        FlipCardKeyDownResult::ToggleAndPreventDefault
    );
    assert_eq!(
        resolve_flip_card_key_down("Space", false, false),
        FlipCardKeyDownResult::ToggleAndPreventDefault
    );
    assert_eq!(
        resolve_flip_card_key_down("Enter", false, true),
        FlipCardKeyDownResult::Ignored
    );
    assert_eq!(
        resolve_flip_card_key_down("Escape", false, false),
        FlipCardKeyDownResult::Ignored
    );
}

#[test]
fn flip_card_contract_maps_locale_and_aria_attrs() {
    let (is_flipped, set_is_flipped) = signal(false);
    let contract = use_flip_card(FlipCardOptions {
        is_disabled: false,
        is_flipped: Signal::derive(move || is_flipped.get()),
        request_is_flipped_change: Callback::new(move |next| set_is_flipped.set(next)),
        flip_on_hover: false,
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.attrs.role, "button");
    assert_eq!(contract.attrs.tabindex, 0);
    assert!(!contract.attrs.aria_pressed.get_untracked());
    assert_eq!(contract.attrs.aria_disabled, None);
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
}

#[test]
fn flip_card_handlers_toggle_and_normalize_hover_focus_state() {
    let (is_flipped, set_is_flipped) = signal(false);
    let contract = use_flip_card(FlipCardOptions {
        is_disabled: false,
        is_flipped: Signal::derive(move || is_flipped.get()),
        request_is_flipped_change: Callback::new(move |next| set_is_flipped.set(next)),
        flip_on_hover: true,
        lang: None,
        dir: None,
    });

    contract.handlers.on_click.run(());
    assert!(contract.state.is_flipped.get_untracked());

    let prevented = contract
        .handlers
        .on_key_down
        .run(("Enter".to_string(), false));
    assert!(prevented);
    assert!(!contract.state.is_flipped.get_untracked());

    contract.handlers.on_pointer_enter.run(());
    assert!(contract.state.is_hovered.get_untracked());
    assert!(contract.state.is_flipped.get_untracked());

    contract.handlers.on_focus.run(());
    assert!(contract.state.is_focused.get_untracked());

    contract.handlers.on_pointer_leave.run(());
    assert!(contract.state.is_hovered.get_untracked());
    assert!(!contract.state.is_flipped.get_untracked());

    contract.handlers.on_blur.run(());
    assert!(!contract.state.is_focused.get_untracked());
    assert!(!contract.state.is_hovered.get_untracked());
}

#[test]
fn disabled_flip_card_does_not_toggle_or_enter_hover() {
    let (is_flipped, set_is_flipped) = signal(false);
    let contract = use_flip_card(FlipCardOptions {
        is_disabled: true,
        is_flipped: Signal::derive(move || is_flipped.get()),
        request_is_flipped_change: Callback::new(move |next| set_is_flipped.set(next)),
        flip_on_hover: true,
        lang: None,
        dir: None,
    });

    contract.handlers.on_click.run(());
    let prevented = contract
        .handlers
        .on_key_down
        .run(("Enter".to_string(), false));
    contract.handlers.on_pointer_enter.run(());

    assert!(!prevented);
    assert!(!contract.state.is_flipped.get_untracked());
    assert!(!contract.state.is_hovered.get_untracked());
    assert_eq!(contract.attrs.aria_disabled, Some("true"));
    assert_eq!(contract.attrs.tabindex, -1);
}
