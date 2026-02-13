use leptos::{html, prelude::*};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ListMotion {
    pub active_highlight: crate::active_highlight::ActiveHighlightMotion,
}

pub fn sanitize_motion(motion: ListMotion) -> ListMotion {
    motion
}

pub fn attach_motion(
    container_ref: NodeRef<html::Div>,
    highlight_ref: NodeRef<html::Div>,
    active_index: ReadSignal<usize>,
    option_id: Callback<usize, String>,
    motion: ListMotion,
) {
    crate::active_highlight::attach_active_highlight_motion(
        container_ref,
        highlight_ref,
        active_index,
        option_id,
        motion.active_highlight,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_uses_active_highlight_defaults() {
        let motion = ListMotion::default();
        assert_eq!(
            motion.active_highlight,
            crate::active_highlight::ActiveHighlightMotion::default()
        );
    }

    #[test]
    fn sanitize_motion_preserves_active_highlight_contract() {
        let motion = ListMotion {
            active_highlight: crate::active_highlight::ActiveHighlightMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: 220.0,
                    damping: 18.0,
                    mass: 1.0,
                    precision: 0.002,
                },
            },
        };

        assert_eq!(sanitize_motion(motion), motion);
    }
}
