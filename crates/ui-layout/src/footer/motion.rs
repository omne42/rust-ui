#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FooterMotion {
    pub transition_duration_ms: u16,
}

impl Default for FooterMotion {
    fn default() -> Self {
        Self {
            transition_duration_ms: 160,
        }
    }
}

pub fn sanitize_motion(motion: FooterMotion) -> FooterMotion {
    FooterMotion {
        transition_duration_ms: motion.transition_duration_ms.clamp(60, 1200),
    }
}

pub fn source_attr(motion: FooterMotion) -> &'static str {
    if sanitize_motion(motion) == FooterMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: FooterMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-footer-motion-duration: {}ms;",
        motion.transition_duration_ms
    ));

    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_footer_duration() {
        assert_eq!(
            sanitize_motion(FooterMotion {
                transition_duration_ms: 0,
            }),
            FooterMotion {
                transition_duration_ms: 60,
            }
        );
        assert_eq!(
            sanitize_motion(FooterMotion {
                transition_duration_ms: 9999,
            }),
            FooterMotion {
                transition_duration_ms: 1200,
            }
        );
    }

    #[test]
    fn attach_motion_adds_footer_motion_variable() {
        let style = attach_motion(
            Some("--ui-initial: 1;".to_string()),
            FooterMotion {
                transition_duration_ms: 240,
            },
        );

        assert!(style.contains("--ui-footer-motion-duration: 240ms;"));
    }
}
