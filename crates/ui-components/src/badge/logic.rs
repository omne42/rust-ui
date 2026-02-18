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
        state.variant_class.to_string(),
        state.fill_class.to_string(),
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
mod tests {
    use super::*;

    #[test]
    fn compose_class_name_includes_state_markers() {
        let solid = resolve_state(BadgeStateInput {
            variant: BadgeVariant::Accent,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-badge".to_string()), solid);

        for token in [
            "ui-badge",
            "ui-badge--variant-accent",
            "ui-badge--fill-solid",
            "ui-badge--custom-class",
            "docs-badge",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }

    #[test]
    fn resolve_agent_contract_emits_machine_readable_markers() {
        let contract = resolve_agent_contract(resolve_state(BadgeStateInput {
            variant: BadgeVariant::Outline,
            has_custom_class_name: true,
        }));

        assert_eq!(contract.schema_attr, "ui.badge.agent-contract");
        assert_eq!(contract.schema_version_attr, "1");
        assert_eq!(contract.intent_attr, "status-display");
        assert_eq!(contract.action_attr, "initialize");
        assert_eq!(contract.state_attr, "outline");
        assert_eq!(contract.source_attr, "custom");
        assert_eq!(contract.stream_support_attr, "unsupported");
        assert_eq!(contract.stream_fallback_attr, "snapshot");
        assert_eq!(contract.stream_mode_attr, "snapshot");
        assert_eq!(contract.output_status_attr, "verified");
        assert_eq!(contract.class_source_attr, "custom");
    }
}
