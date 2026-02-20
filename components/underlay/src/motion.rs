#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnderlayMotion {
    pub enabled: bool,
}

impl UnderlayMotion {
    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

impl Default for UnderlayMotion {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    motion: UnderlayMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(motion);

    Effect::new(move |_| {
        std::hint::black_box(is_open.get());
        let Some(node) = node_ref.get() else {
            return;
        };

        let element: leptos::web_sys::HtmlElement = node.unchecked_into();
        let style = element.style();
        let disable_motion =
            !motion.get_value().enabled || ui_motion::web::prefers_reduced_motion();

        if disable_motion {
            drop(style.set_property("--ui-underlay-runtime-duration", "1ms"));
            drop(style.set_property("--ui-underlay-runtime-visibility-duration", "1ms"));
            return;
        }

        drop(style.remove_property("--ui-underlay-runtime-duration"));
        drop(style.remove_property("--ui-underlay-runtime-visibility-duration"));
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _is_open: leptos::prelude::Signal<bool>,
    _motion: UnderlayMotion,
) {
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
