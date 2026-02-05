use leptos::prelude::*;

pub const DEFAULT_ROUTE: &str = "docs/welcome";

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

/// Returns a `(route, navigate)` pair.
///
/// - `route`: the current logical route, without `#/`.
/// - `navigate`: sets the route and updates the URL hash.
pub fn use_hash_route() -> (ReadSignal<String>, Callback<&'static str>) {
    let (route, set_route) = signal(read_hash_route());

    #[cfg(target_arch = "wasm32")]
    {
        use gloo_events::EventListener;

        let Some(window) = web_sys::window() else {
            return (
                route,
                Callback::new(move |next| set_route.set(normalize_route(next))),
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

    let navigate = Callback::new(move |next: &'static str| {
        let next = normalize_route(next);
        set_route.set(next.clone());
        set_hash_route(&next);
    });

    (route, navigate)
}
