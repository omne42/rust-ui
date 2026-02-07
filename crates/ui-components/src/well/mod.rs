mod logic;
pub mod styles;
mod view;

pub use logic::WellTone;
pub use view::Well;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WellDensity {
    Comfortable,
    Compact,
}

impl Default for WellDensity {
    fn default() -> Self {
        Self::Comfortable
    }
}

impl WellDensity {
    pub fn class_name(self) -> &'static str {
        match self {
            WellDensity::Comfortable => "ui-well--density-comfortable",
            WellDensity::Compact => "ui-well--density-compact",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            WellDensity::Comfortable => "comfortable",
            WellDensity::Compact => "compact",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WellStateInput {
    pub tone: WellTone,
    pub density: WellDensity,
    pub inset: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WellState {
    pub tone: WellTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub density: WellDensity,
    pub density_class: &'static str,
    pub density_attr: &'static str,
    pub is_inset: bool,
    pub is_not_inset: bool,
    pub has_custom_label: bool,
    pub has_custom_class_name: bool,
    pub label_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
