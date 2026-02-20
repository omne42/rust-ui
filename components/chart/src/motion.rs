pub type ChartMotion = ui_visual_primitive::active_highlight::ActiveHighlightMotion;

pub fn sanitize_motion(motion: ChartMotion) -> ChartMotion {
    motion
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    container_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    highlight_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    active_index: leptos::prelude::ReadSignal<usize>,
    option_id: leptos::prelude::Callback<usize, String>,
    motion: ChartMotion,
) {
    ui_visual_primitive::active_highlight::attach_active_highlight_motion(
        container_ref,
        highlight_ref,
        active_index,
        option_id,
        sanitize_motion(motion),
    );
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    container_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    highlight_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    active_index: leptos::prelude::ReadSignal<usize>,
    option_id: leptos::prelude::Callback<usize, String>,
    motion: ChartMotion,
) {
    ui_visual_primitive::active_highlight::attach_active_highlight_motion(
        container_ref,
        highlight_ref,
        active_index,
        option_id,
        sanitize_motion(motion),
    );
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
