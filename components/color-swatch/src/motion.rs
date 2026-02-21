use ui_theme::default_swatch_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorSwatchMotion {
    pub enabled: bool,
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_y_px: f64,
    pub initial_opacity: f64,
}

impl Default for ColorSwatchMotion {
    fn default() -> Self {
        let tokens = default_swatch_motion_tokens();
        Self {
            enabled: true,
            spring: ui_motion::spring::SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            },
            initial_y_px: 8.0,
            initial_opacity: 0.0,
        }
    }
}

impl ColorSwatchMotion {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: ColorSwatchMotion) -> ColorSwatchMotion {
    let default = ColorSwatchMotion::default();
    ColorSwatchMotion {
        enabled: motion.enabled,
        spring: ui_motion::spring::sanitize_config(motion.spring, default.spring),
        initial_y_px: sanitize_number(motion.initial_y_px, default.initial_y_px)
            .abs()
            .clamp(0.0, 120.0),
        initial_opacity: sanitize_number(motion.initial_opacity, default.initial_opacity)
            .clamp(0.0, 1.0),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: ColorSwatchMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = sanitize_motion(motion);
    let animator = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let Some(div) = node_ref.get() else {
            return;
        };
        if animator.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = div.unchecked_into();
        let style = element.style();
        let initial_progress = if motion.enabled && !ui_motion::web::prefers_reduced_motion() {
            0.0
        } else {
            1.0
        };
        let initial_y = motion.initial_y_px * (1.0 - initial_progress);
        let initial_opacity =
            motion.initial_opacity + ((1.0 - motion.initial_opacity) * initial_progress);

        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-color-swatch-opacity",
            &format!("{initial_opacity}")
        );
        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-color-swatch-y",
            &format!("{initial_y}px")
        );

        let style_for_apply = style.clone();
        let spring =
            ui_motion::spring::SpringAnimator::new(initial_progress, motion.spring, move |next| {
                let progress = next.clamp(0.0, 1.0);
                let y = motion.initial_y_px * (1.0 - progress);
                let opacity = motion.initial_opacity + ((1.0 - motion.initial_opacity) * progress);

                ui_observability::set_css_property_observed_auto!(
                    &(style_for_apply),
                    "--ui-color-swatch-opacity",
                    &format!("{opacity}")
                );
                ui_observability::set_css_property_observed_auto!(
                    &(style_for_apply),
                    "--ui-color-swatch-y",
                    &format!("{y}px")
                );
            });

        if motion.enabled {
            spring.set_target(1.0);
        } else {
            spring.set_target(initial_progress);
        }
        animator.set_value(Some(spring));
    });

    let animator_for_cleanup = animator;
    on_cleanup(move || {
        if let Some(spring) = animator_for_cleanup.get_value() {
            spring.stop();
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: ColorSwatchMotion,
) {
    std::hint::black_box(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
