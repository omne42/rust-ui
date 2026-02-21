pub use crate::color_slider::ColorSliderMotion as ColorEditorMotion;

pub fn sanitize_motion(motion: ColorEditorMotion) -> ColorEditorMotion {
    crate::color_slider::motion::sanitize_motion(motion)
}

pub fn attach_motion(motion: ColorEditorMotion) -> ColorEditorMotion {
    sanitize_motion(motion)
}

pub fn source_attr(motion: ColorEditorMotion) -> &'static str {
    if sanitize_motion(motion) == ColorEditorMotion::default() {
        "default"
    } else {
        "custom"
    }
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
