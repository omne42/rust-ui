pub type ColorSwatchMotion = crate::illustrated_message::IllustratedMessageMotion;

pub fn sanitize_motion(motion: ColorSwatchMotion) -> ColorSwatchMotion {
    crate::illustrated_message::motion::sanitize_motion(motion)
}

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: ColorSwatchMotion,
) {
    crate::illustrated_message::motion::attach_motion(node_ref, sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
