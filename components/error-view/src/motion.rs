#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ErrorViewMotion {
    pub enabled: bool,
    pub spring: ui_motion::spring::SpringConfig,
    pub hidden_translate_px: f64,
    pub hidden_opacity: f64,
    pub hidden_scale: f64,
}

impl Default for ErrorViewMotion {
    fn default() -> Self {
        Self {
            enabled: true,
            spring: ui_motion::presets::spring_soft(),
            hidden_translate_px: 8.0,
            hidden_opacity: 0.0,
            hidden_scale: 0.96,
        }
    }
}

impl ErrorViewMotion {
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

pub fn sanitize_motion(motion: ErrorViewMotion) -> ErrorViewMotion {
    let default = ErrorViewMotion::default();

    let spring = motion.spring;
    let stiffness = if spring.stiffness.is_finite() && spring.stiffness > 0.0 {
        spring.stiffness
    } else {
        default.spring.stiffness
    };
    let damping = if spring.damping.is_finite() && spring.damping > 0.0 {
        spring.damping
    } else {
        default.spring.damping
    };
    let mass = if spring.mass.is_finite() && spring.mass > 0.0 {
        spring.mass
    } else {
        default.spring.mass
    };
    let precision = if spring.precision.is_finite() && spring.precision > 0.0 {
        spring.precision
    } else {
        default.spring.precision
    };

    ErrorViewMotion {
        enabled: motion.enabled,
        spring: ui_motion::spring::SpringConfig {
            stiffness,
            damping,
            mass,
            precision,
        },
        hidden_translate_px: sanitize_number(
            motion.hidden_translate_px,
            default.hidden_translate_px,
        )
        .clamp(-240.0, 240.0)
        .abs(),
        hidden_opacity: sanitize_number(motion.hidden_opacity, default.hidden_opacity)
            .clamp(0.0, 1.0),
        hidden_scale: sanitize_number(motion.hidden_scale, default.hidden_scale).clamp(0.5, 1.0),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    visible: leptos::prelude::Signal<bool>,
    motion: ErrorViewMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = sanitize_motion(motion);

    let last_visible = StoredValue::new(None::<bool>);
    let springs = StoredValue::new_local(
        None::<(
            ui_motion::spring::SpringAnimator,
            ui_motion::spring::SpringAnimator,
            ui_motion::spring::SpringAnimator,
        )>,
    );

    Effect::new(move |_| {
        let Some(node) = node_ref.get() else {
            return;
        };

        if springs.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = node.unchecked_into();
        let style = element.style();

        let open_now = visible.get_untracked();
        let initial_translate = if open_now {
            0.0
        } else {
            motion.hidden_translate_px
        };
        let initial_opacity = if open_now { 1.0 } else { motion.hidden_opacity };
        let initial_scale = if open_now { 1.0 } else { motion.hidden_scale };

        drop(style.set_property(
            "--ui-error-view-translate-y",
            &format!("{initial_translate}px"),
        ));
        drop(style.set_property("--ui-error-view-opacity", &format!("{initial_opacity}")));
        drop(style.set_property("--ui-error-view-scale", &format!("{initial_scale}")));
        let style_translate = style.clone();
        let translate =
            ui_motion::spring::SpringAnimator::new(initial_translate, motion.spring, move |next| {
                let next = next.clamp(-1000.0, 1000.0);
                drop(
                    style_translate
                        .set_property("--ui-error-view-translate-y", &format!("{next}px")),
                );
            });

        let style_opacity = style.clone();
        let opacity =
            ui_motion::spring::SpringAnimator::new(initial_opacity, motion.spring, move |next| {
                let next = next.clamp(0.0, 1.0);
                drop(style_opacity.set_property("--ui-error-view-opacity", &format!("{next}")));
            });

        let style_scale = style.clone();
        let scale =
            ui_motion::spring::SpringAnimator::new(initial_scale, motion.spring, move |next| {
                let next = next.clamp(0.5, 1.2);
                drop(style_scale.set_property("--ui-error-view-scale", &format!("{next}")));
            });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((translate, opacity, scale)) = springs_for_cleanup.get_value() {
                translate.stop();
                opacity.stop();
                scale.stop();
            }
        });

        springs.set_value(Some((translate, opacity, scale)));
    });

    Effect::new(move |_| {
        let now_visible = visible.get();
        let Some(previous_visible) = last_visible.get_value() else {
            last_visible.set_value(Some(now_visible));
            return;
        };

        if now_visible == previous_visible {
            return;
        }
        last_visible.set_value(Some(now_visible));

        let Some((translate, opacity, scale)) = springs.get_value() else {
            return;
        };

        if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
            translate.clear_on_rest();
            opacity.clear_on_rest();
            scale.clear_on_rest();

            if now_visible {
                translate.set_target(0.0);
                opacity.set_target(1.0);
                scale.set_target(1.0);
            } else {
                translate.set_target(motion.hidden_translate_px);
                opacity.set_target(motion.hidden_opacity);
                scale.set_target(motion.hidden_scale);
            }
            return;
        }

        translate.clear_on_rest();
        opacity.clear_on_rest();
        scale.clear_on_rest();

        if now_visible {
            translate.set_target(0.0);
            opacity.set_target(1.0);
            scale.set_target(1.0);
        } else {
            translate.set_target(motion.hidden_translate_px);
            opacity.set_target(motion.hidden_opacity);
            scale.set_target(motion.hidden_scale);
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _visible: leptos::prelude::Signal<bool>,
    _motion: ErrorViewMotion,
) {
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
