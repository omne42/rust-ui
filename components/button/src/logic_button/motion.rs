#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogicButtonMotion {
    pub transition_ms: u16,
    pub press_scale_pct: u16,
}

impl Default for LogicButtonMotion {
    fn default() -> Self {
        Self {
            transition_ms: 160,
            press_scale_pct: 97,
        }
    }
}

pub fn sanitize_motion(motion: LogicButtonMotion) -> LogicButtonMotion {
    let default = LogicButtonMotion::default();

    LogicButtonMotion {
        transition_ms: if motion.transition_ms == 0 {
            default.transition_ms
        } else {
            motion.transition_ms.min(1200)
        },
        press_scale_pct: motion.press_scale_pct.clamp(50, 120),
    }
}

pub fn compose_style_vars(motion: LogicButtonMotion) -> String {
    let press_scale = (motion.press_scale_pct as f64) / 100.0;
    format!(
        "--ui-logic-button-transition-ms:{}ms;--ui-logic-button-press-scale:{press_scale:.3};",
        motion.transition_ms
    )
}

#[cfg(test)]
#[path = "../../test/logic_button/motion.rs"]
mod tests;
