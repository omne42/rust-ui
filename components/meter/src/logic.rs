pub use ui_state_primitives::meter::{
    MeterPhase, MeterRange, MeterSize, MeterState, MeterStateInput, MeterVariant, clamp_to_range,
    compose_class_name, normalize_optional_text, normalize_progress, resolve_aria_label,
    resolve_phase, resolve_state, resolve_value_label,
};

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
