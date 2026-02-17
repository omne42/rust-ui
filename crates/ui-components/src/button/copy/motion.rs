use crate::button::ButtonMotion;
use leptos::{html, prelude::*};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonCopyMotion {
    pub button: ButtonMotion,
    pub copied_feedback_spring: ui_motion::spring::SpringConfig,
    pub copied_feedback_scale: f64,
    pub copied_feedback_glow: f64,
}

impl Default for ButtonCopyMotion {
    fn default() -> Self {
        Self {
            button: ButtonMotion::default(),
            copied_feedback_spring: ui_motion::presets::spring_soft(),
            copied_feedback_scale: 0.08,
            copied_feedback_glow: 1.0,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = ButtonCopyMotion::default().copied_feedback_spring;
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

pub fn sanitize_motion(motion: ButtonCopyMotion) -> ButtonCopyMotion {
    let default = ButtonCopyMotion::default();
    ButtonCopyMotion {
        button: crate::button::motion::sanitize_motion(motion.button),
        copied_feedback_spring: sanitize_spring(motion.copied_feedback_spring),
        copied_feedback_scale: sanitize_number(
            motion.copied_feedback_scale,
            default.copied_feedback_scale,
        )
        .clamp(0.0, 0.25),
        copied_feedback_glow: sanitize_number(
            motion.copied_feedback_glow,
            default.copied_feedback_glow,
        )
        .clamp(0.0, 2.0),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: NodeRef<html::Span>,
    is_copied: ReadSignal<bool>,
    motion: ButtonCopyMotion,
) {
    use crate::observability::set_css_property_observed;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);
    let last_state = StoredValue::new(None::<bool>);

    Effect::new(move |_| {
        let config = motion.get_value().copied_feedback_spring;
        let Some(node) = node_ref.get() else {
            return;
        };
        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = node.unchecked_into();
        let style = element.style();
        let copied_feedback_scale = motion.get_value().copied_feedback_scale;
        let copied_feedback_glow = motion.get_value().copied_feedback_glow;

        let animator = ui_motion::spring::SpringAnimator::new(0.0, config, move |progress| {
            let progress = progress.clamp(0.0, 1.0);
            set_css_property_observed(
                &style,
                "--ui-button-copy-burst",
                &format!("{progress}"),
                "button.copy.motion.burst",
            );
            set_css_property_observed(
                &style,
                "--ui-button-copy-feedback-scale",
                &format!("{copied_feedback_scale}"),
                "button.copy.motion.scale",
            );
            set_css_property_observed(
                &style,
                "--ui-button-copy-feedback-glow",
                &format!("{copied_feedback_glow}"),
                "button.copy.motion.glow",
            );
        });

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
        });

        spring.set_value(Some(animator));
    });

    Effect::new(move |_| {
        let copied = is_copied.get();
        if last_state.get_value() == Some(copied) {
            return;
        }
        last_state.set_value(Some(copied));

        let Some(spring) = spring.get_value() else {
            return;
        };
        spring.set_target(if copied { 1.0 } else { 0.0 });
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: NodeRef<html::Span>,
    _is_copied: ReadSignal<bool>,
    motion: ButtonCopyMotion,
) {
    let _ = sanitize_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_matches_button_contract_defaults() {
        let motion = ButtonCopyMotion::default();

        assert_eq!(motion.button, ButtonMotion::default());
        assert_eq!(motion.button.hover_scale, 1.05);
        assert_eq!(motion.button.tap_scale, 0.95);
        assert_eq!(
            motion.copied_feedback_spring,
            ui_motion::presets::spring_soft()
        );
        assert_eq!(motion.copied_feedback_scale, 0.08);
        assert_eq!(motion.copied_feedback_glow, 1.0);
    }

    #[test]
    fn sanitize_motion_delegates_to_button_contract() {
        let input = ButtonMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            hover_scale: f64::NAN,
            tap_scale: f64::INFINITY,
        };
        let motion = sanitize_motion(ButtonCopyMotion {
            button: input,
            ..ButtonCopyMotion::default()
        });
        let expected = crate::button::motion::sanitize_motion(input);

        assert_eq!(motion.button, expected);
        assert_eq!(motion.button.hover_scale, 1.05);
        assert_eq!(motion.button.tap_scale, 0.95);
        assert_eq!(motion.copied_feedback_scale, 0.08);
        assert_eq!(motion.copied_feedback_glow, 1.0);
    }

    #[test]
    fn supports_custom_button_motion_contract() {
        let motion = ButtonCopyMotion {
            button: ButtonMotion {
                spring: ui_motion::spring::SpringConfig {
                    stiffness: 288.0,
                    damping: 19.0,
                    mass: 1.0,
                    precision: 0.002,
                },
                hover_scale: 1.08,
                tap_scale: 0.93,
            },
            ..ButtonCopyMotion::default()
        };

        assert_eq!(motion.button.spring.stiffness, 288.0);
        assert_eq!(motion.button.spring.damping, 19.0);
        assert_eq!(motion.button.spring.mass, 1.0);
        assert_eq!(motion.button.spring.precision, 0.002);
        assert_eq!(motion.button.hover_scale, 1.08);
        assert_eq!(motion.button.tap_scale, 0.93);
    }

    #[test]
    fn sanitize_motion_clamps_feedback_values() {
        let motion = sanitize_motion(ButtonCopyMotion {
            button: ButtonMotion::default(),
            copied_feedback_spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            copied_feedback_scale: f64::INFINITY,
            copied_feedback_glow: -999.0,
        });

        assert_eq!(
            motion.copied_feedback_spring,
            ui_motion::presets::spring_soft()
        );
        assert_eq!(motion.copied_feedback_scale, 0.08);
        assert_eq!(motion.copied_feedback_glow, 0.0);
    }
}
