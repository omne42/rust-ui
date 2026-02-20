use super::*;

fn init_executor() {
    drop(any_spawner::Executor::init_futures_executor());
}

#[test]
fn escape_reports_stop_propagation_when_open() {
    init_executor();

    let (is_open, set_open) = signal(true);
    let (item_count, _) = signal(2usize);
    let aria = use_combo_box(ComboBoxOptions {
        is_disabled: false,
        id_base: "city".to_string(),
        is_open: is_open.into(),
        set_open: Callback::new(move |next| set_open.set(next)),
        item_count,
        selected_index: Signal::derive(|| None),
        on_action: None,
        is_item_disabled: None,
        lang: None,
        dir: None,
    });

    let result = aria.handlers.on_input_key_down.run("Escape".to_string());
    assert!(result.handled);
    assert!(result.stop_propagation);
    assert!(!is_open.get_untracked());
}

#[test]
fn controls_are_present_only_when_open() {
    init_executor();

    let (is_open, set_open) = signal(false);
    let (item_count, _) = signal(1usize);
    let aria = use_combo_box(ComboBoxOptions {
        is_disabled: false,
        id_base: "lang".to_string(),
        is_open: is_open.into(),
        set_open: Callback::new(move |next| set_open.set(next)),
        item_count,
        selected_index: Signal::derive(|| None),
        on_action: None,
        is_item_disabled: None,
        lang: Some(" en-US ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(aria.input.aria_controls.get_untracked(), None);
    assert_eq!(aria.input.lang.as_deref(), Some("en-US"));
    assert_eq!(aria.input.dir, Some("rtl"));
    assert_eq!(aria.listbox.lang.as_deref(), Some("en-US"));
    assert_eq!(aria.listbox.dir, Some("rtl"));

    set_open.set(true);
    assert_eq!(
        aria.input.aria_controls.get_untracked(),
        Some("lang-listbox".to_string())
    );
}
