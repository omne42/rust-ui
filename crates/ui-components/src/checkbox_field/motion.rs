#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxFieldMotion {
    pub transition_ms: u16,
    pub indicator_scale_pct: u16,
}

impl Default for CheckboxFieldMotion {
    fn default() -> Self {
        Self {
            transition_ms: 160,
            indicator_scale_pct: 100,
        }
    }
}

pub fn sanitize_motion(motion: CheckboxFieldMotion) -> CheckboxFieldMotion {
    let default = CheckboxFieldMotion::default();

    CheckboxFieldMotion {
        transition_ms: if motion.transition_ms == 0 {
            default.transition_ms
        } else {
            motion.transition_ms.min(1200)
        },
        indicator_scale_pct: motion.indicator_scale_pct.clamp(80, 140),
    }
}

pub fn compose_style_vars(motion: CheckboxFieldMotion) -> String {
    let scale = (motion.indicator_scale_pct as f64) / 100.0;
    format!(
        "--ui-checkbox-field-transition-ms:{}ms;--ui-checkbox-field-indicator-scale:{scale:.3};",
        motion.transition_ms
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_matches_contract() {
        let motion = CheckboxFieldMotion::default();

        assert_eq!(motion.transition_ms, 160);
        assert_eq!(motion.indicator_scale_pct, 100);
    }

    #[test]
    fn sanitize_motion_clamps_invalid_values() {
        let motion = sanitize_motion(CheckboxFieldMotion {
            transition_ms: 0,
            indicator_scale_pct: 200,
        });

        assert_eq!(motion.transition_ms, 160);
        assert_eq!(motion.indicator_scale_pct, 140);

        let motion = sanitize_motion(CheckboxFieldMotion {
            transition_ms: 9000,
            indicator_scale_pct: 40,
        });
        assert_eq!(motion.transition_ms, 1200);
        assert_eq!(motion.indicator_scale_pct, 80);
    }

    #[test]
    fn compose_style_vars_emits_css_custom_properties() {
        let style = compose_style_vars(CheckboxFieldMotion {
            transition_ms: 220,
            indicator_scale_pct: 112,
        });

        assert!(style.contains("--ui-checkbox-field-transition-ms:220ms"));
        assert!(style.contains("--ui-checkbox-field-indicator-scale:1.120"));
    }
}
