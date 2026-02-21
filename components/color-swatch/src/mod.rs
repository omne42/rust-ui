pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    ColorSwatchAgentContract, ColorSwatchAgentSchema, ColorSwatchAgentSchemaVersion,
    ColorSwatchAlpha, ColorSwatchIntent, ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize,
    ColorSwatchState, ColorSwatchStateInput, ColorSwatchUiAction, DEFAULT_ARIA_LABEL,
};
pub use motion::ColorSwatchMotion;
pub use view::ColorSwatch;

pub fn sanitize_color_value(value: Option<String>) -> Option<String> {
    logic::sanitize_color_value(value)
}

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
