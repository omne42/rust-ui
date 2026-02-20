use super::*;
use leptos::prelude::*;

#[test]
fn normalize_helpers_trim_and_fallback() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Rename profile  ".to_string())),
        Some("Rename profile".to_string())
    );

    assert_eq!(
        normalize_aria_label(Some("  Danger action  ".to_string())),
        ("Danger action".to_string(), true)
    );
    assert_eq!(
        normalize_aria_label(Some("\n\t".to_string())),
        (DEFAULT_ARIA_LABEL.into(), false)
    );
}

#[test]
fn kind_contract_is_stable() {
    assert_eq!(
        resolve_selection_indicator(MenuItemKind::Action),
        MenuItemSelectionIndicator::Hidden
    );
    assert_eq!(
        resolve_selection_indicator(MenuItemKind::Checkbox {
            is_checked: Signal::derive(|| false),
        }),
        MenuItemSelectionIndicator::Checkbox
    );
    assert_eq!(
        resolve_selection_indicator(MenuItemKind::Radio {
            is_checked: Signal::derive(|| false),
        }),
        MenuItemSelectionIndicator::Radio
    );

    assert_eq!(resolve_kind_attr(MenuItemKind::Action), "action");
    assert_eq!(
        resolve_kind_class(MenuItemKind::Action),
        "ui-menu-item--kind-action"
    );
}

#[test]
fn resolve_state_tracks_kind_and_state_sources() {
    let state = resolve_state(MenuItemStateInput {
        kind: MenuItemKind::Radio {
            is_checked: Signal::derive(|| true),
        },
        is_checked: true,
        disabled: false,
        focused: true,
        has_submenu: true,
        has_custom_aria_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.role_attr, "menuitemradio");
    assert_eq!(state.kind_attr, "radio");
    assert!(state.is_checkable);
    assert!(state.is_checked);
    assert!(state.is_focused);
    assert!(state.has_submenu);
    assert_eq!(state.data_state_attr, "focused-checked");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "custom");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let state = resolve_state(MenuItemStateInput {
        kind: MenuItemKind::Checkbox {
            is_checked: Signal::derive(|| true),
        },
        is_checked: true,
        disabled: false,
        focused: false,
        has_submenu: false,
        has_custom_aria_label: false,
        has_custom_class_name: true,
    });

    let class_name = compose_class_name(Some("docs-menu-item-custom".to_string()), state);

    for needle in [
        "ui-menu-item",
        "ui-menu-item--kind-checkbox",
        "ui-menu-item--checkable",
        "ui-menu-item--checked",
        "ui-menu-item--custom-class",
        "docs-menu-item-custom",
    ] {
        assert!(
            class_name.contains(needle),
            "MenuItem class list should include `{needle}`"
        );
    }
}

#[test]
fn checked_and_aria_checked_reflect_kind_state() {
    let (checked, set_checked) = signal(true);
    let checkbox = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || checked.get()),
    };

    assert!(resolve_checked(checkbox));
    assert_eq!(resolve_aria_checked(checkbox), Some("true"));

    set_checked.set(false);
    assert!(!resolve_checked(checkbox));
    assert_eq!(resolve_aria_checked(checkbox), Some("false"));

    assert!(!resolve_checked(MenuItemKind::Action));
    assert_eq!(resolve_aria_checked(MenuItemKind::Action), None);
}
