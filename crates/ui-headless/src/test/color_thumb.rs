use super::*;

fn make_state(disabled: bool) -> ColorThumbState {
    ColorThumbState {
        is_disabled: disabled,
        is_focused: false,
        is_dragging: false,
        loupe_visible: false,
        has_color: false,
        x_percent: 50.0,
        y_percent: 50.0,
        x_bucket_class: "ui-color-thumb--x-center",
        y_bucket_class: "ui-color-thumb--y-center",
        x_bucket_attr: "center",
        y_bucket_attr: "center",
        data_state_attr: if disabled { "disabled" } else { "idle" },
        interaction_source_attr: "external",
        aria_source_attr: "default",
        aria_value_text_source_attr: "default",
        class_source_attr: "default",
        loupe_source_attr: "default",
        x_source_attr: "default",
        y_source_attr: "default",
        has_custom_class_name: false,
    }
}

#[test]
fn color_thumb_root_attrs_include_locale_and_state_markers() {
    let contract = use_color_thumb(ColorThumbOptions {
        state: ColorThumbState {
            is_disabled: false,
            is_focused: true,
            is_dragging: true,
            loupe_visible: true,
            has_color: true,
            x_percent: 18.0,
            y_percent: 72.0,
            x_bucket_class: "ui-color-thumb--x-start",
            y_bucket_class: "ui-color-thumb--y-end",
            x_bucket_attr: "start",
            y_bucket_attr: "end",
            data_state_attr: "dragging",
            interaction_source_attr: "external",
            aria_source_attr: "custom",
            aria_value_text_source_attr: "custom",
            class_source_attr: "custom",
            loupe_source_attr: "external",
            x_source_attr: "external",
            y_source_attr: "external",
            has_custom_class_name: true,
        },
        aria_label: "Accent thumb".to_string(),
        aria_value_text: "#09f".to_string(),
        lang: Some(" en-US ".to_string()),
        dir: Some(A11yDirection::Rtl),
    });

    assert_eq!(contract.root_attrs.role, "slider");
    assert_eq!(contract.root_attrs.tabindex, 0);
    assert_eq!(contract.root_attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(contract.root_attrs.dir, Some("rtl"));
    assert_eq!(contract.root_attrs.data_state, "dragging");
    assert_eq!(contract.root_attrs.data_x_bucket, "start");
    assert_eq!(contract.root_attrs.data_y_bucket, "end");
    assert_eq!(contract.root_attrs.data_interaction_source, "external");
    assert_eq!(contract.root_attrs.data_aria_source, "custom");
    assert_eq!(contract.root_attrs.data_aria_valuetext_source, "custom");
    assert_eq!(contract.root_attrs.data_class_source, "custom");
    assert_eq!(contract.root_attrs.data_loupe_source, "external");
    assert_eq!(contract.root_attrs.data_x_source, "external");
    assert_eq!(contract.root_attrs.data_y_source, "external");
}

#[test]
fn color_thumb_keyboard_contract_prevents_slider_navigation_defaults() {
    let enabled = use_color_thumb(ColorThumbOptions {
        state: make_state(false),
        aria_label: "Color thumb".to_string(),
        aria_value_text: "None".to_string(),
        lang: None,
        dir: None,
    });
    assert!(enabled.handlers.on_key_down.run("ArrowLeft".to_string()));
    assert!(enabled.handlers.on_key_down.run("End".to_string()));
    assert!(!enabled.handlers.on_key_down.run("Enter".to_string()));

    let disabled = use_color_thumb(ColorThumbOptions {
        state: make_state(true),
        aria_label: "Color thumb".to_string(),
        aria_value_text: "None".to_string(),
        lang: None,
        dir: None,
    });
    assert!(!disabled.handlers.on_key_down.run("ArrowLeft".to_string()));
}

#[test]
fn color_thumb_pointer_and_focus_handlers_are_callable() {
    let enabled = use_color_thumb(ColorThumbOptions {
        state: make_state(false),
        aria_label: "Color thumb".to_string(),
        aria_value_text: "None".to_string(),
        lang: None,
        dir: None,
    });

    enabled.handlers.on_pointer_down.run(());
    enabled.handlers.on_pointer_up.run(());
    enabled.handlers.on_pointer_cancel.run(());
    enabled.handlers.on_focus.run(());
    enabled.handlers.on_blur.run(());

    let disabled = use_color_thumb(ColorThumbOptions {
        state: make_state(true),
        aria_label: "Color thumb".to_string(),
        aria_value_text: "None".to_string(),
        lang: None,
        dir: None,
    });
    disabled.handlers.on_pointer_down.run(());
    disabled.handlers.on_pointer_up.run(());
    disabled.handlers.on_pointer_cancel.run(());
    disabled.handlers.on_focus.run(());
    disabled.handlers.on_blur.run(());
}
