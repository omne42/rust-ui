#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DateInputGroupMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub enter_scale: f64,
}

impl Default for DateInputGroupMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 230.0,
                damping: 20.0,
                mass: 1.0,
                ..Default::default()
            },
            enter_scale: 0.99,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = DateInputGroupMotion::default().spring;

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

pub fn sanitize_motion(motion: DateInputGroupMotion) -> DateInputGroupMotion {
    let default = DateInputGroupMotion::default();

    DateInputGroupMotion {
        spring: sanitize_spring(motion.spring),
        enter_scale: sanitize_number(motion.enter_scale, default.enter_scale).clamp(0.5, 1.5),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: DateInputGroupMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
    let initialized = StoredValue::new(false);
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        if initialized.get_value() {
            return;
        }

        let Some(node) = node_ref.get() else {
            return;
        };
        initialized.set_value(true);

        let motion = motion.get_value();
        let element: leptos::web_sys::HtmlElement = node.unchecked_into();
        let style = element.style();

        let _ = style.set_property(
            "--ui-date-input-group-scale",
            &format!("{}", motion.enter_scale),
        );

        let animator = ui_motion::spring::SpringAnimator::new(
            motion.enter_scale,
            motion.spring,
            move |scale| {
                let scale = scale.clamp(0.0, 10.0);
                let _ = style.set_property("--ui-date-input-group-scale", &format!("{scale}"));
            },
        );
        animator.set_target(1.0);

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
        });

        spring.set_value(Some(animator));
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: DateInputGroupMotion,
) {
    let _ = sanitize_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_has_reasonable_params() {
        let motion = DateInputGroupMotion::default();
        assert!(motion.spring.stiffness > 0.0);
        assert!(motion.spring.damping > 0.0);
        assert!(motion.spring.mass > 0.0);
        assert!(motion.enter_scale > 0.0);
        assert!(motion.enter_scale <= 1.0);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(DateInputGroupMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            enter_scale: f64::NAN,
        });

        let default = DateInputGroupMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.enter_scale, default.enter_scale);
    }

    #[test]
    fn sanitize_motion_clamps_scale_values() {
        let motion = sanitize_motion(DateInputGroupMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 300.0,
                damping: 22.0,
                mass: 1.0,
                precision: 0.002,
            },
            enter_scale: 8.0,
        });
        assert_eq!(motion.enter_scale, 1.5);
    }
}
