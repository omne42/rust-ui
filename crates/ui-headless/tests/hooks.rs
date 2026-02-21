use leptos::prelude::*;
use ui_headless::{
    ButtonElement, ButtonOptions, CheckboxOptions, FocusRingOptions, HoverOptions, MenuItemKind,
    MenuItemOptions, MenuOptions, PressActivationKeys, PressOptions, SwatchOptions, SwitchOptions,
    use_button, use_checkbox, use_focus_ring, use_hover, use_menu, use_menu_item,
    use_overlay_stack_registration, use_press, use_swatch, use_switch,
};

fn init_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}

fn poll_effects() {
    any_spawner::Executor::poll_local();
}

#[test]
fn hover_tracks_pointer_enter_leave() {
    init_executor();
    let hover = use_hover(HoverOptions { is_disabled: false });

    assert!(!hover.is_hovered.get_untracked());
    hover.handlers.on_pointer_enter.run(());
    assert!(hover.is_hovered.get_untracked());
    hover.handlers.on_pointer_leave.run(());
    assert!(!hover.is_hovered.get_untracked());
}

#[test]
fn hover_ignores_pointer_enter_when_disabled() {
    init_executor();
    let hover = use_hover(HoverOptions { is_disabled: true });

    hover.handlers.on_pointer_enter.run(());
    assert!(!hover.is_hovered.get_untracked());
}

#[test]
fn focus_ring_respects_global_focus_visible_modality() {
    init_executor();
    Owner::new().with(|| {
        let focus_visible = ui_headless::provide_focus_visible();
        let ring = use_focus_ring(FocusRingOptions { is_disabled: false });

        // Default modality is Pointer.
        ring.handlers.on_focus.run(());
        assert!(!ring.is_focus_visible.get_untracked());

        focus_visible.set_modality(ui_headless::Modality::Keyboard);
        assert!(ring.is_focus_visible.get_untracked());

        focus_visible.set_modality(ui_headless::Modality::Pointer);
        assert!(!ring.is_focus_visible.get_untracked());
    });
}

#[test]
fn button_attrs_match_element_kind() {
    init_executor();
    let native = use_button(ButtonOptions {
        element: ButtonElement::Button,
        ..Default::default()
    });
    assert_eq!(native.attrs.role, None);
    assert_eq!(native.attrs.tabindex, None);
    assert_eq!(native.attrs.aria_disabled, None);

    let custom = use_button(ButtonOptions {
        element: ButtonElement::Custom,
        ..Default::default()
    });
    assert_eq!(custom.attrs.role, Some("button"));
    assert_eq!(custom.attrs.tabindex, Some(0));
    assert_eq!(custom.attrs.aria_disabled, None);

    let custom_disabled = use_button(ButtonOptions {
        element: ButtonElement::Custom,
        is_disabled: true,
        ..Default::default()
    });
    assert_eq!(custom_disabled.attrs.role, Some("button"));
    assert_eq!(custom_disabled.attrs.tabindex, Some(-1));
    assert_eq!(custom_disabled.attrs.aria_disabled, Some("true"));

    let native_disabled = use_button(ButtonOptions {
        element: ButtonElement::Button,
        is_disabled: true,
        ..Default::default()
    });
    assert_eq!(native_disabled.attrs.role, None);
    assert_eq!(native_disabled.attrs.tabindex, None);
    assert_eq!(native_disabled.attrs.aria_disabled, Some("true"));
}

#[test]
fn button_press_state_and_click_deduping() {
    init_executor();

    let pressed_count = StoredValue::new(0_usize);

    let aria = use_button(ButtonOptions {
        on_press: Some(Callback::new(move |_| {
            pressed_count.update_value(|n| *n += 1)
        })),
        ..Default::default()
    });

    assert!(!aria.is_pressed.get_untracked());

    aria.handlers.press.on_pointer_down.run(());
    assert!(aria.is_pressed.get_untracked());
    assert_eq!(pressed_count.get_value(), 0);

    aria.handlers.press.on_pointer_up.run(());
    assert!(!aria.is_pressed.get_untracked());
    assert_eq!(pressed_count.get_value(), 1);

    // Pointer path should ignore the subsequent click.
    aria.handlers.press.on_click.run(());
    assert_eq!(pressed_count.get_value(), 1);
}

#[test]
fn button_keyboard_enter_triggers_once_and_suppresses_click() {
    init_executor();

    let pressed_count = StoredValue::new(0_usize);

    let aria = use_button(ButtonOptions {
        element: ButtonElement::Button,
        on_press: Some(Callback::new(move |_| {
            pressed_count.update_value(|n| *n += 1)
        })),
        ..Default::default()
    });

    let prevent_default = aria.handlers.press.on_key_down.run("Enter".to_string());
    assert!(!prevent_default);
    assert!(aria.is_pressed.get_untracked());
    assert_eq!(pressed_count.get_value(), 1);

    // Native buttons may fire click after Enter; the hook must ignore it.
    aria.handlers.press.on_click.run(());
    assert_eq!(pressed_count.get_value(), 1);

    aria.handlers.press.on_key_up.run("Enter".to_string());
    assert!(!aria.is_pressed.get_untracked());
}

