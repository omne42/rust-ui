#[derive(Clone, Copy, Debug)]
pub struct ModalOptions {
    pub is_enabled: bool,
}

impl ModalOptions {
    pub fn enabled() -> Self {
        Self { is_enabled: true }
    }
}

impl Default for ModalOptions {
    fn default() -> Self {
        Self { is_enabled: true }
    }
}

/// Applies minimal modal affordances:
/// - Scroll lock (`body { overflow: hidden }`)
/// - `aria-hidden="true"` on `document.body` children that are not overlay portals
///
/// Overlay portals are identified by having a descendant with `[data-ui-overlay-portal]`.
pub fn use_modal(options: ModalOptions) {
    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    {
        if !options.is_enabled {
            return;
        }
        setup_modal();
    }

    #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
    {
        let _ = options;
    }
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn setup_modal() {
    use leptos::prelude::*;
    use std::cell::RefCell;
    use wasm_bindgen::JsCast;

    const PORTAL_SELECTOR: &str = "[data-ui-overlay-portal]";

    #[derive(Default)]
    struct ModalGlobalState {
        count: usize,
        body_overflow: Option<String>,
        aria_hidden: Vec<(web_sys::Element, Option<String>)>,
    }

    thread_local! {
        static STATE: RefCell<ModalGlobalState> = RefCell::new(ModalGlobalState::default());
    }

    fn apply(state: &mut ModalGlobalState) {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };
        let Some(body) = document.body() else {
            return;
        };

        let style = body.style();
        let previous_overflow = style.get_property_value("overflow").ok();
        state.body_overflow = previous_overflow;
        let _ = style.set_property("overflow", "hidden");

        state.aria_hidden.clear();

        let children = body.children();
        for i in 0..children.length() {
            let Some(child) = children.item(i) else {
                continue;
            };

            // Keep any element that hosts an overlay portal.
            let has_portal = child
                .dyn_ref::<web_sys::Element>()
                .and_then(|el| el.query_selector(PORTAL_SELECTOR).ok().flatten())
                .is_some()
                || child.has_attribute("data-ui-overlay-portal");

            if has_portal {
                continue;
            }

            let original = child.get_attribute("aria-hidden");
            state
                .aria_hidden
                .push((child.clone().unchecked_into(), original));
            let _ = child.set_attribute("aria-hidden", "true");
        }
    }

    fn restore(state: &mut ModalGlobalState) {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };
        let Some(body) = document.body() else {
            return;
        };

        if let Some(previous_overflow) = state.body_overflow.take() {
            let style = body.style();
            if previous_overflow.is_empty() {
                let _ = style.remove_property("overflow");
            } else {
                let _ = style.set_property("overflow", &previous_overflow);
            }
        }

        for (el, original) in state.aria_hidden.drain(..) {
            if let Some(original) = original {
                let _ = el.set_attribute("aria-hidden", &original);
            } else {
                let _ = el.remove_attribute("aria-hidden");
            }
        }
    }

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.count == 0 {
            apply(&mut state);
        }
        state.count += 1;
    });

    on_cleanup(move || {
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            if state.count == 0 {
                return;
            }
            state.count -= 1;
            if state.count == 0 {
                restore(&mut state);
            }
        });
    });
}
