#![cfg(not(target_arch = "wasm32"))]

use ui_motion::{keyframes::MotionKeyframe, options::MotionOptions, web};

#[test]
fn non_wasm_web_backend_prefers_reduced_motion() {
    assert!(web::prefers_reduced_motion());
}

#[test]
fn non_wasm_web_backend_animate_is_safe_noop() {
    let keyframes = vec![
        MotionKeyframe::new().with_offset(0.0).prop("opacity", "0"),
        MotionKeyframe::new().with_offset(1.0).prop("opacity", "1"),
    ];

    web::animate(&(), &keyframes, MotionOptions::default());
}
