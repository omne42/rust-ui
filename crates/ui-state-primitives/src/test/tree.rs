use super::*;

#[test]
fn normalize_nodes_and_collect_ids_build_stable_tree_shape() {
    let nodes = normalize_nodes(vec![
        TreeNode::new(" ", " Root ").with_children(vec![TreeNode::new("child", " ")]),
    ]);

    assert_eq!(nodes[0].id, "node-1");
    assert_eq!(nodes[0].label, "Root");
    assert_eq!(nodes[0].children[0].id, "child");
    assert_eq!(nodes[0].children[0].label, "child");

    let all_ids = collect_all_ids(&nodes);
    assert!(all_ids.contains("node-1"));
    assert!(all_ids.contains("child"));

    let expandable = collect_expandable_ids(&nodes);
    assert!(expandable.contains("node-1"));
    assert!(!expandable.contains("child"));
}

#[test]
fn sanitize_and_toggle_expanded_ids_respect_expandable_nodes() {
    let expandable = BTreeSet::from(["root".to_string(), "group".to_string()]);
    let expanded = sanitize_expanded_ids(
        BTreeSet::from(["root".to_string(), "leaf".to_string()]),
        &expandable,
    );
    assert_eq!(expanded, BTreeSet::from(["root".to_string()]));

    let toggled = toggle_expanded(expanded.clone(), "group", &expandable);
    assert!(toggled.contains("group"));

    let toggled = toggle_expanded(toggled, "group", &expandable);
    assert!(!toggled.contains("group"));
}

#[test]
fn flatten_visible_nodes_tracks_depth_selection_and_disabled() {
    let nodes = vec![
        TreeNode::new("root", "Root")
            .with_children(vec![TreeNode::new("child", "Child")])
            .disabled(true),
    ];

    let visible = flatten_visible_nodes(
        &nodes,
        &BTreeSet::from(["root".to_string()]),
        Some("child"),
        false,
    );

    assert_eq!(visible.len(), 2);
    assert_eq!(visible[0].depth, 0);
    assert_eq!(visible[1].depth, 1);
    assert!(visible[1].is_selected);
    assert!(visible[0].is_disabled);
    assert!(visible[1].is_disabled);
}

#[test]
fn resolve_state_core_tracks_counts_sources_and_flags() {
    let state = resolve_state_core(TreeStateCoreInput {
        disabled: false,
        node_count: 6,
        visible_count: 3,
        expanded_count: 1,
        has_selection: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
    });

    assert!(!state.is_disabled);
    assert_eq!(state.node_count, 6);
    assert_eq!(state.visible_count, 3);
    assert_eq!(state.expanded_count, 1);
    assert!(state.has_selection);
    assert_eq!(state.data_state_attr, "selected");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
}
