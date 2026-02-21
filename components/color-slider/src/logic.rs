use std::borrow::Cow;

#[cfg(test)]
pub use ui_state_primitives::color_slider::ColorSliderStateInput;
pub use ui_state_primitives::color_slider::{
    ColorSliderChannel, ColorSliderState, compose_class_name, format_channel_value,
    normalize_optional_text, resolve_state, sanitize_bounds, sanitize_step, sanitize_track_color,
    sanitize_value, source_attr_from_presence,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSliderControlMode {
    Controlled,
    Uncontrolled,
}

impl ColorSliderControlMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSliderValueSource {
    External,
    DefaultValue,
}

impl ColorSliderValueSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::External => "external",
            Self::DefaultValue => "default_value",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSliderValueChangeSource {
    OnValueChange,
    None,
}

impl ColorSliderValueChangeSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::OnValueChange => "on_value_change",
            Self::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSliderDisabledSource {
    IsDisabled,
    Disabled,
    Default,
}

impl ColorSliderDisabledSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::IsDisabled => "is_disabled",
            Self::Disabled => "disabled",
            Self::Default => "default",
        }
    }
}

pub struct ColorSliderAccessibilityState {
    pub is_disabled: bool,
    pub disabled_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSliderInputPresence {
    pub has_external_value: bool,
    pub has_default_value: bool,
    pub has_value_change_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSliderSourceAttrs {
    pub control_mode_attr: &'static str,
    pub value_source_attr: &'static str,
    pub value_change_source_attr: &'static str,
    pub default_value_source_attr: &'static str,
}

pub fn resolve_source_attrs(presence: ColorSliderInputPresence) -> ColorSliderSourceAttrs {
    let control_mode_attr = if presence.has_external_value {
        ColorSliderControlMode::Controlled.as_attr()
    } else {
        ColorSliderControlMode::Uncontrolled.as_attr()
    };
    let value_source_attr = if presence.has_external_value {
        ColorSliderValueSource::External.as_attr()
    } else {
        ColorSliderValueSource::DefaultValue.as_attr()
    };
    let value_change_source_attr = if presence.has_value_change_handler {
        ColorSliderValueChangeSource::OnValueChange.as_attr()
    } else {
        ColorSliderValueChangeSource::None.as_attr()
    };
    let default_value_source_attr = source_attr_from_presence(presence.has_default_value);

    ColorSliderSourceAttrs {
        control_mode_attr,
        value_source_attr,
        value_change_source_attr,
        default_value_source_attr,
    }
}

pub fn normalize_accessibility_state(
    is_disabled: Option<bool>,
    disabled: bool,
) -> ColorSliderAccessibilityState {
    let source = if is_disabled.is_some() {
        ColorSliderDisabledSource::IsDisabled
    } else if disabled {
        ColorSliderDisabledSource::Disabled
    } else {
        ColorSliderDisabledSource::Default
    };

    ColorSliderAccessibilityState {
        is_disabled: is_disabled.unwrap_or(disabled),
        disabled_source_attr: source.as_attr(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSliderAgentSchema {
    V1,
}

impl ColorSliderAgentSchema {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "ui.color-slider.agent-contract.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSliderAgentSchemaVersion {
    V1,
}

impl ColorSliderAgentSchemaVersion {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSliderStreamSupport {
    Unsupported,
}

impl ColorSliderStreamSupport {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSliderStreamFallback {
    Snapshot,
}

impl ColorSliderStreamFallback {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSliderStreamMode {
    Snapshot,
}

impl ColorSliderStreamMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSliderOutputStatus {
    Verified,
    Submittable,
}

impl ColorSliderOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSliderIntent {
    AdjustColorChannel,
}

impl ColorSliderIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::AdjustColorChannel => "adjust-color-channel",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorSliderUiAction {
    Idle,
    Focus,
    Press,
}

impl ColorSliderUiAction {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Focus => "focus",
            Self::Press => "press",
        }
    }
}

pub fn resolve_ui_action(is_pressed: bool, is_focused: bool) -> ColorSliderUiAction {
    if is_pressed {
        ColorSliderUiAction::Press
    } else if is_focused {
        ColorSliderUiAction::Focus
    } else {
        ColorSliderUiAction::Idle
    }
}

pub struct ColorSliderAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
}

pub fn resolve_agent_contract(has_value_change_handler: bool) -> ColorSliderAgentContract {
    let output_status = if has_value_change_handler {
        ColorSliderOutputStatus::Submittable
    } else {
        ColorSliderOutputStatus::Verified
    };

    ColorSliderAgentContract {
        schema_attr: ColorSliderAgentSchema::V1.as_attr(),
        schema_version_attr: ColorSliderAgentSchemaVersion::V1.as_attr(),
        stream_support_attr: ColorSliderStreamSupport::Unsupported.as_attr(),
        stream_fallback_attr: ColorSliderStreamFallback::Snapshot.as_attr(),
        stream_mode_attr: ColorSliderStreamMode::Snapshot.as_attr(),
        output_status_attr: output_status.as_attr(),
        intent_attr: ColorSliderIntent::AdjustColorChannel.as_attr(),
    }
}

pub fn normalize_default_value(
    channel: ColorSliderChannel,
    default_value: Option<f64>,
    min: f64,
    max: f64,
    step: f64,
) -> f64 {
    let raw_default_value = default_value.unwrap_or_else(|| channel.default_value());
    sanitize_value(channel, raw_default_value, min, max, step)
}

pub fn normalize_label(value: Option<String>, channel: ColorSliderChannel) -> (String, bool) {
    normalize_text_with_fallback(value, channel.default_label())
}

pub fn normalize_aria_label(
    value: Option<String>,
    label: &str,
    channel: ColorSliderChannel,
) -> (String, bool) {
    if let Some(aria_label) = normalize_optional_text(value) {
        return (aria_label, true);
    }

    let label = label.trim();
    if !label.is_empty() {
        return (format!("{label} slider"), false);
    }

    let normalized: Cow<'static, str> = Cow::Borrowed(channel.default_aria_label());
    (normalized.into_owned(), false)
}

fn normalize_text_with_fallback(value: Option<String>, fallback: &'static str) -> (String, bool) {
    let normalized = normalize_optional_text(value);
    let has_custom = normalized.is_some();
    let normalized: Cow<'static, str> = normalized
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed(fallback));
    (normalized.into_owned(), has_custom)
}

pub fn compose_inline_style(track_start: Option<&str>, track_end: Option<&str>) -> Option<String> {
    let mut declarations = Vec::new();

    if let Some(track_start) = track_start {
        declarations.push(format!("--ui-color-slider-track-start: {track_start};"));
    }

    if let Some(track_end) = track_end {
        declarations.push(format!("--ui-color-slider-track-end: {track_end};"));
    }

    if declarations.is_empty() {
        None
    } else {
        Some(declarations.join(" "))
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
