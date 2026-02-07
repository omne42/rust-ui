use crate::action_button::ActionButtonSize;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ActionButtonGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            ActionButtonGroupOrientation::Horizontal => "ui-action-button-group--horizontal",
            ActionButtonGroupOrientation::Vertical => "ui-action-button-group--vertical",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ActionButtonGroupOrientation::Horizontal => "horizontal",
            ActionButtonGroupOrientation::Vertical => "vertical",
        }
    }

    pub fn aria_orientation(self) -> &'static str {
        self.as_attr()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActionButtonGroupDensity {
    #[default]
    Regular,
    Compact,
}

impl ActionButtonGroupDensity {
    pub fn class_name(self) -> &'static str {
        match self {
            ActionButtonGroupDensity::Regular => "ui-action-button-group--density-regular",
            ActionButtonGroupDensity::Compact => "ui-action-button-group--density-compact",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ActionButtonGroupDensity::Regular => "regular",
            ActionButtonGroupDensity::Compact => "compact",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionButtonGroupContextValue {
    pub size: ActionButtonSize,
    pub density: ActionButtonGroupDensity,
    pub orientation: ActionButtonGroupOrientation,
    pub is_justified: bool,
    pub is_quiet: bool,
    pub is_disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionButtonGroupState {
    pub orientation: ActionButtonGroupOrientation,
    pub orientation_attr: &'static str,
    pub density: ActionButtonGroupDensity,
    pub density_attr: &'static str,
    pub is_horizontal: bool,
    pub is_vertical: bool,
    pub is_regular: bool,
    pub is_compact: bool,
    pub is_justified: bool,
    pub is_not_justified: bool,
    pub is_quiet: bool,
    pub is_filled: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_explicit_label: bool,
    pub has_fallback_label: bool,
    pub has_custom_class_name: bool,
}

pub(crate) fn use_action_button_group_context() -> Option<ActionButtonGroupContextValue> {
    use_context::<ActionButtonGroupContextValue>()
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(aria_label: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(aria_label) {
        return (label, true);
    }

    ("Action button group".to_string(), false)
}

pub fn resolve_state(
    orientation: ActionButtonGroupOrientation,
    density: ActionButtonGroupDensity,
    is_justified: bool,
    is_quiet: bool,
    disabled: bool,
    has_explicit_label: bool,
    has_custom_class_name: bool,
) -> ActionButtonGroupState {
    ActionButtonGroupState {
        orientation,
        orientation_attr: orientation.as_attr(),
        density,
        density_attr: density.as_attr(),
        is_horizontal: matches!(orientation, ActionButtonGroupOrientation::Horizontal),
        is_vertical: matches!(orientation, ActionButtonGroupOrientation::Vertical),
        is_regular: matches!(density, ActionButtonGroupDensity::Regular),
        is_compact: matches!(density, ActionButtonGroupDensity::Compact),
        is_justified,
        is_not_justified: !is_justified,
        is_quiet,
        is_filled: !is_quiet,
        is_disabled: disabled,
        is_enabled: !disabled,
        has_explicit_label,
        has_fallback_label: !has_explicit_label,
        has_custom_class_name,
    }
}

pub fn compose_class_name(
    base_class_name: Option<String>,
    state: ActionButtonGroupState,
) -> String {
    let mut classes = vec![
        "ui-action-button-group".to_string(),
        state.orientation.class_name().to_string(),
        state.density.class_name().to_string(),
    ];

    if state.is_justified {
        classes.push("ui-action-button-group--justified".to_string());
    }
    if state.is_quiet {
        classes.push("ui-action-button-group--quiet".to_string());
    }
    if state.is_disabled {
        classes.push("ui-action-button-group--disabled".to_string());
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
    fn density_class_names_are_stable() {
        assert_eq!(
            ActionButtonGroupDensity::Regular.class_name(),
            "ui-action-button-group--density-regular"
        );
        assert_eq!(ActionButtonGroupDensity::Regular.as_attr(), "regular");
        assert_eq!(
            ActionButtonGroupDensity::Compact.class_name(),
            "ui-action-button-group--density-compact"
        );
        assert_eq!(ActionButtonGroupDensity::Compact.as_attr(), "compact");
    }

    #[test]
    fn orientation_attributes_match_variants() {
        assert_eq!(
            ActionButtonGroupOrientation::Horizontal.aria_orientation(),
            "horizontal"
        );
        assert_eq!(
            ActionButtonGroupOrientation::Horizontal.as_attr(),
            "horizontal"
        );
        assert_eq!(
            ActionButtonGroupOrientation::Vertical.aria_orientation(),
            "vertical"
        );
        assert_eq!(ActionButtonGroupOrientation::Vertical.as_attr(), "vertical");
    }

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(
            normalize_optional_text(Some("  Group  ".to_string())),
            Some("Group".to_string())
        );
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(normalize_optional_text(None), None);
    }

    #[test]
    fn normalize_aria_label_uses_trimmed_label_or_fallback() {
        let (label, explicit) = normalize_aria_label(Some("  Actions  ".to_string()));
        assert_eq!(label, "Actions");
        assert!(explicit);

        let (label, explicit) = normalize_aria_label(Some("   ".to_string()));
        assert_eq!(label, "Action button group");
        assert!(!explicit);

        let (label, explicit) = normalize_aria_label(None);
        assert_eq!(label, "Action button group");
        assert!(!explicit);
    }

    #[test]
    fn resolve_state_tracks_orientation_density_and_flags() {
        let state = resolve_state(
            ActionButtonGroupOrientation::Vertical,
            ActionButtonGroupDensity::Compact,
            true,
            true,
            true,
            false,
            true,
        );

        assert_eq!(state.orientation_attr, "vertical");
        assert_eq!(state.density_attr, "compact");
        assert!(!state.is_horizontal);
        assert!(state.is_vertical);
        assert!(!state.is_regular);
        assert!(state.is_compact);
        assert!(state.is_justified);
        assert!(!state.is_not_justified);
        assert!(state.is_quiet);
        assert!(!state.is_filled);
        assert!(state.is_disabled);
        assert!(!state.is_enabled);
        assert!(!state.has_explicit_label);
        assert!(state.has_fallback_label);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(
                ActionButtonGroupOrientation::Horizontal,
                ActionButtonGroupDensity::Compact,
                true,
                true,
                true,
                true,
                true,
            ),
        );

        for token in [
            "ui-action-button-group",
            "ui-action-button-group--horizontal",
            "ui-action-button-group--density-compact",
            "ui-action-button-group--justified",
            "ui-action-button-group--quiet",
            "ui-action-button-group--disabled",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
