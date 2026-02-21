use super::*;

#[test]
fn color_wheel_contract_maps_locale_and_aria_attrs() {
    let (value, _) = signal(42.0);
    let contract = use_color_wheel(ColorWheelOptions {
        is_disabled: false,
        value: value.into(),
        step: 5.0,
        aria_label: "Hue wheel".to_string(),
        label_id: "wheel-label".to_string(),
        value_id: Some("wheel-value".to_string()),
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.root_attrs.role, "group");
    assert_eq!(contract.root_attrs.aria_labelledby, "wheel-label");
    assert_eq!(contract.root_attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.root_attrs.dir, Some("rtl"));

    assert_eq!(contract.input_attrs.role, "slider");
    assert_eq!(contract.input_attrs.aria_label, "Hue wheel");
    assert_eq!(contract.input_attrs.aria_labelledby, "wheel-label");
    assert_eq!(
        contract.input_attrs.aria_describedby.as_deref(),
        Some("wheel-value")
    );
    assert_eq!(contract.input_attrs.aria_disabled, None);
    assert_eq!(contract.input_attrs.aria_valuemin, "0");
    assert_eq!(contract.input_attrs.aria_valuemax, "359");
}

#[test]
fn color_wheel_handlers_normalize_keyboard_and_input_values() {
    let (value, _) = signal(355.0);
    let contract = use_color_wheel(ColorWheelOptions {
        is_disabled: false,
        value: value.into(),
        step: 5.0,
        aria_label: "Hue wheel".to_string(),
        label_id: "wheel-label".to_string(),
        value_id: None,
        lang: None,
        dir: None,
    });

    assert_eq!(contract.handlers.on_input.run("14".to_string()), Some(15.0));
    assert_eq!(contract.handlers.on_input.run("invalid".to_string()), None);

    let up = match contract.handlers.on_key_down.run("ArrowUp".to_string()) {
        Some(next) => next,
        None => panic!("arrow up should produce next value"),
    };
    assert_eq!(up.next_value, 0.0);
    assert!(up.prevent_default);

    let end = match contract.handlers.on_key_down.run("End".to_string()) {
        Some(next) => next,
        None => panic!("end should produce max"),
    };
    assert_eq!(end.next_value, 359.0);

    assert_eq!(contract.handlers.on_key_down.run("Enter".to_string()), None);
}

#[test]
fn color_wheel_pointer_handlers_gate_drag_state() {
    let (value, _) = signal(120.0);
    let contract = use_color_wheel(ColorWheelOptions {
        is_disabled: false,
        value: value.into(),
        step: 1.0,
        aria_label: "Hue wheel".to_string(),
        label_id: "wheel-label".to_string(),
        value_id: None,
        lang: None,
        dir: None,
    });

    assert_eq!(contract.track_attrs.data_dragging.get_untracked(), None);
    assert_eq!(contract.handlers.on_track_pointer_move.run(120.0), None);

    assert_eq!(
        contract.handlers.on_track_pointer_down.run(80.0),
        Some(80.0)
    );
    assert_eq!(
        contract.track_attrs.data_dragging.get_untracked(),
        Some("true")
    );
    assert_eq!(
        contract.handlers.on_track_pointer_move.run(83.0),
        Some(83.0)
    );

    contract.handlers.on_track_pointer_up.run(());
    assert_eq!(contract.track_attrs.data_dragging.get_untracked(), None);
}

#[test]
fn disabled_color_wheel_contract_rejects_mutating_events() {
    let (value, _) = signal(120.0);
    let contract = use_color_wheel(ColorWheelOptions {
        is_disabled: true,
        value: value.into(),
        step: 1.0,
        aria_label: "Hue wheel".to_string(),
        label_id: "wheel-label".to_string(),
        value_id: None,
        lang: None,
        dir: None,
    });

    assert_eq!(contract.input_attrs.aria_disabled, Some("true"));
    assert_eq!(contract.handlers.on_input.run("180".to_string()), None);
    assert_eq!(
        contract.handlers.on_key_down.run("ArrowUp".to_string()),
        None
    );
    assert_eq!(contract.handlers.on_track_pointer_down.run(180.0), None);
}
