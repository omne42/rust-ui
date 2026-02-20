pub use ui_state_primitives::badge::{
    BadgeState, BadgeStateInput, BadgeVariant, normalize_optional_text, resolve_state,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BadgeAgentContract {
    pub schema_attr: &'static str,
    pub schema_version_attr: &'static str,
    pub intent_attr: &'static str,
    pub action_attr: &'static str,
    pub state_attr: &'static str,
    pub source_attr: &'static str,
    pub stream_support_attr: &'static str,
    pub stream_fallback_attr: &'static str,
    pub stream_mode_attr: &'static str,
    pub output_status_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn resolve_agent_contract(state: BadgeState) -> BadgeAgentContract {
    let class_source_attr = if state.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    BadgeAgentContract {
        schema_attr: "ui.badge.agent-contract",
        schema_version_attr: "1",
        intent_attr: "status-display",
        action_attr: "initialize",
        state_attr: state.fill_attr,
        source_attr: class_source_attr,
        stream_support_attr: "unsupported",
        stream_fallback_attr: "snapshot",
        stream_mode_attr: "snapshot",
        output_status_attr: "verified",
        class_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: BadgeState) -> String {
    let mut classes = vec![
        "ui-badge".to_string(),
        state.variant_class.into(),
        state.fill_class.into(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-badge--custom-class".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
