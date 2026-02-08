use crate::surface::{SurfaceState, SurfaceStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Surface";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SurfaceTone {
    #[default]
    Default,
    Subtle,
    Strong,
}

impl SurfaceTone {
    pub fn class_name(self) -> &'static str {
        match self {
            SurfaceTone::Default => "ui-surface--tone-default",
            SurfaceTone::Subtle => "ui-surface--tone-subtle",
            SurfaceTone::Strong => "ui-surface--tone-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SurfaceTone::Default => "default",
            SurfaceTone::Subtle => "subtle",
            SurfaceTone::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SurfaceElevation {
    Flat,
    #[default]
    Raised,
    Floating,
}

impl SurfaceElevation {
    pub fn class_name(self) -> &'static str {
        match self {
            SurfaceElevation::Flat => "ui-surface--elevation-flat",
            SurfaceElevation::Raised => "ui-surface--elevation-raised",
            SurfaceElevation::Floating => "ui-surface--elevation-floating",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SurfaceElevation::Flat => "flat",
            SurfaceElevation::Raised => "raised",
            SurfaceElevation::Floating => "floating",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: SurfaceStateInput) -> SurfaceState {
    let data_state_attr = if input.bordered && input.padded {
        "framed"
    } else if input.bordered {
        "bordered"
    } else if input.padded {
        "padded"
    } else {
        "plain"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    SurfaceState {
        tone: input.tone,
        tone_class: input.tone.class_name(),
        tone_attr: input.tone.as_attr(),
        elevation: input.elevation,
        elevation_class: input.elevation.class_name(),
        elevation_attr: input.elevation.as_attr(),
        is_bordered: input.bordered,
        is_padded: input.padded,
        is_plain: !input.bordered && !input.padded,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SurfaceState) -> String {
    let mut classes = vec![
        "ui-surface".to_string(),
        state.tone_class.to_string(),
        state.elevation_class.to_string(),
    ];

    if state.is_bordered {
        classes.push("ui-surface--bordered".to_string());
    }

    if state.is_padded {
        classes.push("ui-surface--padded".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-surface--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surface::SurfaceStateInput;

    #[test]
    fn tone_and_elevation_contracts_are_stable() {
        assert_eq!(
            SurfaceTone::Default.class_name(),
            "ui-surface--tone-default"
        );
        assert_eq!(SurfaceTone::Subtle.class_name(), "ui-surface--tone-subtle");
        assert_eq!(SurfaceTone::Strong.class_name(), "ui-surface--tone-strong");

        assert_eq!(
            SurfaceElevation::Flat.class_name(),
            "ui-surface--elevation-flat"
        );
        assert_eq!(
            SurfaceElevation::Raised.class_name(),
            "ui-surface--elevation-raised"
        );
        assert_eq!(
            SurfaceElevation::Floating.class_name(),
            "ui-surface--elevation-floating"
        );
    }

    #[test]
    fn normalize_aria_label_falls_back_to_default() {
        assert_eq!(
            normalize_aria_label(Some("  Dashboard card  ".to_string())),
            ("Dashboard card".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(Some("\n\t".to_string())),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_state_and_source_markers() {
        let state = resolve_state(SurfaceStateInput {
            tone: SurfaceTone::Strong,
            elevation: SurfaceElevation::Floating,
            bordered: true,
            padded: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.tone_attr, "strong");
        assert_eq!(state.elevation_attr, "floating");
        assert!(state.is_bordered);
        assert!(state.is_padded);
        assert!(!state.is_plain);
        assert_eq!(state.data_state_attr, "framed");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_merges_custom_class() {
        let state = resolve_state(SurfaceStateInput {
            tone: SurfaceTone::Subtle,
            elevation: SurfaceElevation::Raised,
            bordered: false,
            padded: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-surface-custom".to_string()), state);

        assert!(class_name.contains("ui-surface"));
        assert!(class_name.contains("ui-surface--tone-subtle"));
        assert!(class_name.contains("ui-surface--elevation-raised"));
        assert!(class_name.contains("ui-surface--padded"));
        assert!(class_name.contains("ui-surface--custom-class"));
        assert!(class_name.contains("docs-surface-custom"));
    }
}
