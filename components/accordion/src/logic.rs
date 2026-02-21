use std::collections::{BTreeMap, BTreeSet};

use ui_state_primitives::expansion::{
    ExpansionMode, ExpansionOpenCommitPlan, ExpansionSummary, apply_external_open_key_sync,
    normalize_default_open_keys, normalize_open_keys,
    plan_open_commit as plan_expansion_open_commit, summarize, toggle_open_key,
};

pub type AccordionSelectionMode = ExpansionMode;
pub const DEFAULT_ID_BASE_PREFIX: &str = "ui-accordion";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AccordionVariant {
    #[default]
    Light,
    Shadow,
    Bordered,
    Splitted,
}

impl AccordionVariant {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Shadow => "shadow",
            Self::Bordered => "bordered",
            Self::Splitted => "splitted",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AccordionSlotProjection {
    Lazy,
    #[default]
    KeepAlive,
    Eager,
}

impl AccordionSlotProjection {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lazy => "lazy",
            Self::KeepAlive => "keep-alive",
            Self::Eager => "eager",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccordionPanelLifecycleEvent {
    NotifyShown,
    NotifyHidden,
}

impl AccordionPanelLifecycleEvent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotifyShown => "notify-shown",
            Self::NotifyHidden => "notify-hidden",
        }
    }
}

pub fn should_render_panel_surface(
    projection: AccordionSlotProjection,
    is_open: bool,
    has_opened_once: bool,
) -> bool {
    match projection {
        AccordionSlotProjection::Lazy => is_open,
        AccordionSlotProjection::KeepAlive => is_open || has_opened_once,
        AccordionSlotProjection::Eager => true,
    }
}

