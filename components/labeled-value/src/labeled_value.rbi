pub type LabeledValueOrientation = ui_state_primitives::labeled_value::LabeledValueOrientation;
pub type LabeledValueTone = ui_state_primitives::labeled_value::LabeledValueTone;
pub type LabeledValueMotion = ui_labeled_value::motion::LabeledValueMotion;

pub fn LabeledValue(
    label: Option<String>,
    value: Option<String>,
    description: Option<String>,
    orientation: LabeledValueOrientation,
    tone: LabeledValueTone,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    motion: LabeledValueMotion,
) -> impl leptos::prelude::IntoView;
