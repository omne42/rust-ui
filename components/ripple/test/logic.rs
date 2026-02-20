use super::*;

#[test]
fn resolve_render_state_keeps_default_priority_in_logic() {
    let render = resolve_render_state(RippleRenderInput {
        is_bounded: None,
        motion: RippleMotion::default(),
        class_name: Some(" docs-ripple-item ".to_string()),
    });

    assert_eq!(render.state.boundary_attr, "bounded");
    assert_eq!(render.state.motion_source_attr, "default");
    assert_eq!(
        render.motion.duration_ms,
        RippleMotion::default().duration_ms
    );
    assert!(render.class_name.contains("docs-ripple-item"));
    assert!(render.style_vars.contains("--ui-ripple-duration-ms:"));
}

#[test]
fn resolve_render_state_tracks_custom_sources() {
    let render = resolve_render_state(RippleRenderInput {
        is_bounded: Some(false),
        motion: RippleMotion {
            duration_ms: 620,
            ..RippleMotion::default()
        },
        class_name: Some("docs-ripple-custom".to_string()),
    });

    assert_eq!(render.state.boundary_attr, "unbounded");
    assert_eq!(render.state.motion_source_attr, "custom");
    assert_eq!(render.state.class_source_attr, "custom");
    assert_eq!(render.motion.duration_ms, 620);
    assert!(render.class_name.contains("docs-ripple-custom"));
    assert!(
        render
            .style_vars
            .contains("--ui-ripple-duration-ms: 620ms;")
    );
}
