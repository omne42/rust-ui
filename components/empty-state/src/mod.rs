mod i18n;
mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use i18n::EmptyStateStrings;
pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_DESCRIPTION, DEFAULT_TITLE, EmptyStateAlign, EmptyStateState,
    EmptyStateStateInput, EmptyStateTone,
};
pub use motion::EmptyStateMotion;
pub use view::EmptyState;
