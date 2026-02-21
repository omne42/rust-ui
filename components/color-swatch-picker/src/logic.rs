use std::borrow::Cow;

pub use ui_state_primitives::swatch_picker::{
    DEFAULT_ARIA_LABEL, SwatchPickerItem as ColorSwatchPickerItem,
    SwatchPickerState as ColorSwatchPickerState,
    SwatchPickerStateInput as ColorSwatchPickerStateInput,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchPickerAgentSchema {
    V1,
}

impl ColorSwatchPickerAgentSchema {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "ui.color-swatch-picker.agent-contract.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchPickerAgentSchemaVersion {
    V1,
}

impl ColorSwatchPickerAgentSchemaVersion {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchPickerStreamSupport {
    Unsupported,
}

impl ColorSwatchPickerStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchPickerStreamFallback {
    Snapshot,
}

impl ColorSwatchPickerStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchPickerOutputStatus {
    Verified,
}

impl ColorSwatchPickerOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchPickerIntent {
    PickColorSwatch,
}

impl ColorSwatchPickerIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::PickColorSwatch => "pick-color-swatch",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchPickerUiAction {
    Select,
    Sync,
}

impl ColorSwatchPickerUiAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Select => "select",
            Self::Sync => "sync",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchPickerUiState {
    Active,
    Disabled,
    Empty,
}

impl ColorSwatchPickerUiState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Disabled => "disabled",
            Self::Empty => "empty",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSwatchPickerUiSource {
    Interaction,
    External,
    DefaultValue,
    Internal,
}

impl ColorSwatchPickerUiSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Interaction => "interaction",
            Self::External => "external",
            Self::DefaultValue => "default",
            Self::Internal => "internal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSwatchPickerAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
}

pub fn resolve_agent_contract() -> ColorSwatchPickerAgentContract {
    ColorSwatchPickerAgentContract {
        schema_attr: ColorSwatchPickerAgentSchema::V1.as_attr(),
        schema_version_attr: ColorSwatchPickerAgentSchemaVersion::V1.as_attr(),
        stream_support_attr: ColorSwatchPickerStreamSupport::Unsupported.as_attr(),
        stream_fallback_attr: ColorSwatchPickerStreamFallback::Snapshot.as_attr(),
        output_status_attr: ColorSwatchPickerOutputStatus::Verified.as_attr(),
        intent_attr: ColorSwatchPickerIntent::PickColorSwatch.as_attr(),
    }
}

pub fn resolve_ui_action(selection_source_attr: &'static str) -> ColorSwatchPickerUiAction {
    if selection_source_attr == ColorSwatchPickerUiSource::Interaction.as_attr() {
        ColorSwatchPickerUiAction::Select
    } else {
        ColorSwatchPickerUiAction::Sync
    }
}

pub fn resolve_ui_state(is_disabled: bool, is_empty: bool) -> ColorSwatchPickerUiState {
    if is_disabled {
        ColorSwatchPickerUiState::Disabled
    } else if is_empty {
        ColorSwatchPickerUiState::Empty
    } else {
        ColorSwatchPickerUiState::Active
    }
}

pub fn resolve_ui_source(selection_source_attr: &'static str) -> ColorSwatchPickerUiSource {
    match selection_source_attr {
        "interaction" => ColorSwatchPickerUiSource::Interaction,
        "external" => ColorSwatchPickerUiSource::External,
        "default" => ColorSwatchPickerUiSource::DefaultValue,
        "internal" => ColorSwatchPickerUiSource::Internal,
        _ => ColorSwatchPickerUiSource::Internal,
    }
}

pub const DEFAULT_ID_BASE: &str = "ui-color-swatch-picker";

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::swatch_picker::normalize_optional_text(value)
}

pub fn normalize_id_base(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.to_string())
}

pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {
    is_disabled.unwrap_or(disabled)
}
const _: fn(Option<bool>, bool) -> bool = normalize_is_disabled;

