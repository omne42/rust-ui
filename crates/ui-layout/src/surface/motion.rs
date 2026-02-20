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
#[path = "test/motion.rs"]
mod tests;
