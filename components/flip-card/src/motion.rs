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
    let prefers_reduced_motion = StoredValue::new(ui_motion::web::prefers_reduced_motion());
    let last_state = StoredValue::new(None::<(bool, bool)>);
    let springs = StoredValue::new_local(None::<ui_motion::spring::SpringAnimatorTriplet>);

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
        last_state.set_value(Some((flipped_now, hovered_now)));

        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-flip-card-rotation",
            &format!("{rotation_initial}deg")
        );
        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-flip-card-scale",
            &format!("{scale_initial}")
        );
        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-flip-card-tilt",
            &format!("{tilt_initial}deg")
        );
        if prefers_reduced_motion.get_value() {
            // Reduced-motion mode keeps semantic state updates but skips spring runtime.
            return;
        }

        let style_for_rotation = style.clone();
        let style_for_scale = style.clone();
        let style_for_tilt = style.clone();
        let triplet = ui_motion::spring::SpringAnimatorTriplet::new(
            [rotation_initial, scale_initial, tilt_initial],
            config,
            move |v| {
                let v = v.clamp(-360.0, 360.0);
                ui_observability::set_css_property_observed_auto!(
                    &(style_for_rotation),
                    "--ui-flip-card-rotation",
                    &format!("{v}deg")
                );
            },
            move |v| {
                let v = v.clamp(0.6, 1.8);
                ui_observability::set_css_property_observed_auto!(
                    &(style_for_scale),
                    "--ui-flip-card-scale",
                    &format!("{v}")
                );
            },
            move |v| {
                let v = v.clamp(-12.0, 12.0);
                ui_observability::set_css_property_observed_auto!(
                    &(style_for_tilt),
                    "--ui-flip-card-tilt",
                    &format!("{v}deg")
                );
            },
        );

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some(triplet) = springs_for_cleanup.get_value() {
                triplet.stop();
            }
        });

        springs.set_value(Some(triplet));
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

        let motion = motion.get_value();
        let rotation = if flipped { 180.0 } else { 0.0 };
        let scale = if hovered { motion.hover_scale } else { 1.0 };
        let tilt = if hovered { motion.hover_tilt_deg } else { 0.0 };

        if prefers_reduced_motion.get_value() {
            let Some(div) = node_ref.get() else {
                return;
            };
            let element: leptos::web_sys::HtmlElement = div.unchecked_into();
            let style = element.style();
            ui_observability::set_css_property_observed_auto!(
                &(style),
                "--ui-flip-card-rotation",
                &format!("{rotation}deg")
            );
            ui_observability::set_css_property_observed_auto!(
                &(style),
                "--ui-flip-card-scale",
                &format!("{scale}")
            );
            ui_observability::set_css_property_observed_auto!(
                &(style),
                "--ui-flip-card-tilt",
                &format!("{tilt}deg")
            );
            return;
        }

        let Some(triplet) = springs.get_value() else {
            return;
        };

        triplet.set_targets([rotation, scale, tilt]);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _is_flipped: leptos::prelude::Signal<bool>,
    _is_hovered: leptos::prelude::Signal<bool>,
    motion: FlipCardMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
