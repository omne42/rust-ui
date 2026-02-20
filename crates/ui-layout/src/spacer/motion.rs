#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SpacerMotion {
    pub animate_in: bool,
}

pub fn sanitize_motion(motion: SpacerMotion) -> SpacerMotion {
    motion
}

pub fn source_attr(motion: SpacerMotion) -> &'static str {
    if sanitize_motion(motion) == SpacerMotion::default() {
        "default"
    } else {
        "custom"
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(_node_ref: leptos::prelude::NodeRef<leptos::html::Div>, motion: SpacerMotion) {
    sanitize_motion(motion);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(_node_ref: leptos::prelude::NodeRef<leptos::html::Div>, motion: SpacerMotion) {
    sanitize_motion(motion);
}

#[cfg(test)]
#[path = "test/motion.rs"]
mod tests;
