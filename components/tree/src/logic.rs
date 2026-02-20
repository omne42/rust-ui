use crate::{TreeState, TreeStateInput};
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
#[path = "../test/logic.rs"]
mod tests;
