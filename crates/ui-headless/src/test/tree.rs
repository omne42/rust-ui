use super::*;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn root_attrs_expose_role_label_and_locale() {
    let attrs = tree_root_attrs(
        "Files".to_string(),
        Some(" en-US ".to_string()),
        Some(A11yDirection::Rtl),
    );
    assert_eq!(attrs.role, "tree");
    assert_eq!(attrs.aria_label, "Files");
    assert_eq!(attrs.lang.as_deref(), Some("en-US"));
    assert_eq!(attrs.dir, Some("rtl"));
}

#[test]
fn tree_item_contract_wires_attrs_and_handlers() {
    let selected_calls = Arc::new(AtomicUsize::new(0));
    let toggle_calls = Arc::new(AtomicUsize::new(0));
    let selected_calls_2 = Arc::clone(&selected_calls);
    let toggle_calls_2 = Arc::clone(&toggle_calls);

    let contract = use_tree_item(
        TreeItemA11yInput {
            depth: 2,
            has_children: true,
            is_expanded: true,
            is_selected: false,
            is_disabled: false,
            is_tree_disabled: false,
            is_first_visible: false,
        },
        TreeItemOptions {
            on_select: Callback::new(move |_| {
                selected_calls_2.fetch_add(1, Ordering::SeqCst);
            }),
            on_toggle: Some(Callback::new(move |_| {
                toggle_calls_2.fetch_add(1, Ordering::SeqCst);
            })),
        },
    );

    assert_eq!(contract.attrs.role, "treeitem");
    assert_eq!(contract.attrs.aria_level, 3);
    assert_eq!(contract.attrs.aria_expanded, Some("true"));
    assert_eq!(contract.attrs.aria_selected, "false");
    assert_eq!(contract.attrs.aria_disabled, None);
    assert_eq!(contract.attrs.tabindex, -1);
    assert!(contract.state.is_interactive);
    assert!(contract.state.toggles_expansion);

    contract.handlers.on_click.run(());
    assert_eq!(selected_calls.load(Ordering::SeqCst), 1);
    assert_eq!(toggle_calls.load(Ordering::SeqCst), 1);

    let consumed = contract.handlers.on_key_down.run("Enter".to_string());
    assert!(consumed);
    assert_eq!(selected_calls.load(Ordering::SeqCst), 2);
    assert_eq!(toggle_calls.load(Ordering::SeqCst), 2);
}

#[test]
fn disabled_item_does_not_emit_interactions() {
    let selected_calls = Arc::new(AtomicUsize::new(0));
    let selected_calls_2 = Arc::clone(&selected_calls);

    let contract = use_tree_item(
        TreeItemA11yInput {
            depth: 0,
            has_children: false,
            is_expanded: false,
            is_selected: true,
            is_disabled: true,
            is_tree_disabled: false,
            is_first_visible: false,
        },
        TreeItemOptions {
            on_select: Callback::new(move |_| {
                selected_calls_2.fetch_add(1, Ordering::SeqCst);
            }),
            on_toggle: None,
        },
    );

    assert!(!contract.state.is_interactive);
    assert!(!contract.state.toggles_expansion);
    assert_eq!(contract.attrs.aria_expanded, None);
    assert_eq!(contract.attrs.aria_selected, "true");
    assert_eq!(contract.attrs.aria_disabled, Some("true"));
    assert_eq!(contract.attrs.tabindex, 0);

    contract.handlers.on_click.run(());
    assert_eq!(selected_calls.load(Ordering::SeqCst), 0);
    assert!(!contract.handlers.on_key_down.run(" ".to_string()));
    assert_eq!(selected_calls.load(Ordering::SeqCst), 0);
}
