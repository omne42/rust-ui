#[cfg(feature = "progress_bar")]
pub mod bar;
#[cfg(feature = "progress_circle")]
pub mod circle;
#[cfg(feature = "progress")]
mod logic;
#[cfg(feature = "progress")]
mod motion;
#[cfg(feature = "progress")]
pub mod styles;
#[cfg(feature = "progress")]
mod view;

#[cfg(feature = "progress")]
pub use logic::ProgressRange;
#[cfg(feature = "progress")]
pub use motion::ProgressMotion;
#[cfg(feature = "progress")]
pub use view::Progress;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
