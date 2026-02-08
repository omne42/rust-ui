use crate::view::{ViewState, ViewStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "View";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ViewBackground {
    #[default]
    Default,
    Subtle,
    Accent,
}

impl ViewBackground {
    pub fn class_name(self) -> &'static str {
        match self {
            ViewBackground::Default => "ui-view--background-default",
            ViewBackground::Subtle => "ui-view--background-subtle",
            ViewBackground::Accent => "ui-view--background-accent",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ViewBackground::Default => "default",
            ViewBackground::Subtle => "subtle",
            ViewBackground::Accent => "accent",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ViewBorder {
    #[default]
    None,
    Subtle,
    Strong,
}

impl ViewBorder {
    pub fn class_name(self) -> &'static str {
        match self {
            ViewBorder::None => "ui-view--border-none",
            ViewBorder::Subtle => "ui-view--border-subtle",
            ViewBorder::Strong => "ui-view--border-strong",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ViewBorder::None => "none",
            ViewBorder::Subtle => "subtle",
            ViewBorder::Strong => "strong",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ViewPadding {
    #[default]
    None,
    Sm,
    Md,
    Lg,
}

impl ViewPadding {
    pub fn class_name(self) -> &'static str {
        match self {
            ViewPadding::None => "ui-view--padding-none",
            ViewPadding::Sm => "ui-view--padding-sm",
            ViewPadding::Md => "ui-view--padding-md",
            ViewPadding::Lg => "ui-view--padding-lg",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ViewPadding::None => "none",
            ViewPadding::Sm => "sm",
            ViewPadding::Md => "md",
            ViewPadding::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ViewRadius {
    #[default]
    None,
    Sm,
    Md,
    Lg,
}

impl ViewRadius {
    pub fn class_name(self) -> &'static str {
        match self {
            ViewRadius::None => "ui-view--radius-none",
            ViewRadius::Sm => "ui-view--radius-sm",
            ViewRadius::Md => "ui-view--radius-md",
            ViewRadius::Lg => "ui-view--radius-lg",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ViewRadius::None => "none",
            ViewRadius::Sm => "sm",
            ViewRadius::Md => "md",
            ViewRadius::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ViewShadow {
    #[default]
    None,
    Sm,
    Md,
}

impl ViewShadow {
    pub fn class_name(self) -> &'static str {
        match self {
            ViewShadow::None => "ui-view--shadow-none",
            ViewShadow::Sm => "ui-view--shadow-sm",
            ViewShadow::Md => "ui-view--shadow-md",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ViewShadow::None => "none",
            ViewShadow::Sm => "sm",
            ViewShadow::Md => "md",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ViewElement {
    #[default]
    Div,
    Span,
    Section,
}

impl ViewElement {
    pub fn class_name(self) -> &'static str {
        match self {
            ViewElement::Div => "ui-view--element-div",
            ViewElement::Span => "ui-view--element-span",
            ViewElement::Section => "ui-view--element-section",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            ViewElement::Div => "div",
            ViewElement::Span => "span",
            ViewElement::Section => "section",
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
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: ViewStateInput) -> ViewState {
    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let data_state_attr = if input.fluid {
        "fluid"
    } else if input.border == ViewBorder::Strong || input.shadow == ViewShadow::Md {
        "emphasis"
    } else {
        "default"
    };

    ViewState {
        background: input.background,
        background_class: input.background.class_name(),
        background_attr: input.background.as_attr(),
        border: input.border,
        border_class: input.border.class_name(),
        border_attr: input.border.as_attr(),
        padding: input.padding,
        padding_class: input.padding.class_name(),
        padding_attr: input.padding.as_attr(),
        radius: input.radius,
        radius_class: input.radius.class_name(),
        radius_attr: input.radius.as_attr(),
        shadow: input.shadow,
        shadow_class: input.shadow.class_name(),
        shadow_attr: input.shadow.as_attr(),
        element: input.element,
        element_class: input.element.class_name(),
        element_attr: input.element.as_attr(),
        is_fluid: input.fluid,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: ViewState) -> String {
    let mut classes = vec![
        "ui-view".to_string(),
        state.background_class.to_string(),
        state.border_class.to_string(),
        state.padding_class.to_string(),
        state.radius_class.to_string(),
        state.shadow_class.to_string(),
        state.element_class.to_string(),
    ];

    if state.is_fluid {
        classes.push("ui-view--fluid".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-view--custom-class".to_string());
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
    fn class_and_attr_contracts_are_stable() {
        assert_eq!(
            ViewBackground::Default.class_name(),
            "ui-view--background-default"
        );
        assert_eq!(ViewBackground::Accent.as_attr(), "accent");
        assert_eq!(ViewBorder::Strong.class_name(), "ui-view--border-strong");
        assert_eq!(ViewPadding::Md.as_attr(), "md");
        assert_eq!(ViewRadius::Lg.class_name(), "ui-view--radius-lg");
        assert_eq!(ViewShadow::Sm.as_attr(), "sm");
        assert_eq!(
            ViewElement::Section.class_name(),
            "ui-view--element-section"
        );
    }

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("\n  \t".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-view  ".to_string())),
            Some("docs-view".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_fallback_when_missing() {
        let (custom_label, is_custom) = normalize_aria_label(Some("  Region  ".to_string()));
        assert_eq!(custom_label, "Region");
        assert!(is_custom);

        let (fallback_label, is_custom) = normalize_aria_label(Some("  ".to_string()));
        assert_eq!(fallback_label, DEFAULT_ARIA_LABEL);
        assert!(!is_custom);
    }

    #[test]
    fn resolve_state_tracks_sources_and_priority_state() {
        let state = resolve_state(ViewStateInput {
            background: ViewBackground::Subtle,
            border: ViewBorder::Strong,
            padding: ViewPadding::Lg,
            radius: ViewRadius::Md,
            shadow: ViewShadow::Sm,
            element: ViewElement::Section,
            fluid: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.background_attr, "subtle");
        assert_eq!(state.border_attr, "strong");
        assert_eq!(state.padding_attr, "lg");
        assert_eq!(state.radius_attr, "md");
        assert_eq!(state.shadow_attr, "sm");
        assert_eq!(state.element_attr, "section");
        assert_eq!(state.data_state_attr, "fluid");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_custom_marker_and_user_class() {
        let state = resolve_state(ViewStateInput {
            background: ViewBackground::Accent,
            border: ViewBorder::Subtle,
            padding: ViewPadding::Md,
            radius: ViewRadius::Lg,
            shadow: ViewShadow::Md,
            element: ViewElement::Span,
            fluid: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-view-custom".to_string()), state);

        for token in [
            "ui-view",
            "ui-view--background-accent",
            "ui-view--border-subtle",
            "ui-view--padding-md",
            "ui-view--radius-lg",
            "ui-view--shadow-md",
            "ui-view--element-span",
            "ui-view--custom-class",
            "docs-view-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
