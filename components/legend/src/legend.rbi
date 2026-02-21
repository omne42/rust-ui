pub type LegendTone = ui_state_primitives::legend::LegendTone;

pub struct LegendMotion {
    pub duration_ms: f64,
    pub spring: ui_motion::spring::SpringConfig,
}

pub fn Legend(
    text: Option<String>,
    tone: LegendTone,
    is_required: Option<bool>,
    is_disabled: Option<bool>,
    motion: LegendMotion,
    required_indicator: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
