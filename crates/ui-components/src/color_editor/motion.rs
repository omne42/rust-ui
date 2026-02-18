pub use crate::color_slider::ColorSliderMotion as ColorEditorMotion;

pub fn sanitize_motion(motion: ColorEditorMotion) -> ColorEditorMotion {
    crate::color_slider::motion::sanitize_motion(motion)
}

pub fn source_attr(motion: ColorEditorMotion) -> &'static str {
    if sanitize_motion(motion) == ColorEditorMotion::default() {
        "default"
    } else {
        "custom"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_attr_tracks_default_vs_custom_motion() {
        assert_eq!(source_attr(ColorEditorMotion::default()), "default");
        assert_eq!(source_attr(ColorEditorMotion::disabled()), "custom");
    }
}
