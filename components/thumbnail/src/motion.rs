#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThumbnailMotion {
    pub enabled: bool,
    pub spring: ui_motion::spring::SpringConfig,
    pub active_scale: f64,
    pub active_ring_opacity: f64,
}

impl Default for ThumbnailMotion {
    fn default() -> Self {
        Self {
            enabled: true,
            spring: ui_motion::spring::SpringConfig {
                stiffness: 260.0,
                damping: 19.0,
                mass: 1.0,
                ..Default::default()
            },
            active_scale: 1.03,
            active_ring_opacity: 1.0,
        }
    }
}

impl ThumbnailMotion {
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

pub fn sanitize_motion(motion: ThumbnailMotion) -> ThumbnailMotion {
    let default = ThumbnailMotion::default();

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

    ThumbnailMotion {
        enabled: motion.enabled,
        spring: ui_motion::spring::SpringConfig {
            stiffness,
            damping,
            mass,
            precision,
        },
        active_scale: sanitize_number(motion.active_scale, default.active_scale).clamp(1.0, 1.2),
        active_ring_opacity: sanitize_number(
            motion.active_ring_opacity,
            default.active_ring_opacity,
        )
        .clamp(0.0, 1.0),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    active: leptos::prelude::Signal<bool>,
    motion: ThumbnailMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = sanitize_motion(motion);

    let last_active = StoredValue::new(None::<bool>);
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

        let is_active = active.get_untracked();
        let initial_scale = if is_active { motion.active_scale } else { 1.0 };
        let initial_ring = if is_active {
            motion.active_ring_opacity
        } else {
            0.0
        };

        drop(style.set_property("--ui-thumbnail-scale", &format!("{initial_scale}")));
        drop(style.set_property("--ui-thumbnail-ring-opacity", &format!("{initial_ring}")));
        let style_scale = style.clone();
        let scale =
            ui_motion::spring::SpringAnimator::new(initial_scale, motion.spring, move |next| {
                let next = next.clamp(0.9, 1.2);
                drop(style_scale.set_property("--ui-thumbnail-scale", &format!("{next}")));
            });

        let style_ring = style.clone();
        let ring =
            ui_motion::spring::SpringAnimator::new(initial_ring, motion.spring, move |next| {
                let next = next.clamp(0.0, 1.0);
                drop(style_ring.set_property("--ui-thumbnail-ring-opacity", &format!("{next}")));
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
        let is_active = active.get();
        let Some(previous_active) = last_active.get_value() else {
            last_active.set_value(Some(is_active));
            return;
        };

        if is_active == previous_active {
            return;
        }
        last_active.set_value(Some(is_active));

        let Some((scale, ring)) = springs.get_value() else {
            return;
        };

        let target_scale = if is_active { motion.active_scale } else { 1.0 };
        let target_ring = if is_active {
            motion.active_ring_opacity
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
    _active: leptos::prelude::Signal<bool>,
    _motion: ThumbnailMotion,
) {
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
