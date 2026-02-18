use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollAreaMotion {
    pub duration_ms: f64,
}

impl Default for ScrollAreaMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        Self {
            duration_ms: f64::from(tokens.duration_ms),
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: ScrollAreaMotion) -> ScrollAreaMotion {
    let default = ScrollAreaMotion::default();

    ScrollAreaMotion {
        duration_ms: sanitize_number(motion.duration_ms, default.duration_ms).clamp(1.0, 1000.0),
    }
}

pub fn source_attr(motion: ScrollAreaMotion) -> &'static str {
    if sanitize_motion(motion) == ScrollAreaMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: ScrollAreaMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-scroll-area-motion-duration: {}ms;",
        motion.duration_ms
    ));

    style
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_theme::default_text_field_motion_tokens;

    #[test]
    fn default_motion_comes_from_theme_tokens() {
        let motion = ScrollAreaMotion::default();
        let tokens = default_text_field_motion_tokens();
        assert_eq!(motion.duration_ms, f64::from(tokens.duration_ms));
    }

    #[test]
    fn sanitize_motion_clamps_values() {
        assert_eq!(
            sanitize_motion(ScrollAreaMotion {
                duration_ms: f64::NAN
            }),
            ScrollAreaMotion::default()
        );
        assert_eq!(
            sanitize_motion(ScrollAreaMotion { duration_ms: 0.0 }),
            ScrollAreaMotion { duration_ms: 1.0 }
        );
        assert_eq!(
            sanitize_motion(ScrollAreaMotion {
                duration_ms: 5000.0
            }),
            ScrollAreaMotion {
                duration_ms: 1000.0
            }
        );
    }

    #[test]
    fn source_attr_reflects_default_vs_custom_motion() {
        assert_eq!(source_attr(ScrollAreaMotion::default()), "default");
        assert_eq!(
            source_attr(ScrollAreaMotion { duration_ms: 240.0 }),
            "custom"
        );
    }

    #[test]
    fn attach_motion_outputs_css_variable() {
        assert_eq!(
            attach_motion(None, ScrollAreaMotion { duration_ms: 240.0 }),
            " --ui-scroll-area-motion-duration: 240ms;"
        );

        let style = attach_motion(
            Some("--ui-scroll-area-max-h: 200px;".to_string()),
            ScrollAreaMotion { duration_ms: 240.0 },
        );
        assert!(style.contains("--ui-scroll-area-max-h: 200px;"));
        assert!(style.contains("--ui-scroll-area-motion-duration: 240ms;"));
    }
}