#[test]
fn button_keyboard_space_triggers_on_key_up_and_can_request_prevent_default_for_custom() {
    init_executor();

    let pressed_count = StoredValue::new(0_usize);

    let aria = use_button(ButtonOptions {
        element: ButtonElement::Custom,
        on_press: Some(Callback::new(move |_| {
            pressed_count.update_value(|n| *n += 1)
        })),
        ..Default::default()
    });

    let prevent_default = aria.handlers.press.on_key_down.run(" ".to_string());
    assert!(prevent_default);
    assert!(aria.is_pressed.get_untracked());
    assert_eq!(pressed_count.get_value(), 0);

    let prevent_default = aria.handlers.press.on_key_up.run(" ".to_string());
    assert!(prevent_default);
    assert!(!aria.is_pressed.get_untracked());
    assert_eq!(pressed_count.get_value(), 1);
}

#[test]
fn checkbox_attrs_reflect_checked_state_and_disabled() {
    init_executor();
    let (checked, set_checked) = signal(false);

    let toggles = StoredValue::new(0_usize);
    let on_press = Callback::new(move |_| {
        toggles.update_value(|n| *n += 1);
        set_checked.update(|v| *v = !*v);
    });

    let aria = use_checkbox(CheckboxOptions {
        is_disabled: false,
        is_checked: checked,
        on_press: Some(on_press),
        lang: None,
        dir: None,
    });

    assert_eq!(aria.attrs.role, "checkbox");
    assert_eq!(aria.attrs.tabindex, 0);
    assert_eq!(aria.attrs.aria_disabled, None);
    assert_eq!(aria.attrs.aria_checked.get_untracked(), "false");

    aria.handlers.press.on_click.run(());
    assert_eq!(toggles.get_value(), 1);
    assert_eq!(aria.attrs.aria_checked.get_untracked(), "true");

    let aria_disabled = use_checkbox(CheckboxOptions {
        is_disabled: true,
        is_checked: checked,
        on_press: Some(on_press),
        lang: None,
        dir: None,
    });
    assert_eq!(aria_disabled.attrs.tabindex, -1);
    assert_eq!(aria_disabled.attrs.aria_disabled, Some("true"));
}

#[test]
fn switch_attrs_reflect_checked_state_and_disabled() {
    init_executor();
    let (checked, set_checked) = signal(false);

    let toggles = StoredValue::new(0_usize);
    let on_press = Callback::new(move |_| {
        toggles.update_value(|n| *n += 1);
        set_checked.update(|v| *v = !*v);
    });

    let aria = use_switch(SwitchOptions {
        is_disabled: false,
        is_checked: checked,
        on_press: Some(on_press),
        lang: None,
        dir: None,
    });

    assert_eq!(aria.attrs.role, "switch");
    assert_eq!(aria.attrs.tabindex, 0);
    assert_eq!(aria.attrs.aria_disabled, None);
    assert_eq!(aria.attrs.aria_checked.get_untracked(), "false");

    aria.handlers.press.on_click.run(());
    assert_eq!(toggles.get_value(), 1);
    assert_eq!(aria.attrs.aria_checked.get_untracked(), "true");

    let aria_disabled = use_switch(SwitchOptions {
        is_disabled: true,
        is_checked: checked,
        on_press: Some(on_press),
        lang: None,
        dir: None,
    });
    assert_eq!(aria_disabled.attrs.tabindex, -1);
    assert_eq!(aria_disabled.attrs.aria_disabled, Some("true"));
}

