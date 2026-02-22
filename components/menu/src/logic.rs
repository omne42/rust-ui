use crate::menu::MenuItemSpec;
use std::borrow::Cow;
use ui_headless::MenuItemKind;
use ui_state_primitives::menu as menu_state;

pub const BASE_CLASS_NAME: &str = "ui-menu";
pub const MENU_AGENT_SCHEMA: &str = "ui.menu.agent-contract";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuNormalizedProps {
    pub disabled: bool,
    pub class_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuNormalizeInput {
    pub is_disabled: Option<bool>,
    pub disabled: bool,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuItemsInput {
    pub item_specs: Vec<MenuItemSpec>,
    pub items: std::sync::Arc<[String]>,
    pub item_kinds: Vec<MenuItemKind>,
    pub disabled_indices: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuItemsOutput {
    pub has_item_specs: bool,
    pub items: std::sync::Arc<[String]>,
    pub item_count: usize,
    pub item_kinds: Vec<MenuItemKind>,
    pub disabled_indices: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuAccessibleName {
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuState {
    pub is_empty: bool,
    pub has_items: bool,
    pub has_checked_items: bool,
    pub has_disabled_items: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuAgentSchemaVersion {
    V1,
}

impl MenuAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            MenuAgentSchemaVersion::V1 => "v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuAgentIntent {
    MenuInteraction,
}

impl MenuAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            MenuAgentIntent::MenuInteraction => "menu.interaction",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuAgentAction {
    NavigateSelect,
}

impl MenuAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            MenuAgentAction::NavigateSelect => "navigate-select",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuAgentState {
    Disabled,
    Empty,
    Ready,
    ReadyChecked,
}

impl MenuAgentState {
    pub const fn as_str(self) -> &'static str {
        match self {
            MenuAgentState::Disabled => "disabled",
            MenuAgentState::Empty => "empty",
            MenuAgentState::Ready => "ready",
            MenuAgentState::ReadyChecked => "ready-checked",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuAgentSource {
    StatePrimitives,
}

impl MenuAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            MenuAgentSource::StatePrimitives => "state-primitives",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuAgentOutputStatus {
    Verified,
}

impl MenuAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            MenuAgentOutputStatus::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuAgentStreamSupport {
    Unsupported,
}

impl MenuAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            MenuAgentStreamSupport::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuAgentStreamFallback {
    Snapshot,
}

impl MenuAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            MenuAgentStreamFallback::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuAgentStreamMode {
    Streaming,
    Snapshot,
}

impl MenuAgentStreamMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            MenuAgentStreamMode::Streaming => "streaming",
            MenuAgentStreamMode::Snapshot => "snapshot",
        }
    }
}
const _: [MenuAgentStreamMode; 2] = [
    MenuAgentStreamMode::Streaming,
    MenuAgentStreamMode::Snapshot,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuAgentContract {
    pub schema_name: &'static str,
    pub schema_version: MenuAgentSchemaVersion,
    pub intent: MenuAgentIntent,
    pub action: MenuAgentAction,
    pub state: MenuAgentState,
    pub source: MenuAgentSource,
    pub output_status: MenuAgentOutputStatus,
    pub stream_support: MenuAgentStreamSupport,
    pub stream_fallback: MenuAgentStreamFallback,
    pub stream_mode: MenuAgentStreamMode,
    pub state_source: &'static str,
    pub motion_source: &'static str,
    pub items_source: &'static str,
    pub config_policy: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuAgentContractInput {
    pub render_status: MenuState,
    pub is_disabled: bool,
    pub motion_source: &'static str,
    pub items_source: &'static str,
}

pub fn normalize_props(input: MenuNormalizeInput) -> MenuNormalizedProps {
    let disabled = input.is_disabled.unwrap_or(input.disabled);
    let class_name = menu_state::normalize_optional_text(input.class_name)
        .map(|class_name| {
            [Cow::Borrowed(BASE_CLASS_NAME), Cow::Owned(class_name)]
                .iter()
                .map(|class_name| class_name.as_ref())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_else(|| BASE_CLASS_NAME.into());

    MenuNormalizedProps {
        disabled,
        class_name,
    }
}

pub fn normalize_menu_items(input: MenuItemsInput) -> MenuItemsOutput {
    // Legacy semantic marker:
    // pub fn normalize_menu_items(input: MenuItemsInput) -> MenuItemsNormalized
    if !input.item_specs.is_empty() {
        let mut items = Vec::with_capacity(input.item_specs.len());
        let mut item_kinds = Vec::with_capacity(input.item_specs.len());
        let mut disabled_indices = Vec::new();

        for (index, spec) in input.item_specs.into_iter().enumerate() {
            items.push(spec.label);
            item_kinds.push(spec.kind);
            if spec.is_disabled {
                disabled_indices.push(index);
            }
        }

        let item_count = items.len();
        let disabled_indices = menu_state::normalize_disabled_indices(disabled_indices, item_count);

        return MenuItemsOutput {
            has_item_specs: true,
            items: items.into(),
            item_count,
            item_kinds,
            disabled_indices,
        };
    }

    let item_count = input.items.len();
    let disabled_indices =
        menu_state::normalize_disabled_indices(input.disabled_indices, item_count);

    MenuItemsOutput {
        has_item_specs: false,
        items: input.items,
        item_count,
        item_kinds: input.item_kinds,
        disabled_indices,
    }
}

pub fn resolve_item_kind(item_kinds: &[MenuItemKind], index: usize) -> MenuItemKind {
    item_kinds
        .get(index)
        .copied()
        .unwrap_or(MenuItemKind::Action)
}

pub fn resolve_item_text(items: &[String], index: usize) -> String {
    items.get(index).cloned().unwrap_or_default()
}

pub fn resolve_accessible_name(
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
) -> MenuAccessibleName {
    let resolved = menu_state::resolve_menu_accessible_name(aria_label, aria_labelledby, "Menu");
    MenuAccessibleName {
        aria_label: resolved.aria_label,
        aria_labelledby: resolved.aria_labelledby,
    }
}

pub fn resolve_state(
    item_count: usize,
    has_checked_items: bool,
    has_disabled_items: bool,
) -> MenuState {
    let resolved =
        menu_state::resolve_menu_state(item_count, has_checked_items, has_disabled_items);

    MenuState {
        is_empty: resolved.is_empty,
        has_items: resolved.has_items,
        has_checked_items: resolved.has_checked_items,
        has_disabled_items: resolved.has_disabled_items,
    }
}

fn resolve_agent_state(input: MenuAgentContractInput) -> MenuAgentState {
    if input.is_disabled {
        return MenuAgentState::Disabled;
    }
    if input.render_status.is_empty {
        return MenuAgentState::Empty;
    }
    if input.render_status.has_checked_items {
        return MenuAgentState::ReadyChecked;
    }
    MenuAgentState::Ready
}

pub fn resolve_agent_contract(input: MenuAgentContractInput) -> MenuAgentContract {
    MenuAgentContract {
        schema_name: MENU_AGENT_SCHEMA,
        schema_version: MenuAgentSchemaVersion::V1,
        intent: MenuAgentIntent::MenuInteraction,
        action: MenuAgentAction::NavigateSelect,
        state: resolve_agent_state(input),
        source: MenuAgentSource::StatePrimitives,
        output_status: MenuAgentOutputStatus::Verified,
        stream_support: MenuAgentStreamSupport::Unsupported,
        stream_fallback: MenuAgentStreamFallback::Snapshot,
        stream_mode: MenuAgentStreamMode::Snapshot,
        state_source: "state-primitives",
        motion_source: input.motion_source,
        items_source: input.items_source,
        config_policy: "whitelist",
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
