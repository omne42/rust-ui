#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EmptyStateMotion {
    pub animate_in: bool,
}

pub fn sanitize_motion(motion: EmptyStateMotion) -> EmptyStateMotion {
    EmptyStateMotion {
        animate_in: motion.animate_in,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion<E>(node_ref: leptos::prelude::NodeRef<E>, motion: EmptyStateMotion)
where
    E: leptos::tachys::html::element::ElementType,
    E::Output: leptos::wasm_bindgen::JsCast + Clone + 'static,
{
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = sanitize_motion(motion);
    if !motion.animate_in || ui_motion::web::prefers_reduced_motion() {
        return;
    }

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
        drop(style.set_property("--ui-empty-state-enter", "0"));
        let style_for_apply = style.clone();
        let animator = ui_motion::spring::SpringAnimator::new(
            0.0,
            ui_motion::presets::spring_soft(),
            move |v| {
                let v = v.clamp(0.0, 1.0);
                drop(style_for_apply.set_property("--ui-empty-state-enter", &format!("{v}")));
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
pub fn attach_motion<E>(_node_ref: leptos::prelude::NodeRef<E>, motion: EmptyStateMotion)
where
    E: leptos::tachys::html::element::ElementType,
    E::Output: 'static,
{
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_is_static() {
        let motion = EmptyStateMotion::default();
        assert!(!motion.animate_in);
    }

    #[test]
    fn sanitize_motion_keeps_explicit_animation_flag() {
        let motion = sanitize_motion(EmptyStateMotion { animate_in: true });
        assert!(motion.animate_in);
    }
}
