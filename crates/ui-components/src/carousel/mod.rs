mod logic;
pub mod styles;
mod view;

pub use crate::active_highlight::ActiveHighlightMotion as CarouselMotion;
pub use logic::{CarouselItem, CarouselOrientation};
pub use view::Carousel;
