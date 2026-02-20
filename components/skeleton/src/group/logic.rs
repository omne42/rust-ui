use super::{SkeletonGroupState, SkeletonGroupStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Skeleton group";
pub const DEFAULT_IS_LOADING: bool = true;
pub const DEFAULT_IS_SKELETON_ONLY: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SkeletonGroupVariant {
    #[default]
    Shimmer,
    Pulse,
    None,
}

impl SkeletonGroupVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            SkeletonGroupVariant::Shimmer => "ui-skeleton-group--variant-shimmer",
            SkeletonGroupVariant::Pulse => "ui-skeleton-group--variant-pulse",
            SkeletonGroupVariant::None => "ui-skeleton-group--variant-none",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SkeletonGroupVariant::Shimmer => "shimmer",
            SkeletonGroupVariant::Pulse => "pulse",
            SkeletonGroupVariant::None => "none",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SkeletonGroupLayout {
    #[default]
    Vertical,
    Horizontal,
}

impl SkeletonGroupLayout {
    pub fn class_name(self) -> &'static str {
        match self {
            SkeletonGroupLayout::Vertical => "ui-skeleton-group--layout-vertical",
            SkeletonGroupLayout::Horizontal => "ui-skeleton-group--layout-horizontal",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SkeletonGroupLayout::Vertical => "vertical",
            SkeletonGroupLayout::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SkeletonGroupDensity {
    Compact,
    #[default]
    Comfortable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SkeletonGroupViewInput {
    pub is_loading: Option<bool>,
    pub is_skeleton_only: Option<bool>,
    pub variant: Option<SkeletonGroupVariant>,
    pub layout: Option<SkeletonGroupLayout>,
    pub density: Option<SkeletonGroupDensity>,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

impl SkeletonGroupDensity {
    pub fn class_name(self) -> &'static str {
        match self {
            SkeletonGroupDensity::Compact => "ui-skeleton-group--density-compact",
            SkeletonGroupDensity::Comfortable => "ui-skeleton-group--density-comfortable",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            SkeletonGroupDensity::Compact => "compact",
            SkeletonGroupDensity::Comfortable => "comfortable",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        (value, true)
    } else {
        (DEFAULT_ARIA_LABEL.into(), false)
    }
}

pub fn normalize_state_input(input: SkeletonGroupViewInput) -> SkeletonGroupStateInput {
    SkeletonGroupStateInput {
        is_loading: input.is_loading.unwrap_or(DEFAULT_IS_LOADING),
        is_skeleton_only: input.is_skeleton_only.unwrap_or(DEFAULT_IS_SKELETON_ONLY),
        variant: input.variant.unwrap_or_default(),
        layout: input.layout.unwrap_or_default(),
        density: input.density.unwrap_or_default(),
        has_custom_is_loading: input.is_loading.is_some(),
        has_custom_is_skeleton_only: input.is_skeleton_only.is_some(),
        has_custom_variant: input.variant.is_some(),
        has_custom_layout: input.layout.is_some(),
        has_custom_density: input.density.is_some(),
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn resolve_state(input: SkeletonGroupStateInput) -> SkeletonGroupState {
    let is_loaded = !input.is_loading;
    let should_hide_root = input.is_skeleton_only && is_loaded;

    let state_attr = if input.is_loading {
        "loading"
    } else {
        "loaded"
    };

    let visibility_attr = if should_hide_root {
        "hidden"
    } else {
        "visible"
    };

    let loading_mode_attr = if input.is_skeleton_only {
        "skeleton-only"
    } else {
        "mixed"
    };

    SkeletonGroupState {
        is_loading: input.is_loading,
        is_loaded,
        is_skeleton_only: input.is_skeleton_only,
        should_hide_root,
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        layout: input.layout,
        layout_class: input.layout.class_name(),
        layout_attr: input.layout.as_attr(),
        density: input.density,
        density_class: input.density.class_name(),
        density_attr: input.density.as_attr(),
        state_attr,
        visibility_attr,
        loading_mode_attr,
        loading_source_attr: if input.has_custom_is_loading {
            "prop"
        } else {
            "default"
        },
        skeleton_only_source_attr: if input.has_custom_is_skeleton_only {
            "prop"
        } else {
            "default"
        },
        variant_source_attr: if input.has_custom_variant {
            "prop"
        } else {
            "default"
        },
        layout_source_attr: if input.has_custom_layout {
            "prop"
        } else {
            "default"
        },
        density_source_attr: if input.has_custom_density {
            "prop"
        } else {
            "default"
        },
        label_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: SkeletonGroupState) -> String {
    let mut classes = vec![
        "ui-skeleton-group".to_string(),
        state.variant_class.into(),
        state.layout_class.into(),
        state.density_class.into(),
    ];

    if state.is_loading {
        classes.push("ui-skeleton-group--loading".to_string());
    } else {
        classes.push("ui-skeleton-group--loaded".to_string());
    }

    if state.is_skeleton_only {
        classes.push("ui-skeleton-group--skeleton-only".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-skeleton-group--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../../test/group/logic.rs"]
mod tests;
