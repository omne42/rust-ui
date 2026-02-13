#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextfieldMotion {
    pub transition_duration_ms: u16,
}

impl Default for TextfieldMotion {
    fn default() -> Self {
        Self {
            transition_duration_ms: 140,
        }
    }
}

pub fn sanitize_motion(motion: TextfieldMotion) -> TextfieldMotion {
    TextfieldMotion {
        transition_duration_ms: motion.transition_duration_ms.clamp(60, 1200),
    }
}

pub fn source_attr(motion: TextfieldMotion) -> &'static str {
    if sanitize_motion(motion) == TextfieldMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: TextfieldMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-textfield-motion-duration: {}ms;",
        motion.transition_duration_ms
    ));

    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_textfield_duration() {
        assert_eq!(
            sanitize_motion(TextfieldMotion {
                transition_duration_ms: 10,
            }),
            TextfieldMotion {
                transition_duration_ms: 60,
            }
        );

        assert_eq!(
            sanitize_motion(TextfieldMotion {
                transition_duration_ms: 2500,
            }),
            TextfieldMotion {
                transition_duration_ms: 1200,
            }
        );
    }

    #[test]
    fn attach_motion_adds_textfield_motion_var() {
        let style = attach_motion(
            None,
            TextfieldMotion {
                transition_duration_ms: 220,
            },
        );

        assert!(style.contains("--ui-textfield-motion-duration: 220ms;"));
    }
}
