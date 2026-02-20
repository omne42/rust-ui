use super::*;

#[test]
fn resolve_render_state_centralizes_default_priority() {
    let render = resolve_render_state(SpinnerRenderInput {
        size: SpinnerSize::Md,
        aria_label: Some("  ".to_string()),
        class_name: Some(" docs-spinner ".to_string()),
        motion: SpinnerMotion {
            rotation_duration_ms: 1200,
        },
        default_aria_label: "Loading",
    });

    assert_eq!(render.aria_label, "Loading");
    assert_eq!(render.state.label_source_attr, "default");
    assert_eq!(render.state.class_source_attr, "custom");
    assert_eq!(render.motion_source, "custom");
    assert!(render.class_name.contains("docs-spinner"));
    assert!(
        render
            .style_vars
            .contains("--ui-spinner-rotation-duration: 1200ms;")
    );
}
