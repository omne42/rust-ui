#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActionBarMotion {
    pub enabled: bool,
    pub spring: ui_motion::spring::SpringConfig,
    pub hidden_translate_px: f64,
    pub hidden_opacity: f64,
}

impl Default for ActionBarMotion {
    fn default() -> Self {
        Self {
            enabled: true,
            spring: ui_motion::presets::spring_soft(),
            hidden_translate_px: 28.0,
            hidden_opacity: 0.0,
        }
    }
}

impl ActionBarMotion {
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

pub fn sanitize_motion(motion: ActionBarMotion) -> ActionBarMotion {
    let default = ActionBarMotion::default();
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

    let hidden_translate_px =
        sanitize_number(motion.hidden_translate_px, default.hidden_translate_px)
            .clamp(-400.0, 400.0)
            .abs();
    let hidden_opacity =
        sanitize_number(motion.hidden_opacity, default.hidden_opacity).clamp(0.0, 1.0);

    ActionBarMotion {
        enabled: motion.enabled,
        spring: ui_motion::spring::SpringConfig {
            stiffness,
            damping,
            mass,
            precision,
        },
        hidden_translate_px,
        hidden_opacity,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    visible: leptos::prelude::Signal<bool>,
    motion: ActionBarMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = sanitize_motion(motion);

    let last_visible = StoredValue::new(None::<bool>);
    let springs = StoredValue::new_local(
        None::<(
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
        let closed_translate = motion.hidden_translate_px;
        let closed_opacity = motion.hidden_opacity;

        let initial_translate = if open_now { 0.0 } else { closed_translate };
        let initial_opacity = if open_now { 1.0 } else { closed_opacity };

        let _ = style.set_property(
            "--ui-action-bar-translate-y",
            &format!("{initial_translate}px"),
        );
        let _ = style.set_property("--ui-action-bar-opacity", &format!("{initial_opacity}"));

        let style_translate = style.clone();
        let translate =
            ui_motion::spring::SpringAnimator::new(initial_translate, motion.spring, move |next| {
                let next = next.clamp(-1000.0, 1000.0);
                let _ = style_translate
                    .set_property("--ui-action-bar-translate-y", &format!("{next}px"));
            });

        let style_opacity = style.clone();
        let opacity =
            ui_motion::spring::SpringAnimator::new(initial_opacity, motion.spring, move |next| {
                let next = next.clamp(0.0, 1.0);
                let _ = style_opacity.set_property("--ui-action-bar-opacity", &format!("{next}"));
            });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((translate, opacity)) = springs_for_cleanup.get_value() {
                translate.stop();
                opacity.stop();
            }
        });

        springs.set_value(Some((translate, opacity)));
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

        let Some((translate, opacity)) = springs.get_value() else {
            return;
        };

        if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
            translate.clear_on_rest();
            opacity.clear_on_rest();
            if now_visible {
                translate.set_target(0.0);
                opacity.set_target(1.0);
            } else {
                translate.set_target(motion.hidden_translate_px);
                opacity.set_target(motion.hidden_opacity);
            }
            return;
        }

        translate.clear_on_rest();
        opacity.clear_on_rest();

        if now_visible {
            translate.set_target(0.0);
            opacity.set_target(1.0);
        } else {
            translate.set_target(motion.hidden_translate_px);
            opacity.set_target(motion.hidden_opacity);
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _visible: leptos::prelude::Signal<bool>,
    _motion: ActionBarMotion,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(ActionBarMotion {
            enabled: true,
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            hidden_translate_px: f64::NAN,
            hidden_opacity: f64::INFINITY,
        });

        let default = ActionBarMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.hidden_translate_px, default.hidden_translate_px);
        assert_eq!(motion.hidden_opacity, default.hidden_opacity);
    }

    #[test]
    fn disabled_constructor_turns_motion_off() {
        assert!(!ActionBarMotion::disabled().enabled);
    }
}
