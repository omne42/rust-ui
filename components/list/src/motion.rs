use ui_visual_primitive::active_highlight::ActiveHighlightMotion;

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = ActiveHighlightMotion::default().spring;

    ui_motion::spring::SpringConfig {
        stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {
            value.stiffness
        } else {
            default.stiffness
        },
        damping: if value.damping.is_finite() && value.damping > 0.0 {
            value.damping
        } else {
            default.damping
        },
        mass: if value.mass.is_finite() && value.mass > 0.0 {
            value.mass
        } else {
            default.mass
        },
        precision: if value.precision.is_finite() && value.precision > 0.0 {
            value.precision
        } else {
            default.precision
        },
    }
}

pub type ListMotion = ActiveHighlightMotion;

pub fn sanitize_motion(motion: ListMotion) -> ListMotion {
    ActiveHighlightMotion {
        spring: sanitize_spring(motion.spring),
    }
}

pub fn resolve_motion(motion: ListMotion) -> (ListMotion, bool) {
    let motion = sanitize_motion(motion);
    let is_custom = motion != ListMotion::default();
    (motion, is_custom)
}

pub fn attach_motion(
    container_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    highlight_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    active_index: leptos::prelude::ReadSignal<usize>,
    option_id: leptos::prelude::Callback<usize, String>,
    motion: ListMotion,
) {
    ui_visual_primitive::active_highlight::attach_active_highlight_motion(
        container_ref,
        highlight_ref,
        active_index,
        option_id,
        motion,
    );
}

pub type ListSectionMotion = ui_illustrated_message::IllustratedMessageMotion;

pub fn sanitize_section_motion(motion: ListSectionMotion) -> ListSectionMotion {
    ui_illustrated_message::motion::sanitize_motion(motion)
}

pub fn attach_section_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: ListSectionMotion,
) {
    ui_illustrated_message::motion::attach_motion(node_ref, sanitize_section_motion(motion));
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
