pub use ui_state_primitives::scroll_area::{
    DEFAULT_ARIA_LABEL, ScrollAreaOrientation, ScrollAreaState, ScrollAreaStateInput,
    normalize_aria_label, normalize_optional_text, resolve_state,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaDisabledSourceAttr {
    IsProp,
    Default,
}

impl ScrollAreaDisabledSourceAttr {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::IsProp => "is-prop",
            Self::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaAgentSchema {
    V1,
}

impl ScrollAreaAgentSchema {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::V1 => "ui.scroll-area.agent-contract.v1",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaStreamSupport {
    Unsupported,
}

impl ScrollAreaStreamSupport {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaStreamFallback {
    Snapshot,
}

impl ScrollAreaStreamFallback {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaStreamMode {
    Snapshot,
}

impl ScrollAreaStreamMode {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaOutputStatus {
    Verified,
}

impl ScrollAreaOutputStatus {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Verified => "verified",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaAgentIntent {
    InspectRegion,
}

impl ScrollAreaAgentIntent {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::InspectRegion => "inspect-region",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaAgentAction {
    Observe,
    Disabled,
}

impl ScrollAreaAgentAction {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAreaAgentState {
    Enabled,
    Disabled,
}

impl ScrollAreaAgentState {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
        }
    }
}

pub struct ScrollAreaAgentContract {
    pub schema_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollAreaDisableInput {
    pub is_disabled: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollAreaDisableState {
    pub is_disabled: bool,
    pub disabled_source_attr: ScrollAreaDisabledSourceAttr,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScrollAreaRootInput {
    pub class_name: Option<String>,
    pub aria_label: Option<String>,
    pub fallback_aria_label: String,
    pub orientation: ScrollAreaOrientation,
    pub max_height_px: Option<u32>,
    pub disabled: ScrollAreaDisableInput,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScrollAreaRootState {
    pub class_name: Option<String>,
    pub aria_label: String,
    pub state: ScrollAreaState,
    pub disabled_source_attr: ScrollAreaDisabledSourceAttr,
}

pub fn normalize_disable_state(input: ScrollAreaDisableInput) -> ScrollAreaDisableState {
    if let Some(is_disabled) = input.is_disabled {
        ScrollAreaDisableState {
            is_disabled,
            disabled_source_attr: ScrollAreaDisabledSourceAttr::IsProp,
        }
    } else {
        ScrollAreaDisableState {
            is_disabled: false,
            disabled_source_attr: ScrollAreaDisabledSourceAttr::Default,
        }
    }
}

pub fn normalize_aria_label_with_fallback(
    aria_label: Option<String>,
    fallback_aria_label: &str,
) -> (String, bool) {
    if let Some(label) = normalize_optional_text(aria_label) {
        return (label, true);
    }

    let fallback = normalize_optional_text(Some(fallback_aria_label.into()))
        .unwrap_or_else(|| normalize_aria_label(None).0);
    (fallback, false)
}

pub fn normalize_root_state(input: ScrollAreaRootInput) -> ScrollAreaRootState {
    let class_name = normalize_optional_text(input.class_name);
    let (aria_label, has_custom_aria_label) =
        normalize_aria_label_with_fallback(input.aria_label, input.fallback_aria_label.as_str());
    let disable = normalize_disable_state(input.disabled);

    let state = resolve_state(ScrollAreaStateInput {
        orientation: input.orientation,
        disabled: disable.is_disabled,
        max_height_px: input.max_height_px,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    ScrollAreaRootState {
        class_name,
        aria_label,
        state,
        disabled_source_attr: disable.disabled_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ScrollAreaState) -> String {
    let orientation_class_name = match state.orientation {
        ScrollAreaOrientation::Vertical => "ui-scroll-area--vertical",
        ScrollAreaOrientation::Horizontal => "ui-scroll-area--horizontal",
        ScrollAreaOrientation::Both => "ui-scroll-area--both",
    };

    let mut classes = vec!["ui-scroll-area".to_string(), orientation_class_name.into()];

    if state.disabled {
        classes.push("ui-scroll-area--disabled".to_string());
    }

    if state.has_custom_max_height {
        classes.push("ui-scroll-area--max-height-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-scroll-area--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn resolve_agent_contract(
    state: ScrollAreaState,
    disabled_source_attr: ScrollAreaDisabledSourceAttr,
) -> ScrollAreaAgentContract {
    let action = if state.disabled {
        ScrollAreaAgentAction::Disabled
    } else {
        ScrollAreaAgentAction::Observe
    };
    let state_axis = if state.disabled {
        ScrollAreaAgentState::Disabled
    } else {
        ScrollAreaAgentState::Enabled
    };

    ScrollAreaAgentContract {
        schema_attr: ScrollAreaAgentSchema::V1.as_attr(),
        stream_support_attr: ScrollAreaStreamSupport::Unsupported.as_attr(),
        stream_fallback_attr: ScrollAreaStreamFallback::Snapshot.as_attr(),
        stream_mode_attr: ScrollAreaStreamMode::Snapshot.as_attr(),
        output_status_attr: ScrollAreaOutputStatus::Verified.as_attr(),
        intent_attr: ScrollAreaAgentIntent::InspectRegion.as_attr(),
        action_attr: action.as_attr(),
        state_attr: state_axis.as_attr(),
        source_attr: disabled_source_attr.as_attr(),
    }
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
