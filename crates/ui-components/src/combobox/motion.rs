#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ComboboxMotion {
    pub combo_box: crate::ComboBoxMotion,
}

pub fn sanitize_motion(motion: ComboboxMotion) -> ComboboxMotion {
    ComboboxMotion {
        combo_box: crate::combo_box::motion::sanitize_motion(motion.combo_box),
    }
}

impl ComboboxMotion {
    pub fn attach_motion(self) -> crate::ComboBoxMotion {
        sanitize_motion(self).combo_box
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ActiveHighlightMotion, popover::PopoverMotion};

    #[test]
    fn sanitize_motion_delegates_to_combo_box_motion_contract() {
        let input = crate::ComboBoxMotion {
            popover: PopoverMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: f64::NAN,
                    damping: -1.0,
                    mass: 0.0,
                    precision: f64::INFINITY,
                },
                initial_scale: f64::NAN,
                offset_y_px: -9999.0,
            },
            highlight: ActiveHighlightMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: f64::NAN,
                    damping: -1.0,
                    mass: 0.0,
                    precision: f64::INFINITY,
                },
            },
        };

        let motion = sanitize_motion(ComboboxMotion { combo_box: input });
        let expected = crate::combo_box::motion::sanitize_motion(input);

        assert_eq!(motion.combo_box, expected);
    }
}
