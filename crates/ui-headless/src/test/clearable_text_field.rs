use super::*;

fn options(
    is_disabled: bool,
    is_read_only: bool,
    is_clearable: bool,
    is_empty: Signal<bool>,
    on_clear: Option<Callback<()>>,
) -> ClearableTextFieldOptions {
    ClearableTextFieldOptions {
        is_disabled,
        is_read_only,
        is_clearable,
        is_empty,
        on_clear,
    }
}

#[test]
fn escape_clears_when_enabled_and_non_empty() {
    let (is_empty, set_is_empty) = signal(false);
    let (clear_count, set_clear_count) = signal(0usize);
    let on_clear = Callback::new(move |_| set_clear_count.update(|value| *value += 1));
    let clearable =
        use_clearable_text_field(options(false, false, true, is_empty.into(), Some(on_clear)));

    assert!(clearable.state.can_clear_on_escape.get_untracked());
    assert_eq!(
        clearable.attrs.aria_keyshortcuts.get_untracked(),
        Some("Escape")
    );

    assert!(clearable.handlers.on_key_down.run("Escape".to_string()));
    assert_eq!(clear_count.get_untracked(), 1);

    set_is_empty.set(true);
    assert!(!clearable.state.can_clear_on_escape.get_untracked());
    assert_eq!(clearable.attrs.aria_keyshortcuts.get_untracked(), None);
}

#[test]
fn non_escape_key_is_not_handled() {
    let (is_empty, _) = signal(false);
    let clearable = use_clearable_text_field(options(
        false,
        false,
        true,
        is_empty.into(),
        Some(Callback::new(move |_| {})),
    ));

    assert!(!clearable.handlers.on_key_down.run("Enter".to_string()));
}

#[test]
fn escape_is_ignored_when_clearing_is_not_allowed() {
    let (is_empty, _) = signal(false);
    let (clear_count, set_clear_count) = signal(0usize);
    let on_clear = Callback::new(move |_| set_clear_count.update(|value| *value += 1));

    let disabled =
        use_clearable_text_field(options(true, false, true, is_empty.into(), Some(on_clear)));
    assert!(!disabled.handlers.on_key_down.run("Escape".to_string()));
    assert_eq!(clear_count.get_untracked(), 0);

    let read_only =
        use_clearable_text_field(options(false, true, true, is_empty.into(), Some(on_clear)));
    assert!(!read_only.handlers.on_key_down.run("Escape".to_string()));
    assert_eq!(clear_count.get_untracked(), 0);

    let not_clearable = use_clearable_text_field(options(
        false,
        false,
        false,
        is_empty.into(),
        Some(on_clear),
    ));
    assert!(!not_clearable.handlers.on_key_down.run("Escape".to_string()));
    assert_eq!(clear_count.get_untracked(), 0);
}
