#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpinnerMotion {
    pub rotation_duration_ms: u16,
}

impl Default for SpinnerMotion {
    fn default() -> Self {
        Self {
            rotation_duration_ms: 900,
        }
    }
}

pub fn sanitize_motion(motion: SpinnerMotion) -> SpinnerMotion {
    SpinnerMotion {
        rotation_duration_ms: motion.rotation_duration_ms.clamp(240, 4000),
    }
}

pub fn source_attr(motion: SpinnerMotion) -> &'static str {
    if sanitize_motion(motion) == SpinnerMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: SpinnerMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-spinner-rotation-duration: {}ms;",
        motion.rotation_duration_ms
    ));

    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_spinner_rotation_duration() {
        assert_eq!(
            sanitize_motion(SpinnerMotion {
                rotation_duration_ms: 120,
            }),
            SpinnerMotion {
                rotation_duration_ms: 240,
            }
        );
        assert_eq!(
            sanitize_motion(SpinnerMotion {
                rotation_duration_ms: 9000,
            }),
            SpinnerMotion {
                rotation_duration_ms: 4000,
            }
        );
    }

    #[test]
    fn attach_motion_adds_spinner_motion_var() {
        let style = attach_motion(
            None,
            SpinnerMotion {
                rotation_duration_ms: 1200,
            },
        );

        assert!(style.contains("--ui-spinner-rotation-duration: 1200ms;"));
    }
}
