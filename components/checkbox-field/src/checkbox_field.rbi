pub type CheckboxFieldTone = ui_state_primitives::checkbox_field::CheckboxFieldTone;
pub type CheckboxFieldIndicatorPlacement = ui_state_primitives::checkbox_field::CheckboxFieldIndicatorPlacement;

pub struct CheckboxFieldMotion {
    pub enabled: bool,
    pub transition_ms: u16,
    pub indicator_scale_pct: u16,
}

pub fn CheckboxField(
    is_checked: Option<leptos::prelude::ReadSignal<bool>>,
    checked: Option<leptos::prelude::ReadSignal<bool>>,
    on_checked_change: Option<leptos::prelude::WriteSignal<bool>>,
    set_checked: Option<leptos::prelude::WriteSignal<bool>>,
    default_checked: Option<bool>,
    is_disabled: Option<bool>,
    disabled: bool,
    is_invalid: Option<bool>,
    invalid: bool,
    id_base: Option<String>,
    label: Option<String>,
    description: Option<String>,
    aria_label: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    tone: CheckboxFieldTone,
    indicator_placement: CheckboxFieldIndicatorPlacement,
    class_name: Option<String>,
    motion: CheckboxFieldMotion,
) -> impl leptos::prelude::IntoView;
