#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SurfaceMotion {
    pub animate_in: bool,
}

pub fn sanitize_motion(motion: SurfaceMotion) -> SurfaceMotion {
    motion
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Section>,
    motion: SurfaceMotion,
) {
    sanitize_motion(motion);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Section>,
    motion: SurfaceMotion,
) {
    sanitize_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_is_noop_contract() {
        assert_eq!(
            SurfaceMotion::default(),
            SurfaceMotion { animate_in: false }
        );
    }

    #[test]
    fn sanitize_motion_preserves_input() {
        let motion = SurfaceMotion { animate_in: true };
        assert_eq!(sanitize_motion(motion), motion);
    }
}
