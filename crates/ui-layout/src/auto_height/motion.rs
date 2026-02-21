#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutoHeightMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub animate_height: bool,
}

impl Default for AutoHeightMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_soft(),
            animate_height: true,
        }
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = AutoHeightMotion::default().spring;

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

pub fn sanitize_motion(motion: AutoHeightMotion) -> AutoHeightMotion {
    AutoHeightMotion {
        spring: sanitize_spring(motion.spring),
        animate_height: motion.animate_height,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    container_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    content_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: AutoHeightMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::{JsCast, closure::Closure};
    use std::rc::Rc;

    if !motion.animate_height {
        return;
    }

    let motion = StoredValue::new(sanitize_motion(motion));
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);
    let last_height_px = StoredValue::new(None::<f64>);

    let resize_observer = StoredValue::new_local(None::<leptos::web_sys::ResizeObserver>);
    let resize_closure = StoredValue::new_local(
        None::<Closure<dyn FnMut(js_sys::Array, leptos::web_sys::ResizeObserver)>>,
    );

    let update_height = Rc::new(move || {
        let Some(content) = content_ref.get_untracked() else {
            return;
        };
        let height_px = (content.scroll_height() as f64).max(0.0);

        if last_height_px
            .get_value()
            .is_some_and(|prev| (prev - height_px).abs() < 0.5)
        {
            return;
        }
        last_height_px.set_value(Some(height_px));

        let Some(animator) = spring.get_value() else {
            return;
        };
        animator.set_target(height_px);
    });

    Effect::new({
        let update_height = Rc::clone(&update_height);
        move |_| {
            let config = motion.get_value().spring;
            let Some(container) = container_ref.get() else {
                return;
            };
            let Some(content) = content_ref.get() else {
                return;
            };

            if spring.get_value().is_none() {
                let initial_height_px = (content.scroll_height() as f64).max(0.0);
                last_height_px.set_value(Some(initial_height_px));

                let element: leptos::web_sys::HtmlElement = container.unchecked_into();
                let style = element.style();

                ui_observability::set_css_property_observed_auto!(
                    &(style),
                    "--ui-auto-height-height",
                    &format!("{initial_height_px}px")
                );

                let style_for_apply = style.clone();
                let animator =
                    ui_motion::spring::SpringAnimator::new(initial_height_px, config, move |v| {
                        let v = v.max(0.0);
                        ui_observability::set_css_property_observed_auto!(
                            &(style_for_apply),
                            "--ui-auto-height-height",
                            &format!("{v}px")
                        );
                    });

                let spring_for_cleanup = spring;
                on_cleanup(move || {
                    if let Some(animator) = spring_for_cleanup.get_value() {
                        animator.stop();
                    }
                    spring_for_cleanup.set_value(None);
                });

                spring.set_value(Some(animator));
            }

            update_height.as_ref()();

            if resize_observer.get_value().is_some() {
                return;
            }

            let element: leptos::web_sys::Element = content.unchecked_into();

            let update_for_observer = Rc::clone(&update_height);
            let closure = Closure::wrap(Box::new(
                move |_: js_sys::Array, _: leptos::web_sys::ResizeObserver| {
                    update_for_observer.as_ref()();
                },
            )
                as Box<dyn FnMut(js_sys::Array, leptos::web_sys::ResizeObserver)>);

            if let Ok(observer) =
                leptos::web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref())
            {
                observer.observe(&element);
                resize_observer.set_value(Some(observer));
                resize_closure.set_value(Some(closure));
            }

            let resize_observer_for_cleanup = resize_observer;
            let resize_closure_for_cleanup = resize_closure;
            on_cleanup(move || {
                if let Some(observer) = resize_observer_for_cleanup.get_value() {
                    observer.disconnect();
                }
                resize_observer_for_cleanup.set_value(None);
                resize_closure_for_cleanup.set_value(None);
            });
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _container_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _content_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: AutoHeightMotion,
) {
    sanitize_motion(motion);
}

#[cfg(test)]
#[path = "test/motion.rs"]
mod tests;
