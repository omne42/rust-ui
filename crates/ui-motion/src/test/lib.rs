use super::{keyframes::MotionKeyframe, options::MotionOptions, web};

#[test]
fn non_wasm_web_backend_is_predictable_noop() {
    assert!(web::prefers_reduced_motion());
    web::animate(&(), &[MotionKeyframe::default()], MotionOptions::default());
}
