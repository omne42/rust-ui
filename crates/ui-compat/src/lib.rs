//! `ui-compat` — compatibility shims and naming parity layers.
//!
//! This crate exists to keep `ui-components` focused on actual UI components while still
//! supporting upstream naming patterns (baseline / RAC / tooling).

#[cfg(feature = "provider")]
pub mod provider;

#[cfg(feature = "rac")]
pub mod rac;

#[cfg(feature = "s2")]
pub mod s2;

#[cfg(feature = "story_utils")]
pub mod story_utils;

#[cfg(feature = "style_macro_s1")]
pub mod style_macro_s1;

#[cfg(feature = "test_utils")]
pub mod test_utils;

#[cfg(feature = "utils")]
pub mod utils;

#[cfg(feature = "provider")]
pub use provider::Provider;
