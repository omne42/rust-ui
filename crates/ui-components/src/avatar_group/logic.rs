use crate::avatar::AvatarSize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarGroupStateInput {
    pub total_count: usize,
    pub max_visible: usize,
    pub size: AvatarSize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvatarGroupState {
    pub total_count: usize,
    pub visible_count: usize,
    pub overflow_count: usize,
    pub max_visible: usize,
    pub size: AvatarSize,
    pub size_attr: &'static str,
    pub is_empty: bool,
    pub has_items: bool,
    pub has_overflow: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_max_visible(value: Option<usize>) -> usize {
    value.unwrap_or(4)
}

pub fn resolve_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    ("Avatar group".to_string(), false)
}

pub fn resolve_state(input: AvatarGroupStateInput) -> AvatarGroupState {
    let visible_count = input.total_count.min(input.max_visible);
    let overflow_count = input.total_count.saturating_sub(visible_count);

    AvatarGroupState {
        total_count: input.total_count,
        visible_count,
        overflow_count,
        max_visible: input.max_visible,
        size: input.size,
        size_attr: input.size.as_str(),
        is_empty: input.total_count == 0,
        has_items: input.total_count > 0,
        has_overflow: overflow_count > 0,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: AvatarGroupState) -> String {
    let mut classes = vec![
        "ui-avatar-group".to_string(),
        format!("ui-avatar-group--size-{}", state.size_attr),
    ];

    if state.is_empty {
        classes.push("ui-avatar-group--empty".to_string());
    }
    if state.has_overflow {
        classes.push("ui-avatar-group--overflow".to_string());
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
    fn normalize_optional_text_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("   ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Team avatars  ".to_string())),
            Some("Team avatars".to_string())
        );
    }

    #[test]
    fn normalize_max_visible_falls_back_to_default() {
        assert_eq!(normalize_max_visible(None), 4);
        assert_eq!(normalize_max_visible(Some(3)), 3);
        assert_eq!(normalize_max_visible(Some(0)), 0);
    }

    #[test]
    fn resolve_aria_label_defaults_and_trims() {
        assert_eq!(
            resolve_aria_label(None),
            ("Avatar group".to_string(), false)
        );
        assert_eq!(
            resolve_aria_label(Some("  Team  ".to_string())),
            ("Team".to_string(), true)
        );
    }

    #[test]
    fn resolve_state_tracks_overflow_and_size_metadata() {
        let state = resolve_state(AvatarGroupStateInput {
            total_count: 6,
            max_visible: 4,
            size: AvatarSize::Lg,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.total_count, 6);
        assert_eq!(state.visible_count, 4);
        assert_eq!(state.overflow_count, 2);
        assert_eq!(state.max_visible, 4);
        assert_eq!(state.size, AvatarSize::Lg);
        assert_eq!(state.size_attr, "lg");
        assert!(!state.is_empty);
        assert!(state.has_items);
        assert!(state.has_overflow);
        assert!(state.has_custom_aria_label);
        assert!(state.has_custom_class_name);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(AvatarGroupStateInput {
                total_count: 0,
                max_visible: 4,
                size: AvatarSize::Sm,
                has_custom_aria_label: false,
                has_custom_class_name: true,
            }),
        );

        for token in [
            "ui-avatar-group",
            "ui-avatar-group--size-sm",
            "ui-avatar-group--empty",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
