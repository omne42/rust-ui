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
