use super::*;

#[test]
fn use_sidebar_root_maps_attrs_and_triggers_shortcut_handler() {
    let (toggled, set_toggled) = signal(false);
    let contract = use_sidebar_root(SidebarRootOptions {
        is_disabled: false,
        shortcut_key: Some("b".to_string()),
        aria_label: "Workspace sidebar".to_string(),
        lang: Some("  en-US ".to_string()),
        dir: Some(A11yDirection::Rtl),
        on_shortcut_toggle: Some(Callback::new(move |_| set_toggled.set(true))),
    });

    assert_eq!(contract.attrs.role, "complementary");
    assert_eq!(contract.attrs.aria_label, "Workspace sidebar");
    assert_eq!(contract.attrs.aria_keyshortcuts.as_deref(), Some("Ctrl+b"));
    assert_eq!(contract.attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert!(contract.state.has_shortcut_key);
    assert_eq!(contract.state.shortcut_source_attr, "provided");

    let handled = contract.handlers.on_key_down.run(SidebarKeyDownInput {
        key: "B".to_string(),
        ctrl_key: true,
        meta_key: false,
    });
    assert!(handled);
    assert!(toggled.get_untracked());
}

#[test]
fn should_toggle_for_shortcut_respects_modifier_and_disabled_guards() {
    let matched = SidebarKeyDownInput {
        key: "b".to_string(),
        ctrl_key: true,
        meta_key: false,
    };
    assert!(should_toggle_for_shortcut(&matched, Some("b"), false));

    let no_modifier = SidebarKeyDownInput {
        key: "b".to_string(),
        ctrl_key: false,
        meta_key: false,
    };
    assert!(!should_toggle_for_shortcut(&no_modifier, Some("b"), false));
    assert!(!should_toggle_for_shortcut(&matched, Some("b"), true));
    assert!(!should_toggle_for_shortcut(&matched, None, false));
}

#[test]
fn sidebar_toggle_button_a11y_attrs_maps_expanded_and_locale() {
    let (open, set_open) = signal(false);
    let attrs = sidebar_toggle_button_a11y_attrs(
        open.into(),
        SidebarToggleButtonA11yOptions {
            is_disabled: true,
            aria_label: "Toggle sidebar".to_string(),
            lang: Some("  zh-CN ".to_string()),
            dir: Some(A11yDirection::Ltr),
        },
    );

    assert_eq!(attrs.aria_disabled, Some("true"));
    assert_eq!(attrs.aria_expanded.get_untracked(), "false");
    assert_eq!(attrs.aria_label, "Toggle sidebar");
    assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(attrs.dir, Some("ltr"));

    set_open.set(true);
    assert_eq!(attrs.aria_expanded.get_untracked(), "true");
}