#[test]
fn swatch_attrs_reflect_mixed_and_decorative_state() {
    init_executor();
    let (selected, _set_selected) = signal(false);

    let swatch = use_swatch(SwatchOptions {
        is_disabled: false,
        is_decorative: false,
        is_mixed_value: true,
        is_selected: selected.into(),
        aria_label: Some("Mixed".to_string()),
        on_press: None,
        lang: Some(" en-US ".to_string()),
        dir: Some(ui_headless::A11yDirection::Ltr),
    });

    assert_eq!(swatch.attrs.role, Some("button"));
    assert_eq!(swatch.attrs.tabindex, None);
    assert_eq!(swatch.attrs.aria_checked, Some("mixed"));
    assert_eq!(swatch.attrs.aria_pressed.get_untracked(), None);
    assert_eq!(swatch.attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(swatch.attrs.dir, Some("ltr"));

    let decorative = use_swatch(SwatchOptions {
        is_disabled: false,
        is_decorative: true,
        is_mixed_value: false,
        is_selected: selected.into(),
        aria_label: Some("Decorative".to_string()),
        on_press: None,
        lang: None,
        dir: None,
    });

    assert_eq!(decorative.attrs.role, None);
    assert_eq!(decorative.attrs.aria_hidden, Some("true"));
    assert_eq!(decorative.attrs.aria_label, None);
}

#[test]
fn listbox_selects_active_option_and_calls_on_action() {
    init_executor();

    let (count, _set_count) = signal(3_usize);
    let (selected, set_selected) = signal(None::<usize>);

    let action_calls: StoredValue<Vec<usize>> = StoredValue::new(Vec::new());
    let on_action = Callback::new(move |index: usize| {
        action_calls.update_value(|v| v.push(index));
    });

    let aria = ui_headless::use_listbox(ui_headless::ListBoxOptions {
        is_disabled: false,
        should_loop: true,
        id_base: "demo".to_string(),
        default_index: 0,
        sync_active_index_to_selected: true,
        item_count: count,
        selected_index: selected,
        set_selected_index: set_selected,
        on_action: Some(on_action),
        is_item_disabled: Some(Callback::new(|index: usize| index == 1)),
        item_text: Some(Callback::new(|index: usize| match index {
            0 => "Apple".to_string(),
            1 => "Banana".to_string(),
            2 => "Cherry".to_string(),
            _ => "".to_string(),
        })),
    });

    poll_effects();

    assert_eq!(aria.option_id.run(2), "demo-option-2");
    assert_eq!(
        aria.attrs.aria_activedescendant.get_untracked(),
        Some("demo-option-0".to_string())
    );

    // Disabled option ignores click.
    aria.handlers.on_option_click.run(1);
    assert_eq!(selected.get_untracked(), None);

    // Typeahead should skip disabled (Banana) and land on Cherry.
    assert!(aria.handlers.on_key_down.run("c".to_string()));
    assert_eq!(aria.active_index.get_untracked(), 2);
    assert_eq!(
        aria.attrs.aria_activedescendant.get_untracked(),
        Some("demo-option-2".to_string())
    );

    // Enter selects the active option.
    assert!(aria.handlers.on_key_down.run("Enter".to_string()));
    assert_eq!(selected.get_untracked(), Some(2));
    assert_eq!(action_calls.get_value(), vec![2]);
}

#[test]
fn menu_calls_on_action_for_active_item_and_menu_item_roles() {
    init_executor();

    let (count, _set_count) = signal(3_usize);

    let action_calls: StoredValue<Vec<usize>> = StoredValue::new(Vec::new());
    let on_action = Callback::new(move |index: usize| action_calls.update_value(|v| v.push(index)));

    let aria = use_menu(MenuOptions {
        is_disabled: false,
        should_loop: true,
        id_base: "demo".to_string(),
        item_count: count,
        default_index: 0,
        on_action: Some(on_action),
        is_item_disabled: Some(Callback::new(|index: usize| index == 1)),
        item_text: Some(Callback::new(|index: usize| match index {
            0 => "New".to_string(),
            1 => "Open".to_string(),
            2 => "Save".to_string(),
            _ => "".to_string(),
        })),
    });

    poll_effects();
    assert_eq!(aria.option_id.run(2), "demo-item-2");
    assert_eq!(
        aria.attrs.aria_activedescendant.get_untracked(),
        Some("demo-item-0".to_string())
    );

    // Arrow navigation skips disabled item 1.
    assert!(aria.handlers.on_key_down.run("ArrowDown".to_string()));
    assert_eq!(aria.active_index.get_untracked(), 2);

    assert!(aria.handlers.on_key_down.run("Enter".to_string()));
    assert_eq!(action_calls.get_value(), vec![2]);

    let (checked, set_checked) = signal(false);
    let item = use_menu_item(
        &aria,
        MenuItemOptions {
            index: 0,
            kind: MenuItemKind::Checkbox {
                is_checked: checked.into(),
            },
            is_disabled: false,
        },
    );
    assert_eq!(item.attrs.role, "menuitemcheckbox");
    assert_eq!(item.attrs.aria_checked.get_untracked(), Some("false"));
    set_checked.set(true);
    assert_eq!(item.attrs.aria_checked.get_untracked(), Some("true"));
}

#[test]
fn overlay_stack_defaults_to_topmost_without_provider() {
    init_executor();
    let reg = use_overlay_stack_registration();
    assert!(reg.is_topmost.get_untracked());
}

#[test]
fn overlay_stack_tracks_topmost_registration() {
    init_executor();
    Owner::new().with(|| {
        ui_headless::provide_overlay_stack();

        let a = use_overlay_stack_registration();
        let b = use_overlay_stack_registration();

        poll_effects();
        assert!(!a.is_topmost.get_untracked());
        assert!(b.is_topmost.get_untracked());
    });
}

#[test]
fn press_respects_activation_keys_and_prevent_default() {
    init_executor();

    let pressed_count = StoredValue::new(0_usize);
    let on_press = Callback::new(move |_| pressed_count.update_value(|n| *n += 1));

    let press = use_press(PressOptions {
        on_press: Some(on_press),
        activation_keys: PressActivationKeys::ENTER,
        prevent_default_for_keyboard: true,
        ..Default::default()
    });

    // Space is ignored.
    assert!(!press.handlers.on_key_down.run(" ".to_string()));
    assert!(!press.handlers.on_key_up.run(" ".to_string()));
    assert_eq!(pressed_count.get_value(), 0);

    // Enter triggers immediately and asks to prevent default.
    assert!(press.handlers.on_key_down.run("Enter".to_string()));
    assert!(press.is_pressed.get_untracked());
    assert_eq!(pressed_count.get_value(), 1);
}
