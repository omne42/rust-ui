pub use crate::motion::AutocompleteMotion;
pub use ui_headless::A11yDirection;

pub fn sanitize_motion(
    motion: crate::motion::AutocompleteMotion,
) -> crate::motion::AutocompleteMotion;

pub fn sanitize_popover_motion(
    motion: crate::motion::PopoverMotion,
) -> crate::motion::PopoverMotion;

pub const AUTOCOMPLETE_AGENT_SCHEMA: &'static str;

pub enum AutocompleteAgentSchemaVersion {
    V1,
}

pub enum AutocompleteAgentIntent {
    SuggestAndSelect,
}

pub enum AutocompleteAgentAction {
    Idle,
    Query,
    CommitSelection,
}

pub enum AutocompleteAgentState {
    Open,
    Closed,
    Disabled,
}

pub enum AutocompleteAgentSource {
    StatePrimitives,
}

pub enum AutocompleteAgentOutputStatus {
    Verified,
}

pub enum AutocompleteAgentStreamSupport {
    Unsupported,
}

pub enum AutocompleteAgentStreamFallback {
    Snapshot,
}

pub enum AutocompleteAgentStreamMode {
    Streaming,
    Snapshot,
}

pub struct AutocompleteAgentContract {
    pub schema_name: &'static str,
    pub schema_version: crate::logic::AutocompleteAgentSchemaVersion,
    pub intent: crate::logic::AutocompleteAgentIntent,
    pub action: crate::logic::AutocompleteAgentAction,
    pub state: crate::logic::AutocompleteAgentState,
    pub source: crate::logic::AutocompleteAgentSource,
    pub output_status: crate::logic::AutocompleteAgentOutputStatus,
    pub stream_support: crate::logic::AutocompleteAgentStreamSupport,
    pub stream_fallback: crate::logic::AutocompleteAgentStreamFallback,
    pub stream_mode: crate::logic::AutocompleteAgentStreamMode,
    pub state_source: &'static str,
    pub motion_source: &'static str,
    pub selected_source: &'static str,
    pub selected_change_source: &'static str,
    pub open_value_source: &'static str,
    pub config_policy: &'static str,
}

pub struct AutocompleteAgentContractInput {
    pub is_open: bool,
    pub is_disabled: bool,
    pub has_typed: bool,
    pub has_selection: bool,
    pub is_open_controlled: bool,
    pub selected_source: crate::logic::SelectedSource,
    pub selected_change_source: crate::logic::SelectedChangeSource,
    pub render_state: ui_state_primitives::autocomplete::AutocompleteState,
}

pub fn resolve_agent_contract(
    input: crate::logic::AutocompleteAgentContractInput,
) -> crate::logic::AutocompleteAgentContract;

pub fn Autocomplete(
    id_base: String,
    label: String,
    items: Vec<String>,
    selected_index: Option<leptos::prelude::Signal<Option<usize>>>,
    default_selected_index: Option<usize>,
    on_selected_index_change: Option<leptos::prelude::Callback<Option<usize>>>,
    set_selected_index: Option<leptos::prelude::WriteSignal<Option<usize>>>,
    is_disabled: Option<bool>,
    disabled: bool,
    disabled_indices: Vec<usize>,
    is_required: Option<leptos::prelude::Signal<bool>>,
    required: Option<leptos::prelude::Signal<bool>>,
    is_invalid: Option<leptos::prelude::Signal<bool>>,
    invalid: Option<leptos::prelude::Signal<bool>>,
    aria_describedby: leptos::prelude::Signal<Option<String>>,
    description: Option<String>,
    error: Option<String>,
    placeholder: Option<String>,
    empty_message: Option<String>,
    is_open: Option<leptos::prelude::Signal<bool>>,
    open: Option<leptos::prelude::Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    motion: crate::motion::AutocompleteMotion,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
