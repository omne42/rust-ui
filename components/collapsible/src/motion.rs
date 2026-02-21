use leptos::{html, prelude::*};
use ui_disclosure::DisclosureMotion;

pub fn sanitize_motion(motion: DisclosureMotion) -> DisclosureMotion {
    ui_disclosure::motion::sanitize_motion(motion)
}

pub fn attach_indicator_motion(
    node_ref: NodeRef<html::Span>,
    is_open: Signal<bool>,
    motion: DisclosureMotion,
) {
    ui_disclosure::motion::attach_indicator_motion(node_ref, is_open, sanitize_motion(motion));
}

pub fn attach_panel_motion(
    panel_ref: NodeRef<html::Div>,
    surface_ref: NodeRef<html::Div>,
    is_open: Signal<bool>,
    is_hidden: RwSignal<bool>,
    motion: DisclosureMotion,
) {
    ui_disclosure::motion::attach_panel_motion(
        panel_ref,
        surface_ref,
        is_open,
        is_hidden,
        sanitize_motion(motion),
    );
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
