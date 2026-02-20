use ui_theme::default_button_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnippetMotion {
    pub enabled: bool,
    pub spring: ui_motion::spring::SpringConfig,
    pub copied_scale: f64,
}

impl Default for SnippetMotion {
    fn default() -> Self {
        let tokens = default_button_motion_tokens();
        Self {
            enabled: true,
            spring: ui_motion::spring::SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            },
            copied_scale: tokens.hover_scale,
        }
    }
}

impl SnippetMotion {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }
}

pub fn sanitize_motion(motion: SnippetMotion) -> SnippetMotion {
    let default = SnippetMotion::default();
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

    let copied_scale = if motion.copied_scale.is_finite() {
        motion.copied_scale.clamp(1.0, 1.18)
    } else {
        default.copied_scale
    };

    SnippetMotion {
        enabled: motion.enabled,
        spring: ui_motion::spring::SpringConfig {
            stiffness,
            damping,
            mass,
            precision,
        },
        copied_scale,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    copied: leptos::prelude::Signal<bool>,
    motion: SnippetMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = sanitize_motion(motion);
    let animator = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let Some(node) = node_ref.get() else {
            return;
        };

        if animator.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = node.unchecked_into();
        let style = element.style();

        let initial = if copied.get_untracked() {
            motion.copied_scale
        } else {
            1.0
        };

        drop(style.set_property("--ui-snippet-scale", &format!("{initial}")));
        let style_for_animator = style.clone();
        let spring = ui_motion::spring::SpringAnimator::new(initial, motion.spring, move |next| {
            let next = next.clamp(0.94, 1.22);
            drop(style_for_animator.set_property("--ui-snippet-scale", &format!("{next}")));
        });

        animator.set_value(Some(spring));
    });

    Effect::new(move |_| {
        let target = if copied.get() {
            motion.copied_scale
        } else {
            1.0
        };

        let Some(spring) = animator.get_value() else {
            return;
        };

        if !motion.enabled || ui_motion::web::prefers_reduced_motion() {
            spring.clear_on_rest();
            spring.set_target(target);
            return;
        }

        spring.clear_on_rest();
        spring.set_target(target);
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
    _copied: leptos::prelude::Signal<bool>,
    motion: SnippetMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
