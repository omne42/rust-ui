use super::*;

#[test]
fn resolve_selection_intent_respects_keyboard_activation() {
    let current = 0;
    let candidate = 1;
    let is_disabled = |_| false;

    let manual_focus = resolve_tabs_selection_intent(
        current,
        candidate,
        3,
        is_disabled,
        TabsKeyboardActivation::Manual,
        TabsInteractionKind::Focus,
    );
    assert_eq!(manual_focus, None);

    let automatic_focus = resolve_tabs_selection_intent(
        current,
        candidate,
        3,
        is_disabled,
        TabsKeyboardActivation::Automatic,
        TabsInteractionKind::Focus,
    );
    assert_eq!(automatic_focus, Some(candidate));
}

#[test]
fn resolve_selection_intent_ignores_disabled_candidate() {
    let next = resolve_tabs_selection_intent(
        0,
        1,
        3,
        |idx| idx == 1,
        TabsKeyboardActivation::Automatic,
        TabsInteractionKind::Press,
    );

    assert_eq!(next, None);
}

#[test]
fn list_attrs_expose_locale_and_trimmed_label() {
    let attrs = tabs_list_a11y_attrs(
        Some("  Main tabs  ".to_string()),
        Some("  en-US ".to_string()),
        Some(A11yDirection::Rtl),
    );

    assert_eq!(attrs.role, "tablist");
    assert_eq!(attrs.aria_label.as_deref(), Some("Main tabs"));
    assert_eq!(attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(attrs.dir, Some("rtl"));
}

#[test]
fn tab_attrs_track_selected_and_locale() {
    let (selected, set_selected) = signal(false);
    let attrs = tabs_tab_a11y_attrs(
        selected.into(),
        "panel-1".to_string(),
        true,
        Some(" zh-CN ".to_string()),
        Some(A11yDirection::Ltr),
    );

    assert_eq!(attrs.role, "tab");
    assert_eq!(attrs.aria_selected.get_untracked(), "false");
    assert_eq!(attrs.aria_controls, "panel-1");
    assert_eq!(attrs.aria_disabled, Some("true"));
    assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(attrs.dir, Some("ltr"));

    set_selected.set(true);
    assert_eq!(attrs.aria_selected.get_untracked(), "true");
}
