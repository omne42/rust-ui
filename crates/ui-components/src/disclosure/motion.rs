use ui_motion::spring::SpringConfig;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisclosureMotion {
    pub spring: SpringConfig,
    pub closed_rotation_deg: f64,
    pub open_rotation_deg: f64,
}

impl Default for DisclosureMotion {
    fn default() -> Self {
        Self {
            spring: SpringConfig {
                stiffness: 260.0,
                damping: 18.0,
                mass: 1.0,
                ..Default::default()
            },
            closed_rotation_deg: 0.0,
            open_rotation_deg: 90.0,
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_indicator_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    is_open: leptos::prelude::ReadSignal<bool>,
    motion: DisclosureMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(motion);
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);
    let last_open = StoredValue::new(None::<bool>);

    Effect::new(move |_| {
        let config = motion.get_value().spring;

        let Some(indicator) = node_ref.get() else {
            return;
        };

        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = indicator.unchecked_into();
        let style = element.style();
        let initial = motion.get_value().closed_rotation_deg;

        let animator = ui_motion::spring::SpringAnimator::new(initial, config, move |deg| {
            let _ = style.set_property("--ui-disclosure-indicator-rotation", &format!("{deg}deg"));
        });

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
        });

        spring.set_value(Some(animator));
    });

    Effect::new(move |_| {
        let open = is_open.get();
        if last_open.get_value() == Some(open) {
            return;
        }
        last_open.set_value(Some(open));

        let motion = motion.get_value();
        let target = if open {
            motion.open_rotation_deg
        } else {
            motion.closed_rotation_deg
        };

        let Some(spring) = spring.get_value() else {
            return;
        };
        spring.set_target(target);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_indicator_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Span>,
    _is_open: leptos::prelude::ReadSignal<bool>,
    _motion: DisclosureMotion,
) {
}
