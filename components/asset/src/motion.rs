pub type AssetMotion = ui_thumbnail::ThumbnailMotion;

pub fn sanitize_motion(motion: AssetMotion) -> AssetMotion {
    ui_thumbnail::motion::sanitize_motion(motion)
}

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    active: leptos::prelude::Signal<bool>,
    motion: AssetMotion,
) {
    ui_thumbnail::motion::attach_motion(node_ref, active, sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
