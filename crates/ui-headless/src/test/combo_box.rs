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

#[test]
fn option_attrs_are_derived_from_headless_selection_focus_and_disabled_state() {
    init_executor();

    let (is_open, set_open) = signal(true);
    let (item_count, _set_item_count) = signal(3usize);
    let (selected_index, set_selected_index) = signal(Some(1usize));

    let aria = use_combo_box(ComboBoxOptions {
        is_disabled: false,
        id_base: "city".to_string(),
        is_open: is_open.into(),
        set_open: Callback::new(move |next| set_open.set(next)),
        item_count,
        selected_index: selected_index.into(),
        on_action: None,
        is_item_disabled: Some(Callback::new(|index| index == 2)),
        lang: None,
        dir: None,
    });

    let selected = aria.option_attrs.run(1);
    assert_eq!(selected.role, "option");
    assert_eq!(selected.aria_selected, Some("true"));
    assert_eq!(selected.data_selected, Some("true"));
    assert_eq!(selected.data_focused, None);
    assert_eq!(selected.aria_disabled, None);
    assert_eq!(selected.data_disabled, None);

    let active = aria.option_attrs.run(0);
    assert_eq!(active.data_focused, Some("true"));

    let disabled = aria.option_attrs.run(2);
    assert_eq!(disabled.aria_selected, None);
    assert_eq!(disabled.aria_disabled, Some("true"));
    assert_eq!(disabled.data_disabled, Some("true"));

    set_selected_index.set(Some(0));
    let updated = aria.option_attrs.run(0);
    assert_eq!(updated.aria_selected, Some("true"));
}
