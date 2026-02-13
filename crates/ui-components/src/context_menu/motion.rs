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
    let _ = sanitize_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::popover::PopoverMotion;

    #[test]
    fn default_motion_uses_default_popover_motion() {
        let motion = ContextMenuMotion::default();
        assert_eq!(motion.popover, PopoverMotion::default());
    }

    #[test]
    fn sanitize_motion_delegates_to_popover_contract() {
        let input = PopoverMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            initial_scale: f64::NAN,
            offset_y_px: -9999.0,
        };

        let motion = sanitize_motion(ContextMenuMotion { popover: input });
        let expected = crate::popover::motion::sanitize_motion(input);

        assert_eq!(motion.popover, expected);
        assert_eq!(motion.popover.initial_scale, 0.98);
        assert_eq!(motion.popover.offset_y_px, 240.0);
    }
}
