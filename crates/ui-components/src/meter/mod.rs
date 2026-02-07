mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{MeterRange, clamp_to_range, normalize_progress};
pub use motion::MeterMotion;
pub use view::Meter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MeterVariant {
    #[default]
    Default,
    Danger,
}

impl MeterVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            MeterVariant::Default => "ui-meter--variant-default",
            MeterVariant::Danger => "ui-meter--variant-danger",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            MeterVariant::Default => "default",
            MeterVariant::Danger => "danger",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MeterSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl MeterSize {
    pub fn class_name(self) -> &'static str {
        match self {
            MeterSize::Sm => "ui-meter--size-sm",
            MeterSize::Default => "ui-meter--size-default",
            MeterSize::Lg => "ui-meter--size-lg",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            MeterSize::Sm => "sm",
            MeterSize::Default => "default",
            MeterSize::Lg => "lg",
        }
    }
}
