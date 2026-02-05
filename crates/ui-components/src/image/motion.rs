use ui_headless::{HoverOptions, use_hover};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageMotion {
    pub zoom_spring: ui_motion::spring::SpringConfig,
    pub zoom_scale: f64,
}

impl Default for ImageMotion {
    fn default() -> Self {
        Self {
            zoom_spring: ui_motion::presets::spring_soft(),
            zoom_scale: 1.03,
        }
    }
}

#[derive(Clone)]
pub struct ImageMotionState {
    pub hover: ui_headless::HoverState,
}

pub fn use_image_motion(is_disabled: bool) -> ImageMotionState {
    let hover = use_hover(HoverOptions { is_disabled });
    ImageMotionState { hover }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_zoom_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_zoomed: bool,
    is_hovered: leptos::prelude::ReadSignal<bool>,
    motion: ImageMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    if !is_zoomed {
        return;
    }

    let motion = StoredValue::new(motion);
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let config = motion.get_value().zoom_spring;
        let Some(div) = node_ref.get() else {
            return;
        };
        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = div.unchecked_into();
        let style = element.style();

        let _ = style.set_property("--ui-image-zoom", "1");

        let zoom_scale = motion.get_value().zoom_scale;
        let style_for_apply = style.clone();
        let animator = ui_motion::spring::SpringAnimator::new(1.0, config, move |v| {
            let v = v.clamp(0.1, 4.0);
            let _ = style_for_apply.set_property("--ui-image-zoom", &format!("{v}"));
        });

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
        });

        if is_hovered.get_untracked() {
            animator.set_target(zoom_scale);
        }

        spring.set_value(Some(animator));
    });

    Effect::new(move |_| {
        let hovered = is_hovered.get();
        if let Some(animator) = spring.get_value() {
            let target = if hovered {
                motion.get_value().zoom_scale
            } else {
                1.0
            };
            animator.set_target(target);
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_zoom_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _is_zoomed: bool,
    _is_hovered: leptos::prelude::ReadSignal<bool>,
    _motion: ImageMotion,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_has_reasonable_params() {
        let motion = ImageMotion::default();
        assert_eq!(motion.zoom_spring, ui_motion::presets::spring_soft());
        assert!(motion.zoom_scale > 1.0);
    }
}
