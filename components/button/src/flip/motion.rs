use super::FlipDirection;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlipButtonMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for FlipButtonMotion {
    fn default() -> Self {
        Self {
            spring: crate::button::motion::ButtonMotion::default().spring,
        }
    }
}

pub fn sanitize_motion(motion: FlipButtonMotion) -> FlipButtonMotion {
    let base = crate::button::motion::ButtonMotion::default();
    let spring = crate::button::motion::sanitize_spring_with_fallback(motion.spring, base.spring);
    let sanitized = crate::button::motion::sanitize_motion(crate::button::motion::ButtonMotion {
        spring,
        ..base
    });

    FlipButtonMotion {
        spring: sanitized.spring,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_active: leptos::prelude::Signal<bool>,
    from: FlipDirection,
    motion: FlipButtonMotion,
) {
    use crate::observability::set_css_property_observed;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
    let last_active = StoredValue::new(None::<bool>);
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let config = motion.get_value().spring;

        let Some(container) = node_ref.get() else {
            return;
        };

        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = container.unchecked_into();
        let style = element.style();

        // Apply initial direction-dependent offsets.
        let (front_offset, back_offset, axis, rotate_axis) = match from {
            FlipDirection::Top => ("50%", "-50%", "y", "x"),
            FlipDirection::Bottom => ("-50%", "50%", "y", "x"),
            FlipDirection::Left => ("50%", "-50%", "x", "y"),
            FlipDirection::Right => ("-50%", "50%", "x", "y"),
        };
        set_css_property_observed(
            &style,
            "--ui-flip-front-offset",
            front_offset,
            "button.flip.motion.front_offset",
        );
        set_css_property_observed(
            &style,
            "--ui-flip-back-offset",
            back_offset,
            "button.flip.motion.back_offset",
        );
        set_css_property_observed(&style, "--ui-flip-axis", axis, "button.flip.motion.axis");
        set_css_property_observed(
            &style,
            "--ui-flip-rotate-axis",
            rotate_axis,
            "button.flip.motion.rotate_axis",
        );

        let initial = if is_active.get_untracked() { 1.0 } else { 0.0 };
        let style_for_anim = style.clone();
        let animator = ui_motion::spring::SpringAnimator::new(initial, config, move |progress| {
            let progress = progress.clamp(0.0, 1.0);
            set_css_property_observed(
                &style_for_anim,
                "--ui-flip-progress",
                &format!("{progress}"),
                "button.flip.motion.progress",
            );
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
        let active = is_active.get();
        let Some(prev) = last_active.get_value() else {
            last_active.set_value(Some(active));
            return;
        };
        if prev == active {
            return;
        }
        last_active.set_value(Some(active));

        let Some(spring) = spring.get_value() else {
            return;
        };
        spring.set_target(if active { 1.0 } else { 0.0 });
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _is_active: leptos::prelude::Signal<bool>,
    _from: FlipDirection,
    motion: FlipButtonMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../../test/flip/motion.rs"]
mod tests;
