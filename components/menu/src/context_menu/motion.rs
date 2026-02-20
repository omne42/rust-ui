pub use crate::dropdown_menu::DropdownMenuMotion as ContextMenuMotion;

pub fn sanitize_motion(motion: ContextMenuMotion) -> ContextMenuMotion {
    ContextMenuMotion {
        popover: crate::popover::motion::sanitize_motion(motion.popover),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    content_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    placement: leptos::prelude::Signal<ui_headless::PopoverPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: ContextMenuMotion,
) {
    crate::popover::motion::attach_motion(
        content_ref,
        is_open,
        placement,
        on_exit_complete,
        sanitize_motion(motion).popover,
    );
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _content_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _is_open: leptos::prelude::Signal<bool>,
    _placement: leptos::prelude::Signal<ui_headless::PopoverPlacement>,
    _on_exit_complete: leptos::prelude::Callback<()>,
    motion: ContextMenuMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../../test/context_menu/motion.rs"]
mod tests;
