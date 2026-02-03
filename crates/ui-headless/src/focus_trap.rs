use leptos::{html, prelude::*};

#[derive(Clone, Copy)]
pub struct FocusTrapOptions {
    pub container: NodeRef<html::Div>,
    pub is_enabled: bool,
    pub should_restore_focus: bool,
}

impl FocusTrapOptions {
    pub fn enabled(container: NodeRef<html::Div>) -> Self {
        Self {
            container,
            is_enabled: true,
            should_restore_focus: true,
        }
    }
}

#[derive(Clone)]
pub struct FocusTrapHandlers {
    pub on_key_down: Callback<(String, bool), bool>,
}

pub fn use_focus_trap(options: FocusTrapOptions) -> FocusTrapHandlers {
    setup_focus_trap(options)
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn setup_focus_trap(options: FocusTrapOptions) -> FocusTrapHandlers {
    use send_wrapper::SendWrapper;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::JsCast;

    let previous_focus: SendWrapper<Rc<RefCell<Option<web_sys::HtmlElement>>>> =
        SendWrapper::new(Rc::new(RefCell::new(None)));

    if options.is_enabled {
        let previous_focus = previous_focus.clone();
        options
            .container
            .on_load(move |container: web_sys::HtmlDivElement| {
                let Some(window) = web_sys::window() else {
                    return;
                };
                let Some(document) = window.document() else {
                    return;
                };

                if options.should_restore_focus {
                    *previous_focus.borrow_mut() = document
                        .active_element()
                        .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok());
                }

                focus_first_in_container(&document, &container);
            });
    }

    if options.is_enabled && options.should_restore_focus {
        let previous_focus = previous_focus.clone();
        on_cleanup(move || {
            if let Some(el) = previous_focus.borrow_mut().take() {
                let _ = el.focus();
            }
        });
    }

    let container = options.container;
    let on_key_down = Callback::new(move |(key, shift): (String, bool)| -> bool {
        if !options.is_enabled || key != "Tab" {
            return false;
        }

        let Some(container) = container.get_untracked() else {
            return false;
        };
        let Some(window) = web_sys::window() else {
            return false;
        };
        let Some(document) = window.document() else {
            return false;
        };

        trap_tab_key(&document, &container, shift)
    });

    FocusTrapHandlers { on_key_down }
}

#[cfg(not(all(feature = "web", target_arch = "wasm32")))]
fn setup_focus_trap(_options: FocusTrapOptions) -> FocusTrapHandlers {
    let on_key_down = Callback::new(|_input: (String, bool)| false);
    FocusTrapHandlers { on_key_down }
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn focus_first_in_container(document: &web_sys::Document, container: &web_sys::HtmlDivElement) {
    let focusable = collect_focusable(container);
    if let Some(first) = focusable.first() {
        let _ = first.focus();
        return;
    }

    // Fallback: focus the container itself (requires the component to set tabindex).
    let _ = container.focus();

    // Ensure the browser has a chance to update activeElement.
    let _ = document.active_element();
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn trap_tab_key(
    document: &web_sys::Document,
    container: &web_sys::HtmlDivElement,
    shift: bool,
) -> bool {
    use wasm_bindgen::JsCast;

    let focusable = collect_focusable(container);
    if focusable.is_empty() {
        let _ = container.focus();
        return true;
    }

    let active = document.active_element();
    let active_node: Option<web_sys::Node> = active.as_ref().map(|el| el.clone().into());

    let mut active_index = None;
    if let Some(active_node) = active_node.as_ref() {
        for (i, el) in focusable.iter().enumerate() {
            let node: web_sys::Node = el.clone().into();
            if node.is_same_node(Some(active_node)) {
                active_index = Some(i);
                break;
            }
        }
    }

    let last_index = focusable.len().saturating_sub(1);

    if shift {
        if active_index.is_none() || active_index == Some(0) {
            let last = focusable[last_index].clone();
            let _ = last.focus();
            return true;
        }
    } else if active_index.is_none() || active_index == Some(last_index) {
        let first = focusable[0].clone();
        let _ = first.focus();
        return true;
    }

    // If focus is outside the overlay, pull it back in.
    if let Some(active) = active.as_ref() {
        if !container.contains(Some(active.unchecked_ref())) {
            if shift {
                let last = focusable[last_index].clone();
                let _ = last.focus();
            } else {
                let first = focusable[0].clone();
                let _ = first.focus();
            }
            return true;
        }
    }

    false
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn collect_focusable(container: &web_sys::HtmlDivElement) -> Vec<web_sys::HtmlElement> {
    use wasm_bindgen::JsCast;

    let selector = r#"a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"]), [contenteditable="true"]"#;

    let Ok(list) = container.query_selector_all(selector) else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for i in 0..list.length() {
        let Some(node) = list.item(i) else {
            continue;
        };
        if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
            out.push(el);
        }
    }
    out
}
