#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StatusLightVariant {
    #[default]
    Default,
    Accent,
    Danger,
}

impl StatusLightVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            StatusLightVariant::Default => "ui-status-light--variant-default",
            StatusLightVariant::Accent => "ui-status-light--variant-accent",
            StatusLightVariant::Danger => "ui-status-light--variant-danger",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            StatusLightVariant::Default => "default",
            StatusLightVariant::Accent => "accent",
            StatusLightVariant::Danger => "danger",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusLightRole {
    Status,
}

impl StatusLightRole {
    pub fn as_attr(self) -> &'static str {
        match self {
            StatusLightRole::Status => "status",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusLightStateInput {
    pub variant: StatusLightVariant,
    pub role: Option<StatusLightRole>,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusLightState {
    pub variant: StatusLightVariant,
    pub variant_class: &'static str,
    pub variant_attr: &'static str,
    pub role: Option<StatusLightRole>,
    pub role_attr: Option<&'static str>,
    pub state_class: &'static str,
    pub state_attr: &'static str,
    pub role_source_class: &'static str,
    pub role_source_attr: &'static str,
    pub is_live: bool,
    pub has_custom_class_name: bool,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_state(input: StatusLightStateInput) -> StatusLightState {
    let role_attr = input.role.map(StatusLightRole::as_attr);
    let is_live = role_attr.is_some();

    let (state_class, state_attr) = if is_live {
        ("ui-status-light--live", "live")
    } else {
        ("ui-status-light--static", "static")
    };

    let (role_source_class, role_source_attr) = if is_live {
        ("ui-status-light--role-custom", "custom")
    } else {
        ("ui-status-light--role-none", "none")
    };

    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    StatusLightState {
        variant: input.variant,
        variant_class: input.variant.class_name(),
        variant_attr: input.variant.as_attr(),
        role: input.role,
        role_attr,
        state_class,
        state_attr,
        role_source_class,
        role_source_attr,
        is_live,
        has_custom_class_name: input.has_custom_class_name,
        class_source_attr,
    }
}

#[cfg(test)]
#[path = "test/status_light.rs"]
mod tests;
