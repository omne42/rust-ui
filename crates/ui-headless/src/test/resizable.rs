use super::*;

#[test]
fn resizable_contract_exposes_locale_and_separator_attrs() {
    let (split, _set_split) = signal(48.0_f64);
    let contract = use_resizable(ResizableOptions {
        orientation: ResizableOrientation::Horizontal,
        split_percent: split.into(),
        bounds: SplitBounds {
            min: 20.0,
            max: 80.0,
        },
        is_disabled: false,
        is_controlled: false,
        with_handle: true,
        has_custom_class_name: false,
        aria_label: "Pane splitter".to_string(),
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
        on_split_percent_change: Callback::new(|_| {}),
    });

    assert_eq!(contract.handle_attrs.role, "separator");
    assert_eq!(contract.handle_attrs.tabindex, 0);
    assert_eq!(contract.handle_attrs.aria_label, "Pane splitter");
    assert_eq!(
        contract.handle_attrs.aria_orientation.get_untracked(),
        "horizontal"
    );
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.handle_attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.handle_attrs.dir, Some("rtl"));
}

#[test]
fn pointer_and_keyboard_handlers_update_split_value() {
    let (split, set_split) = signal(50.0_f64);
    let contract = use_resizable(ResizableOptions {
        orientation: ResizableOrientation::Horizontal,
        split_percent: split.into(),
        bounds: SplitBounds {
            min: 25.0,
            max: 75.0,
        },
        is_disabled: false,
        is_controlled: true,
        with_handle: true,
        has_custom_class_name: false,
        aria_label: "Splitter".to_string(),
        lang: None,
        dir: None,
        on_split_percent_change: Callback::new(move |next| set_split.set(next)),
    });

    assert!(contract.handlers.on_handle_pointer_down.run((100.0, 0.0)));
    assert!(contract.state.is_dragging.get_untracked());

    contract
        .handlers
        .on_pointer_move
        .run((200.0, 0.0, 200.0, 100.0));
    assert_eq!(split.get_untracked(), 75.0);

    assert!(
        contract
            .handlers
            .on_handle_key_down
            .run(("ArrowLeft".to_string(), false))
    );
    assert_eq!(split.get_untracked(), 73.0);

    contract.handlers.on_pointer_up.run(());
    assert!(!contract.state.is_dragging.get_untracked());
}

#[test]
fn disabled_contract_is_noop_for_interaction_handlers() {
    let (split, set_split) = signal(44.0_f64);
    let contract = use_resizable(ResizableOptions {
        orientation: ResizableOrientation::Vertical,
        split_percent: split.into(),
        bounds: SplitBounds {
            min: 10.0,
            max: 90.0,
        },
        is_disabled: true,
        is_controlled: false,
        with_handle: false,
        has_custom_class_name: false,
        aria_label: "Splitter".to_string(),
        lang: None,
        dir: None,
        on_split_percent_change: Callback::new(move |next| set_split.set(next)),
    });

    assert!(!contract.handlers.on_handle_pointer_down.run((10.0, 10.0)));
    contract
        .handlers
        .on_pointer_move
        .run((10.0, 60.0, 100.0, 100.0));
    assert!(
        !contract
            .handlers
            .on_handle_key_down
            .run(("ArrowDown".to_string(), false))
    );
    assert_eq!(split.get_untracked(), 44.0);
    assert_eq!(contract.handle_attrs.tabindex, -1);
    assert_eq!(
        contract.handle_attrs.aria_disabled.get_untracked(),
        Some("true")
    );
}
