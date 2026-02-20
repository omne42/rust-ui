use super::*;

#[test]
fn default_motion_uses_default_popover_and_highlight_motion() {
    let motion = ComboBoxMotion::default();

    assert_eq!(motion.popover, PopoverMotion::default());
    assert_eq!(motion.highlight, ActiveHighlightMotion::default());
}

#[test]
fn supports_custom_popover_and_highlight_motion_contracts() {
    let motion = ComboBoxMotion {
        popover: PopoverMotion {
            initial_scale: 0.95,
            offset_y_px: 10.0,
            ..PopoverMotion::default()
        },
        highlight: ActiveHighlightMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 240.0,
                damping: 22.0,
                mass: 1.0,
                precision: 0.002,
            },
        },
    };

    assert_eq!(motion.popover.initial_scale, 0.95);
    assert_eq!(motion.popover.offset_y_px, 10.0);
    assert_eq!(motion.highlight.spring.stiffness, 240.0);
    assert_eq!(motion.highlight.spring.damping, 22.0);
    assert_eq!(motion.highlight.spring.mass, 1.0);
    assert_eq!(motion.highlight.spring.precision, 0.002);
}

#[test]
fn sanitize_motion_falls_back_for_invalid_nested_values() {
    let input = ComboBoxMotion {
        popover: PopoverMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            initial_scale: f64::NAN,
            offset_y_px: -9999.0,
        },
        highlight: ActiveHighlightMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
        },
    };
    let motion = sanitize_motion(input);

    assert_eq!(motion.popover, sanitize_popover_motion(input.popover));
    assert_eq!(motion.popover.initial_scale, 0.98);
    assert_eq!(motion.popover.offset_y_px, 240.0);

    let default_highlight = ActiveHighlightMotion::default();
    assert_eq!(
        motion.highlight.spring.stiffness,
        default_highlight.spring.stiffness
    );
    assert_eq!(
        motion.highlight.spring.damping,
        default_highlight.spring.damping
    );
    assert_eq!(motion.highlight.spring.mass, default_highlight.spring.mass);
    assert_eq!(
        motion.highlight.spring.precision,
        default_highlight.spring.precision
    );
}

#[test]
fn placement_offset_y_follows_vertical_direction_contract() {
    assert_eq!(
        placement_offset_y(PopoverPlacement::BottomStart, 10.0),
        10.0
    );
    assert_eq!(placement_offset_y(PopoverPlacement::BottomEnd, -4.0), 4.0);
    assert_eq!(placement_offset_y(PopoverPlacement::TopStart, 10.0), -10.0);
    assert_eq!(placement_offset_y(PopoverPlacement::TopEnd, -4.0), -4.0);
}
