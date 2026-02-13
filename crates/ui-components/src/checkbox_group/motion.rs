#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckboxGroupMotion {
    pub transition_duration_ms: u16,
}

impl Default for CheckboxGroupMotion {
    fn default() -> Self {
        Self {
            transition_duration_ms: 140,
        }
    }
}

pub fn sanitize_motion(motion: CheckboxGroupMotion) -> CheckboxGroupMotion {
    CheckboxGroupMotion {
        transition_duration_ms: motion.transition_duration_ms.clamp(60, 1200),
    }
}

pub fn source_attr(motion: CheckboxGroupMotion) -> &'static str {
    if sanitize_motion(motion) == CheckboxGroupMotion::default() {
        "default"
    } else {
        "custom"
    }
}

pub fn attach_motion(base_vars: Option<String>, motion: CheckboxGroupMotion) -> String {
    let motion = sanitize_motion(motion);
    let mut style = base_vars.unwrap_or_default();

    if !style.trim().is_empty() && !style.trim_end().ends_with(';') {
        style.push(';');
    }

    style.push_str(&format!(
        " --ui-checkbox-group-motion-duration: {}ms;",
        motion.transition_duration_ms
    ));

    style
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_duration_to_contract_range() {
        assert_eq!(
            sanitize_motion(CheckboxGroupMotion {
                transition_duration_ms: 10,
            }),
            CheckboxGroupMotion {
                transition_duration_ms: 60,
            }
        );

        assert_eq!(
            sanitize_motion(CheckboxGroupMotion {
                transition_duration_ms: 2600,
            }),
            CheckboxGroupMotion {
                transition_duration_ms: 1200,
            }
        );
    }

    #[test]
    fn attach_motion_appends_css_variable_contract() {
        let style = attach_motion(
            Some("--ui-local-var: 1".to_string()),
            CheckboxGroupMotion {
                transition_duration_ms: 220,
            },
        );

        assert!(style.contains("--ui-local-var: 1;"));
        assert!(style.contains("--ui-checkbox-group-motion-duration: 220ms;"));
    }
}
