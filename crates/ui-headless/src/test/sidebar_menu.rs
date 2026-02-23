use super::*;

#[test]
fn resolve_sidebar_menu_key_action_maps_shortcut_navigation_and_activate() {
    let shortcut = SidebarMenuKeyDownInput {
        key: "k".to_string(),
        ctrl_key: true,
        meta_key: false,
    };
    assert_eq!(
        resolve_sidebar_menu_key_action(&shortcut, Some("k"), false),
        SidebarMenuKeyAction::FocusFirst
    );

    let next = SidebarMenuKeyDownInput {
        key: "ArrowDown".to_string(),
        ctrl_key: false,
        meta_key: false,
    };
    assert_eq!(
        resolve_sidebar_menu_key_action(&next, None, false),
        SidebarMenuKeyAction::MoveNext
    );

    let end = SidebarMenuKeyDownInput {
        key: "End".to_string(),
        ctrl_key: false,
        meta_key: false,
    };
    assert_eq!(
        resolve_sidebar_menu_key_action(&end, None, false),
        SidebarMenuKeyAction::End
    );

    let activate = SidebarMenuKeyDownInput {
        key: "Enter".to_string(),
        ctrl_key: false,
        meta_key: false,
    };
    assert_eq!(
        resolve_sidebar_menu_key_action(&activate, None, false),
        SidebarMenuKeyAction::Activate
    );
}

#[test]
fn resolve_sidebar_menu_key_action_uses_disabled_guard() {
    let input = SidebarMenuKeyDownInput {
        key: "ArrowDown".to_string(),
        ctrl_key: false,
        meta_key: false,
    };
    assert_eq!(
        resolve_sidebar_menu_key_action(&input, None, true),
        SidebarMenuKeyAction::None
    );
}

#[test]
fn use_sidebar_menu_keyboard_exposes_typed_attrs_handlers_and_state() {
    let contract = use_sidebar_menu_keyboard(SidebarMenuKeyboardOptions {
        is_disabled: false,
        shortcut_key: Some("k".to_string()),
    });

    assert_eq!(contract.attrs.aria_keyshortcuts.as_deref(), Some("Ctrl+k"));
    assert_eq!(contract.attrs.shortcut_source_attr, "provided");
    assert!(contract.state.has_shortcut_key);
    assert_eq!(contract.state.shortcut_source_attr, "provided");

    let action = contract.handlers.on_key_down.run(SidebarMenuKeyDownInput {
        key: "ArrowUp".to_string(),
        ctrl_key: false,
        meta_key: false,
    });
    assert_eq!(action, SidebarMenuKeyAction::MovePrevious);
}
