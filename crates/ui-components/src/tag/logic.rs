use crate::tag::{TagState, TagStateInput};

pub const DEFAULT_REMOVE_ARIA_LABEL: &str = "Remove tag";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TagVariant {
    #[default]
    Default,
    Surface,
}

impl TagVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            TagVariant::Default => "ui-tag--variant-default",
            TagVariant::Surface => "ui-tag--variant-surface",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TagVariant::Default => "default",
            TagVariant::Surface => "surface",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TagSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl TagSize {
    pub fn class_name(self) -> &'static str {
        match self {
            TagSize::Sm => "ui-tag--size-sm",
            TagSize::Md => "ui-tag--size-md",
            TagSize::Lg => "ui-tag--size-lg",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TagSize::Sm => "sm",
            TagSize::Md => "md",
            TagSize::Lg => "lg",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_remove_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_REMOVE_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: TagStateInput) -> TagState {
    let is_removable = input.removable && input.has_remove_handler;

    let (state_class, state_attr) = if input.disabled {
        ("ui-tag--disabled", "disabled")
    } else if is_removable {
        ("ui-tag--removable", "removable")
    } else {
        ("ui-tag--static", "static")
    };

    TagState {
        variant: input.variant,
        size: input.size,
        variant_class: input.variant.class_name(),
        size_class: input.size.class_name(),
        variant_attr: input.variant.as_attr(),
        size_attr: input.size.as_attr(),
        state_class,
        state_attr,
        is_enabled: !input.disabled,
        is_disabled: input.disabled,
        is_removable,
        is_static: !is_removable,
        has_remove_handler: input.has_remove_handler,
        has_custom_remove_aria_label: input.has_custom_remove_aria_label,
        remove_label_source_attr: if input.has_custom_remove_aria_label {
            "custom"
        } else {
            "default"
        },
        has_custom_class_name: input.has_custom_class_name,
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: TagState) -> String {
    let mut classes = vec![
        "ui-tag".to_string(),
        state.variant_class.to_string(),
        state.size_class.to_string(),
        state.state_class.to_string(),
    ];

    if state.is_enabled {
        classes.push("ui-tag--enabled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-tag--custom-class".to_string());
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
    fn variant_and_size_contract_are_stable() {
        assert_eq!(TagVariant::Default.class_name(), "ui-tag--variant-default");
        assert_eq!(TagVariant::Surface.class_name(), "ui-tag--variant-surface");
        assert_eq!(TagVariant::Default.as_attr(), "default");
        assert_eq!(TagVariant::Surface.as_attr(), "surface");

        assert_eq!(TagSize::Sm.class_name(), "ui-tag--size-sm");
        assert_eq!(TagSize::Md.class_name(), "ui-tag--size-md");
        assert_eq!(TagSize::Lg.class_name(), "ui-tag--size-lg");

        assert_eq!(TagSize::Sm.as_attr(), "sm");
        assert_eq!(TagSize::Md.as_attr(), "md");
        assert_eq!(TagSize::Lg.as_attr(), "lg");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  release-ready  ".to_string())),
            Some("release-ready".to_string())
        );

        assert_eq!(
            normalize_remove_aria_label(Some("  Remove framework  ".to_string())),
            ("Remove framework".to_string(), true)
        );
        assert_eq!(
            normalize_remove_aria_label(None),
            (DEFAULT_REMOVE_ARIA_LABEL.to_string(), false)
        );
    }

    #[test]
    fn resolve_state_tracks_removable_and_source_markers() {
        let removable_state = resolve_state(TagStateInput {
            variant: TagVariant::Surface,
            size: TagSize::Lg,
            disabled: false,
            removable: true,
            has_remove_handler: true,
            has_custom_remove_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(removable_state.state_attr, "removable");
        assert!(removable_state.is_enabled);
        assert!(removable_state.is_removable);
        assert!(!removable_state.is_static);
        assert_eq!(removable_state.remove_label_source_attr, "custom");
        assert_eq!(removable_state.class_source_attr, "custom");

        let disabled_state = resolve_state(TagStateInput {
            variant: TagVariant::Default,
            size: TagSize::Md,
            disabled: true,
            removable: true,
            has_remove_handler: true,
            has_custom_remove_aria_label: false,
            has_custom_class_name: false,
        });

        assert_eq!(disabled_state.state_attr, "disabled");
        assert!(disabled_state.is_disabled);
        assert!(!disabled_state.is_enabled);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(TagStateInput {
            variant: TagVariant::Default,
            size: TagSize::Sm,
            disabled: false,
            removable: false,
            has_remove_handler: false,
            has_custom_remove_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-tag-custom".to_string()), state);

        for token in [
            "ui-tag",
            "ui-tag--variant-default",
            "ui-tag--size-sm",
            "ui-tag--static",
            "ui-tag--enabled",
            "ui-tag--custom-class",
            "docs-tag-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
