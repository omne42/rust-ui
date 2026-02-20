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
        (!trimmed.is_empty()).then(|| trimmed.into())
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
    let mut classes = vec!["ui-skeleton".to_string(), state.variant_class.into()];

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
#[path = "test/skeleton.rs"]
mod tests;
