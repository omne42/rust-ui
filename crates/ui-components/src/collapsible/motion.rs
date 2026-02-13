pub fn sanitize_motion(
    motion: crate::disclosure::DisclosureMotion,
) -> crate::disclosure::DisclosureMotion {
    crate::disclosure::motion::sanitize_motion(motion)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_delegates_to_disclosure_contract() {
        let input = crate::disclosure::DisclosureMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            closed_rotation_deg: f64::NAN,
            open_rotation_deg: f64::INFINITY,
            panel_offset_y_px: -500.0,
        };
        let expected = crate::disclosure::motion::sanitize_motion(input);

        assert_eq!(sanitize_motion(input), expected);
    }
}
