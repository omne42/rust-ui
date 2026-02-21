use leptos::{html, prelude::*};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RestorePolicy {
    Selector(String),
    NearestFocusableSibling,
    FallbackTo(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocusTrapFrame {
    pub trap_id: u64,
    pub scope_id: String,
    pub restore_policy: RestorePolicy,
}

#[derive(Clone)]
pub struct FocusTrapOptions {
    pub container: NodeRef<html::Div>,
    pub is_enabled: bool,
    pub should_restore_focus: bool,
    pub scope_id: String,
    pub restore_policy: Option<RestorePolicy>,
    pub fallback_selector: Option<String>,
}

impl FocusTrapOptions {
    pub fn enabled(container: NodeRef<html::Div>) -> Self {
        Self {
            container,
            is_enabled: true,
            should_restore_focus: true,
            scope_id: "overlay".to_string(),
            restore_policy: None,
            fallback_selector: Some(FOCUSABLE_SELECTOR.to_string()),
        }
    }

    pub fn with_scope_id(mut self, scope_id: impl Into<String>) -> Self {
        self.scope_id = scope_id.into();
        self
    }

    pub fn with_restore_policy(mut self, restore_policy: RestorePolicy) -> Self {
        self.restore_policy = Some(restore_policy);
        self
    }

    pub fn with_fallback_selector(mut self, fallback_selector: impl Into<String>) -> Self {
        self.fallback_selector = Some(fallback_selector.into());
        self
    }
}

#[derive(Clone)]
pub struct FocusTrapHandlers {
    pub on_key_down: Callback<(String, bool), bool>,
}

pub fn use_focus_trap(options: FocusTrapOptions) -> FocusTrapHandlers {
    setup_focus_trap(options)
}

const FOCUSABLE_SELECTOR: &str = r#"a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex=\"-1\"]), [contenteditable=\"true\"]"#;

#[cfg(all(feature = "web", target_arch = "wasm32"))]
thread_local! {
    static FOCUS_MANAGER_STACK: std::cell::RefCell<Vec<FocusTrapFrame>> = std::cell::RefCell::new(Vec::new());
    static FOCUS_MANAGER_NEXT_ID: std::cell::Cell<u64> = std::cell::Cell::new(1);
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn setup_focus_trap(options: FocusTrapOptions) -> FocusTrapHandlers {
    use send_wrapper::SendWrapper;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::JsCast;

    let previous_focus = SendWrapper::new(Rc::new(RefCell::new(None::<web_sys::HtmlElement>)));
    let trap_id = SendWrapper::new(Rc::new(RefCell::new(None::<u64>)));

    if options.is_enabled {
        let previous_focus = previous_focus.clone();
        let trap_id = trap_id.clone();
        let container_ref = options.container.clone();
        let should_restore_focus = options.should_restore_focus;
        let scope_id = options.scope_id.clone();
        let restore_policy = options.restore_policy.clone();
        let fallback_selector = options.fallback_selector.clone();

        container_ref.on_load(move |container: web_sys::HtmlDivElement| {
            let Some(window) = web_sys::window() else {
                return;
            };
            let Some(document) = window.document() else {
                return;
            };

            let previous = if should_restore_focus {
                document
                    .active_element()
                    .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
            } else {
                None
            };

            *previous_focus.borrow_mut() = previous.clone();

            let restore_policy = derive_restore_policy(
                previous.as_ref(),
                restore_policy.clone(),
                fallback_selector.as_deref(),
            );

            let id = focus_manager_push_trap(FocusTrapFrame {
                trap_id: 0,
                scope_id: scope_id.clone(),
                restore_policy,
            });
            *trap_id.borrow_mut() = Some(id);

            focus_first_in_container(&document, &container);
        });
    }

    if options.is_enabled {
        let previous_focus = previous_focus.clone();
        let trap_id = trap_id.clone();
        let should_restore_focus = options.should_restore_focus;
        let fallback_selector = options.fallback_selector.clone();

        on_cleanup(move || {
            let popped_frame = trap_id.borrow_mut().take().and_then(focus_manager_pop_trap);

            if !should_restore_focus {
                return;
            }

            let Some(window) = web_sys::window() else {
                return;
            };
            let Some(document) = window.document() else {
                return;
            };

            let restored = restore_focus_chain(
                &document,
                previous_focus.borrow_mut().take(),
                popped_frame.as_ref().map(|frame| &frame.restore_policy),
                fallback_selector.as_deref(),
            );

            if !restored {
                if let Some(body) = document.body() {
                    ui_observability::observe_js_result!(body.focus());
                }
            }
        });
    }

    let container = options.container.clone();
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

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn focus_manager_push_trap(mut frame: FocusTrapFrame) -> u64 {
    let trap_id = FOCUS_MANAGER_NEXT_ID.with(|next| {
        let current = next.get();
        next.set(current + 1);
        current
    });
    frame.trap_id = trap_id;
    FOCUS_MANAGER_STACK.with(|stack| {
        stack.borrow_mut().push(frame);
    });
    trap_id
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn focus_manager_pop_trap(trap_id: u64) -> Option<FocusTrapFrame> {
    FOCUS_MANAGER_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        let idx = stack.iter().rposition(|frame| frame.trap_id == trap_id)?;
        Some(stack.remove(idx))
    })
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn focus_manager_peek_trap() -> Option<FocusTrapFrame> {
    FOCUS_MANAGER_STACK.with(|stack| stack.borrow().last().cloned())
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn derive_restore_policy(
    previous_focus: Option<&web_sys::HtmlElement>,
    explicit_policy: Option<RestorePolicy>,
    fallback_selector: Option<&str>,
) -> RestorePolicy {
    if let Some(policy) = explicit_policy {
        return policy;
    }

    if let Some(previous_focus) = previous_focus {
        if let Some(selector) = selector_for_element(previous_focus) {
            return RestorePolicy::Selector(selector);
        }
    }

    if let Some(fallback_selector) = fallback_selector {
        return RestorePolicy::FallbackTo(fallback_selector.to_string());
    }

    RestorePolicy::NearestFocusableSibling
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn selector_for_element(el: &web_sys::HtmlElement) -> Option<String> {
    let id = el.get_attribute("id").unwrap_or_default();
    if !id.trim().is_empty() {
        return Some(format!("#{id}"));
    }

    let slot = el.get_attribute("data-slot").unwrap_or_default();
    if !slot.trim().is_empty() {
        return Some(format!(r#"[data-slot=\"{slot}\"]"#));
    }

    None
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn restore_focus_chain(
    document: &web_sys::Document,
    previous_focus: Option<web_sys::HtmlElement>,
    popped_restore_policy: Option<&RestorePolicy>,
    fallback_selector: Option<&str>,
) -> bool {
    if let Some(previous_focus) = previous_focus {
        if previous_focus.focus().is_ok() {
            return true;
        }
    }

    if let Some(policy) = popped_restore_policy {
        if restore_focus_by_policy(document, policy) {
            return true;
        }
    }

    if let Some(frame) = focus_manager_peek_trap() {
        if restore_focus_by_policy(document, &frame.restore_policy) {
            return true;
        }
    }

    if let Some(selector) = fallback_selector {
        if let Some(target) = resolve_selector_target(document, selector) {
            if target.focus().is_ok() {
                return true;
            }
        }
    }

    false
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn restore_focus_by_policy(document: &web_sys::Document, policy: &RestorePolicy) -> bool {
    let target = match policy {
        RestorePolicy::Selector(selector) | RestorePolicy::FallbackTo(selector) => {
            resolve_selector_target(document, selector)
        }
        RestorePolicy::NearestFocusableSibling => {
            resolve_selector_target(document, FOCUSABLE_SELECTOR)
        }
    };

    if let Some(target) = target {
        return target.focus().is_ok();
    }

    false
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn resolve_selector_target(
    document: &web_sys::Document,
    selector: &str,
) -> Option<web_sys::HtmlElement> {
    use wasm_bindgen::JsCast;

    document
        .query_selector(selector)
        .ok()
        .flatten()
        .and_then(|node| node.dyn_into::<web_sys::HtmlElement>().ok())
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
        ui_observability::observe_js_result!(first.focus());
        return;
    }

    // Fallback: focus the container itself (requires the component to set tabindex).
    ui_observability::observe_js_result!(container.focus());
    // Ensure the browser has a chance to update activeElement.
    drop(document.active_element());
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
        ui_observability::observe_js_result!(container.focus());
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
            ui_observability::observe_js_result!(last.focus());
            return true;
        }
    } else if active_index.is_none() || active_index == Some(last_index) {
        let first = focusable[0].clone();
        ui_observability::observe_js_result!(first.focus());
        return true;
    }

    // If focus is outside the overlay, pull it back in.
    if let Some(active) = active.as_ref() {
        if !container.contains(Some(active.unchecked_ref())) {
            if shift {
                let last = focusable[last_index].clone();
                ui_observability::observe_js_result!(last.focus());
            } else {
                let first = focusable[0].clone();
                ui_observability::observe_js_result!(first.focus());
            }
            return true;
        }
    }

    false
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn collect_focusable(container: &web_sys::HtmlDivElement) -> Vec<web_sys::HtmlElement> {
    use wasm_bindgen::JsCast;

    let Ok(list) = container.query_selector_all(FOCUSABLE_SELECTOR) else {
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
