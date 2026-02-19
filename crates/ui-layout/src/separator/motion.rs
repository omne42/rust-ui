#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SeparatorMotion {
    pub animate_in: bool,
}

pub fn sanitize_motion(motion: SeparatorMotion) -> SeparatorMotion {
    SeparatorMotion {
        animate_in: motion.animate_in,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion<E>(
    node_ref: leptos::prelude::NodeRef<E>,
    orientation: super::SeparatorOrientation,
    motion: SeparatorMotion,
) where
    E: leptos::tachys::html::element::ElementType,
    E::Output: leptos::wasm_bindgen::JsCast + Clone + 'static,
{
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = sanitize_motion(motion);

    if !motion.animate_in || ui_motion::web::prefers_reduced_motion() {
        return;
    }

    let orientation = StoredValue::new(orientation);
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let Some(node) = node_ref.get() else {
            return;
        };

        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = node.unchecked_into();
        let style = element.style();

        let is_horizontal = matches!(
            orientation.get_value(),
            super::SeparatorOrientation::Horizontal
        );

        let initial_scale_x = if is_horizontal { 0.0 } else { 1.0 };
        let initial_scale_y = if is_horizontal { 1.0 } else { 0.0 };

        drop(style.set_property("--ui-separator-scale-x", &format!("{initial_scale_x}")));
        drop(style.set_property("--ui-separator-scale-y", &format!("{initial_scale_y}")));
        drop(style.set_property("--ui-separator-opacity", "0"));
        let style_for_apply = style.clone();
        let animator = ui_motion::spring::SpringAnimator::new(
            0.0,
            ui_motion::presets::spring_fast(),
            move |v| {
                let v = v.clamp(0.0, 1.0);
                let scale_x = if is_horizontal { v } else { 1.0 };
                let scale_y = if is_horizontal { 1.0 } else { v };

                drop(style_for_apply.set_property("--ui-separator-scale-x", &format!("{scale_x}")));
                drop(style_for_apply.set_property("--ui-separator-scale-y", &format!("{scale_y}")));
                drop(style_for_apply.set_property("--ui-separator-opacity", &format!("{v}")));
            },
        );

        animator.set_target(1.0);

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
            spring_for_cleanup.set_value(None);
        });

        spring.set_value(Some(animator));
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion<E>(
    _node_ref: leptos::prelude::NodeRef<E>,
    _orientation: super::SeparatorOrientation,
    motion: SeparatorMotion,
) where
    E: leptos::tachys::html::element::ElementType,
    E::Output: 'static,
{
    sanitize_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::{SeparatorMotion, sanitize_motion};

    #[test]
    fn default_motion_disables_entry_animation() {
        let motion = SeparatorMotion::default();

        assert!(
            !motion.animate_in,
            "SeparatorMotion defaults should avoid unexpected decorative motion."
        );
    }

    #[test]
    fn sanitize_motion_keeps_explicit_entry_flag() {
        let motion = sanitize_motion(SeparatorMotion { animate_in: true });

        assert!(
            motion.animate_in,
            "SeparatorMotion sanitize contract should preserve explicit animation requests."
        );
    }

    #[test]
    fn motion_contract_supports_explicit_entry_animation() {
        let motion = SeparatorMotion { animate_in: true };

        assert!(
            motion.animate_in,
            "SeparatorMotion should allow explicit entry animation for custom motion presets."
        );
    }
}
