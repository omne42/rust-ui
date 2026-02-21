use super::*;

type AttachMotionFn = fn(
    leptos::prelude::NodeRef<leptos::html::Div>,
    leptos::prelude::NodeRef<leptos::html::Div>,
    leptos::prelude::ReadSignal<usize>,
    leptos::prelude::Callback<usize, String>,
    MenuMotion,
);

#[test]
fn sanitize_motion_falls_back_for_invalid_values() {
    let motion = sanitize_motion(MenuMotion {
        highlight: ActiveHighlightMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
        },
    });

    let default = ActiveHighlightMotion::default();
    assert_eq!(motion.highlight.spring.stiffness, default.spring.stiffness);
    assert_eq!(motion.highlight.spring.damping, default.spring.damping);
    assert_eq!(motion.highlight.spring.mass, default.spring.mass);
    assert_eq!(motion.highlight.spring.precision, default.spring.precision);
}

#[test]
fn attach_motion_exposes_component_level_mount_signature() {
    let _attach: AttachMotionFn = attach_motion;
}
