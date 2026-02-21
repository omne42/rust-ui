use super::*;
use ui_theme::default_label_motion_tokens;

#[test]
fn default_motion_uses_theme_tokens() {
    let tokens = default_label_motion_tokens();
    let motion = LabelMotion::default();

    assert_eq!(motion.color_transition_ms, tokens.color_duration_ms);
    assert_eq!(motion.weight_transition_ms, tokens.weight_duration_ms);
}

#[test]
fn motion_source_tracks_default_vs_custom() {
    assert_eq!(motion_source_attr(LabelMotion::default()), "default");

    assert_eq!(
        motion_source_attr(LabelMotion {
            color_transition_ms: LabelMotion::default().color_transition_ms + 10,
            ..LabelMotion::default()
        }),
        "custom"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn attach_motion_uses_predictable_non_wasm_reduced_fallback() {
    let style = attach_motion(
        Some("--existing: 1;".to_string()),
        LabelMotion {
            color_transition_ms: 480,
            weight_transition_ms: 320,
        },
    );

    assert!(style.contains("--existing: 1;"));
    assert!(style.contains("--ui-label-motion-color-duration: 1ms;"));
    assert!(style.contains("--ui-label-motion-weight-duration: 1ms;"));
}
