#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListStateInput {
    pub item_count: usize,
    pub disabled: bool,
    pub has_disabled_items: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_aria_labelledby: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListState {
    pub has_items: bool,
    pub content_class: &'static str,
    pub content_attr: &'static str,
    pub is_disabled: bool,
    pub disabled_class: &'static str,
    pub disabled_attr: &'static str,
    pub has_disabled_items: bool,
    pub disabled_items_class: &'static str,
    pub disabled_items_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn resolve_state(input: ListStateInput) -> ListState {
    let has_items = input.item_count > 0;
    let content_class = if has_items {
        "ui-list--with-items"
    } else {
        "ui-list--empty"
    };
    let content_attr = if has_items { "with-items" } else { "empty" };

    let disabled_class = if input.disabled {
        "ui-list--disabled"
    } else {
        "ui-list--enabled"
    };
    let disabled_attr = if input.disabled {
        "disabled"
    } else {
        "enabled"
    };

    let disabled_items_class = if input.has_disabled_items {
        "ui-list--has-disabled-items"
    } else {
        "ui-list--all-enabled-items"
    };
    let disabled_items_attr = if input.has_disabled_items {
        "has-disabled-items"
    } else {
        "all-enabled-items"
    };

    let aria_source_attr = if input.has_custom_aria_label {
        "aria-label"
    } else if input.has_custom_aria_labelledby {
        "aria-labelledby"
    } else {
        "fallback"
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    ListState {
        has_items,
        content_class,
        content_attr,
        is_disabled: input.disabled,
        disabled_class,
        disabled_attr,
        has_disabled_items: input.has_disabled_items,
        disabled_items_class,
        disabled_items_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ListState) -> String {
    let mut classes = vec![
        "ui-list".to_string(),
        state.content_class.to_string(),
        state.disabled_class.to_string(),
        state.disabled_items_class.to_string(),
    ];

    if state.has_custom_class_name {
        classes.push("ui-list--custom-class".to_string());
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
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-list ".to_string())),
            Some("docs-list".to_string())
        );
    }

    #[test]
    fn resolve_state_tracks_content_a11y_and_disabled_contracts() {
        let state = resolve_state(ListStateInput {
            item_count: 0,
            disabled: true,
            has_disabled_items: true,
            has_custom_aria_label: false,
            has_custom_aria_labelledby: true,
            has_custom_class_name: true,
        });

        assert!(!state.has_items);
        assert_eq!(state.content_class, "ui-list--empty");
        assert_eq!(state.content_attr, "empty");
        assert!(state.is_disabled);
        assert_eq!(state.disabled_class, "ui-list--disabled");
        assert_eq!(state.disabled_attr, "disabled");
        assert!(state.has_disabled_items);
        assert_eq!(state.disabled_items_class, "ui-list--has-disabled-items");
        assert_eq!(state.disabled_items_attr, "has-disabled-items");
        assert_eq!(state.aria_source_attr, "aria-labelledby");
        assert_eq!(state.class_source_attr, "custom");
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn resolve_state_prefers_aria_label_source_when_both_are_present() {
        let state = resolve_state(ListStateInput {
            item_count: 3,
            disabled: false,
            has_disabled_items: false,
            has_custom_aria_label: true,
            has_custom_aria_labelledby: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.aria_source_attr, "aria-label");
    }

    #[test]
    fn compose_class_name_includes_state_and_custom_tokens() {
        let class_name = compose_class_name(
            Some("docs-list".to_string()),
            resolve_state(ListStateInput {
                item_count: 2,
                disabled: false,
                has_disabled_items: false,
                has_custom_aria_label: false,
                has_custom_aria_labelledby: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-list",
            "ui-list--with-items",
            "ui-list--enabled",
            "ui-list--all-enabled-items",
            "ui-list--custom-class",
            "docs-list",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
