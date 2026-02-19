use ui_theme::default_swatch_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwatchMotion {
    pub enabled: bool,
    pub spring: ui_motion::spring::SpringConfig,
    pub selected_scale: f64,
    pub selected_ring_opacity: f64,
}

impl Default for SwatchMotion {
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
            selected_scale: tokens.selected_scale,
            selected_ring_opacity: tokens.selected_ring_opacity,
        }
    }
}

impl SwatchMotion {
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

pub fn sanitize_motion(motion: SwatchMotion) -> SwatchMotion {
    let default = SwatchMotion::default();

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

    SwatchMotion {
        enabled: motion.enabled,
        spring: ui_motion::spring::SpringConfig {
            stiffness,
            damping,
            mass,
            precision,
        },
        selected_scale: sanitize_number(motion.selected_scale, default.selected_scale)
            .clamp(1.0, 1.18),
        selected_ring_opacity: sanitize_number(
            motion.selected_ring_opacity,
            default.selected_ring_opacity,
        )
        .clamp(0.0, 1.0),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    selected: leptos::prelude::Signal<bool>,
    motion: SwatchMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = sanitize_motion(motion);

    let last_selected = StoredValue::new(None::<bool>);
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

        let is_selected_now = selected.get_untracked();
        let initial_scale = if is_selected_now {
            motion.selected_scale
        } else {
            1.0
        };
        let initial_ring = if is_selected_now {
            motion.selected_ring_opacity
        } else {
            0.0
        };

        drop(style.set_property("--ui-swatch-scale", &format!("{initial_scale}")));
        drop(style.set_property("--ui-swatch-ring-opacity", &format!("{initial_ring}")));
        let style_scale = style.clone();
        let scale =
            ui_motion::spring::SpringAnimator::new(initial_scale, motion.spring, move |next| {
                let next = next.clamp(0.92, 1.24);
                drop(style_scale.set_property("--ui-swatch-scale", &format!("{next}")));
            });

        let style_ring = style.clone();
        let ring =
            ui_motion::spring::SpringAnimator::new(initial_ring, motion.spring, move |next| {
                let next = next.clamp(0.0, 1.0);
                drop(style_ring.set_property("--ui-swatch-ring-opacity", &format!("{next}")));
            });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((scale, ring)) = springs_for_cleanup.get_value() {
                scale.stop();
                ring.stop();
            }
        });

        springs.set_value(Some((scale, ring)));
    });

    Effect::new(move |_| {
        let is_selected_now = selected.get();
        let Some(previous_selected) = last_selected.get_value() else {
            last_selected.set_value(Some(is_selected_now));
            return;
        };

        if is_selected_now == previous_selected {
            return;
        }
        last_selected.set_value(Some(is_selected_now));

        let Some((scale, ring)) = springs.get_value() else {
            return;
        };

        let target_scale = if is_selected_now {
            motion.selected_scale
        } else {
            1.0
        };

        let target_ring = if is_selected_now {
            motion.selected_ring_opacity
        } else {
            0.0
        };

        if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
            scale.clear_on_rest();
            ring.clear_on_rest();
            scale.set_target(target_scale);
            ring.set_target(target_ring);
            return;
        }

        scale.clear_on_rest();
        ring.clear_on_rest();
        scale.set_target(target_scale);
        ring.set_target(target_ring);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _selected: leptos::prelude::Signal<bool>,
    motion: SwatchMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(SwatchMotion {
            enabled: true,
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            selected_scale: f64::NAN,
            selected_ring_opacity: f64::INFINITY,
        });

        let default = SwatchMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.selected_scale, default.selected_scale);
        assert_eq!(motion.selected_ring_opacity, 1.0);
    }

    #[test]
    fn default_motion_reads_theme_tokens() {
        let motion = SwatchMotion::default();
        let tokens = default_swatch_motion_tokens();

        assert_eq!(
            motion.spring,
            ui_motion::spring::SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            }
        );
        assert_eq!(motion.selected_scale, tokens.selected_scale);
        assert_eq!(motion.selected_ring_opacity, tokens.selected_ring_opacity);
    }

    #[test]
    fn disabled_constructor_turns_motion_off() {
        let motion = SwatchMotion::disabled();
        assert!(!motion.enabled);
    }
}
