use crate::overlay::{OverlayMotion, motion as overlay_motion};

pub const MODAL_MOTION_CONTRACT_STIFFNESS: f64 = 150.0;
pub const MODAL_MOTION_CONTRACT_DAMPING: f64 = 25.0;
pub const MODAL_MOTION_CONTRACT_MASS: f64 = 1.0;
pub const MODAL_MOTION_CONTRACT_PRECISION: f64 = 0.001;
pub const MODAL_MOTION_CONTRACT_INITIAL_SCALE: f64 = 0.96;
pub const MODAL_MOTION_CONTRACT_INITIAL_Y_PX: f64 = 8.0;

pub fn default_motion_contract() -> OverlayMotion {
    OverlayMotion {
        spring: ui_motion::spring::SpringConfig {
            stiffness: MODAL_MOTION_CONTRACT_STIFFNESS,
            damping: MODAL_MOTION_CONTRACT_DAMPING,
            mass: MODAL_MOTION_CONTRACT_MASS,
            precision: MODAL_MOTION_CONTRACT_PRECISION,
        },
        initial_scale: MODAL_MOTION_CONTRACT_INITIAL_SCALE,
        initial_y_px: MODAL_MOTION_CONTRACT_INITIAL_Y_PX,
    }
}

pub fn normalize_motion(motion: OverlayMotion) -> OverlayMotion {
    let sanitized = overlay_motion::sanitize_motion(motion);
    if sanitized == OverlayMotion::default() {
        return overlay_motion::sanitize_motion(default_motion_contract());
    }
    sanitized
}

pub fn is_custom_motion(motion: OverlayMotion) -> bool {
    motion != default_motion_contract()
}
