pub(crate) mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use logic::{SwatchBorder, SwatchRounding, SwatchShape, SwatchSize};
pub use motion::SwatchMotion;
pub use view::Swatch;
