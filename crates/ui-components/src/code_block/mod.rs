mod i18n;
mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use i18n::CodeBlockStrings;
pub use logic::{CodeBlockViewState, resolve_view_state};
pub use motion::CodeBlockMotion;
pub use view::CodeBlock;
