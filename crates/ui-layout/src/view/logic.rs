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
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
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
        state.background_class.into(),
        state.border_class.into(),
        state.padding_class.into(),
        state.radius_class.into(),
        state.shadow_class.into(),
        state.element_class.into(),
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
#[path = "test/logic.rs"]
mod tests;
