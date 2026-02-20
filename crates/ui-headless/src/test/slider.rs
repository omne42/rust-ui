use super::*;

#[test]
fn slider_contract_sanitizes_input_and_updates_value() {
    let (value, set_value) = signal(0.0);
    let slider = use_slider(SliderOptions {
        is_disabled: false,
        value: Some(value.into()),
        default_value: None,
        on_value_change: Some(Callback::new(move |next| set_value.set(next))),
        min: 0.0,
        max: 100.0,
        step: 5.0,
        lang: Some("  zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(slider.input.role, "slider");
    assert_eq!(slider.input.lang.as_deref(), Some("zh-CN"));
    assert_eq!(slider.input.dir, Some("rtl"));
    assert_eq!(slider.input.aria_valuemin.get_untracked(), "0");
    assert_eq!(slider.input.aria_valuemax.get_untracked(), "100");
    assert_eq!(slider.input.aria_valuenow.get_untracked(), "0");
    assert_eq!(slider.input.aria_valuetext.get_untracked(), "0%");

    slider.handlers.on_input.run("22".to_string());
    assert_eq!(value.get_untracked(), 20.0);
    assert_eq!(slider.input.aria_valuenow.get_untracked(), "20");
    assert_eq!(slider.input.aria_valuetext.get_untracked(), "20%");
}

#[test]
fn disabled_slider_ignores_input_and_clears_interaction_flags() {
    let (value, set_value) = signal(10.0);
    let slider = use_slider(SliderOptions {
        is_disabled: true,
        value: Some(value.into()),
        default_value: None,
        on_value_change: Some(Callback::new(move |next| set_value.set(next))),
        min: 0.0,
        max: 100.0,
        step: 1.0,
        lang: None,
        dir: None,
    });

    slider.handlers.on_pointer_down.run(());
    slider.handlers.on_pointer_enter.run(());
    slider.handlers.on_focus.run(());
    slider.handlers.on_input.run("80".to_string());

    assert_eq!(value.get_untracked(), 10.0);
    assert_eq!(slider.input.aria_disabled, Some("true"));
    assert!(!slider.state.is_pressed.get_untracked());
    assert!(!slider.state.is_hovered.get_untracked());
    assert!(!slider.state.is_focused.get_untracked());
    assert!(!slider.state.is_focus_visible.get_untracked());
}

#[test]
fn blur_handler_resets_focus_and_press_state() {
    let (value, set_value) = signal(30.0);
    let slider = use_slider(SliderOptions {
        is_disabled: false,
        value: Some(value.into()),
        default_value: None,
        on_value_change: Some(Callback::new(move |next| set_value.set(next))),
        min: 0.0,
        max: 100.0,
        step: 1.0,
        lang: None,
        dir: None,
    });

    slider.handlers.on_pointer_down.run(());
    slider.handlers.on_focus.run(());
    assert!(slider.state.is_pressed.get_untracked());
    assert!(slider.state.is_focused.get_untracked());

    slider.handlers.on_blur.run(());
    assert!(!slider.state.is_pressed.get_untracked());
    assert!(!slider.state.is_focused.get_untracked());
}

#[test]
fn uncontrolled_slider_uses_default_value_and_internal_updates() {
    let slider = use_slider(SliderOptions {
        is_disabled: false,
        value: None,
        default_value: Some(42.0),
        on_value_change: None,
        min: 0.0,
        max: 100.0,
        step: 1.0,
        lang: None,
        dir: None,
    });

    assert_eq!(slider.input.aria_valuenow.get_untracked(), "42");
    slider.handlers.on_input.run("58".to_string());
    assert_eq!(slider.input.aria_valuenow.get_untracked(), "58");
    assert_eq!(slider.input.aria_valuetext.get_untracked(), "58%");
}
