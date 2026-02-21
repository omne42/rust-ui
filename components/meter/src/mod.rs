mod logic;
pub mod motion;
mod protocol;
pub mod styles;
mod view;

pub use logic::{
    MeterPhase, MeterRange, MeterSize, MeterState, MeterStateInput, MeterVariant, clamp_to_range,
    normalize_progress,
};
pub use motion::MeterMotion;
pub use protocol::{MeterComponentSchemaVersion, MeterComponentSpec};
pub use view::Meter;

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
