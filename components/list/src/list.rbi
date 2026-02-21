pub type ListAccessibleName = ui_state_primitives::list::ListAccessibleName;
pub type ListState = ui_state_primitives::list::ListViewState;
pub type ListItemSelectionIndicator = ui_state_primitives::list::ListItemSelectionIndicator;
pub type ListSectionHeadingTone = ui_state_primitives::list::ListSectionHeadingTone;
pub type ListMotion = ui_visual_primitive::active_highlight::ActiveHighlightMotion;
pub type ListSectionMotion = ui_illustrated_message::IllustratedMessageMotion;
pub type A11yDirection = ui_headless::A11yDirection;

pub const DEFAULT_LIST_CLASS_NAME: &str;
pub const DEFAULT_ID_BASE: &str;
pub const LIST_AGENT_SCHEMA: &str;

pub enum ListAgentSchemaVersion {
    V1,
}

impl ListAgentSchemaVersion {
    pub const fn as_attr(self) -> &'static str;
}

pub enum ListAgentIntent {
    CollectionSelection,
}

impl ListAgentIntent {
    pub const fn as_attr(self) -> &'static str;
}

pub enum ListAgentAction {
    NavigateSelect,
}

impl ListAgentAction {
    pub const fn as_attr(self) -> &'static str;
}

pub enum ListAgentState {
    Empty,
    SelectionEmpty,
    HasSelection,
    Disabled,
}

impl ListAgentState {
    pub const fn as_attr(self) -> &'static str;
}

pub enum ListAgentSource {
    Controlled,
    Uncontrolled,
}

impl ListAgentSource {
    pub const fn as_attr(self) -> &'static str;
}

pub enum ListAgentStreamSupport {
    Optional,
}

impl ListAgentStreamSupport {
    pub const fn as_attr(self) -> &'static str;
}

pub enum ListAgentStreamFallback {
    Snapshot,
}

impl ListAgentStreamFallback {
    pub const fn as_attr(self) -> &'static str;
}

pub enum ListAgentOutputStatus {
    Verified,
}

impl ListAgentOutputStatus {
    pub const fn as_attr(self) -> &'static str;
}

pub enum ListAgentConfigPolicy {
    Whitelist,
}

impl ListAgentConfigPolicy {
    pub const fn as_attr(self) -> &'static str;
}

pub struct ListAgentContractInput {
    pub state: ListState,
    pub is_disabled: bool,
    pub is_controlled: bool,
}

pub struct ListAgentContract {
    pub schema_name: &'static str,
    pub schema_version: ListAgentSchemaVersion,
    pub intent: ListAgentIntent,
    pub action: ListAgentAction,
    pub state: ListAgentState,
    pub source: ListAgentSource,
    pub stream_support: ListAgentStreamSupport,
    pub stream_fallback: ListAgentStreamFallback,
    pub output_status: ListAgentOutputStatus,
    pub config_policy: ListAgentConfigPolicy,
}

pub fn resolve_agent_contract(input: ListAgentContractInput) -> ListAgentContract;

pub fn List(
    id_base: Option<String>,
    items: std::sync::Arc<[String]>,
    selected_index: Option<leptos::prelude::Signal<Option<usize>>>,
    default_selected_index: Option<usize>,
    on_selected_index_change: Option<leptos::prelude::Callback<Option<usize>>>,
    id: Option<String>,
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    is_disabled: bool,
    disabled_indices: Vec<usize>,
    on_action: Option<leptos::prelude::Callback<usize>>,
    default_active_index: usize,
    is_active_index_synced_to_selected: bool,
    motion: ListMotion,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;

pub fn ListItem(
    children: leptos::children::Children,
    id: Option<String>,
    index: Option<usize>,
    is_selected: bool,
    is_focused: bool,
    is_disabled: bool,
    is_selection_indicator_visible: bool,
    is_divider_visible: bool,
    aria_label: Option<String>,
    selected_text: Option<String>,
    unselected_text: Option<String>,
    on_press: Option<leptos::prelude::Callback<()>>,
    on_pointer_move: Option<leptos::prelude::Callback<()>>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;

pub fn ListSection(
    children: leptos::children::Children,
    title: Option<String>,
    item_count: Option<usize>,
    heading_tone: ListSectionHeadingTone,
    is_disabled: bool,
    is_sticky_heading: bool,
    is_divider_visible: bool,
    motion: ListSectionMotion,
    aria_label: Option<String>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
