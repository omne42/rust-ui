#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorHandleMotion {
    pub duration_ms: u16,
}

impl Default for ColorHandleMotion {
    fn default() -> Self {
        Self { duration_ms: 140 }
    }
}

pub fn sanitize_motion(motion: ColorHandleMotion) -> ColorHandleMotion {
    ColorHandleMotion {
        duration_ms: motion.duration_ms.clamp(60, 1200),
    }
}

pub fn source_attr(motion: ColorHandleMotion) -> &'static str {
    if sanitize_motion(motion) == ColorHandleMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: ColorHandleMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-color-handle-motion-duration: {}ms;",
        motion.duration_ms
    ));

    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_contract_values() {
        assert_eq!(
            sanitize_motion(ColorHandleMotion { duration_ms: 12 }),
            ColorHandleMotion { duration_ms: 60 }
        );
        assert_eq!(
            sanitize_motion(ColorHandleMotion { duration_ms: 3000 }),
            ColorHandleMotion { duration_ms: 1200 }
        );
    }

    #[test]
    fn attach_motion_exports_css_variables() {
        let style = attach_motion(None, ColorHandleMotion { duration_ms: 220 });
        assert!(style.contains("--ui-color-handle-motion-duration: 220ms;"));
    }
}
