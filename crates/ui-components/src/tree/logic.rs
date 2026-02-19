use crate::tree::{TreeState, TreeStateInput};
use leptos::prelude::Callback;
use std::collections::BTreeSet;
use ui_state_primitives::tree::{TreeStateCoreInput, resolve_aria_label, resolve_state_core};

pub use ui_state_primitives::tree::{
    TreeNode, TreeVisibleNode, collect_all_ids, collect_expandable_ids, count_nodes,
    flatten_visible_nodes, normalize_nodes, normalize_optional_text, sanitize_expanded_ids,
    sanitize_selected_id, toggle_expanded,
};

pub const DEFAULT_ARIA_LABEL: &str = "Tree";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TreeTone {
    #[default]
    Default,
    Quiet,
    Strong,
}

impl TreeTone {
    pub fn class_name(self) -> &'static str {
        match self {
            TreeTone::Default => "ui-tree--tone-default",
            TreeTone::Quiet => "ui-tree--tone-quiet",
            TreeTone::Strong => "ui-tree--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TreeTone::Default => "default",
            TreeTone::Quiet => "quiet",
            TreeTone::Strong => "strong",
        }
    }
}

pub fn normalize_aria_label_with_fallback(value: Option<String>, fallback: &str) -> (String, bool) {
    resolve_aria_label(value, fallback)
}

pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {
    is_disabled.unwrap_or(disabled)
}

pub fn normalize_expanded_ids_change_handler(
    on_expanded_ids_change: Option<Callback<BTreeSet<String>>>,
    on_expanded_change: Option<Callback<BTreeSet<String>>>,
) -> Option<Callback<BTreeSet<String>>> {
    on_expanded_ids_change.or(on_expanded_change)
}

