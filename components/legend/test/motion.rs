use super::*;
use ui_theme::default_text_field_motion_tokens;

#[test]
fn default_motion_is_stable() {
    let tokens = default_text_field_motion_tokens();
    assert_eq!(
        LegendMotion::default(),
        LegendMotion {
            duration_ms: f64::from(tokens.duration_ms),
            spring: ui_motion::presets::spring_soft(),
        }
    );
}

#[test]
fn sanitize_motion_clamps_values() {
    assert_eq!(
        sanitize_motion(LegendMotion {
            duration_ms: f64::NAN,
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: f64::INFINITY,
                precision: 0.0,
            },
        }),
        LegendMotion::default()
    );
    assert_eq!(
        sanitize_motion(LegendMotion {
            duration_ms: -10.0,
            spring: ui_motion::presets::spring_soft(),
        }),
        LegendMotion {
            duration_ms: 1.0,
            spring: ui_motion::presets::spring_soft(),
        }
    );
    assert_eq!(
        sanitize_motion(LegendMotion {
            duration_ms: 9999.0,
            spring: ui_motion::presets::spring_soft(),
        }),
        LegendMotion {
            duration_ms: 800.0,
            spring: ui_motion::presets::spring_soft(),
        }
    );
}

#[test]
fn resolve_effective_motion_respects_reduced_motion_branch() {
    let default = LegendMotion::default();
    let spring = ui_motion::spring::SpringConfig {
        stiffness: default.spring.stiffness + 20.0,
        damping: default.spring.damping + 4.0,
        mass: default.spring.mass + 0.1,
        precision: default.spring.precision * 2.0,
    };
    let custom_duration = default.duration_ms + 40.0;
    let motion = LegendMotion {
        duration_ms: custom_duration,
        spring,
    };

    assert_eq!(
        resolve_effective_motion(motion, false),
        EffectiveLegendMotion {
            duration_ms: custom_duration,
            spring,
            reduced: false,
        }
    );
    assert_eq!(
        resolve_effective_motion(motion, true),
        EffectiveLegendMotion {
            duration_ms: 1.0,
            spring,
            reduced: true,
        }
    );
}

#[test]
fn attach_motion_outputs_contract_css_variables() {
    let default = LegendMotion::default();
    let custom_duration = default.duration_ms + 40.0;
    let source = attach_motion(LegendMotion {
        duration_ms: custom_duration,
        spring: ui_motion::spring::SpringConfig {
            stiffness: default.spring.stiffness + 20.0,
            damping: default.spring.damping + 4.0,
            mass: default.spring.mass + 0.1,
            precision: default.spring.precision * 2.0,
        },
    });

    for needle in [
        "--ui-legend-motion-duration:",
        "--ui-legend-motion-stiffness:",
        "--ui-legend-motion-damping:",
        "--ui-legend-motion-mass:",
        "--ui-legend-motion-precision:",
        "--ui-legend-motion-reduced:",
    ] {
        assert!(
            source.contains(needle),
            "legend motion style vars should include `{needle}`."
        );
    }

    let expected_duration = if cfg!(target_arch = "wasm32") {
        format!("--ui-legend-motion-duration: {custom_duration}ms;")
    } else {
        "--ui-legend-motion-duration: 1ms;".to_string()
    };
    assert!(
        source.contains(&expected_duration),
        "legend duration var should reflect runtime reduced-motion branch."
    );

    let expected_reduced = if cfg!(target_arch = "wasm32") {
        "--ui-legend-motion-reduced: false;"
    } else {
        "--ui-legend-motion-reduced: true;"
    };
    assert!(
        source.contains(expected_reduced),
        "legend reduced-motion marker should expose effective runtime branch."
    );
}
