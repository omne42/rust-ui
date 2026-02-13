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
mod tests {
    use super::*;

    #[test]
    fn default_motion_matches_contract() {
        let motion = LogicButtonMotion::default();

        assert_eq!(motion.transition_ms, 160);
        assert_eq!(motion.press_scale_pct, 97);
    }

    #[test]
    fn sanitize_motion_clamps_values() {
        let motion = sanitize_motion(LogicButtonMotion {
            transition_ms: 0,
            press_scale_pct: 10,
        });

        assert_eq!(motion.transition_ms, 160);
        assert_eq!(motion.press_scale_pct, 50);

        let motion = sanitize_motion(LogicButtonMotion {
            transition_ms: 3400,
            press_scale_pct: 160,
        });

        assert_eq!(motion.transition_ms, 1200);
        assert_eq!(motion.press_scale_pct, 120);
    }

    #[test]
    fn compose_style_vars_exposes_css_variables() {
        let vars = compose_style_vars(LogicButtonMotion {
            transition_ms: 220,
            press_scale_pct: 93,
        });

        assert!(vars.contains("--ui-logic-button-transition-ms:220ms"));
        assert!(vars.contains("--ui-logic-button-press-scale:0.930"));
    }
}
