use super::RippleMotion;
pub use ui_state_primitives::ripple::{
    RippleState, RippleStateInput, compose_class_name, normalize_optional_text, resolve_boundary,
    resolve_phase, resolve_state,
};
use ui_visual_primitive::ripple;

#[derive(Debug)]
pub struct RippleRenderInput {
    pub is_bounded: Option<bool>,
    pub motion: RippleMotion,
    pub class_name: Option<String>,
}

#[derive(Debug)]
pub struct RippleRenderState {
    pub class_name: String,
    pub style_vars: String,
    pub state: RippleState,
    pub motion: RippleMotion,
}

pub fn resolve_render_state(input: RippleRenderInput) -> RippleRenderState {
    let class_name = normalize_optional_text(input.class_name);
    let is_bounded = input.is_bounded.unwrap_or(true);

    let motion = ripple::sanitize_motion(input.motion);
    let motion_source = ripple::source_attr(motion);
    let style_vars = ripple::attach_motion(None, motion);

    let state = resolve_state(RippleStateInput {
        phase: resolve_phase(motion.enabled),
        boundary: resolve_boundary(is_bounded),
        has_custom_motion: motion_source == "custom",
        has_custom_class_name: class_name.is_some(),
    });

    let class_name = compose_class_name(class_name, state);

    RippleRenderState {
        class_name,
        style_vars,
        state,
        motion,
    }
}

#[cfg(test)]
mod tests {
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
}
