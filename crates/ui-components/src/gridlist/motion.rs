pub use crate::active_highlight::ActiveHighlightMotion as GridListMotion;

pub fn is_custom_motion(motion: GridListMotion) -> bool {
    motion != GridListMotion::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_motion_detection_tracks_default_contract() {
        assert!(!is_custom_motion(GridListMotion::default()));
        assert!(is_custom_motion(GridListMotion {
            spring: ui_motion::presets::spring_soft(),
        }));
    }
}
