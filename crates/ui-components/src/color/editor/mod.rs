pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{
    ColorEditorStateInput, DEFAULT_ALPHA, DEFAULT_AREA, DEFAULT_ARIA_LABEL, DEFAULT_HUE,
    DEFAULT_LABEL,
};
pub use motion::ColorEditorMotion;
pub use ui_state_primitives::color_editor::{ColorEditorFormat, ColorEditorState};
pub use view::ColorEditor;
