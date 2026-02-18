#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SkeletonVariant {
    #[default]
    Rect,
    Circle,
}

impl SkeletonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            SkeletonVariant::Rect => "ui-skeleton--variant-rect",
            SkeletonVariant::Circle => "ui-skeleton--variant-circle",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            SkeletonVariant::Rect => "rect",
            SkeletonVariant::Circle => "circle",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SkeletonStateInput {
    pub variant: SkeletonVariant,
    pub is_shimmer: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SkeletonState {
    pub variant: SkeletonVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub state_attr: &'static str,
    pub has_shimmer: bool,
    pub is_still: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: SkeletonStateInput) -> SkeletonState {
    SkeletonState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_str(),
        state_attr: if input.is_shimmer { "shimmer" } else { "still" },
        has_shimmer: input.is_shimmer,
        is_still: !input.is_shimmer,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SkeletonState) -> String {
    let mut classes = vec!["ui-skeleton".to_string(), state.variant_class.to_string()];

    if state.has_shimmer {
        classes.push("ui-skeleton--shimmer".to_string());
    }
    if state.is_still {
        classes.push("ui-skeleton--still".to_string());
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
    fn variant_mappings_are_stable() {
        assert_eq!(
            SkeletonVariant::Rect.class_name(),
            "ui-skeleton--variant-rect"
        );
        assert_eq!(
            SkeletonVariant::Circle.class_name(),
            "ui-skeleton--variant-circle"
        );

        assert_eq!(SkeletonVariant::Rect.as_str(), "rect");
        assert_eq!(SkeletonVariant::Circle.as_str(), "circle");
    }

    #[test]
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-skeleton  ".to_string())),
            Some("docs-skeleton".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_variant_and_animation_flags() {
        let state = resolve_state(SkeletonStateInput {
            variant: SkeletonVariant::Circle,
            is_shimmer: false,
            has_custom_class_name: true,
        });

        assert_eq!(state.variant, SkeletonVariant::Circle);
        assert_eq!(state.variant_class, "ui-skeleton--variant-circle");
        assert_eq!(state.variant_attr, "circle");
        assert_eq!(state.state_attr, "still");
        assert!(!state.has_shimmer);
        assert!(state.is_still);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(SkeletonStateInput {
                variant: SkeletonVariant::Rect,
                is_shimmer: true,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-skeleton",
            "ui-skeleton--variant-rect",
            "ui-skeleton--shimmer",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
