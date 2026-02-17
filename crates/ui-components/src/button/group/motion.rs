#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonGroupMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub enter_scale: f64,
}

impl Default for ButtonGroupMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 240.0,
                damping: 20.0,
                mass: 1.0,
                ..Default::default()
            },
            enter_scale: 0.985,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default_spring = ButtonGroupMotion::default().spring;
    crate::button::motion::sanitize_spring_with_fallback(value, default_spring)
}

pub fn sanitize_motion(motion: ButtonGroupMotion) -> ButtonGroupMotion {
    let default = ButtonGroupMotion::default();

    ButtonGroupMotion {
        spring: sanitize_spring(motion.spring),
        enter_scale: sanitize_number(motion.enter_scale, default.enter_scale).clamp(0.5, 1.5),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: ButtonGroupMotion,
) {
    use crate::observability::set_css_property_observed;
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

        set_css_property_observed(
            &style,
            "--ui-button-group-scale",
            &format!("{}", motion.enter_scale),
            "button.group.motion.initial_scale",
        );

        let animator = ui_motion::spring::SpringAnimator::new(
            motion.enter_scale,
            motion.spring,
            move |scale| {
                let scale = scale.clamp(0.0, 10.0);
                set_css_property_observed(
                    &style,
                    "--ui-button-group-scale",
                    &format!("{scale}"),
                    "button.group.motion.scale",
                );
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
    motion: ButtonGroupMotion,
) {
    sanitize_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_matches_button_group_spring_contract() {
        let motion = ButtonGroupMotion::default();
        assert!(motion.spring.stiffness > 0.0);
        assert!(motion.spring.damping > 0.0);
        assert!(motion.spring.mass > 0.0);
        assert!(motion.enter_scale > 0.0);
        assert!(motion.enter_scale <= 1.0);
    }

    #[test]
    fn supports_custom_motion_contract_values() {
        let motion = sanitize_motion(ButtonGroupMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 300.0,
                damping: 22.0,
                mass: 1.2,
                precision: 0.002,
            },
            enter_scale: 1.08,
        });

        assert_eq!(motion.spring.stiffness, 300.0);
        assert_eq!(motion.spring.damping, 22.0);
        assert_eq!(motion.spring.mass, 1.2);
        assert_eq!(motion.spring.precision, 0.002);
        assert_eq!(motion.enter_scale, 1.08);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(ButtonGroupMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            enter_scale: f64::NAN,
        });

        let default = ButtonGroupMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.enter_scale, default.enter_scale);
    }

    #[test]
    fn sanitize_motion_clamps_scale_values() {
        let motion = sanitize_motion(ButtonGroupMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 260.0,
                damping: 18.0,
                mass: 1.1,
                precision: 0.001,
            },
            enter_scale: 9.0,
        });
        assert_eq!(motion.enter_scale, 1.5);
    }
}
