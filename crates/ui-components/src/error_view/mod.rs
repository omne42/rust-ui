mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_MESSAGE};
pub use motion::ErrorViewMotion;
pub use ui_state_primitives::error_view::{ErrorViewState, ErrorViewStateInput, ErrorViewTone};
pub use view::ErrorView;
