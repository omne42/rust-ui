use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct ModalOptions {
    pub is_enabled: Signal<bool>,
}

impl ModalOptions {
    pub fn enabled() -> Self {
        Self {
            is_enabled: Signal::derive(|| true),
        }
    }

    pub fn from_signal(is_enabled: Signal<bool>) -> Self {
        Self { is_enabled }
    }
}

impl Default for ModalOptions {
    fn default() -> Self {
        Self::enabled()
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
        setup_modal(options.is_enabled);
    }

    #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
    {
        let _unused_options = options;
    }
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn setup_modal(is_enabled: Signal<bool>) {
    use std::cell::RefCell;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
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
        ui_observability::set_css_property_observed_auto!(&(style), "overflow", "hidden");
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
            drop(child.set_attribute("aria-hidden", "true"));
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
                drop(style.remove_property("overflow"));
            } else {
                ui_observability::set_css_property_observed_auto!(
                    &(style),
                    "overflow",
                    &previous_overflow
                );
            }
        }

        for (el, original) in state.aria_hidden.drain(..) {
            if let Some(original) = original {
                drop(el.set_attribute("aria-hidden", &original));
            } else {
                drop(el.remove_attribute("aria-hidden"));
            }
        }
    }

    fn acquire() {
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            if state.count == 0 {
                apply(&mut state);
            }
            state.count += 1;
        });
    }

    fn release() {
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
    }

    let is_active = Arc::new(AtomicBool::new(false));

    Effect::new({
        let is_active = Arc::clone(&is_active);
        move |_| {
            let enabled = is_enabled.get();
            let active = is_active.load(Ordering::Relaxed);

            if enabled && !active {
                acquire();
                is_active.store(true, Ordering::Relaxed);
            } else if !enabled && active {
                release();
                is_active.store(false, Ordering::Relaxed);
            }
        }
    });

    on_cleanup(move || {
        if is_active.swap(false, Ordering::Relaxed) {
            release();
        }
    });
}
