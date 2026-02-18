pub use ui_state_primitives::well::{
    DEFAULT_ARIA_LABEL, WellDensity, WellState, WellStateInput, WellTone, resolve_state,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WellNormalizeInput {
    pub tone: Option<WellTone>,
    pub density: Option<WellDensity>,
    pub is_inset: Option<bool>,
    pub aria_label: Option<String>,
    pub fallback_aria_label: String,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WellNormalizedProps {
    pub state_input: WellStateInput,
    pub aria_label: String,
    pub class_name: Option<String>,
    pub tone_source_attr: &'static str,
    pub density_source_attr: &'static str,
    pub inset_source_attr: &'static str,
}

pub fn normalize_tone(value: Option<WellTone>) -> WellTone {
    value.unwrap_or_default()
}

pub fn normalize_density(value: Option<WellDensity>) -> WellDensity {
    value.unwrap_or_default()
}

pub fn normalize_is_inset(value: Option<bool>) -> bool {
    value.unwrap_or(false)
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    ui_state_primitives::well::normalize_optional_text(value)
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    ui_state_primitives::well::normalize_aria_label(value)
}

pub fn normalize_aria_label_with_fallback(
    aria_label: Option<String>,
    fallback_aria_label: &str,
) -> (String, bool) {
    if let Some(label) = normalize_optional_text(aria_label) {
        return (label, true);
    }

    let fallback = normalize_optional_text(Some(fallback_aria_label.to_string()))
        .unwrap_or_else(|| normalize_aria_label(None).0);
    (fallback, false)
}

pub fn source_attr_from_presence(is_present: bool) -> &'static str {
    if is_present { "prop" } else { "default" }
}

pub fn normalize_props(input: WellNormalizeInput) -> WellNormalizedProps {
    let tone_source_attr = source_attr_from_presence(input.tone.is_some());
    let density_source_attr = source_attr_from_presence(input.density.is_some());
    let inset_source_attr = source_attr_from_presence(input.is_inset.is_some());

    let tone = normalize_tone(input.tone);
    let density = normalize_density(input.density);
    let is_inset = normalize_is_inset(input.is_inset);
    let class_name = normalize_optional_text(input.class_name);
    let (aria_label, has_custom_label) =
        normalize_aria_label_with_fallback(input.aria_label, input.fallback_aria_label.as_str());

    WellNormalizedProps {
        state_input: WellStateInput {
            tone,
            density,
            inset: is_inset,
            has_custom_label,
            has_custom_class_name: class_name.is_some(),
        },
        aria_label,
        class_name,
        tone_source_attr,
        density_source_attr,
        inset_source_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: WellState) -> String {
    let mut classes = vec![
        "ui-well".to_string(),
        state.tone_class.to_string(),
        state.density_class.to_string(),
    ];

    if state.is_inset {
        classes.push("ui-well--inset".to_string());
    }

    if state.has_custom_label {
        classes.push("ui-well--label-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-well--custom-class".to_string());

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
    fn normalize_defaults_are_explicit_and_stable() {
        assert_eq!(normalize_tone(None), WellTone::Default);
        assert_eq!(normalize_tone(Some(WellTone::Strong)), WellTone::Strong);

        assert_eq!(normalize_density(None), WellDensity::Comfortable);
        assert_eq!(
            normalize_density(Some(WellDensity::Compact)),
            WellDensity::Compact
        );

        assert!(!normalize_is_inset(None));
        assert!(normalize_is_inset(Some(true)));
    }

    #[test]
    fn normalize_props_centralizes_state_input_and_sources() {
        let normalized = normalize_props(WellNormalizeInput {
            tone: None,
            density: Some(WellDensity::Compact),
            is_inset: None,
            aria_label: Some("  Selection summary  ".to_string()),
            fallback_aria_label: "Content well".to_string(),
            class_name: Some(" docs-well ".to_string()),
        });

        assert_eq!(normalized.state_input.tone, WellTone::Default);
        assert_eq!(normalized.state_input.density, WellDensity::Compact);
        assert!(!normalized.state_input.inset);
        assert!(normalized.state_input.has_custom_label);
        assert!(normalized.state_input.has_custom_class_name);
        assert_eq!(normalized.aria_label, "Selection summary");
        assert_eq!(normalized.class_name, Some("docs-well".to_string()));
        assert_eq!(normalized.tone_source_attr, "default");
        assert_eq!(normalized.density_source_attr, "prop");
        assert_eq!(normalized.inset_source_attr, "default");
    }

    #[test]
    fn normalize_aria_label_with_fallback_prefers_explicit_and_sanitizes_fallback() {
        assert_eq!(
            normalize_aria_label_with_fallback(
                Some("  Summary  ".to_string()),
                "Localized default"
            ),
            ("Summary".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label_with_fallback(None, "  Localized default  "),
            ("Localized default".to_string(), false)
        );
        assert_eq!(
            normalize_aria_label_with_fallback(None, "   "),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(WellStateInput {
            tone: WellTone::Quiet,
            density: WellDensity::Comfortable,
            inset: true,
            has_custom_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-well".to_string()), state);
        for token in [
            "ui-well",
            "ui-well--tone-quiet",
            "ui-well--density-comfortable",
            "ui-well--inset",
            "ui-well--custom-class",
            "docs-well",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }

    #[test]
    fn source_attr_from_presence_is_closed_set() {
        assert_eq!(source_attr_from_presence(false), "default");
        assert_eq!(source_attr_from_presence(true), "prop");
    }
}
