use ui_visual_primitive::ripple;

pub use ui_visual_primitive::ripple::{RippleMotion, trigger_ripple, trigger_ripple_at};

pub(crate) fn resolve_motion(input: RippleMotion) -> (RippleMotion, &'static str, String) {
    let motion = ripple::sanitize_motion(input);
    let motion_source = ripple::source_attr(motion);
    let style_vars = ripple::attach_motion(None, motion);
    (motion, motion_source, style_vars)
}
