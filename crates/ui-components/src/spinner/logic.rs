#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpinnerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl SpinnerSize {
    pub fn class_name(self) -> &'static str {
        match self {
            SpinnerSize::Sm => "ui-spinner--size-sm",
            SpinnerSize::Md => "ui-spinner--size-md",
            SpinnerSize::Lg => "ui-spinner--size-lg",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            SpinnerSize::Sm => "sm",
            SpinnerSize::Md => "md",
            SpinnerSize::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpinnerStateInput {
    pub size: SpinnerSize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpinnerState {
    pub size: SpinnerSize,
    pub size_class: &'static str,
    pub size_attr: &'static str,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    ("Loading".to_string(), false)
}

pub fn resolve_state(input: SpinnerStateInput) -> SpinnerState {
    SpinnerState {
        size: input.size,
        size_class: input.size.class_name(),
        size_attr: input.size.as_str(),
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SpinnerState) -> String {
    let mut classes = vec!["ui-spinner".to_string(), state.size_class.to_string()];

    if state.has_custom_aria_label {
        classes.push("ui-spinner--custom-label".to_string());
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
    fn size_mappings_are_stable() {
        assert_eq!(SpinnerSize::Sm.class_name(), "ui-spinner--size-sm");
        assert_eq!(SpinnerSize::Md.class_name(), "ui-spinner--size-md");
        assert_eq!(SpinnerSize::Lg.class_name(), "ui-spinner--size-lg");

        assert_eq!(SpinnerSize::Sm.as_str(), "sm");
        assert_eq!(SpinnerSize::Md.as_str(), "md");
        assert_eq!(SpinnerSize::Lg.as_str(), "lg");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  custom-spinner  ".to_string())),
            Some("custom-spinner".to_string())
        );
    }

    #[test]
    fn resolve_aria_label_defaults_and_trims() {
        assert_eq!(resolve_aria_label(None), ("Loading".to_string(), false));
        assert_eq!(
            resolve_aria_label(Some(" Fetching activity ".to_string())),
            ("Fetching activity".to_string(), true)
        );
    }

    #[test]
    fn resolve_state_tracks_size_and_source_flags() {
        let state = resolve_state(SpinnerStateInput {
            size: SpinnerSize::Lg,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.size, SpinnerSize::Lg);
        assert_eq!(state.size_class, "ui-spinner--size-lg");
        assert_eq!(state.size_attr, "lg");
        assert!(state.has_custom_aria_label);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(SpinnerStateInput {
                size: SpinnerSize::Sm,
                has_custom_aria_label: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-spinner",
            "ui-spinner--size-sm",
            "ui-spinner--custom-label",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
