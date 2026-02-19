#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorThumbMotion {
    pub handle_duration_ms: u16,
    pub loupe_duration_ms: u16,
}

impl Default for ColorThumbMotion {
    fn default() -> Self {
        Self {
            handle_duration_ms: 140,
            loupe_duration_ms: 120,
        }
    }
}

pub fn sanitize_motion(motion: ColorThumbMotion) -> ColorThumbMotion {
    ColorThumbMotion {
        handle_duration_ms: motion.handle_duration_ms.clamp(60, 1200),
        loupe_duration_ms: motion.loupe_duration_ms.clamp(60, 1200),
    }
}

pub fn source_attr(motion: ColorThumbMotion) -> &'static str {
    if sanitize_motion(motion) == ColorThumbMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: ColorThumbMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-color-thumb-handle-duration: {}ms; --ui-color-thumb-loupe-duration: {}ms;",
        motion.handle_duration_ms, motion.loupe_duration_ms
    ));

    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_contract_values() {
        assert_eq!(
            sanitize_motion(ColorThumbMotion {
                handle_duration_ms: 12,
                loupe_duration_ms: 3000,
            }),
            ColorThumbMotion {
                handle_duration_ms: 60,
                loupe_duration_ms: 1200,
            }
        );
    }

    #[test]
    fn attach_motion_exports_css_variables() {
        let style = attach_motion(
            None,
            ColorThumbMotion {
                handle_duration_ms: 220,
                loupe_duration_ms: 200,
            },
        );

        assert!(style.contains("--ui-color-thumb-handle-duration: 220ms;"));
        assert!(style.contains("--ui-color-thumb-loupe-duration: 200ms;"));
    }
}
