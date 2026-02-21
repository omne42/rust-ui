pub const DEFAULT_ARIA_LABEL: &str = "Breadcrumb";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbRootStateInput {
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbRootState {
    pub state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbSlotStateInput {
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbSlotState {
    pub state_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbLinkStateInput {
    pub has_href: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbLinkState {
    pub state_attr: &'static str,
    pub href_state_attr: &'static str,
    pub class_source_attr: &'static str,
    pub interactive: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbSeparatorStateInput {
    pub has_custom_content: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbSeparatorState {
    pub state_attr: &'static str,
    pub content_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
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

pub fn normalize_href(value: Option<String>) -> Option<String> {
    normalize_optional_text(value)
}

pub fn resolve_root_state(input: BreadcrumbRootStateInput) -> BreadcrumbRootState {
    BreadcrumbRootState {
        state_attr: if input.has_custom_aria_label || input.has_custom_class_name {
            "customized"
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
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn resolve_slot_state(input: BreadcrumbSlotStateInput) -> BreadcrumbSlotState {
    BreadcrumbSlotState {
        state_attr: if input.has_custom_class_name {
            "customized"
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

pub fn resolve_link_state(input: BreadcrumbLinkStateInput) -> BreadcrumbLinkState {
    BreadcrumbLinkState {
        state_attr: match (input.has_href, input.has_custom_class_name) {
            (true, true) => "interactive-customized",
            (true, false) => "interactive",
            (false, true) => "placeholder-customized",
            (false, false) => "placeholder",
        },
        href_state_attr: if input.has_href { "present" } else { "absent" },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
        interactive: input.has_href,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn resolve_separator_state(input: BreadcrumbSeparatorStateInput) -> BreadcrumbSeparatorState {
    BreadcrumbSeparatorState {
        state_attr: match (input.has_custom_content, input.has_custom_class_name) {
            (true, true) => "customized",
            (true, false) => "custom-content",
            (false, true) => "custom-class",
            (false, false) => "default",
        },
        content_source_attr: if input.has_custom_content {
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

#[cfg(test)]
#[path = "test/breadcrumb.rs"]
mod tests;
