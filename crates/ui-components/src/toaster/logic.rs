pub const DEFAULT_ARIA_LABEL: &str = "Toaster notifications";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToasterPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

impl ToasterPosition {
    pub fn as_attr(self) -> &'static str {
        match self {
            ToasterPosition::TopLeft => "top-left",
            ToasterPosition::TopCenter => "top-center",
            ToasterPosition::TopRight => "top-right",
            ToasterPosition::BottomLeft => "bottom-left",
            ToasterPosition::BottomCenter => "bottom-center",
            ToasterPosition::BottomRight => "bottom-right",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToasterStateInput {
    pub position: ToasterPosition,
    pub portal: bool,
    pub max_toasts: usize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToasterState {
    pub position: ToasterPosition,
    pub position_attr: &'static str,
    pub portal: bool,
    pub portal_attr: &'static str,
    pub max_toasts: usize,
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

pub fn normalize_max_toasts(max_toasts: usize) -> usize {
    max_toasts.max(1)
}

pub fn resolve_state(input: ToasterStateInput) -> ToasterState {
    ToasterState {
        position: input.position,
        position_attr: input.position.as_attr(),
        portal: input.portal,
        portal_attr: if input.portal { "true" } else { "false" },
        max_toasts: normalize_max_toasts(input.max_toasts),
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

pub fn compose_toaster_class_name(base_class_name: Option<String>, state: &ToasterState) -> String {
    let mut classes = vec![
        "ui-toaster".to_string(),
        format!("ui-toaster--{}", state.position_attr),
        if state.portal {
            "ui-toaster--portal".to_string()
        } else {
            "ui-toaster--inline".to_string()
        },
    ];

    if state.has_custom_class_name {
        classes.push("ui-toaster--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn compose_sonner_class_name(position: ToasterPosition) -> String {
    format!(
        "ui-toaster__sonner ui-toaster__sonner--{}",
        position.as_attr()
    )
}
