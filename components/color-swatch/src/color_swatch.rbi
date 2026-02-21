pub type ColorSwatchState = ui_state_primitives::color_swatch::ColorSwatchState;
pub type ColorSwatchStateInput = ui_state_primitives::color_swatch::ColorSwatchStateInput;
pub type ColorSwatchAlpha = ui_state_primitives::color_swatch::ColorSwatchAlpha;
pub type ColorSwatchSize = ui_state_primitives::color_swatch::ColorSwatchSize;
pub type ColorSwatchRounding = ui_state_primitives::color_swatch::ColorSwatchRounding;
pub type ColorSwatchShape = ui_state_primitives::color_swatch::ColorSwatchShape;
pub type ColorSwatchMotion = crate::ColorSwatchMotion;
pub type ColorSwatchAgentContract = crate::ColorSwatchAgentContract;
pub type ColorSwatchAgentSchema = crate::ColorSwatchAgentSchema;
pub type ColorSwatchAgentSchemaVersion = crate::ColorSwatchAgentSchemaVersion;
pub type ColorSwatchIntent = crate::ColorSwatchIntent;
pub type ColorSwatchUiAction = crate::ColorSwatchUiAction;

pub const DEFAULT_ARIA_LABEL: &str;

pub fn sanitize_color_value(value: Option<String>) -> Option<String>;
pub fn resolve_agent_contract() -> ColorSwatchAgentContract;

pub fn ColorSwatch(
    color: Option<String>,
    color_name: Option<String>,
    size: ColorSwatchSize,
    rounding: ColorSwatchRounding,
    shape: ColorSwatchShape,
    is_bordered: Option<bool>,
    is_decorative: Option<bool>,
    motion: ColorSwatchMotion,
    aria_label: Option<String>,
    class_name: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
) -> impl leptos::prelude::IntoView;
