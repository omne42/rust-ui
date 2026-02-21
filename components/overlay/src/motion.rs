#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverlayMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_scale: f64,
    pub initial_y_px: f64,
}

impl Default for OverlayMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_flip_3d(),
            initial_scale: 0.96,
            initial_y_px: 8.0,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = OverlayMotion::default().spring;

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

pub fn sanitize_motion(motion: OverlayMotion) -> OverlayMotion {
    let default = OverlayMotion::default();

    OverlayMotion {
        spring: sanitize_spring(motion.spring),
        initial_scale: sanitize_number(motion.initial_scale, default.initial_scale).clamp(0.0, 3.0),
        initial_y_px: sanitize_number(motion.initial_y_px, default.initial_y_px)
            .abs()
            .clamp(0.0, 320.0),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    finish_exit: leptos::prelude::Callback<()>,
    motion: OverlayMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
    let last_state = StoredValue::new(None::<bool>);
    let springs = StoredValue::new_local(
        None::<(
            ui_motion::spring::SpringAnimator,
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
        let motion = motion.get_value();

        let open_now = is_open.get_untracked();
        // Always initialize in the closed state so mounting while open animates in.
        let backdrop_initial = 0.0;
        let panel_opacity_initial = 0.0;
        let panel_scale_initial = motion.initial_scale;
        let panel_y_initial = motion.initial_y_px;

        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-overlay-backdrop-opacity",
            &format!("{backdrop_initial}"),
        );
        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-overlay-panel-opacity",
            &format!("{panel_opacity_initial}"),
        );
        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-overlay-panel-scale",
            &format!("{panel_scale_initial}"),
        );
        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-overlay-panel-y",
            &format!("{panel_y_initial}px")
        );
        let style_for_backdrop = style.clone();
        let backdrop = ui_motion::spring::SpringAnimator::new(backdrop_initial, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            ui_observability::set_css_property_observed_auto!(
                &(style_for_backdrop),
                "--ui-overlay-backdrop-opacity",
                &format!("{v}")
            );
        });

        let style_for_opacity = style.clone();
        let panel_opacity =
            ui_motion::spring::SpringAnimator::new(panel_opacity_initial, config, move |v| {
                let v = v.clamp(0.0, 1.0);
                ui_observability::set_css_property_observed_auto!(
                    &(style_for_opacity),
                    "--ui-overlay-panel-opacity",
                    &format!("{v}")
                );
            });

        let style_for_scale = style.clone();
        let panel_scale =
            ui_motion::spring::SpringAnimator::new(panel_scale_initial, config, move |v| {
                let v = v.clamp(0.0, 10.0);
                ui_observability::set_css_property_observed_auto!(
                    &(style_for_scale),
                    "--ui-overlay-panel-scale",
                    &format!("{v}")
                );
            });

        let style_for_y = style.clone();
        let panel_y = ui_motion::spring::SpringAnimator::new(panel_y_initial, config, move |v| {
            let v = v.clamp(-1000.0, 1000.0);
            ui_observability::set_css_property_observed_auto!(
                &(style_for_y),
                "--ui-overlay-panel-y",
                &format!("{v}px")
            );
        });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((backdrop, opacity, scale, y)) = springs_for_cleanup.get_value() {
                backdrop.stop();
                opacity.stop();
                scale.stop();
                y.stop();
            }
        });

        if open_now {
            backdrop.set_target(1.0);
            panel_opacity.set_target(1.0);
            panel_scale.set_target(1.0);
            panel_y.set_target(0.0);
        }

        springs.set_value(Some((backdrop, panel_opacity, panel_scale, panel_y)));
    });

    Effect::new(move |_| {
        let open = is_open.get();
        let Some(prev) = last_state.get_value() else {
            last_state.set_value(Some(open));
            return;
        };
        if prev == open {
            return;
        }
        last_state.set_value(Some(open));

        let Some((backdrop, opacity, scale, y)) = springs.get_value() else {
            return;
        };

        let motion = motion.get_value();
        if open {
            backdrop.clear_on_rest();
            opacity.clear_on_rest();
            scale.clear_on_rest();
            y.clear_on_rest();

            backdrop.set_target(1.0);
            opacity.set_target(1.0);
            scale.set_target(1.0);
            y.set_target(0.0);
            return;
        }

        // Closing: animate out, then unmount when the last spring settles.
        backdrop.set_target(0.0);
        opacity.set_target(0.0);
        scale.set_target(motion.initial_scale);
        y.set_target(motion.initial_y_px);

        let finish_exit = finish_exit.clone();
        scale.set_on_rest(move || finish_exit.run(()));
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    finish_exit: leptos::prelude::Callback<()>,
    _motion: OverlayMotion,
) {
    // SSR/desktop tool builds: no motion. Presence becomes immediate.
    use leptos::prelude::*;

    Effect::new(move |_| {
        if !is_open.get() {
            finish_exit.run(());
        }
    });
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
