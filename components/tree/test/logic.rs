use super::*;
use crate::TreeDensity;
use leptos::prelude::Callable;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn tone_class_names_and_attrs_are_stable() {
    assert_eq!(TreeTone::Default.class_name(), "ui-tree--tone-default");
    assert_eq!(TreeTone::Quiet.class_name(), "ui-tree--tone-quiet");
    assert_eq!(TreeTone::Strong.class_name(), "ui-tree--tone-strong");

    assert_eq!(TreeTone::Default.as_attr(), "default");
    assert_eq!(TreeTone::Quiet.as_attr(), "quiet");
    assert_eq!(TreeTone::Strong.as_attr(), "strong");
}

#[test]
fn normalize_aria_label_uses_default_when_empty() {
    let (label, custom) =
        normalize_aria_label_with_fallback(Some("  ".to_string()), DEFAULT_ARIA_LABEL);
    assert_eq!(label, DEFAULT_ARIA_LABEL);
    assert!(!custom);

    let (label, custom) = normalize_aria_label_with_fallback(
        Some("  Explorer Tree ".to_string()),
        DEFAULT_ARIA_LABEL,
    );
    assert_eq!(label, "Explorer Tree");
    assert!(custom);
}

#[test]
fn normalize_is_disabled_prefers_is_prefixed_input() {
    assert!(normalize_is_disabled(Some(true), false));
    assert!(!normalize_is_disabled(Some(false), true));
    assert!(normalize_is_disabled(None, true));
    assert!(!normalize_is_disabled(None, false));
}

#[test]
fn normalize_expanded_ids_change_handler_prefers_new_name_and_falls_back_to_alias() {
    let new_called = Arc::new(AtomicUsize::new(0));
    let old_called = Arc::new(AtomicUsize::new(0));
    let new_called_2 = Arc::clone(&new_called);
    let old_called_2 = Arc::clone(&old_called);

    let new_handler = Callback::new(move |_: BTreeSet<String>| {
        new_called_2.fetch_add(1, Ordering::SeqCst);
    });
    let old_handler = Callback::new(move |_: BTreeSet<String>| {
        old_called_2.fetch_add(1, Ordering::SeqCst);
    });

    let handler =
        normalize_expanded_ids_change_handler(Some(new_handler), Some(old_handler)).unwrap();
    handler.run(BTreeSet::from(["root".to_string()]));
    assert_eq!(new_called.load(Ordering::SeqCst), 1);
    assert_eq!(old_called.load(Ordering::SeqCst), 0);

    let old_called_3 = Arc::new(AtomicUsize::new(0));
    let old_called_4 = Arc::clone(&old_called_3);
    let old_handler = Callback::new(move |_: BTreeSet<String>| {
        old_called_4.fetch_add(1, Ordering::SeqCst);
    });
    let handler = normalize_expanded_ids_change_handler(None, Some(old_handler)).unwrap();
    handler.run(BTreeSet::new());
    assert_eq!(old_called_3.load(Ordering::SeqCst), 1);
}

#[test]
fn normalize_selected_id_change_handler_prefers_new_name_and_falls_back_to_alias() {
    let new_called = Arc::new(AtomicUsize::new(0));
    let old_called = Arc::new(AtomicUsize::new(0));
    let new_called_2 = Arc::clone(&new_called);
    let old_called_2 = Arc::clone(&old_called);

    let new_handler = Callback::new(move |_: Option<String>| {
        new_called_2.fetch_add(1, Ordering::SeqCst);
    });
    let old_handler = Callback::new(move |_: Option<String>| {
        old_called_2.fetch_add(1, Ordering::SeqCst);
    });

    let handler =
        normalize_selected_id_change_handler(Some(new_handler), Some(old_handler)).unwrap();
    handler.run(Some("child".to_string()));
    assert_eq!(new_called.load(Ordering::SeqCst), 1);
    assert_eq!(old_called.load(Ordering::SeqCst), 0);

    let old_called_3 = Arc::new(AtomicUsize::new(0));
    let old_called_4 = Arc::clone(&old_called_3);
    let old_handler = Callback::new(move |_: Option<String>| {
        old_called_4.fetch_add(1, Ordering::SeqCst);
    });
    let handler = normalize_selected_id_change_handler(None, Some(old_handler)).unwrap();
    handler.run(None);
    assert_eq!(old_called_3.load(Ordering::SeqCst), 1);
}

#[test]
fn normalize_expanded_axis_centralizes_default_and_handler_priority() {
    let expandable_ids = BTreeSet::from(["root".to_string()]);
    let default_expanded_ids = BTreeSet::from(["root".to_string(), "leaf".to_string()]);
    let called = Arc::new(AtomicUsize::new(0));
    let called_2 = Arc::clone(&called);

    let normalized = normalize_expanded_axis(
        TreeExpandedAxisInput {
            is_controlled: true,
            default_expanded_ids: Some(default_expanded_ids),
            on_expanded_ids_change: Some(Callback::new(move |_| {
                called_2.fetch_add(1, Ordering::SeqCst);
            })),
            on_expanded_change: Some(Callback::new(|_| {})),
        },
        &expandable_ids,
    );

    assert_eq!(
        normalized.default_expanded_ids,
        BTreeSet::from(["root".to_string()])
    );
    assert_eq!(normalized.control_mode, TreeControlMode::Controlled);
    assert_eq!(normalized.default_source, TreeDefaultSource::Provided);
    assert_eq!(normalized.change_source, TreeChangeSource::Provided);
    normalized
        .on_expanded_change
        .expect("normalized handler should exist")
        .run(BTreeSet::new());
    assert_eq!(called.load(Ordering::SeqCst), 1);
}