pub fn normalize_selected_id_change_handler(
    on_selected_id_change: Option<Callback<Option<String>>>,
    on_selected_change: Option<Callback<Option<String>>>,
) -> Option<Callback<Option<String>>> {
    on_selected_id_change.or(on_selected_change)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TreeControlMode {
    Controlled,
    #[default]
    Uncontrolled,
}

impl TreeControlMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TreeDefaultSource {
    Provided,
    #[default]
    Empty,
}

impl TreeDefaultSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Provided => "provided",
            Self::Empty => "empty",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TreeChangeSource {
    Provided,
    #[default]
    None,
}

impl TreeChangeSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Provided => "provided",
            Self::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeAgentSchemaVersion {
    V1,
}

impl TreeAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeAgentIntent {
    HierarchyNavigation,
}

impl TreeAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HierarchyNavigation => "hierarchy-navigation",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeAgentAction {
    Initialize,
    ToggleNode,
    SelectNode,
    ExternalSync,
    ProgrammaticReplay,
}

impl TreeAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::ToggleNode => "toggle-node",
            Self::SelectNode => "select-node",
            Self::ExternalSync => "external-sync",
            Self::ProgrammaticReplay => "programmatic-replay",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeAgentStateAxis {
    Disabled,
    Empty,
    Collapsed,
    Expanded,
    Selected,
}

impl TreeAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Empty => "empty",
            Self::Collapsed => "collapsed",
            Self::Expanded => "expanded",
            Self::Selected => "selected",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeAgentSource {
    Init,
    Pointer,
    Keyboard,
    ExternalSync,
    ProgrammaticReplay,
}

impl TreeAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Pointer => "pointer",
            Self::Keyboard => "keyboard",
            Self::ExternalSync => "external-sync",
            Self::ProgrammaticReplay => "programmatic-replay",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeAgentStreamSupport {
    Optional,
}

impl TreeAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Optional => "optional",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeAgentStreamFallback {
    Snapshot,
}

impl TreeAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeAgentOutputStatus {
    Draft,
    Verified,
    Submittable,
}

impl TreeAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeStreamMode {
    Snapshot,
}

impl TreeStreamMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeAgentCapabilities {
    pub can_expand: bool,
    pub can_select: bool,
    pub can_external_sync: bool,
    pub can_programmatic_replay: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeAgentContract {
    pub schema_name: &'static str,
    pub schema_version: TreeAgentSchemaVersion,
    pub intent: TreeAgentIntent,
    pub action: TreeAgentAction,
    pub state: TreeAgentStateAxis,
    pub source: TreeAgentSource,
    pub stream_support: TreeAgentStreamSupport,
    pub stream_fallback: TreeAgentStreamFallback,
    pub output_status: TreeAgentOutputStatus,
    pub capabilities: TreeAgentCapabilities,
}

pub fn resolve_agent_action(source: TreeAgentSource) -> TreeAgentAction {
    match source {
        TreeAgentSource::Init => TreeAgentAction::Initialize,
        TreeAgentSource::Pointer => TreeAgentAction::ToggleNode,
        TreeAgentSource::Keyboard => TreeAgentAction::SelectNode,
        TreeAgentSource::ExternalSync => TreeAgentAction::ExternalSync,
        TreeAgentSource::ProgrammaticReplay => TreeAgentAction::ProgrammaticReplay,
    }
}

pub fn resolve_agent_state_axis(state: TreeState) -> TreeAgentStateAxis {
    if state.is_disabled {
        TreeAgentStateAxis::Disabled
    } else if state.is_empty {
        TreeAgentStateAxis::Empty
    } else if state.has_selection {
        TreeAgentStateAxis::Selected
    } else if state.expanded_count > 0 {
        TreeAgentStateAxis::Expanded
    } else {
        TreeAgentStateAxis::Collapsed
    }
}

pub fn resolve_agent_output_status(source: TreeAgentSource) -> TreeAgentOutputStatus {
    match source {
        TreeAgentSource::Pointer | TreeAgentSource::Keyboard => TreeAgentOutputStatus::Draft,
        TreeAgentSource::Init | TreeAgentSource::ExternalSync => TreeAgentOutputStatus::Verified,
        TreeAgentSource::ProgrammaticReplay => TreeAgentOutputStatus::Submittable,
    }
}

pub fn resolve_agent_contract(state: TreeState, source: TreeAgentSource) -> TreeAgentContract {
    TreeAgentContract {
        schema_name: "ui.tree.agent-contract",
        schema_version: TreeAgentSchemaVersion::V1,
        intent: TreeAgentIntent::HierarchyNavigation,
        action: resolve_agent_action(source),
        state: resolve_agent_state_axis(state),
        source,
        stream_support: TreeAgentStreamSupport::Optional,
        stream_fallback: TreeAgentStreamFallback::Snapshot,
        output_status: resolve_agent_output_status(source),
        capabilities: TreeAgentCapabilities {
            can_expand: !state.is_empty && !state.is_disabled,
            can_select: !state.is_empty && !state.is_disabled,
            can_external_sync: true,
            can_programmatic_replay: true,
        },
    }
}

#[derive(Clone)]
pub struct TreeExpandedAxisInput {
    pub is_controlled: bool,
    pub default_expanded_ids: Option<BTreeSet<String>>,
    pub on_expanded_ids_change: Option<Callback<BTreeSet<String>>>,
    pub on_expanded_change: Option<Callback<BTreeSet<String>>>,
}

#[derive(Clone)]
pub struct TreeExpandedAxis {
    pub default_expanded_ids: BTreeSet<String>,
    pub on_expanded_change: Option<Callback<BTreeSet<String>>>,
    pub control_mode: TreeControlMode,
    pub default_source: TreeDefaultSource,
    pub change_source: TreeChangeSource,
}

pub fn normalize_expanded_axis(
    input: TreeExpandedAxisInput,
    expandable_ids: &BTreeSet<String>,
) -> TreeExpandedAxis {
    let has_default_expanded_ids = input.default_expanded_ids.is_some();
    let on_expanded_change = normalize_expanded_ids_change_handler(
        input.on_expanded_ids_change,
        input.on_expanded_change,
    );
    let has_on_expanded_change = on_expanded_change.is_some();
    let default_expanded_ids = sanitize_expanded_ids(
        input.default_expanded_ids.unwrap_or_default(),
        expandable_ids,
    );

    TreeExpandedAxis {
        default_expanded_ids,
        on_expanded_change,
        control_mode: if input.is_controlled {
            TreeControlMode::Controlled
        } else {
            TreeControlMode::Uncontrolled
        },
        default_source: if has_default_expanded_ids {
            TreeDefaultSource::Provided
        } else {
            TreeDefaultSource::Empty
        },
        change_source: if has_on_expanded_change {
            TreeChangeSource::Provided
        } else {
            TreeChangeSource::None
        },
    }
}

#[derive(Clone)]
pub struct TreeSelectedAxisInput {
    pub is_controlled: bool,
    pub default_selected_id: Option<String>,
    pub on_selected_id_change: Option<Callback<Option<String>>>,
    pub on_selected_change: Option<Callback<Option<String>>>,
}

#[derive(Clone)]
pub struct TreeSelectedAxis {
    pub default_selected_id: Option<String>,
    pub on_selected_change: Option<Callback<Option<String>>>,
    pub control_mode: TreeControlMode,
    pub default_source: TreeDefaultSource,
    pub change_source: TreeChangeSource,
}

pub fn normalize_selected_axis(
    input: TreeSelectedAxisInput,
    all_ids: &BTreeSet<String>,
) -> TreeSelectedAxis {
    let has_default_selected_id = input.default_selected_id.is_some();
    let on_selected_change =
        normalize_selected_id_change_handler(input.on_selected_id_change, input.on_selected_change);
    let has_on_selected_change = on_selected_change.is_some();
    let default_selected_id =
        sanitize_selected_id(normalize_optional_text(input.default_selected_id), all_ids);

    TreeSelectedAxis {
        default_selected_id,
        on_selected_change,
        control_mode: if input.is_controlled {
            TreeControlMode::Controlled
        } else {
            TreeControlMode::Uncontrolled
        },
        default_source: if has_default_selected_id {
            TreeDefaultSource::Provided
        } else {
            TreeDefaultSource::Empty
        },
        change_source: if has_on_selected_change {
            TreeChangeSource::Provided
        } else {
            TreeChangeSource::None
        },
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeDerivedStateInput {
    pub nodes: Vec<TreeNode>,
    pub expanded_ids: BTreeSet<String>,
    pub selected_id: Option<String>,
    pub expandable_ids: BTreeSet<String>,
    pub all_ids: BTreeSet<String>,
    pub is_disabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeDerivedState {
    pub expanded_ids: BTreeSet<String>,
    pub selected_id: Option<String>,
    pub visible_nodes: Vec<TreeVisibleNode>,
}

pub fn derive_state(input: TreeDerivedStateInput) -> TreeDerivedState {
    let expanded_ids = sanitize_expanded_ids(input.expanded_ids, &input.expandable_ids);
    let selected_id = sanitize_selected_id(input.selected_id, &input.all_ids);
    let visible_nodes = flatten_visible_nodes(
        &input.nodes,
        &expanded_ids,
        selected_id.as_deref(),
        input.is_disabled,
    );

    TreeDerivedState {
        expanded_ids,
        selected_id,
        visible_nodes,
    }
}

pub fn resolve_expanded_toggle_request(
    expanded_ids: BTreeSet<String>,
    node_id: &str,
    expandable_ids: &BTreeSet<String>,
) -> BTreeSet<String> {
    let expanded_ids = sanitize_expanded_ids(expanded_ids, expandable_ids);
    toggle_expanded(expanded_ids, node_id, expandable_ids)
}

pub fn depth_class_name(depth: usize) -> &'static str {
    match depth {
        0 => "ui-tree__item--depth-0",
        1 => "ui-tree__item--depth-1",
        2 => "ui-tree__item--depth-2",
        3 => "ui-tree__item--depth-3",
        4 => "ui-tree__item--depth-4",
        _ => "ui-tree__item--depth-5-plus",
    }
}

pub fn resolve_state(input: TreeStateInput) -> TreeState {
    let core = resolve_state_core(TreeStateCoreInput {
        disabled: input.disabled,
        node_count: input.node_count,
        visible_count: input.visible_count,
        expanded_count: input.expanded_count,
        has_selection: input.has_selection,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
    });

    TreeState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        density: input.density,
        density_class: input.density.class_name(),
        density_attr: input.density.as_attr(),
        is_disabled: core.is_disabled,
        node_count: core.node_count,
        visible_count: core.visible_count,
        expanded_count: core.expanded_count,
        has_selection: core.has_selection,
        is_empty: core.is_empty,
        data_state_attr: core.data_state_attr,
        aria_source_attr: core.aria_source_attr,
        class_source_attr: core.class_source_attr,
        has_custom_class_name: core.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: TreeState) -> String {
    let mut classes = vec![
        "ui-tree".to_string(),
        state.tone_class.into(),
        state.density_class.into(),
    ];

    if state.is_disabled {
        classes.push("ui-tree--disabled".to_string());
    }
    if state.is_empty {
        classes.push("ui-tree--empty".to_string());
    }
    if state.has_selection {
        classes.push("ui-tree--has-selection".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-tree--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::TreeDensity;
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
        let nodes = vec![
            TreeNode::new("root", "Root").with_children(vec![TreeNode::new("child", "Child")]),
        ];
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
            density: crate::tree::TreeDensity::Comfortable,
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
}
