use crate::divider::DividerOrientation;

pub type DividerMotion = crate::separator::SeparatorMotion;

pub fn sanitize_motion(motion: DividerMotion) -> DividerMotion {
    crate::separator::motion::sanitize_motion(motion)
}

fn map_orientation(orientation: DividerOrientation) -> crate::separator::SeparatorOrientation {
    match orientation {
        DividerOrientation::Horizontal => crate::separator::SeparatorOrientation::Horizontal,
        DividerOrientation::Vertical => crate::separator::SeparatorOrientation::Vertical,
    }
}

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    orientation: DividerOrientation,
    motion: DividerMotion,
) {
    crate::separator::motion::attach_motion(
        node_ref,
        map_orientation(orientation),
        sanitize_motion(motion),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_preserves_separator_contract() {
        let motion = sanitize_motion(DividerMotion { animate_in: true });
        assert!(motion.animate_in);
    }
}
