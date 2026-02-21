use super::*;

#[test]
fn resolve_accessible_name_prefers_explicit_aria_label() {
    assert_eq!(
        resolve_accessible_name(
            Some("  File actions  ".to_string()),
            Some("trigger-id".to_string())
        ),
        MenuAccessibleName {
            aria_label: Some("File actions".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_accessible_name_uses_labelledby_when_label_missing() {
    assert_eq!(
        resolve_accessible_name(None, Some("  trigger-id  ".to_string())),
        MenuAccessibleName {
            aria_label: None,
            aria_labelledby: Some("trigger-id".to_string()),
        }
    );
}

#[test]
fn resolve_accessible_name_defaults_when_none_provided() {
    assert_eq!(
        resolve_accessible_name(None, None),
        MenuAccessibleName {
            aria_label: Some("Menu".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_accessible_name_ignores_blank_inputs() {
    assert_eq!(
        resolve_accessible_name(Some("  ".to_string()), Some("".to_string())),
        MenuAccessibleName {
            aria_label: Some("Menu".to_string()),
            aria_labelledby: None,
        }
    );
}

#[test]
fn resolve_state_tracks_item_checked_and_disabled_flags() {
    let state = resolve_state(3, true, true);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(state.has_checked_items);
    assert!(state.has_disabled_items);
}

#[test]
fn resolve_state_handles_empty_menu() {
    let state = resolve_state(0, false, false);
    assert!(state.is_empty);
    assert!(!state.has_items);
    assert!(!state.has_checked_items);
    assert!(!state.has_disabled_items);
}

#[test]
fn normalize_props_centralizes_disabled_and_class_defaults() {
    assert_eq!(
        normalize_props(MenuNormalizeInput {
            is_disabled: Some(true),
            disabled: false,
            class_name: Some("  docs-menu  ".to_string()),
        }),
        MenuNormalizedProps {
            disabled: true,
            class_name: "ui-menu docs-menu".to_string(),
        }
    );

    assert_eq!(
        normalize_props(MenuNormalizeInput {
            is_disabled: None,
            disabled: false,
            class_name: Some("  ".to_string()),
        }),
        MenuNormalizedProps {
            disabled: false,
            class_name: "ui-menu".to_string(),
        }
    );
}

#[test]
fn item_kind_and_text_fallbacks_are_centralized() {
    assert_eq!(resolve_item_kind(&[], 0), ui_headless::MenuItemKind::Action);
    assert_eq!(resolve_item_text(&["Open".to_string()], 1), String::new());
}

#[test]
fn normalize_menu_items_prefers_typed_item_specs() {
    let output = normalize_menu_items(MenuItemsInput {
        item_specs: vec![
            MenuItemSpec::action("Open"),
            MenuItemSpec::action("Share").with_disabled(true),
        ],
        items: vec!["legacy".to_string()].into(),
        item_kinds: vec![ui_headless::MenuItemKind::Action],
        disabled_indices: vec![99],
    });

    assert!(output.has_item_specs);
    assert_eq!(output.item_count, 2);
    assert_eq!(
        output.items.as_ref(),
        &["Open".to_string(), "Share".to_string()]
    );
    assert_eq!(
        output.item_kinds,
        vec![
            ui_headless::MenuItemKind::Action,
            ui_headless::MenuItemKind::Action,
        ]
    );
    assert_eq!(output.disabled_indices, vec![1]);
}

#[test]
fn normalize_menu_items_keeps_legacy_arrays_when_specs_empty() {
    let output = normalize_menu_items(MenuItemsInput {
        item_specs: Vec::new(),
        items: vec!["Open".to_string(), "Share".to_string()].into(),
        item_kinds: vec![
            ui_headless::MenuItemKind::Action,
            ui_headless::MenuItemKind::Checkbox {
                is_checked: leptos::prelude::Signal::derive(|| true),
            },
        ],
        disabled_indices: vec![1, 3],
    });

    assert!(!output.has_item_specs);
    assert_eq!(output.item_count, 2);
    assert_eq!(output.disabled_indices, vec![1]);
}
