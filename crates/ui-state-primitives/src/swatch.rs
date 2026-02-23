#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwatchSize {
    Xs,
    S,
    #[default]
    M,
    L,
}

impl SwatchSize {
    pub fn class_name(self) -> &'static str {
        match self {
            SwatchSize::Xs => "ui-swatch--size-xs",
            SwatchSize::S => "ui-swatch--size-s",
            SwatchSize::M => "ui-swatch--size-m",
            SwatchSize::L => "ui-swatch--size-l",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SwatchSize::Xs => "xs",
            SwatchSize::S => "s",
            SwatchSize::M => "m",
            SwatchSize::L => "l",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwatchBorder {
    #[default]
    Default,
    Light,
    None,
}

impl SwatchBorder {
    pub fn class_name(self) -> &'static str {
        match self {
            SwatchBorder::Default => "ui-swatch--border-default",
            SwatchBorder::Light => "ui-swatch--border-light",
            SwatchBorder::None => "ui-swatch--border-none",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SwatchBorder::Default => "default",
            SwatchBorder::Light => "light",
            SwatchBorder::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwatchRounding {
    #[default]
    Default,
    None,
    Full,
}

impl SwatchRounding {
    pub fn class_name(self) -> &'static str {
        match self {
            SwatchRounding::Default => "ui-swatch--rounding-default",
            SwatchRounding::None => "ui-swatch--rounding-none",
            SwatchRounding::Full => "ui-swatch--rounding-full",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SwatchRounding::Default => "default",
            SwatchRounding::None => "none",
            SwatchRounding::Full => "full",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SwatchShape {
    #[default]
    Square,
    Rectangle,
}

impl SwatchShape {
    pub fn class_name(self) -> &'static str {
        match self {
            SwatchShape::Square => "ui-swatch--shape-square",
            SwatchShape::Rectangle => "ui-swatch--shape-rectangle",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SwatchShape::Square => "square",
            SwatchShape::Rectangle => "rectangle",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchStateInput {
    pub size: SwatchSize,
    pub border: SwatchBorder,
    pub rounding: SwatchRounding,
    pub shape: SwatchShape,
    pub has_color: bool,
    pub nothing: bool,
    pub mixed_value: bool,
    pub disabled: bool,
    pub decorative: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchState {
    pub size: SwatchSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub border: SwatchBorder,
    pub border_class: &'static str,
    pub border_attr: &'static str,
    pub rounding: SwatchRounding,
    pub rounding_class: &'static str,
    pub rounding_attr: &'static str,
    pub shape: SwatchShape,
    pub shape_class: &'static str,
    pub shape_attr: &'static str,
    pub has_color: bool,
    pub show_nothing: bool,
    pub show_mixed_value: bool,
    pub disabled: bool,
    pub decorative: bool,
    pub is_interactive: bool,
    pub has_custom_class_name: bool,
    pub data_state_attr: &'static str,
}

pub const DEFAULT_ARIA_LABEL: &str = "Swatch";
pub const DEFAULT_MIXED_ARIA_LABEL: &str = "Mixed";
pub const DEFAULT_NOTHING_ARIA_LABEL: &str = "No fill";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchAriaLabelFallbacks<'a> {
    pub mixed: &'a str,
    pub nothing: &'a str,
    pub default: &'a str,
}

impl SwatchAriaLabelFallbacks<'_> {
    pub const fn defaults() -> Self {
        Self {
            mixed: DEFAULT_MIXED_ARIA_LABEL,
            nothing: DEFAULT_NOTHING_ARIA_LABEL,
            default: DEFAULT_ARIA_LABEL,
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

fn is_allowed_color_char(ch: char) -> bool {
    matches!(
        ch,
        '#' | '(' | ')' | ',' | '.' | '%' | '-' | '/' | ' ' | '[' | ']' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9'
    )
}

pub fn sanitize_color_value(value: Option<String>) -> Option<String> {
    let value = normalize_optional_text(value)?;
    if value.len() > 96 {
        return None;
    }
    if value.chars().all(is_allowed_color_char) {
        Some(value)
    } else {
        None
    }
}

pub fn resolve_aria_label(
    aria_label: Option<String>,
    label: Option<String>,
    color: Option<&str>,
    nothing: bool,
    mixed_value: bool,
) -> (String, &'static str) {
    resolve_aria_label_with_fallbacks(
        aria_label,
        label,
        color,
        nothing,
        mixed_value,
        SwatchAriaLabelFallbacks::defaults(),
    )
}

pub fn resolve_aria_label_with_fallbacks(
    aria_label: Option<String>,
    label: Option<String>,
    color: Option<&str>,
    nothing: bool,
    mixed_value: bool,
    fallbacks: SwatchAriaLabelFallbacks<'_>,
) -> (String, &'static str) {
    let mixed_label = fallbacks.mixed.trim();
    let nothing_label = fallbacks.nothing.trim();
    let default_label = fallbacks.default.trim();

    if let Some(custom) = normalize_optional_text(aria_label) {
        return (custom, "custom");
    }

    if let Some(label) = normalize_optional_text(label) {
        return (label, "label");
    }

    if mixed_value {
        return (
            if mixed_label.is_empty() {
                DEFAULT_MIXED_ARIA_LABEL.into()
            } else {
                mixed_label.into()
            },
            "mixed",
        );
    }

    if nothing {
        return (
            if nothing_label.is_empty() {
                DEFAULT_NOTHING_ARIA_LABEL.into()
            } else {
                nothing_label.into()
            },
            "nothing",
        );
    }

    if let Some(color) = color.filter(|color| !color.trim().is_empty()) {
        return (color.to_string(), "color");
    }

    (
        if default_label.is_empty() {
            DEFAULT_ARIA_LABEL.into()
        } else {
            default_label.into()
        },
        "default",
    )
}

pub fn resolve_state(input: SwatchStateInput) -> SwatchState {
    let show_mixed_value = input.mixed_value;
    let show_nothing = !show_mixed_value && input.nothing;
    let has_color = input.has_color && !show_mixed_value && !show_nothing;
    let is_interactive = !input.disabled && !input.decorative && !show_mixed_value;

    let data_state_attr = if input.disabled {
        "disabled"
    } else if show_mixed_value {
        "mixed"
    } else if show_nothing {
        "nothing"
    } else if has_color {
        "color"
    } else {
        "empty"
    };

    SwatchState {
        size: input.size,
        size_class: input.size.class_name(),
        size_attr: input.size.as_attr(),
        border: input.border,
        border_class: input.border.class_name(),
        border_attr: input.border.as_attr(),
        rounding: input.rounding,
        rounding_class: input.rounding.class_name(),
        rounding_attr: input.rounding.as_attr(),
        shape: input.shape,
        shape_class: input.shape.class_name(),
        shape_attr: input.shape.as_attr(),
        has_color,
        show_nothing,
        show_mixed_value,
        disabled: input.disabled,
        decorative: input.decorative,
        is_interactive,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
    }
}

pub fn normalize_default_selected(default_selected: Option<bool>) -> bool {
    default_selected.unwrap_or(false)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentSchemaVersion {
    V1,
}

impl SwatchAgentSchemaVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentIntent {
    ColorSelection,
}

impl SwatchAgentIntent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ColorSelection => "color-selection",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentAction {
    Initialize,
    TogglePress,
}

impl SwatchAgentAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialize => "initialize",
            Self::TogglePress => "toggle-press",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentStateAxis {
    Selected,
    Unselected,
    Disabled,
    Mixed,
    Nothing,
}

impl SwatchAgentStateAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Selected => "selected",
            Self::Unselected => "unselected",
            Self::Disabled => "disabled",
            Self::Mixed => "mixed",
            Self::Nothing => "nothing",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentSource {
    ControlledExternal,
    UncontrolledDefault,
    UncontrolledImplicitDefault,
    TogglePress,
}

impl SwatchAgentSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlledExternal => "controlled-external",
            Self::UncontrolledDefault => "uncontrolled-default",
            Self::UncontrolledImplicitDefault => "uncontrolled-implicit-default",
            Self::TogglePress => "toggle-press",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentOutputStatus {
    Verified,
    Submittable,
}

impl SwatchAgentOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentStreamSupport {
    Unsupported,
}

impl SwatchAgentStreamSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwatchAgentStreamFallback {
    FullSnapshot,
}

impl SwatchAgentStreamFallback {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSnapshot => "full-snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchAgentCapabilities {
    pub can_toggle: bool,
    pub can_disable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchAgentContract {
    pub schema_name: &'static str,
    pub schema_version: SwatchAgentSchemaVersion,
    pub intent: SwatchAgentIntent,
    pub action: SwatchAgentAction,
    pub state: SwatchAgentStateAxis,
    pub source: SwatchAgentSource,
    pub output_status: SwatchAgentOutputStatus,
    pub stream_support: SwatchAgentStreamSupport,
    pub stream_fallback: SwatchAgentStreamFallback,
    pub capabilities: SwatchAgentCapabilities,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchSelectionControlInput {
    pub has_controlled_selected: bool,
    pub default_selected: Option<bool>,
    pub has_on_selected_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwatchSelectionControlState {
    pub default_selected: bool,
    pub is_controlled_selected: bool,
    pub is_uncontrolled_selected: bool,
    pub control_mode_attr: &'static str,
    pub default_selected_source_attr: &'static str,
    pub selected_change_source_attr: &'static str,
}

pub fn resolve_selection_control_state(
    input: SwatchSelectionControlInput,
) -> SwatchSelectionControlState {
    let default_selected = normalize_default_selected(input.default_selected);
    let is_controlled_selected = input.has_controlled_selected;
    let is_uncontrolled_selected = !is_controlled_selected;
    let control_mode_attr = if is_controlled_selected {
        "controlled"
    } else {
        "uncontrolled"
    };
    let default_selected_source_attr = if input.default_selected.is_some() {
        "custom"
    } else {
        "default"
    };
    let selected_change_source_attr = if input.has_on_selected_change {
        "custom"
    } else {
        "none"
    };

    SwatchSelectionControlState {
        default_selected,
        is_controlled_selected,
        is_uncontrolled_selected,
        control_mode_attr,
        default_selected_source_attr,
        selected_change_source_attr,
    }
}

pub fn resolve_agent_source(selection: SwatchSelectionControlState) -> SwatchAgentSource {
    if selection.is_controlled_selected {
        SwatchAgentSource::ControlledExternal
    } else if selection.default_selected_source_attr == "custom" {
        SwatchAgentSource::UncontrolledDefault
    } else {
        SwatchAgentSource::UncontrolledImplicitDefault
    }
}

pub fn resolve_agent_action(source: SwatchAgentSource) -> SwatchAgentAction {
    match source {
        SwatchAgentSource::TogglePress => SwatchAgentAction::TogglePress,
        SwatchAgentSource::ControlledExternal
        | SwatchAgentSource::UncontrolledDefault
        | SwatchAgentSource::UncontrolledImplicitDefault => SwatchAgentAction::Initialize,
    }
}

pub fn resolve_agent_state_axis(state: SwatchState, selected: bool) -> SwatchAgentStateAxis {
    if state.disabled {
        SwatchAgentStateAxis::Disabled
    } else if state.show_mixed_value {
        SwatchAgentStateAxis::Mixed
    } else if state.show_nothing {
        SwatchAgentStateAxis::Nothing
    } else if selected {
        SwatchAgentStateAxis::Selected
    } else {
        SwatchAgentStateAxis::Unselected
    }
}

pub fn resolve_agent_output_status(source: SwatchAgentSource) -> SwatchAgentOutputStatus {
    match source {
        SwatchAgentSource::TogglePress => SwatchAgentOutputStatus::Submittable,
        SwatchAgentSource::ControlledExternal
        | SwatchAgentSource::UncontrolledDefault
        | SwatchAgentSource::UncontrolledImplicitDefault => SwatchAgentOutputStatus::Verified,
    }
}

pub fn resolve_agent_contract(
    state: SwatchState,
    selected: bool,
    source: SwatchAgentSource,
) -> SwatchAgentContract {
    SwatchAgentContract {
        schema_name: "ui.swatch.agent-contract",
        schema_version: SwatchAgentSchemaVersion::V1,
        intent: SwatchAgentIntent::ColorSelection,
        action: resolve_agent_action(source),
        state: resolve_agent_state_axis(state, selected),
        source,
        output_status: resolve_agent_output_status(source),
        stream_support: SwatchAgentStreamSupport::Unsupported,
        stream_fallback: SwatchAgentStreamFallback::FullSnapshot,
        capabilities: SwatchAgentCapabilities {
            can_toggle: state.is_interactive,
            can_disable: true,
        },
    }
}

#[cfg(test)]
#[path = "test/swatch.rs"]
mod tests;
