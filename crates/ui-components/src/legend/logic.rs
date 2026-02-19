pub use ui_state_primitives::legend::{
    DEFAULT_REQUIRED_INDICATOR, DEFAULT_TEXT, LegendState, LegendStateInput, LegendTone,
    normalize_optional_text, normalize_required_indicator, normalize_text, resolve_state,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendRequiredSource {
    IsRequired,
    Required,
    Default,
}

impl LegendRequiredSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::IsRequired => "is_required",
            Self::Required => "required",
            Self::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendDisabledSource {
    IsDisabled,
    Disabled,
    Default,
}

impl LegendDisabledSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::IsDisabled => "is_disabled",
            Self::Disabled => "disabled",
            Self::Default => "default",
        }
    }
}

pub struct RequiredState {
    pub is_required: bool,
    pub required_source_attr: &'static str,
}

pub fn normalize_required_state(is_required: Option<bool>, required: bool) -> RequiredState {
    let source = if is_required.is_some() {
        LegendRequiredSource::IsRequired
    } else if required {
        LegendRequiredSource::Required
    } else {
        LegendRequiredSource::Default
    };

    RequiredState {
        is_required: is_required.unwrap_or(required),
        required_source_attr: source.as_attr(),
    }
}

pub struct AccessibilityState {
    pub is_disabled: bool,
    pub disabled_source_attr: &'static str,
}

pub fn normalize_accessibility_state(
    is_disabled: Option<bool>,
    disabled: bool,
) -> AccessibilityState {
    let source = if is_disabled.is_some() {
        LegendDisabledSource::IsDisabled
    } else if disabled {
        LegendDisabledSource::Disabled
    } else {
        LegendDisabledSource::Default
    };

    AccessibilityState {
        is_disabled: is_disabled.unwrap_or(disabled),
        disabled_source_attr: source.as_attr(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendAgentSchema {
    V1,
}

impl LegendAgentSchema {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "ui.legend.agent-contract.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendAgentSchemaVersion {
    V1,
}

impl LegendAgentSchemaVersion {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendStreamSupport {
    Unsupported,
}

impl LegendStreamSupport {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendStreamFallback {
    Snapshot,
}

impl LegendStreamFallback {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendStreamMode {
    Snapshot,
}

impl LegendStreamMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendOutputStatus {
    Verified,
}

impl LegendOutputStatus {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendIntent {
    DescribeFieldset,
}

impl LegendIntent {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::DescribeFieldset => "describe-fieldset",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendUiAction {
    Idle,
}

impl LegendUiAction {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Idle => "idle",
        }
    }
}

pub struct LegendAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
}

pub fn resolve_agent_contract() -> LegendAgentContract {
    LegendAgentContract {
        schema_attr: LegendAgentSchema::V1.as_attr(),
        schema_version_attr: LegendAgentSchemaVersion::V1.as_attr(),
        stream_support_attr: LegendStreamSupport::Unsupported.as_attr(),
        stream_fallback_attr: LegendStreamFallback::Snapshot.as_attr(),
        stream_mode_attr: LegendStreamMode::Snapshot.as_attr(),
        output_status_attr: LegendOutputStatus::Verified.as_attr(),
        intent_attr: LegendIntent::DescribeFieldset.as_attr(),
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: LegendState) -> String {
    let mut classes = vec!["ui-legend".to_string(), state.tone_class.into()];

    if state.is_required {
        classes.push("ui-legend--required".to_string());
    }

    if state.is_disabled {
        classes.push("ui-legend--disabled".to_string());
    }

    if state.has_custom_text {
        classes.push("ui-legend--text-custom".to_string());
    }

    if state.has_custom_indicator {
        classes.push("ui-legend--indicator-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-legend--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tone_class_names_and_attrs_are_stable() {
        assert_eq!(LegendTone::Default.class_name(), "ui-legend--tone-default");
        assert_eq!(LegendTone::Muted.class_name(), "ui-legend--tone-muted");
        assert_eq!(LegendTone::Strong.class_name(), "ui-legend--tone-strong");

        assert_eq!(LegendTone::Default.as_attr(), "default");
        assert_eq!(LegendTone::Muted.as_attr(), "muted");
        assert_eq!(LegendTone::Strong.as_attr(), "strong");
    }

    #[test]
    fn normalize_helpers_fallback_to_defaults() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Preferences  ".to_string())),
            Some("Preferences".to_string())
        );

        assert_eq!(
            normalize_text(Some("  Notification settings  ".to_string())),
            ("Notification settings".to_string(), true)
        );
        assert_eq!(normalize_text(None), (DEFAULT_TEXT.into(), false));

        assert_eq!(
            normalize_required_indicator(Some("  (required)  ".to_string())),
            ("(required)".to_string(), true)
        );
        assert_eq!(
            normalize_required_indicator(None),
            (DEFAULT_REQUIRED_INDICATOR.into(), false)
        );
    }

    #[test]
    fn normalize_required_and_disabled_states_track_sources() {
        let required = normalize_required_state(Some(false), true);
        assert!(!required.is_required);
        assert_eq!(required.required_source_attr, "is_required");

        let required = normalize_required_state(None, true);
        assert!(required.is_required);
        assert_eq!(required.required_source_attr, "required");

        let disabled = normalize_accessibility_state(Some(false), true);
        assert!(!disabled.is_disabled);
        assert_eq!(disabled.disabled_source_attr, "is_disabled");

        let disabled = normalize_accessibility_state(None, true);
        assert!(disabled.is_disabled);
        assert_eq!(disabled.disabled_source_attr, "disabled");
    }

    #[test]
    fn resolve_state_tracks_required_disabled_and_sources() {
        let state = resolve_state(LegendStateInput {
            tone: LegendTone::Strong,
            is_required: true,
            is_disabled: true,
            has_custom_text: true,
            has_custom_indicator: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.tone_attr, "strong");
        assert!(state.is_required);
        assert!(!state.is_optional);
        assert!(state.is_disabled);
        assert_eq!(state.text_source_attr, "custom");
        assert_eq!(state.indicator_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(LegendStateInput {
            tone: LegendTone::Muted,
            is_required: true,
            is_disabled: false,
            has_custom_text: false,
            has_custom_indicator: true,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-legend".to_string()), state);
        for token in [
            "ui-legend",
            "ui-legend--tone-muted",
            "ui-legend--required",
            "ui-legend--indicator-custom",
            "ui-legend--custom-class",
            "docs-legend",
        ] {
            assert!(class_name.contains(token), "class should contain `{token}`");
        }
    }

    #[test]
    fn agent_contract_is_stable() {
        let contract = resolve_agent_contract();
        assert_eq!(contract.schema_attr, "ui.legend.agent-contract.v1");
        assert_eq!(contract.schema_version_attr, "1");
        assert_eq!(contract.stream_support_attr, "unsupported");
        assert_eq!(contract.stream_fallback_attr, "snapshot");
        assert_eq!(contract.stream_mode_attr, "snapshot");
        assert_eq!(contract.output_status_attr, "verified");
        assert_eq!(contract.intent_attr, "describe-fieldset");
        assert_eq!(LegendUiAction::Idle.as_attr(), "idle");
    }
}
