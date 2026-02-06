use leptos::prelude::*;

pub const DEFAULT_ROUTE: &str = "docs/welcome";

pub fn route_path(route: &str) -> &str {
    route.split_once('?').map(|(path, _)| path).unwrap_or(route)
}

fn route_query(route: &str) -> Option<&str> {
    route.split_once('?').map(|(_, query)| query)
}

pub fn route_section(route: &str) -> Option<&str> {
    route_query(route).and_then(|query| {
        query
            .split('&')
            .find_map(|pair| pair.strip_prefix("section="))
            .map(str::trim)
            .filter(|value| !value.is_empty())
    })
}

pub fn route_with_section(route: &str, section: &str) -> String {
    let path = route_path(route).trim();
    if path.is_empty() {
        return DEFAULT_ROUTE.to_string();
    }

    let section = section.trim();
    if section.is_empty() {
        return path.to_string();
    }

    format!("{path}?section={section}")
}

fn normalize_route(raw: &str) -> String {
    let raw = raw.trim();
    if raw.is_empty() {
        return DEFAULT_ROUTE.to_string();
    }

    raw.trim_start_matches('#')
        .trim_start_matches('/')
        .trim()
        .to_string()
}

#[cfg(target_arch = "wasm32")]
fn read_hash_route() -> String {
    let Some(window) = web_sys::window() else {
        return DEFAULT_ROUTE.to_string();
    };
    let Ok(hash) = window.location().hash() else {
        return DEFAULT_ROUTE.to_string();
    };
    let hash = hash.strip_prefix('#').unwrap_or(&hash);
    let hash = hash.strip_prefix('/').unwrap_or(hash);
    normalize_route(hash)
}

#[cfg(not(target_arch = "wasm32"))]
fn read_hash_route() -> String {
    DEFAULT_ROUTE.to_string()
}

#[cfg(target_arch = "wasm32")]
fn set_hash_route(route: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let route = normalize_route(route);
    let _ = window
        .location()
        .set_hash(&format!("#/{route}"))
        .map_err(|_| ());
}

#[cfg(not(target_arch = "wasm32"))]
fn set_hash_route(_route: &str) {}

#[cfg(target_arch = "wasm32")]
pub fn scroll_to_id(id: &str) {
    use leptos::wasm_bindgen::{JsCast, closure::Closure};

    fn try_scroll(id: &str) -> bool {
        let Some(window) = web_sys::window() else {
            return false;
        };
        let Some(document) = window.document() else {
            return false;
        };
        let Some(el) = document.get_element_by_id(id) else {
            return false;
        };
        el.scroll_into_view();
        true
    }

    if id.trim().is_empty() {
        return;
    }

    if try_scroll(id) {
        return;
    }

    // The element might not exist yet (route render + WASM mount). Retry once on the next tick.
    let Some(window) = web_sys::window() else {
        return;
    };

    let id = id.to_string();
    let callback = Closure::once_into_js(move || {
        _ = try_scroll(&id);
    });

    let Some(callback) = callback.dyn_ref::<js_sys::Function>() else {
        return;
    };

    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(callback, 0);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn scroll_to_id(_id: &str) {}

/// Returns a `(route, navigate)` pair.
///
/// - `route`: the current logical route, without `#/`.
/// - `navigate`: sets the route and updates the URL hash.
pub fn use_hash_route() -> (ReadSignal<String>, Callback<String>) {
    let (route, set_route) = signal(read_hash_route());

    #[cfg(target_arch = "wasm32")]
    {
        use gloo_events::EventListener;

        let Some(window) = web_sys::window() else {
            return (
                route,
                Callback::new(move |next: String| {
                    set_route.set(normalize_route(&next));
                }),
            );
        };

        // Keep the listener alive for the lifetime of this component.
        let listener = StoredValue::new_local(None::<EventListener>);
        let set_route_for_listener = set_route.clone();
        listener.set_value(Some(EventListener::new(&window, "hashchange", move |_| {
            set_route_for_listener.set(read_hash_route());
        })));

        on_cleanup(move || listener.set_value(None));
    }

    let navigate = Callback::new(move |next: String| {
        let next = normalize_route(&next);
        set_route.set(next.clone());
        set_hash_route(&next);
    });

    (route, navigate)
}
