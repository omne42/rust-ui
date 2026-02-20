#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct DialogMotion {
    pub overlay: crate::overlay::OverlayMotion,
}

pub fn sanitize_motion(motion: DialogMotion) -> DialogMotion {
    DialogMotion {
        overlay: crate::overlay::motion::sanitize_motion(motion.overlay),
    }
}

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    finish_exit: leptos::prelude::Callback<()>,
    motion: DialogMotion,
) {
    crate::overlay::motion::attach_motion(node_ref, is_open, finish_exit, motion.overlay);
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
