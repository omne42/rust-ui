#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSwatchPickerMotion {
    pub transition_ms: u16,
    pub focus_ring_width_px: u16,
}

impl Default for ColorSwatchPickerMotion {
    fn default() -> Self {
        Self {
            transition_ms: 140,
            focus_ring_width_px: 5,
        }
    }
}

pub fn sanitize_motion(motion: ColorSwatchPickerMotion) -> ColorSwatchPickerMotion {
    let default = ColorSwatchPickerMotion::default();

    ColorSwatchPickerMotion {
        transition_ms: if motion.transition_ms == 0 {
            default.transition_ms
        } else {
            motion.transition_ms.min(1200)
        },
        focus_ring_width_px: motion.focus_ring_width_px.clamp(2, 12),
    }
}

pub fn compose_style_vars(motion: ColorSwatchPickerMotion) -> String {
    format!(
        "--ui-color-swatch-picker-transition-ms:{}ms;--ui-color-swatch-picker-focus-ring-width:{}px;",
        motion.transition_ms, motion.focus_ring_width_px
    )
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
