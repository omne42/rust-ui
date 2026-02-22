use ui_state_primitives::color_wheel as primitives;

pub type ColorWheelStateInput = primitives::ColorWheelStateInput;
pub type ColorWheelState = primitives::ColorWheelState;
pub type ColorWheelStatus = primitives::ColorWheelStatus;
pub type ColorWheelValueLabelMode = primitives::ColorWheelValueLabelMode;
pub type ColorWheelSource = primitives::ColorWheelSource;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelInteractionSource {
    None,
    Pointer,
    Keyboard,
    Input,
}

impl ColorWheelInteractionSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Pointer => "pointer",
            Self::Keyboard => "keyboard",
            Self::Input => "input",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelAgentSchema {
    V1,
}

impl ColorWheelAgentSchema {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "ui.color-wheel.agent-contract.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelAgentSchemaVersion {
    V1,
}

impl ColorWheelAgentSchemaVersion {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelStreamSupport {
    Unsupported,
}

impl ColorWheelStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelStreamFallback {
    Snapshot,
}

impl ColorWheelStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelStreamMode {
    Snapshot,
}

impl ColorWheelStreamMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelOutputStatus {
    Verified,
    Submittable,
}

impl ColorWheelOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelIntent {
    SelectHueAngle,
}

impl ColorWheelIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::SelectHueAngle => "select-hue-angle",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelUiAction {
    Idle,
    Drag,
    Pointer,
    Keyboard,
    Input,
}

impl ColorWheelUiAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Drag => "drag",
            Self::Pointer => "pointer",
            Self::Keyboard => "keyboard",
            Self::Input => "input",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelUiState {
    Active,
    Disabled,
}

impl ColorWheelUiState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWheelUiSource {
    OnValueChange,
    None,
}

impl ColorWheelUiSource {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::OnValueChange => "on_value_change",
            Self::None => "none",
        }
    }
}

pub struct ColorWheelAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
    pub source_attr: &'static str,
}

pub fn resolve_agent_contract(has_value_change_handler: bool) -> ColorWheelAgentContract {
    let output_status = if has_value_change_handler {
        ColorWheelOutputStatus::Submittable
    } else {
        ColorWheelOutputStatus::Verified
    };
    let source = if has_value_change_handler {
        ColorWheelUiSource::OnValueChange
    } else {
        ColorWheelUiSource::None
    };

    ColorWheelAgentContract {
        schema_attr: ColorWheelAgentSchema::V1.as_attr(),
        schema_version_attr: ColorWheelAgentSchemaVersion::V1.as_attr(),
        stream_support_attr: ColorWheelStreamSupport::Unsupported.as_attr(),
        stream_fallback_attr: ColorWheelStreamFallback::Snapshot.as_attr(),
        stream_mode_attr: ColorWheelStreamMode::Snapshot.as_attr(),
        output_status_attr: output_status.as_attr(),
        intent_attr: ColorWheelIntent::SelectHueAngle.as_attr(),
        source_attr: source.as_attr(),
    }
}

pub fn resolve_ui_action(
    is_dragging: bool,
    interaction_source: ColorWheelInteractionSource,
) -> ColorWheelUiAction {
    if is_dragging {
        ColorWheelUiAction::Drag
    } else {
        match interaction_source {
            ColorWheelInteractionSource::None => ColorWheelUiAction::Idle,
            ColorWheelInteractionSource::Pointer => ColorWheelUiAction::Pointer,
            ColorWheelInteractionSource::Keyboard => ColorWheelUiAction::Keyboard,
            ColorWheelInteractionSource::Input => ColorWheelUiAction::Input,
        }
    }
}

pub fn resolve_ui_state(is_disabled: bool) -> ColorWheelUiState {
    if is_disabled {
        ColorWheelUiState::Disabled
    } else {
        ColorWheelUiState::Active
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    DragEnd { value: f64, step: f64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorWheelInputBoundary {
    pub status: ColorWheelStatus,
    pub value_label_mode: ColorWheelValueLabelMode,
}

impl ColorWheelInputBoundary {
    pub const fn is_disabled(self) -> bool {
        self.status.is_disabled()
    }

    pub const fn is_value_label_visible(self) -> bool {
        self.value_label_mode.is_visible()
    }
}

#[cfg(test)]
pub const DEFAULT_LABEL: &str = primitives::DEFAULT_LABEL;
pub const DEFAULT_ARIA_LABEL: &str = primitives::DEFAULT_ARIA_LABEL;
pub const MIN_VALUE: f64 = primitives::MIN_VALUE;
pub const MAX_VALUE: f64 = primitives::MAX_VALUE;
pub const DEFAULT_STEP: f64 = primitives::DEFAULT_STEP;

pub fn normalize_state_inputs(
    is_disabled: Option<bool>,
    disabled: bool,
    is_value_label_visible: Option<bool>,
    show_value_label: bool,
) -> ColorWheelInputBoundary {
    ColorWheelInputBoundary {
        status: ColorWheelStatus::from_disabled(is_disabled.unwrap_or(disabled)),
        value_label_mode: ColorWheelValueLabelMode::from_visible(
            is_value_label_visible.unwrap_or(show_value_label),
        ),
    }
}

pub fn source_from_custom_flag(is_custom: bool) -> ColorWheelSource {
    ColorWheelSource::from_custom(is_custom)
}

pub fn resolve_action(action: Action) -> f64 {
    match action {
        Action::DragEnd { value, step } => sanitize_value(value, step),
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    primitives::normalize_optional_text(value)
}

pub fn normalize_label(value: Option<String>) -> (String, bool) {
    primitives::normalize_label(value)
}

pub fn normalize_aria_label(value: Option<String>, label: &str) -> (String, bool) {
    primitives::normalize_aria_label(value, label)
}

pub fn sanitize_step(step: f64) -> f64 {
    primitives::sanitize_step(step)
}

#[cfg(test)]
pub fn normalize_angle(value: f64) -> f64 {
    primitives::normalize_angle(value)
}

pub fn sanitize_value(value: f64, step: f64) -> f64 {
    primitives::sanitize_value(value, step)
}

pub fn resolve_default_value(default_value: Option<f64>, step: f64) -> f64 {
    primitives::resolve_default_value(default_value, step)
}

#[cfg(test)]
pub fn parse_value(value: &str) -> Option<f64> {
    primitives::parse_value(value)
}

#[cfg(test)]
pub fn page_step(step: f64) -> f64 {
    primitives::page_step(step)
}

#[cfg(test)]
pub fn move_value_by_delta(current: f64, delta: f64, step: f64) -> f64 {
    primitives::move_value_by_delta(current, delta, step)
}

#[cfg(test)]
pub fn resolve_percent(value: f64) -> f64 {
    primitives::resolve_percent(value)
}

pub fn format_value_text(value: f64) -> String {
    primitives::format_value_text(value)
}

pub fn resolve_state(input: ColorWheelStateInput) -> ColorWheelState {
    primitives::resolve_state(input)
}

pub fn compose_class_name(base_class_name: Option<String>, state: ColorWheelState) -> String {
    primitives::compose_class_name(base_class_name, state)
}

#[cfg(any(test, target_arch = "wasm32"))]
pub fn pointer_to_hue_angle(
    client_x: f64,
    client_y: f64,
    rect_left: f64,
    rect_top: f64,
    rect_width: f64,
    rect_height: f64,
) -> f64 {
    primitives::pointer_to_hue_angle(
        client_x,
        client_y,
        rect_left,
        rect_top,
        rect_width,
        rect_height,
    )
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
