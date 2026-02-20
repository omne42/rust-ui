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
    let default_spring = ButtonCopyMotion::default().copied_feedback_spring;
    crate::button::motion::sanitize_spring_with_fallback(value, default_spring)
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
    is_copied: Signal<bool>,
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
    _is_copied: Signal<bool>,
    motion: ButtonCopyMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../../test/copy/motion.rs"]
mod tests;