pub fn open_set<const N: usize>(keys: [usize; N]) -> BTreeSet<usize> {
    BTreeSet::from(keys)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccordionItemStateInput {
    pub key: usize,
    pub open: Option<bool>,
    pub default_open: bool,
    pub is_disabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccordionRuntimeInit {
    pub has_controlled_open: bool,
    pub has_default_open: bool,
    pub item_keys: Vec<usize>,
    pub requested_open: BTreeSet<usize>,
    pub has_per_item_disabled: bool,
}

pub fn derive_runtime_init(item_inputs: &[AccordionItemStateInput]) -> AccordionRuntimeInit {
    let has_controlled_open = item_inputs.iter().any(|item| item.open.is_some());
    let has_default_open = item_inputs.iter().any(|item| item.default_open);
    let item_keys = item_inputs.iter().map(|item| item.key).collect::<Vec<_>>();
    let requested_open = item_inputs
        .iter()
        .filter_map(|item| {
            let is_open = item.open.unwrap_or(item.default_open);
            is_open.then_some(item.key)
        })
        .collect::<BTreeSet<_>>();
    let has_per_item_disabled = item_inputs.iter().any(|item| item.is_disabled);

    AccordionRuntimeInit {
        has_controlled_open,
        has_default_open,
        item_keys,
        requested_open,
        has_per_item_disabled,
    }
}

pub fn apply_external_item_sync(
    current: &BTreeSet<usize>,
    key: usize,
    should_open: bool,
) -> BTreeSet<usize> {
    apply_external_open_key_sync(current, key, should_open)
}

pub type AccordionOpenCommitPlan = ExpansionOpenCommitPlan<usize>;

pub fn plan_open_commit(
    mode: AccordionSelectionMode,
    before: &BTreeSet<usize>,
    requested_next: &BTreeSet<usize>,
    item_keys: &[usize],
    callback_keys: &[usize],
    disallow_empty_selection: bool,
) -> Option<AccordionOpenCommitPlan> {
    plan_expansion_open_commit(
        mode,
        before,
        requested_next,
        item_keys,
        callback_keys,
        disallow_empty_selection,
    )
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_id_base(id_base: Option<String>, generated_id_base: String) -> String {
    normalize_optional_text(id_base).unwrap_or(generated_id_base)
}

pub fn resolve_item_label(label: String, index: usize) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        format!("Section {}", index.saturating_add(1))
    } else {
        trimmed.to_string()
    }
}

pub fn resolve_item_key(key: Option<usize>, fallback_key: usize) -> usize {
    key.unwrap_or(fallback_key)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccordionRegistrationAction {
    Register { registration_id: usize },
    Unregister { registration_id: usize },
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AccordionRegistrationState {
    pub items_order: Vec<usize>,
}

pub fn apply_registration_action(
    state: &mut AccordionRegistrationState,
    action: AccordionRegistrationAction,
) {
    match action {
        AccordionRegistrationAction::Register { registration_id } => {
            if !state.items_order.contains(&registration_id) {
                state.items_order.push(registration_id);
            }
        }
        AccordionRegistrationAction::Unregister { registration_id } => {
            state.items_order.retain(|id| *id != registration_id);
        }
    }
}

pub fn reduce_registration_actions(
    actions: &[AccordionRegistrationAction],
) -> AccordionRegistrationState {
    let mut state = AccordionRegistrationState::default();
    for action in actions {
        apply_registration_action(&mut state, *action);
    }
    state
}

pub fn resolve_registered_item_keys(
    actions: &[AccordionRegistrationAction],
    registration_key_pairs: &[(usize, usize)],
) -> Vec<usize> {
    let items_order = reduce_registration_actions(actions).items_order;
    let mut key_by_registration_id = BTreeMap::new();
    for (registration_id, key) in registration_key_pairs {
        key_by_registration_id.insert(*registration_id, *key);
    }

    let mut resolved = Vec::with_capacity(registration_key_pairs.len());
    for registration_id in items_order {
        if let Some(key) = key_by_registration_id.get(&registration_id).copied()
            && !resolved.contains(&key)
        {
            resolved.push(key);
        }
    }
    for (_, key) in registration_key_pairs {
        if !resolved.contains(key) {
            resolved.push(*key);
        }
    }
    resolved
}

pub fn assign_item_keys(configured_keys: &[Option<usize>]) -> Vec<usize> {
    let mut used = Vec::<usize>::new();
    let mut next_auto = 0_usize;

    configured_keys
        .iter()
        .copied()
        .map(|configured_key| {
            let mut resolved = resolve_item_key(configured_key, next_auto);
            if used.contains(&resolved) {
                while used.contains(&next_auto) {
                    next_auto = next_auto.saturating_add(1);
                }
                resolved = next_auto;
            }
            used.push(resolved);
            while used.contains(&next_auto) {
                next_auto = next_auto.saturating_add(1);
            }
            resolved
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccordionOpenStateSource {
    Controlled,
    Uncontrolled,
}

impl AccordionOpenStateSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccordionOpenInitSource {
    External,
    Default,
    Empty,
}

impl AccordionOpenInitSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::External => "external",
            Self::Default => "default",
            Self::Empty => "empty",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccordionOpenChangeSource {
    Init,
    ExternalSync,
    Keyboard,
    Pointer,
    Programmatic,
}

impl AccordionOpenChangeSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::ExternalSync => "external-sync",
            Self::Keyboard => "keyboard",
            Self::Pointer => "pointer",
            Self::Programmatic => "programmatic",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccordionAgentSchemaVersion {
    V1,
}

impl AccordionAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccordionAgentIntent {
    Disclosure,
}

impl AccordionAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disclosure => "disclosure",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccordionAgentAction {
    Initialize,
    ExternalSync,
    ToggleKeyboard,
    TogglePointer,
    ToggleProgrammatic,
}

impl AccordionAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::ExternalSync => "external-sync",
            Self::ToggleKeyboard => "toggle-keyboard",
            Self::TogglePointer => "toggle-pointer",
            Self::ToggleProgrammatic => "toggle-programmatic",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccordionAgentStateAxis {
    Empty,
    AllClosed,
    HasOpen,
}

impl AccordionAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::AllClosed => "all-closed",
            Self::HasOpen => "has-open",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccordionAgentOutputStatus {
    Draft,
    Verified,
    Submittable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccordionAgentStreamSupport {
    Unsupported,
}

impl AccordionAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccordionAgentStreamFallback {
    FullSnapshot,
}

impl AccordionAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSnapshot => "full-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccordionAgentCapabilities {
    pub can_toggle: bool,
    pub can_focus_move: bool,
    pub can_external_sync: bool,
    pub can_programmatic_replay: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccordionAgentContract {
    pub schema_name: &'static str,
    pub schema_version: AccordionAgentSchemaVersion,
    pub intent: AccordionAgentIntent,
    pub action: AccordionAgentAction,
    pub state: AccordionAgentStateAxis,
    pub source: AccordionOpenChangeSource,
    pub output_status: AccordionAgentOutputStatus,
    pub stream_support: AccordionAgentStreamSupport,
    pub stream_fallback: AccordionAgentStreamFallback,
    pub capabilities: AccordionAgentCapabilities,
}

pub fn resolve_agent_action(source: AccordionOpenChangeSource) -> AccordionAgentAction {
    match source {
        AccordionOpenChangeSource::Init => AccordionAgentAction::Initialize,
        AccordionOpenChangeSource::ExternalSync => AccordionAgentAction::ExternalSync,
        AccordionOpenChangeSource::Keyboard => AccordionAgentAction::ToggleKeyboard,
        AccordionOpenChangeSource::Pointer => AccordionAgentAction::TogglePointer,
        AccordionOpenChangeSource::Programmatic => AccordionAgentAction::ToggleProgrammatic,
    }
}

pub fn resolve_agent_state_axis(item_count: usize, open_count: usize) -> AccordionAgentStateAxis {
    if item_count == 0 {
        AccordionAgentStateAxis::Empty
    } else if open_count == 0 {
        AccordionAgentStateAxis::AllClosed
    } else {
        AccordionAgentStateAxis::HasOpen
    }
}

pub fn resolve_agent_output_status(
    source: AccordionOpenChangeSource,
) -> AccordionAgentOutputStatus {
    match source {
        AccordionOpenChangeSource::ExternalSync => AccordionAgentOutputStatus::Verified,
        AccordionOpenChangeSource::Programmatic => AccordionAgentOutputStatus::Submittable,
        AccordionOpenChangeSource::Init
        | AccordionOpenChangeSource::Keyboard
        | AccordionOpenChangeSource::Pointer => AccordionAgentOutputStatus::Draft,
    }
}

pub fn resolve_agent_contract(
    source: AccordionOpenChangeSource,
    item_count: usize,
    open_count: usize,
) -> AccordionAgentContract {
    AccordionAgentContract {
        schema_name: "ui.accordion.agent-contract",
        schema_version: AccordionAgentSchemaVersion::V1,
        intent: AccordionAgentIntent::Disclosure,
        action: resolve_agent_action(source),
        state: resolve_agent_state_axis(item_count, open_count),
        source,
        output_status: resolve_agent_output_status(source),
        stream_support: AccordionAgentStreamSupport::Unsupported,
        stream_fallback: AccordionAgentStreamFallback::FullSnapshot,
        capabilities: AccordionAgentCapabilities {
            can_toggle: true,
            can_focus_move: true,
            can_external_sync: true,
            can_programmatic_replay: true,
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccordionState {
    pub is_empty: bool,
    pub has_items: bool,
    pub open_count: usize,
    pub has_open_items: bool,
    pub has_multiple_open: bool,
    pub has_disabled_items: bool,
}

pub fn resolve_state(
    mode: AccordionSelectionMode,
    item_count: usize,
    open_count: usize,
    has_disabled_items: bool,
) -> AccordionState {
    let ExpansionSummary {
        is_empty,
        has_items,
        open_count,
        has_open_items,
        has_multiple_open,
    } = summarize(mode, item_count, open_count);

    AccordionState {
        is_empty,
        has_items,
        open_count,
        has_open_items,
        has_multiple_open,
        has_disabled_items,
    }
}

pub fn resolve_open_state_source(is_controlled: bool) -> AccordionOpenStateSource {
    if is_controlled {
        AccordionOpenStateSource::Controlled
    } else {
        AccordionOpenStateSource::Uncontrolled
    }
}

pub fn resolve_open_init_source(
    is_controlled: bool,
    has_default_open: bool,
) -> AccordionOpenInitSource {
    if is_controlled {
        AccordionOpenInitSource::External
    } else if has_default_open {
        AccordionOpenInitSource::Default
    } else {
        AccordionOpenInitSource::Empty
    }
}

pub fn normalize_open_for_items(
    mode: AccordionSelectionMode,
    open: &BTreeSet<usize>,
    item_keys: &[usize],
    disallow_empty_selection: bool,
) -> BTreeSet<usize> {
    normalize_open_keys(mode, open, item_keys, disallow_empty_selection)
}

pub fn normalize_default_open_for_items(
    mode: AccordionSelectionMode,
    default_open: Option<&BTreeSet<usize>>,
    item_keys: &[usize],
    disallow_empty_selection: bool,
) -> BTreeSet<usize> {
    normalize_default_open_keys(mode, default_open, item_keys, disallow_empty_selection)
}

pub fn toggle_open_for_items(
    mode: AccordionSelectionMode,
    open: &BTreeSet<usize>,
    key: usize,
    item_keys: &[usize],
    disallow_empty_selection: bool,
) -> BTreeSet<usize> {
    toggle_open_key(mode, open, key, item_keys, disallow_empty_selection)
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