pub fn resolve_selection_mode_attr(is_controlled: bool) -> &'static str {
    if is_controlled {
        "controlled"
    } else {
        "uncontrolled"
    }
}

pub fn resolve_selection_init_source_attr(
    is_controlled: bool,
    has_default_selected_color: bool,
) -> &'static str {
    if is_controlled {
        "external"
    } else if has_default_selected_color {
        "default"
    } else {
        "internal"
    }
}

pub fn resolve_selection_source_attr(
    current_source: &'static str,
    selection_mode_attr: &'static str,
    selection_init_source_attr: &'static str,
    pending_user_selection: bool,
) -> &'static str {
    if pending_user_selection {
        return "interaction";
    }

    if selection_mode_attr == "controlled" {
        return "external";
    }

    if current_source == "interaction" {
        "interaction"
    } else {
        selection_init_source_attr
    }
}

pub fn normalize_items(items: Vec<ColorSwatchPickerItem>) -> Vec<ColorSwatchPickerItem> {
    ui_state_primitives::swatch_picker::normalize_items(items)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    ui_state_primitives::swatch_picker::normalize_aria_label(value)
}

pub fn sanitize_selected_color(selected_color: Option<String>) -> Option<String> {
    ui_state_primitives::swatch_picker::sanitize_selected_color(selected_color)
}

pub fn resolve_selected_index(
    items: &[ColorSwatchPickerItem],
    selected_color: Option<String>,
) -> Option<usize> {
    ui_state_primitives::swatch_picker::resolve_selected_index(items, selected_color)
}

pub fn resolve_selected_color(
    items: &[ColorSwatchPickerItem],
    selected_index: Option<usize>,
) -> Option<String> {
    ui_state_primitives::swatch_picker::resolve_selected_color(items, selected_index)
}

pub fn resolve_option_label(item: &ColorSwatchPickerItem, index: usize) -> String {
    ui_state_primitives::swatch_picker::resolve_option_label(item, index)
}

pub fn resolve_state(input: ColorSwatchPickerStateInput) -> ColorSwatchPickerState {
    ui_state_primitives::swatch_picker::resolve_state(input)
}

pub fn count_disabled_items(items: &[ColorSwatchPickerItem]) -> usize {
    items.iter().filter(|item| item.disabled).count()
}

pub fn is_item_disabled_at(
    is_disabled: bool,
    items: &[ColorSwatchPickerItem],
    index: usize,
) -> bool {
    is_disabled || items.get(index).is_none_or(|item| item.disabled)
}

pub fn resolve_option_disabled(is_disabled: bool, item_disabled: bool) -> bool {
    is_disabled || item_disabled
}

pub fn resolve_option_tabindex(
    option_disabled: bool,
    active_index: usize,
    option_index: usize,
) -> i32 {
    if option_disabled {
        -1
    } else if active_index == option_index {
        0
    } else {
        -1
    }
}

pub fn resolve_component_state(
    is_disabled: bool,
    items: &[ColorSwatchPickerItem],
    selected_index: Option<usize>,
    has_custom_aria_label: bool,
    has_custom_class_name: bool,
) -> ColorSwatchPickerState {
    resolve_state(ColorSwatchPickerStateInput {
        disabled: is_disabled,
        item_count: items.len(),
        selected_index,
        disabled_item_count: count_disabled_items(items),
        has_custom_aria_label,
        has_custom_class_name,
    })
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: ColorSwatchPickerState,
) -> String {
    let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed("ui-color-swatch-picker")];

    if state.is_disabled {
        classes.push(Cow::Borrowed("ui-color-swatch-picker--disabled"));
    }

    if state.has_custom_class_name {
        classes.push(Cow::Borrowed("ui-color-swatch-picker--custom-class"));
        if let Some(base_class_name) = base_class_name {
            classes.push(Cow::Owned(base_class_name));
        }
    }

    classes
        .iter()
        .map(|class_name| class_name.as_ref())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
