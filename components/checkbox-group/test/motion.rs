use super::*;

#[test]
fn sanitize_motion_clamps_duration_and_sanitizes_spring_contract() {
    let default_spring = CheckboxGroupMotion::default().spring;

    assert_eq!(
        sanitize_motion(CheckboxGroupMotion {
            enabled: true,
            transition_duration_ms: 10,
            transition_easing: "ease-in-out",
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: 0.0,
            },
        }),
        CheckboxGroupMotion {
            enabled: true,
            transition_duration_ms: 60,
            transition_easing: "ease-in-out",
            spring: default_spring,
        }
    );

    assert_eq!(
        sanitize_motion(CheckboxGroupMotion {
            enabled: false,
            transition_duration_ms: 2600,
            transition_easing: "ease",
            spring: ui_motion::spring::SpringConfig {
                stiffness: 330.0,
                damping: 19.0,
                mass: 1.5,
                precision: 0.002,
            },
        }),
        CheckboxGroupMotion {
            enabled: false,
            transition_duration_ms: 1200,
            transition_easing: "ease",
            spring: ui_motion::spring::SpringConfig {
                stiffness: 330.0,
                damping: 19.0,
                mass: 1.5,
                precision: 0.002,
            },
        }
    );
}

#[test]
fn resolve_effective_motion_reduces_duration_without_destroying_spring_contract() {
    let resolved = resolve_effective_motion(
        CheckboxGroupMotion {
            enabled: true,
            transition_duration_ms: 240,
            transition_easing: "linear",
            spring: ui_motion::spring::SpringConfig {
                stiffness: 300.0,
                damping: 21.0,
                mass: 1.2,
                precision: 0.003,
            },
        },
        true,
    );

    assert_eq!(resolved.transition_duration_ms, 1);
    assert_eq!(resolved.transition_easing, "linear");
    assert_eq!(resolved.spring.stiffness, 300.0);
    assert_eq!(resolved.spring.damping, 21.0);
    assert!(resolved.reduced);
}

#[test]
fn attach_motion_appends_css_variable_contract() {
    let style = attach_motion(
        Some("--ui-local-var: 1".to_string()),
        CheckboxGroupMotion {
            enabled: true,
            transition_duration_ms: 220,
            transition_easing: "linear",
            spring: ui_motion::spring::SpringConfig {
                stiffness: 280.0,
                damping: 20.0,
                mass: 1.0,
                precision: 0.001,
            },
        },
    );

    assert!(style.contains("--ui-local-var: 1;"));
    assert!(style.contains("--ui-checkbox-group-motion-duration:"));
    assert!(style.contains("--ui-checkbox-group-motion-easing: linear;"));
    assert!(style.contains("--ui-checkbox-group-motion-stiffness: 280;"));
    assert!(style.contains("--ui-checkbox-group-motion-damping: 20;"));
    assert!(style.contains("--ui-checkbox-group-motion-mass: 1;"));
    assert!(style.contains("--ui-checkbox-group-motion-precision: 0.001;"));
    assert!(style.contains("--ui-checkbox-group-motion-reduced:"));
}

#[test]
fn motion_source_attr_reports_default_and_custom() {
    assert_eq!(
        motion_source_attr(CheckboxGroupMotion::default()),
        "default"
    );
    assert_eq!(
        motion_source_attr(CheckboxGroupMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 320.0,
                ..CheckboxGroupMotion::default().spring
            },
            ..CheckboxGroupMotion::default()
        }),
        "custom"
    );
}

#[test]
fn attach_motion_reduced_motion_branch_uses_minimal_feedback_when_disabled() {
    let style = attach_motion(
        None,
        CheckboxGroupMotion {
            enabled: false,
            transition_duration_ms: 420,
            transition_easing: "ease-in",
            spring: ui_motion::spring::SpringConfig {
                stiffness: 290.0,
                damping: 17.0,
                mass: 1.0,
                precision: 0.001,
            },
        },
    );

    assert!(style.contains("--ui-checkbox-group-motion-duration: 1ms;"));
    assert!(style.contains("--ui-checkbox-group-motion-easing: ease-in;"));
    assert!(style.contains("--ui-checkbox-group-motion-stiffness: 290;"));
    assert!(style.contains("--ui-checkbox-group-motion-damping: 17;"));
    assert!(style.contains("--ui-checkbox-group-motion-reduced: true;"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn attach_motion_uses_predictable_non_wasm_reduced_motion_fallback() {
    let style = attach_motion(
        None,
        CheckboxGroupMotion {
            enabled: true,
            transition_duration_ms: 240,
            transition_easing: "ease-out",
            spring: ui_motion::spring::SpringConfig {
                stiffness: 310.0,
                damping: 23.0,
                mass: 1.1,
                precision: 0.002,
            },
        },
    );

    assert!(style.contains("--ui-checkbox-group-motion-duration: 1ms;"));
    assert!(style.contains("--ui-checkbox-group-motion-easing: ease-out;"));
    assert!(style.contains("--ui-checkbox-group-motion-reduced: true;"));
}
