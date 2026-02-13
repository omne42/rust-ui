mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_LABEL_TEXT, DEFAULT_VALUE_TEXT, LabeledValueTone};
pub use motion::LabeledValueMotion;
pub use view::LabeledValue;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LabeledValueOrientation {
    #[default]
    Stacked,
    Inline,
}

impl LabeledValueOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            LabeledValueOrientation::Stacked => "ui-labeled-value--orientation-stacked",
            LabeledValueOrientation::Inline => "ui-labeled-value--orientation-inline",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            LabeledValueOrientation::Stacked => "stacked",
            LabeledValueOrientation::Inline => "inline",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabeledValueStateInput {
    pub orientation: LabeledValueOrientation,
    pub tone: LabeledValueTone,
    pub has_custom_label: bool,
    pub has_custom_value: bool,
    pub has_description: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabeledValueState {
    pub orientation: LabeledValueOrientation,
    pub orientation_class: &'static str,
    pub orientation_attr: &'static str,
    pub tone: LabeledValueTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub has_custom_label: bool,
    pub has_custom_value: bool,
    pub has_description: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub label_source_attr: &'static str,
    pub value_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
}
