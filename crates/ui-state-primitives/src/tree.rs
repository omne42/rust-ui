use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub children: Vec<TreeNode>,
    pub disabled: bool,
}

impl TreeNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            children: Vec::new(),
            disabled: false,
        }
    }

    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeVisibleNode {
    pub id: String,
    pub label: String,
    pub depth: usize,
    pub has_children: bool,
    pub is_expanded: bool,
    pub is_selected: bool,
    pub is_disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeStateCoreInput {
    pub disabled: bool,
    pub node_count: usize,
    pub visible_count: usize,
    pub expanded_count: usize,
    pub has_selection: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeStateCore {
    pub is_disabled: bool,
    pub node_count: usize,
    pub visible_count: usize,
    pub expanded_count: usize,
    pub has_selection: bool,
    pub is_empty: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_aria_label(value: Option<String>, fallback: &str) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (fallback.trim().to_string(), false)
}

fn normalize_node(mut node: TreeNode, path: &[usize]) -> TreeNode {
    let fallback_id = if path.is_empty() {
        "node-root".to_string()
    } else {
        format!(
            "node-{}",
            path.iter()
                .map(|idx| (idx + 1).to_string())
                .collect::<Vec<_>>()
                .join("-")
        )
    };

    node.id = normalize_optional_text(Some(node.id)).unwrap_or(fallback_id);
    node.label = normalize_optional_text(Some(node.label)).unwrap_or_else(|| node.id.clone());
    node.children = node
        .children
        .into_iter()
        .enumerate()
        .map(|(index, child)| {
            let mut child_path = path.to_vec();
            child_path.push(index);
            normalize_node(child, &child_path)
        })
        .collect();

    node
}

pub fn normalize_nodes(nodes: Vec<TreeNode>) -> Vec<TreeNode> {
    nodes
        .into_iter()
        .enumerate()
        .map(|(index, node)| normalize_node(node, &[index]))
        .collect()
}

pub fn collect_all_ids(nodes: &[TreeNode]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();

    fn walk(node: &TreeNode, out: &mut BTreeSet<String>) {
        out.insert(node.id.clone());
        for child in &node.children {
            walk(child, out);
        }
    }

    for node in nodes {
        walk(node, &mut out);
    }

    out
}

pub fn collect_expandable_ids(nodes: &[TreeNode]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();

    fn walk(node: &TreeNode, out: &mut BTreeSet<String>) {
        if !node.children.is_empty() {
            out.insert(node.id.clone());
            for child in &node.children {
                walk(child, out);
            }
        }
    }

    for node in nodes {
        walk(node, &mut out);
    }

    out
}

pub fn count_nodes(nodes: &[TreeNode]) -> usize {
    fn walk(node: &TreeNode) -> usize {
        1 + node.children.iter().map(walk).sum::<usize>()
    }

    nodes.iter().map(walk).sum()
}

pub fn sanitize_expanded_ids(
    expanded_ids: BTreeSet<String>,
    expandable_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    expanded_ids
        .into_iter()
        .filter(|id| expandable_ids.contains(id))
        .collect()
}

pub fn sanitize_selected_id(
    selected_id: Option<String>,
    all_ids: &BTreeSet<String>,
) -> Option<String> {
    selected_id.and_then(|id| all_ids.contains(&id).then_some(id))
}

pub fn toggle_expanded(
    expanded_ids: BTreeSet<String>,
    id: &str,
    expandable_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    if !expandable_ids.contains(id) {
        return expanded_ids;
    }

    let mut next = expanded_ids;
    if !next.insert(id.to_string()) {
        next.remove(id);
    }
    next
}

pub fn flatten_visible_nodes(
    nodes: &[TreeNode],
    expanded_ids: &BTreeSet<String>,
    selected_id: Option<&str>,
    disabled: bool,
) -> Vec<TreeVisibleNode> {
    fn walk(
        node: &TreeNode,
        depth: usize,
        expanded_ids: &BTreeSet<String>,
        selected_id: Option<&str>,
        inherited_disabled: bool,
        out: &mut Vec<TreeVisibleNode>,
    ) {
        let is_disabled = inherited_disabled || node.disabled;
        let has_children = !node.children.is_empty();
        let is_expanded = has_children && expanded_ids.contains(&node.id);

        out.push(TreeVisibleNode {
            id: node.id.clone(),
            label: node.label.clone(),
            depth,
            has_children,
            is_expanded,
            is_selected: selected_id == Some(node.id.as_str()),
            is_disabled,
        });

        if has_children && is_expanded {
            for child in &node.children {
                walk(
                    child,
                    depth + 1,
                    expanded_ids,
                    selected_id,
                    is_disabled,
                    out,
                );
            }
        }
    }

    let mut out = Vec::new();
    for node in nodes {
        walk(node, 0, expanded_ids, selected_id, disabled, &mut out);
    }

    out
}

pub fn resolve_state_core(input: TreeStateCoreInput) -> TreeStateCore {
    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let is_empty = input.node_count == 0;
    let data_state_attr = if input.disabled {
        "disabled"
    } else if is_empty {
        "empty"
    } else if input.has_selection {
        "selected"
    } else {
        "default"
    };

    TreeStateCore {
        is_disabled: input.disabled,
        node_count: input.node_count,
        visible_count: input.visible_count,
        expanded_count: input.expanded_count,
        has_selection: input.has_selection,
        is_empty,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
mod tests {
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
}
