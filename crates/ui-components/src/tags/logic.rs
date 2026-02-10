use crate::tag_group::Tag;
use crate::tags::{TagsState, TagsStateInput};

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn derive_tag_flags(
    tags: &[Tag],
    disabled: bool,
    has_remove_handler: bool,
) -> (bool, bool, bool) {
    let has_tags = !tags.is_empty();
    let has_disabled_tags = has_tags && (disabled || tags.iter().any(|tag| tag.disabled));
    let has_removable_tags =
        has_remove_handler && has_tags && tags.iter().any(|tag| !disabled && !tag.disabled);

    (has_tags, has_disabled_tags, has_removable_tags)
}

pub fn resolve_state(input: TagsStateInput) -> TagsState {
    TagsState {
        is_disabled: input.disabled,
        has_tags: input.has_tags,
        has_disabled_tags: input.has_disabled_tags,
        has_removable_tags: input.has_removable_tags,
        is_invalid: input.is_invalid,
        is_required: input.is_required,
        has_remove_handler: input.has_remove_handler,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_label: input.has_custom_label,
        has_custom_description: input.has_custom_description,
        has_custom_error: input.has_custom_error,
        has_custom_aria_describedby: input.has_custom_aria_describedby,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_variant: input.has_custom_variant,
        has_custom_size: input.has_custom_size,
        state_attr: if input.disabled {
            "disabled"
        } else if input.is_invalid {
            "invalid"
        } else if !input.has_tags {
            "empty"
        } else {
            "ready"
        },
        content_attr: if input.has_tags { "filled" } else { "empty" },
        removal_attr: if input.has_removable_tags {
            "removable"
        } else {
            "static"
        },
        constraint_attr: if input.is_invalid {
            "invalid"
        } else if input.is_required {
            "required"
        } else {
            "optional"
        },
        id_source_attr: if input.has_custom_id_base {
            "custom"
        } else {
            "default"
        },
        label_source_attr: if input.has_custom_label {
            "custom"
        } else {
            "default"
        },
        description_source_attr: if input.has_custom_description {
            "custom"
        } else {
            "default"
        },
        error_source_attr: if input.has_custom_error {
            "custom"
        } else {
            "default"
        },
        describedby_source_attr: if input.has_custom_aria_describedby {
            "custom"
        } else {
            "default"
        },
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        variant_source_attr: if input.has_custom_variant {
            "custom"
        } else {
            "default"
        },
        size_source_attr: if input.has_custom_size {
            "custom"
        } else {
            "default"
        },
        handler_source_attr: if input.has_remove_handler {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: TagsState) -> String {
    let mut classes = vec!["ui-tags".to_string()];

    if state.is_disabled {
        classes.push("ui-tags--disabled".to_string());
    }

    if state.is_invalid {
        classes.push("ui-tags--invalid".to_string());
    }

    if state.is_required {
        classes.push("ui-tags--required".to_string());
    }

    if state.has_tags {
        classes.push("ui-tags--filled".to_string());
    } else {
        classes.push("ui-tags--empty".to_string());
    }

    if state.has_removable_tags {
        classes.push("ui-tags--removable".to_string());
    }

    if state.has_custom_variant {
        classes.push("ui-tags--custom-variant".to_string());
    }

    if state.has_custom_size {
        classes.push("ui-tags--custom-size".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-tags--custom-class".to_string());
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
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Tags docs  ".to_string())),
            Some("Tags docs".to_string())
        );
    }

    #[test]
    fn derive_tag_flags_tracks_disabled_and_removable_paths() {
        let tags = vec![
            Tag::new("rust", "Rust"),
            Tag::disabled("spectrum", "Spectrum"),
        ];

        assert_eq!(derive_tag_flags(&tags, false, true), (true, true, true));
        assert_eq!(derive_tag_flags(&tags, true, true), (true, true, false));
        assert_eq!(derive_tag_flags(&[], false, true), (false, false, false));
    }

    #[test]
    fn resolve_state_tracks_sources_and_modes() {
        let state = resolve_state(TagsStateInput {
            disabled: false,
            has_tags: true,
            has_disabled_tags: true,
            has_removable_tags: true,
            is_invalid: true,
            is_required: false,
            has_remove_handler: true,
            has_custom_id_base: false,
            has_custom_label: true,
            has_custom_description: true,
            has_custom_error: false,
            has_custom_aria_describedby: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_variant: false,
            has_custom_size: true,
        });

        assert_eq!(state.state_attr, "invalid");
        assert_eq!(state.content_attr, "filled");
        assert_eq!(state.removal_attr, "removable");
        assert_eq!(state.constraint_attr, "invalid");
        assert_eq!(state.id_source_attr, "default");
        assert_eq!(state.label_source_attr, "custom");
        assert_eq!(state.description_source_attr, "custom");
        assert_eq!(state.error_source_attr, "default");
        assert_eq!(state.describedby_source_attr, "custom");
        assert_eq!(state.aria_source_attr, "default");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.variant_source_attr, "default");
        assert_eq!(state.size_source_attr, "custom");
        assert_eq!(state.handler_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_markers() {
        let class_name = compose_class_name(
            Some("docs-tags-state".to_string()),
            resolve_state(TagsStateInput {
                disabled: true,
                has_tags: true,
                has_disabled_tags: true,
                has_removable_tags: false,
                is_invalid: true,
                is_required: true,
                has_remove_handler: true,
                has_custom_id_base: true,
                has_custom_label: true,
                has_custom_description: false,
                has_custom_error: true,
                has_custom_aria_describedby: false,
                has_custom_aria_label: true,
                has_custom_class_name: true,
                has_custom_variant: true,
                has_custom_size: true,
            }),
        );

        for token in [
            "ui-tags",
            "ui-tags--disabled",
            "ui-tags--invalid",
            "ui-tags--required",
            "ui-tags--filled",
            "ui-tags--custom-variant",
            "ui-tags--custom-size",
            "ui-tags--custom-class",
            "docs-tags-state",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
