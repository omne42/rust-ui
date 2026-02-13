#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewMotion {
    pub transition_duration_ms: u16,
}

impl Default for ViewMotion {
    fn default() -> Self {
        Self {
            transition_duration_ms: 160,
        }
    }
}

pub fn sanitize_motion(motion: ViewMotion) -> ViewMotion {
    ViewMotion {
        transition_duration_ms: motion.transition_duration_ms.clamp(60, 1200),
    }
}

pub fn source_attr(motion: ViewMotion) -> &'static str {
    if sanitize_motion(motion) == ViewMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: ViewMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-view-motion-duration: {}ms;",
        motion.transition_duration_ms
    ));

    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_view_duration() {
        assert_eq!(
            sanitize_motion(ViewMotion {
                transition_duration_ms: 0,
            }),
            ViewMotion {
                transition_duration_ms: 60,
            }
        );

        assert_eq!(
            sanitize_motion(ViewMotion {
                transition_duration_ms: 5000,
            }),
            ViewMotion {
                transition_duration_ms: 1200,
            }
        );
    }

    #[test]
    fn attach_motion_adds_view_motion_var() {
        let style = attach_motion(
            Some("--ui-view-local: 1;".to_string()),
            ViewMotion {
                transition_duration_ms: 210,
            },
        );

        assert!(style.contains("--ui-view-local: 1;"));
        assert!(style.contains("--ui-view-motion-duration: 210ms;"));
    }
}
