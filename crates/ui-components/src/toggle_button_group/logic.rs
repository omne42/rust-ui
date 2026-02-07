#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToggleButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ToggleButtonGroupOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            ToggleButtonGroupOrientation::Horizontal => "ui-toggle-button-group--horizontal",
            ToggleButtonGroupOrientation::Vertical => "ui-toggle-button-group--vertical",
        }
    }

    pub fn data_orientation(self) -> &'static str {
        match self {
            ToggleButtonGroupOrientation::Horizontal => "horizontal",
            ToggleButtonGroupOrientation::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToggleButtonGroupState {
    pub is_horizontal: bool,
    pub is_vertical: bool,
    pub is_attached: bool,
    pub is_detached: bool,
    pub has_explicit_label: bool,
    pub has_fallback_label: bool,
}

pub fn normalize_aria_label(aria_label: Option<String>) -> (String, bool) {
    if let Some(label) = aria_label {
        let trimmed = label.trim();
        if !trimmed.is_empty() {
            return (trimmed.to_string(), true);
        }
    }

    ("Toggle group".to_string(), false)
}

pub fn resolve_state(
    orientation: ToggleButtonGroupOrientation,
    attached: bool,
    has_explicit_label: bool,
) -> ToggleButtonGroupState {
    ToggleButtonGroupState {
        is_horizontal: matches!(orientation, ToggleButtonGroupOrientation::Horizontal),
        is_vertical: matches!(orientation, ToggleButtonGroupOrientation::Vertical),
        is_attached: attached,
        is_detached: !attached,
        has_explicit_label,
        has_fallback_label: !has_explicit_label,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_class_names_are_stable() {
        assert_eq!(
            ToggleButtonGroupOrientation::Horizontal.class_name(),
            "ui-toggle-button-group--horizontal"
        );
        assert_eq!(
            ToggleButtonGroupOrientation::Vertical.class_name(),
            "ui-toggle-button-group--vertical"
        );
    }

    #[test]
    fn orientation_data_values_are_stable() {
        assert_eq!(
            ToggleButtonGroupOrientation::Horizontal.data_orientation(),
            "horizontal"
        );
        assert_eq!(
            ToggleButtonGroupOrientation::Vertical.data_orientation(),
            "vertical"
        );
    }

    #[test]
    fn normalize_aria_label_uses_trimmed_label_or_fallback() {
        let (label, explicit) = normalize_aria_label(Some("  View mode  ".to_string()));
        assert_eq!(label, "View mode");
        assert!(explicit);

        let (label, explicit) = normalize_aria_label(Some("   ".to_string()));
        assert_eq!(label, "Toggle group");
        assert!(!explicit);

        let (label, explicit) = normalize_aria_label(None);
        assert_eq!(label, "Toggle group");
        assert!(!explicit);
    }

    #[test]
    fn resolve_state_tracks_orientation_attachment_and_label_source() {
        let state = resolve_state(ToggleButtonGroupOrientation::Vertical, true, true);

        assert!(!state.is_horizontal);
        assert!(state.is_vertical);
        assert!(state.is_attached);
        assert!(!state.is_detached);
        assert!(state.has_explicit_label);
        assert!(!state.has_fallback_label);

        let state = resolve_state(ToggleButtonGroupOrientation::Horizontal, false, false);

        assert!(state.is_horizontal);
        assert!(!state.is_vertical);
        assert!(!state.is_attached);
        assert!(state.is_detached);
        assert!(!state.has_explicit_label);
        assert!(state.has_fallback_label);
    }
}
