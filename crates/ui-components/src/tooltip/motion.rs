#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TooltipMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_scale: f64,
}

impl Default for TooltipMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 320.0,
                damping: 26.0,
                mass: 1.0,
                ..Default::default()
            },
            initial_scale: 0.98,
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: TooltipMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(motion);
    let last_state = StoredValue::new(None::<bool>);
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(div) = node_ref.get() else {
            return;
        };
        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = div.unchecked_into();
        let style = element.style();
        let motion = motion.get_value();

        let open_now = is_open.get_untracked();
        // Always initialize in the closed state so mounting while open animates in.
        let initial_progress = 0.0;
        let initial_scale = motion.initial_scale;

        let _ = style.set_property("--ui-tooltip-opacity", &format!("{initial_progress}"));
        let _ = style.set_property("--ui-tooltip-scale", &format!("{initial_scale}"));

        let animator =
            ui_motion::spring::SpringAnimator::new(initial_progress, config, move |progress| {
                let progress = progress.clamp(0.0, 1.0);
                let scale = motion.initial_scale + (1.0 - motion.initial_scale) * progress;
                let _ = style.set_property("--ui-tooltip-opacity", &format!("{progress}"));
                let _ = style.set_property("--ui-tooltip-scale", &format!("{scale}"));
            });

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
        });

        if open_now {
            animator.set_target(1.0);
        }

        spring.set_value(Some(animator));
    });

    Effect::new(move |_| {
        let open = is_open.get();
        let Some(prev) = last_state.get_value() else {
            last_state.set_value(Some(open));
            return;
        };
        if prev == open {
            return;
        }
        last_state.set_value(Some(open));

        let Some(spring) = spring.get_value() else {
            return;
        };

        if open {
            spring.clear_on_rest();
            spring.set_target(1.0);
            return;
        }

        let on_exit_complete = on_exit_complete.clone();
        spring.set_on_rest(move || on_exit_complete.run(()));
        spring.set_target(0.0);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    on_exit_complete: leptos::prelude::Callback<()>,
    _motion: TooltipMotion,
) {
    use leptos::prelude::*;

    Effect::new(move |_| {
        if !is_open.get() {
            on_exit_complete.run(());
        }
    });
}
