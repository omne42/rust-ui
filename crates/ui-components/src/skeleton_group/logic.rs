use crate::skeleton_group::{SkeletonGroupState, SkeletonGroupStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Skeleton group";

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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(value) = normalize_optional_text(value) {
        (value, true)
    } else {
        (DEFAULT_ARIA_LABEL.to_string(), false)
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
        state.variant_class.to_string(),
        state.layout_class.to_string(),
        state.density_class.to_string(),
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
mod tests {
    use super::*;

    #[test]
    fn enums_expose_stable_class_and_attr_names() {
        assert_eq!(
            SkeletonGroupVariant::Shimmer.class_name(),
            "ui-skeleton-group--variant-shimmer"
        );
        assert_eq!(
            SkeletonGroupVariant::Pulse.class_name(),
            "ui-skeleton-group--variant-pulse"
        );
        assert_eq!(
            SkeletonGroupVariant::None.class_name(),
            "ui-skeleton-group--variant-none"
        );

        assert_eq!(
            SkeletonGroupLayout::Vertical.class_name(),
            "ui-skeleton-group--layout-vertical"
        );
        assert_eq!(
            SkeletonGroupLayout::Horizontal.class_name(),
            "ui-skeleton-group--layout-horizontal"
        );

        assert_eq!(
            SkeletonGroupDensity::Compact.class_name(),
            "ui-skeleton-group--density-compact"
        );
        assert_eq!(
            SkeletonGroupDensity::Comfortable.class_name(),
            "ui-skeleton-group--density-comfortable"
        );

        assert_eq!(SkeletonGroupVariant::Shimmer.as_attr(), "shimmer");
        assert_eq!(SkeletonGroupLayout::Vertical.as_attr(), "vertical");
        assert_eq!(SkeletonGroupDensity::Compact.as_attr(), "compact");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-skeleton-group ".to_string())),
            Some("docs-skeleton-group".to_string())
        );

        assert_eq!(
            normalize_aria_label(Some("  Loading cards ".to_string())),
            ("Loading cards".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(None),
            (DEFAULT_ARIA_LABEL.to_string(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_visibility_and_sources() {
        let state = resolve_state(SkeletonGroupStateInput {
            is_loading: false,
            is_skeleton_only: true,
            variant: SkeletonGroupVariant::Pulse,
            layout: SkeletonGroupLayout::Horizontal,
            density: SkeletonGroupDensity::Compact,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert!(!state.is_loading);
        assert!(state.is_loaded);
        assert!(state.should_hide_root);
        assert_eq!(state.state_attr, "loaded");
        assert_eq!(state.visibility_attr, "hidden");
        assert_eq!(state.loading_mode_attr, "skeleton-only");
        assert_eq!(state.variant_attr, "pulse");
        assert_eq!(state.layout_attr, "horizontal");
        assert_eq!(state.density_attr, "compact");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let state = resolve_state(SkeletonGroupStateInput {
            is_loading: true,
            is_skeleton_only: false,
            variant: SkeletonGroupVariant::Shimmer,
            layout: SkeletonGroupLayout::Vertical,
            density: SkeletonGroupDensity::Comfortable,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-skeleton-group".to_string()), state);

        for expected in [
            "ui-skeleton-group",
            "ui-skeleton-group--variant-shimmer",
            "ui-skeleton-group--layout-vertical",
            "ui-skeleton-group--density-comfortable",
            "ui-skeleton-group--loading",
            "ui-skeleton-group--custom-class",
            "docs-skeleton-group",
        ] {
            assert!(
                class_name.contains(expected),
                "composed class name should include `{expected}`"
            );
        }
    }
}
