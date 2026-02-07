#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpacerAxis {
    #[default]
    Vertical,
    Horizontal,
}

impl SpacerAxis {
    pub fn class_name(self) -> &'static str {
        match self {
            SpacerAxis::Vertical => "ui-spacer--axis-vertical",
            SpacerAxis::Horizontal => "ui-spacer--axis-horizontal",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            SpacerAxis::Vertical => "vertical",
            SpacerAxis::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SpacerSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl SpacerSize {
    pub fn class_name(self) -> &'static str {
        match self {
            SpacerSize::Xs => "ui-spacer--size-xs",
            SpacerSize::Sm => "ui-spacer--size-sm",
            SpacerSize::Md => "ui-spacer--size-md",
            SpacerSize::Lg => "ui-spacer--size-lg",
            SpacerSize::Xl => "ui-spacer--size-xl",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            SpacerSize::Xs => "xs",
            SpacerSize::Sm => "sm",
            SpacerSize::Md => "md",
            SpacerSize::Lg => "lg",
            SpacerSize::Xl => "xl",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpacerStateInput {
    pub axis: SpacerAxis,
    pub size: SpacerSize,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpacerState {
    pub axis: SpacerAxis,
    pub size: SpacerSize,
    pub axis_class: &'static str,
    pub size_class: &'static str,
    pub axis_attr: &'static str,
    pub size_attr: &'static str,
    pub is_vertical: bool,
    pub is_horizontal: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: SpacerStateInput) -> SpacerState {
    SpacerState {
        axis: input.axis,
        size: input.size,
        axis_class: input.axis.class_name(),
        size_class: input.size.class_name(),
        axis_attr: input.axis.as_str(),
        size_attr: input.size.as_str(),
        is_vertical: matches!(input.axis, SpacerAxis::Vertical),
        is_horizontal: matches!(input.axis, SpacerAxis::Horizontal),
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SpacerState) -> String {
    let mut classes = vec![
        "ui-spacer".to_string(),
        state.axis_class.to_string(),
        state.size_class.to_string(),
    ];

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
    fn axis_and_size_mappings_are_stable() {
        assert_eq!(
            SpacerAxis::Vertical.class_name(),
            "ui-spacer--axis-vertical"
        );
        assert_eq!(
            SpacerAxis::Horizontal.class_name(),
            "ui-spacer--axis-horizontal"
        );
        assert_eq!(SpacerAxis::Vertical.as_str(), "vertical");
        assert_eq!(SpacerAxis::Horizontal.as_str(), "horizontal");

        assert_eq!(SpacerSize::Xs.class_name(), "ui-spacer--size-xs");
        assert_eq!(SpacerSize::Sm.class_name(), "ui-spacer--size-sm");
        assert_eq!(SpacerSize::Md.class_name(), "ui-spacer--size-md");
        assert_eq!(SpacerSize::Lg.class_name(), "ui-spacer--size-lg");
        assert_eq!(SpacerSize::Xl.class_name(), "ui-spacer--size-xl");

        assert_eq!(SpacerSize::Xs.as_str(), "xs");
        assert_eq!(SpacerSize::Sm.as_str(), "sm");
        assert_eq!(SpacerSize::Md.as_str(), "md");
        assert_eq!(SpacerSize::Lg.as_str(), "lg");
        assert_eq!(SpacerSize::Xl.as_str(), "xl");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-spacer  ".to_string())),
            Some("docs-spacer".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_axis_and_size_flags() {
        let state = resolve_state(SpacerStateInput {
            axis: SpacerAxis::Horizontal,
            size: SpacerSize::Lg,
            has_custom_class_name: true,
        });

        assert_eq!(state.axis, SpacerAxis::Horizontal);
        assert_eq!(state.axis_class, "ui-spacer--axis-horizontal");
        assert_eq!(state.axis_attr, "horizontal");
        assert!(state.is_horizontal);
        assert!(!state.is_vertical);

        assert_eq!(state.size, SpacerSize::Lg);
        assert_eq!(state.size_class, "ui-spacer--size-lg");
        assert_eq!(state.size_attr, "lg");

        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(SpacerStateInput {
                axis: SpacerAxis::Vertical,
                size: SpacerSize::Md,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-spacer",
            "ui-spacer--axis-vertical",
            "ui-spacer--size-md",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
