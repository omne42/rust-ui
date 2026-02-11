#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlipCardMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hover_scale: f64,
    pub hover_tilt_deg: f64,
}

impl Default for FlipCardMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_soft(),
            hover_scale: 1.015,
            hover_tilt_deg: 3.0,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = FlipCardMotion::default().spring;

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

pub fn sanitize_motion(motion: FlipCardMotion) -> FlipCardMotion {
    let default = FlipCardMotion::default();

    FlipCardMotion {
        spring: sanitize_spring(motion.spring),
        hover_scale: sanitize_number(motion.hover_scale, default.hover_scale).clamp(0.6, 1.8),
        hover_tilt_deg: sanitize_number(motion.hover_tilt_deg, default.hover_tilt_deg)
            .clamp(-12.0, 12.0),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_flipped: leptos::prelude::Signal<bool>,
    is_hovered: leptos::prelude::Signal<bool>,
    motion: FlipCardMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
    let last_state = StoredValue::new(None::<(bool, bool)>);
    let springs = StoredValue::new_local(
        None::<(
            ui_motion::spring::SpringAnimator,
            ui_motion::spring::SpringAnimator,
            ui_motion::spring::SpringAnimator,
        )>,
    );

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(div) = node_ref.get() else {
            return;
        };
        if springs.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = div.unchecked_into();
        let style = element.style();

        let flipped_now = is_flipped.get_untracked();
        let hovered_now = is_hovered.get_untracked();
        let motion = motion.get_value();

        let rotation_initial = if flipped_now { 180.0 } else { 0.0 };
        let scale_initial = if hovered_now { motion.hover_scale } else { 1.0 };
        let tilt_initial = if hovered_now {
            motion.hover_tilt_deg
        } else {
            0.0
        };

        let _ = style.set_property("--ui-flip-card-rotation", &format!("{rotation_initial}deg"));
        let _ = style.set_property("--ui-flip-card-scale", &format!("{scale_initial}"));
        let _ = style.set_property("--ui-flip-card-tilt", &format!("{tilt_initial}deg"));

        let style_for_rotation = style.clone();
        let rotation = ui_motion::spring::SpringAnimator::new(rotation_initial, config, move |v| {
            let v = v.clamp(-360.0, 360.0);
            let _ = style_for_rotation.set_property("--ui-flip-card-rotation", &format!("{v}deg"));
        });

        let style_for_scale = style.clone();
        let scale = ui_motion::spring::SpringAnimator::new(scale_initial, config, move |v| {
            let v = v.clamp(0.6, 1.8);
            let _ = style_for_scale.set_property("--ui-flip-card-scale", &format!("{v}"));
        });

        let style_for_tilt = style.clone();
        let tilt = ui_motion::spring::SpringAnimator::new(tilt_initial, config, move |v| {
            let v = v.clamp(-12.0, 12.0);
            let _ = style_for_tilt.set_property("--ui-flip-card-tilt", &format!("{v}deg"));
        });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((rotation, scale, tilt)) = springs_for_cleanup.get_value() {
                rotation.stop();
                scale.stop();
                tilt.stop();
            }
        });

        springs.set_value(Some((rotation, scale, tilt)));
    });

    Effect::new(move |_| {
        let flipped = is_flipped.get();
        let hovered = is_hovered.get();

        let Some(prev) = last_state.get_value() else {
            last_state.set_value(Some((flipped, hovered)));
            return;
        };
        if prev == (flipped, hovered) {
            return;
        }
        last_state.set_value(Some((flipped, hovered)));

        let Some((rotation, scale, tilt)) = springs.get_value() else {
            return;
        };

        let motion = motion.get_value();

        rotation.set_target(if flipped { 180.0 } else { 0.0 });
        scale.set_target(if hovered { motion.hover_scale } else { 1.0 });
        tilt.set_target(if hovered { motion.hover_tilt_deg } else { 0.0 });
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _is_flipped: leptos::prelude::Signal<bool>,
    _is_hovered: leptos::prelude::Signal<bool>,
    motion: FlipCardMotion,
) {
    let _ = sanitize_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_uses_soft_spring_contract() {
        let motion = FlipCardMotion::default();

        assert_eq!(motion.spring, ui_motion::presets::spring_soft());
        assert_eq!(motion.hover_scale, 1.015);
        assert_eq!(motion.hover_tilt_deg, 3.0);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let default = FlipCardMotion::default();

        let motion = sanitize_motion(FlipCardMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            hover_scale: f64::NAN,
            hover_tilt_deg: f64::INFINITY,
        });

        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.hover_scale, default.hover_scale);
        assert_eq!(motion.hover_tilt_deg, default.hover_tilt_deg);
    }

    #[test]
    fn supports_custom_motion_contract() {
        let motion = FlipCardMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 240.0,
                damping: 22.0,
                mass: 1.0,
                precision: 0.002,
            },
            hover_scale: 1.03,
            hover_tilt_deg: 4.5,
        };

        assert_eq!(motion.spring.stiffness, 240.0);
        assert_eq!(motion.spring.damping, 22.0);
        assert_eq!(motion.spring.mass, 1.0);
        assert_eq!(motion.spring.precision, 0.002);
        assert_eq!(motion.hover_scale, 1.03);
        assert_eq!(motion.hover_tilt_deg, 4.5);
    }
}
