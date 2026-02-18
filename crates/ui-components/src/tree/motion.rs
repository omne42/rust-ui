#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TreeMotion {
    pub enabled: bool,
    pub spring: ui_motion::spring::SpringConfig,
    pub collapsed_scale: f64,
    pub collapsed_opacity: f64,
}

impl Default for TreeMotion {
    fn default() -> Self {
        Self {
            enabled: true,
            spring: ui_motion::presets::spring_soft(),
            collapsed_scale: 0.992,
            collapsed_opacity: 0.94,
        }
    }
}

impl TreeMotion {
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

pub fn sanitize_motion(motion: TreeMotion) -> TreeMotion {
    let default = TreeMotion::default();
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

    TreeMotion {
        enabled: motion.enabled,
        spring: ui_motion::spring::SpringConfig {
            stiffness,
            damping,
            mass,
            precision,
        },
        collapsed_scale: sanitize_number(motion.collapsed_scale, default.collapsed_scale)
            .clamp(0.85, 1.0),
        collapsed_opacity: sanitize_number(motion.collapsed_opacity, default.collapsed_opacity)
            .clamp(0.4, 1.0),
    }
}

pub fn resolve_motion_css_vars(has_any_expanded: bool, motion: TreeMotion) -> (f64, f64) {
    let motion = sanitize_motion(motion);
    if has_any_expanded {
        (1.0, 1.0)
    } else {
        (motion.collapsed_scale, motion.collapsed_opacity)
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    expanded: leptos::prelude::Signal<bool>,
    motion: TreeMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = sanitize_motion(motion);
    let last_expanded = StoredValue::new(None::<bool>);
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

        let expanded_now = expanded.get_untracked();
        let initial_scale = if expanded_now {
            1.0
        } else {
            motion.collapsed_scale
        };
        let initial_opacity = if expanded_now {
            1.0
        } else {
            motion.collapsed_opacity
        };

        let _ = style.set_property("--ui-tree-motion-scale", &format!("{initial_scale}"));
        let _ = style.set_property("--ui-tree-motion-opacity", &format!("{initial_opacity}"));

        let scale_style = style.clone();
        let scale =
            ui_motion::spring::SpringAnimator::new(initial_scale, motion.spring, move |next| {
                let _ = scale_style.set_property(
                    "--ui-tree-motion-scale",
                    &format!("{}", next.clamp(0.0, 2.0)),
                );
            });

        let opacity_style = style.clone();
        let opacity =
            ui_motion::spring::SpringAnimator::new(initial_opacity, motion.spring, move |next| {
                let _ = opacity_style.set_property(
                    "--ui-tree-motion-opacity",
                    &format!("{}", next.clamp(0.0, 1.0)),
                );
            });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((scale, opacity)) = springs_for_cleanup.get_value() {
                scale.stop();
                opacity.stop();
            }
        });

        springs.set_value(Some((scale, opacity)));
    });

    Effect::new(move |_| {
        let now_expanded = expanded.get();
        let Some(previous_expanded) = last_expanded.get_value() else {
            last_expanded.set_value(Some(now_expanded));
            return;
        };
        if now_expanded == previous_expanded {
            return;
        }
        last_expanded.set_value(Some(now_expanded));

        let Some((scale, opacity)) = springs.get_value() else {
            return;
        };

        let target_scale = if now_expanded {
            1.0
        } else {
            motion.collapsed_scale
        };
        let target_opacity = if now_expanded {
            1.0
        } else {
            motion.collapsed_opacity
        };

        if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
            scale.clear_on_rest();
            opacity.clear_on_rest();
            scale.set_target(target_scale);
            opacity.set_target(target_opacity);
            return;
        }

        scale.clear_on_rest();
        opacity.clear_on_rest();
        scale.set_target(target_scale);
        opacity.set_target(target_opacity);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _expanded: leptos::prelude::Signal<bool>,
    _motion: TreeMotion,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(TreeMotion {
            enabled: true,
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            collapsed_scale: f64::NAN,
            collapsed_opacity: f64::NEG_INFINITY,
        });

        let default = TreeMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.collapsed_scale, default.collapsed_scale);
        assert_eq!(motion.collapsed_opacity, default.collapsed_opacity);
    }

    #[test]
    fn disabled_constructor_turns_motion_off() {
        assert!(!TreeMotion::disabled().enabled);
    }

    #[test]
    fn resolve_motion_css_vars_matches_expanded_and_collapsed_states() {
        let motion = sanitize_motion(TreeMotion {
            enabled: true,
            spring: ui_motion::presets::spring_soft(),
            collapsed_scale: 0.97,
            collapsed_opacity: 0.88,
        });

        let expanded = resolve_motion_css_vars(true, motion);
        let collapsed = resolve_motion_css_vars(false, motion);

        assert_eq!(expanded, (1.0, 1.0));
        assert_eq!(collapsed, (0.97, 0.88));
    }
}
