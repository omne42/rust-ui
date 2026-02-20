use super::*;

#[test]
fn sanitize_checkbox_group_motion_clamps_duration_to_contract_range() {
    assert_eq!(
        sanitize_checkbox_group_motion(CheckboxGroupMotion {
            transition_duration_ms: 10,
        }),
        CheckboxGroupMotion {
            transition_duration_ms: 60,
        }
    );

    assert_eq!(
        sanitize_checkbox_group_motion(CheckboxGroupMotion {
            transition_duration_ms: 2600,
        }),
        CheckboxGroupMotion {
            transition_duration_ms: 1200,
        }
    );
}

#[test]
fn attach_checkbox_group_motion_appends_css_variable_contract() {
    let style = attach_checkbox_group_motion(
        Some("--ui-local-var: 1".to_string()),
        CheckboxGroupMotion {
            transition_duration_ms: 220,
        },
    );

    assert!(style.contains("--ui-local-var: 1;"));
    assert!(style.contains("--ui-checkbox-group-motion-duration: 220ms;"));
}