#[test]
fn normalize_selected_axis_centralizes_default_and_handler_priority() {
    let all_ids = BTreeSet::from(["child".to_string()]);
    let called = Arc::new(AtomicUsize::new(0));
    let called_2 = Arc::clone(&called);

    let normalized = normalize_selected_axis(
        TreeSelectedAxisInput {
            is_controlled: false,
            default_selected_id: Some("  child ".to_string()),
            on_selected_id_change: Some(Callback::new(move |_| {
                called_2.fetch_add(1, Ordering::SeqCst);
            })),
            on_selected_change: Some(Callback::new(|_| {})),
        },
        &all_ids,
    );

    assert_eq!(normalized.default_selected_id.as_deref(), Some("child"));
    assert_eq!(normalized.control_mode, TreeControlMode::Uncontrolled);
    assert_eq!(normalized.default_source, TreeDefaultSource::Provided);
    assert_eq!(normalized.change_source, TreeChangeSource::Provided);
    normalized
        .on_selected_change
        .expect("normalized handler should exist")
        .run(None);
    assert_eq!(called.load(Ordering::SeqCst), 1);
}

#[test]
fn derive_state_centralizes_runtime_normalization_and_visibility() {
    let nodes =
        vec![TreeNode::new("root", "Root").with_children(vec![TreeNode::new("child", "Child")])];
    let derived = derive_state(TreeDerivedStateInput {
        nodes,
        expanded_ids: BTreeSet::from(["root".to_string(), "missing".to_string()]),
        selected_id: Some("missing".to_string()),
        expandable_ids: BTreeSet::from(["root".to_string()]),
        all_ids: BTreeSet::from(["root".to_string(), "child".to_string()]),
        is_disabled: false,
    });

    assert_eq!(derived.expanded_ids, BTreeSet::from(["root".to_string()]));
    assert_eq!(derived.selected_id, None);
    assert_eq!(derived.visible_nodes.len(), 2);
}

#[test]
fn resolve_expanded_toggle_request_centralizes_event_derivation() {
    let expandable_ids = BTreeSet::from(["root".to_string()]);
    let next = resolve_expanded_toggle_request(
        BTreeSet::from(["root".to_string(), "invalid".to_string()]),
        "root",
        &expandable_ids,
    );
    assert!(next.is_empty());
}

#[test]
fn resolve_state_tracks_counts_sources_and_flags() {
    let state = resolve_state(TreeStateInput {
        tone: TreeTone::Strong,
        density: TreeDensity::Compact,
        disabled: false,
        node_count: 6,
        visible_count: 3,
        expanded_count: 1,
        has_selection: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert_eq!(state.tone_attr, "strong");
    assert_eq!(state.density_attr, "compact");
    assert_eq!(state.data_state_attr, "selected");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}

#[test]
fn compose_class_name_includes_state_markers() {
    let class_name = compose_class_name(
        Some("docs-tree".to_string()),
        resolve_state(TreeStateInput {
            tone: TreeTone::Quiet,
            density: TreeDensity::Comfortable,
            disabled: true,
            node_count: 0,
            visible_count: 0,
            expanded_count: 0,
            has_selection: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        }),
    );

    for token in [
        "ui-tree",
        "ui-tree--tone-quiet",
        "ui-tree--density-comfortable",
        "ui-tree--disabled",
        "ui-tree--empty",
        "ui-tree--custom-class",
        "docs-tree",
    ] {
        assert!(class_name.contains(token), "class should include `{token}`");
    }
}

#[test]
fn tree_agent_contract_is_typed_and_stable() {
    let state = resolve_state(TreeStateInput {
        tone: TreeTone::Default,
        density: crate::TreeDensity::Comfortable,
        disabled: false,
        node_count: 3,
        visible_count: 2,
        expanded_count: 1,
        has_selection: true,
        has_custom_aria_label: false,
        has_custom_class_name: false,
    });
    let contract = resolve_agent_contract(state, TreeAgentSource::Keyboard);

    assert_eq!(contract.schema_name, "ui.tree.agent-contract");
    assert_eq!(contract.schema_version.as_str(), "1");
    assert_eq!(contract.intent.as_str(), "hierarchy-navigation");
    assert_eq!(contract.action.as_str(), "select-node");
    assert_eq!(contract.state.as_str(), "selected");
    assert_eq!(contract.source.as_str(), "keyboard");
    assert_eq!(contract.stream_support.as_str(), "optional");
    assert_eq!(contract.stream_fallback.as_str(), "snapshot");
    assert_eq!(contract.output_status.as_str(), "draft");
}

#[test]
fn tree_stream_mode_strings_are_stable() {
    assert_eq!(TreeStreamMode::Snapshot.as_str(), "snapshot");
}
