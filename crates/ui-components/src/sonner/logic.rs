pub const DEFAULT_ARIA_LABEL: &str = "Notifications";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SonnerPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

impl SonnerPosition {
    pub fn as_attr(self) -> &'static str {
        match self {
            SonnerPosition::TopLeft => "top-left",
            SonnerPosition::TopCenter => "top-center",
            SonnerPosition::TopRight => "top-right",
            SonnerPosition::BottomLeft => "bottom-left",
            SonnerPosition::BottomCenter => "bottom-center",
            SonnerPosition::BottomRight => "bottom-right",
        }
    }

    pub fn class_suffix(self) -> &'static str {
        self.as_attr()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SonnerStateInput {
    pub position: SonnerPosition,
    pub portal: bool,
    pub max_toasts: usize,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SonnerState {
    pub position: SonnerPosition,
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
    if let Some(label) = normalize_optional_text(value) {
        (label, true)
    } else {
        (DEFAULT_ARIA_LABEL.to_string(), false)
    }
}

pub fn normalize_max_toasts(max_toasts: usize) -> usize {
    max_toasts.max(1)
}

pub fn resolve_state(input: SonnerStateInput) -> SonnerState {
    SonnerState {
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

pub fn compose_root_class_name(base_class_name: Option<String>, state: &SonnerState) -> String {
    let mut classes = vec![
        "ui-sonner".to_string(),
        format!("ui-sonner--{}", state.position.class_suffix()),
        if state.portal {
            "ui-sonner--portal".to_string()
        } else {
            "ui-sonner--inline".to_string()
        },
    ];

    if state.has_custom_class_name {
        classes.push("ui-sonner--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn compose_viewport_class_name(position: SonnerPosition) -> String {
    format!(
        "ui-sonner__viewport ui-sonner__viewport--{}",
        position.class_suffix()
    )
}
