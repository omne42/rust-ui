pub const DEFAULT_ARIA_LABEL: &str = "Scrollable region";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ScrollAreaOrientation {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

impl ScrollAreaOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            ScrollAreaOrientation::Vertical => "ui-scroll-area--vertical",
            ScrollAreaOrientation::Horizontal => "ui-scroll-area--horizontal",
            ScrollAreaOrientation::Both => "ui-scroll-area--both",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ScrollAreaOrientation::Vertical => "vertical",
            ScrollAreaOrientation::Horizontal => "horizontal",
            ScrollAreaOrientation::Both => "both",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollAreaStateInput {
    pub orientation: ScrollAreaOrientation,
    pub disabled: bool,
    pub max_height_px: Option<u32>,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollAreaState {
    pub orientation: ScrollAreaOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub disabled: bool,
    pub max_height_px: Option<u32>,
    pub has_custom_max_height: bool,
    pub max_height_attr: &'static str,
    pub has_custom_aria_label: bool,
    pub aria_source_attr: &'static str,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
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

pub fn normalize_max_height(max_height_px: Option<u32>) -> Option<u32> {
    max_height_px.filter(|px| *px > 0)
}

pub fn resolve_state(input: ScrollAreaStateInput) -> ScrollAreaState {
    let max_height_px = normalize_max_height(input.max_height_px);

    ScrollAreaState {
        orientation: input.orientation,
        orientation_class: input.orientation.class_name(),
        orientation_attr: input.orientation.as_attr(),
        disabled: input.disabled,
        max_height_px,
        has_custom_max_height: max_height_px.is_some(),
        max_height_attr: if max_height_px.is_some() {
            "custom"
        } else {
            "default"
        },
        has_custom_aria_label: input.has_custom_aria_label,
        aria_source_attr: if input.has_custom_aria_label {
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

pub fn compose_class_name(base_class_name: Option<String>, state: ScrollAreaState) -> String {
    let mut classes = vec![
        "ui-scroll-area".to_string(),
        state.orientation_class.to_string(),
    ];

    if state.disabled {
        classes.push("ui-scroll-area--disabled".to_string());
    }

    if state.has_custom_max_height {
        classes.push("ui-scroll-area--max-height-custom".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-scroll-area--custom-class".to_string());
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
        assert_eq!(normalize_optional_text(Some("".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  ok  ".to_string())),
            Some("ok".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_default_when_missing() {
        let (aria_label, custom) = normalize_aria_label(None);
        assert_eq!(aria_label, DEFAULT_ARIA_LABEL);
        assert!(!custom);

        let (aria_label, custom) = normalize_aria_label(Some(" Inbox updates ".to_string()));
        assert_eq!(aria_label, "Inbox updates");
        assert!(custom);
    }

    #[test]
    fn normalize_max_height_ignores_zero() {
        assert_eq!(normalize_max_height(Some(0)), None);
        assert_eq!(normalize_max_height(Some(240)), Some(240));
    }

    #[test]
    fn resolve_state_tracks_markers() {
        let state = resolve_state(ScrollAreaStateInput {
            orientation: ScrollAreaOrientation::Horizontal,
            disabled: true,
            max_height_px: Some(180),
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.orientation_attr, "horizontal");
        assert_eq!(state.max_height_attr, "custom");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert!(state.disabled);
    }

    #[test]
    fn compose_class_name_contains_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(ScrollAreaStateInput {
                orientation: ScrollAreaOrientation::Both,
                disabled: true,
                max_height_px: Some(160),
                has_custom_aria_label: false,
                has_custom_class_name: true,
            }),
        );

        for expected in [
            "ui-scroll-area",
            "ui-scroll-area--both",
            "ui-scroll-area--disabled",
            "ui-scroll-area--max-height-custom",
            "ui-scroll-area--custom-class",
            "custom",
        ] {
            assert!(
                class_name.contains(expected),
                "expected class list to contain `{expected}`, got `{class_name}`"
            );
        }
    }
}
