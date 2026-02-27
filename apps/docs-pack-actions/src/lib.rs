use leptos::prelude::*;
use ui::button::Button;
use ui::{Theme, UiRoot};

const DEFAULT_ROUTE: &str = "button";

#[cfg(target_arch = "wasm32")]
fn read_hash_route() -> String {
    let Some(window) = web_sys::window() else {
        return DEFAULT_ROUTE.into();
    };
    let Ok(hash) = window.location().hash() else {
        return DEFAULT_ROUTE.into();
    };
    let route = hash.strip_prefix('#').unwrap_or(&hash);
    let route = route.strip_prefix('/').unwrap_or(route);
    let route = route.trim();
    if route.is_empty() {
        DEFAULT_ROUTE.into()
    } else {
        route.into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_hash_route() -> String {
    DEFAULT_ROUTE.into()
}

fn use_hash_route() -> ReadSignal<String> {
    let (route, _set_route) = signal(read_hash_route());
    route
}

#[component]
pub fn App() -> impl IntoView {
    let route = use_hash_route();
    let (count, set_count) = signal(0_u32);

    view! {
        <UiRoot theme=Signal::derive(Theme::light) safe_area=true inject_components_css=true>
            <main class="shell">
                {move || match route.get().as_str() {
                    "button" => view! {
                        <section class="panel">
                            <h1>"Button"</h1>
                            <p class="muted">"actions pack"</p>
                            <Button on_press=Callback::new(move |_| set_count.update(|v| *v += 1))>
                                "Clicked "
                                {move || count.get()}
                                " times"
                            </Button>
                        </section>
                    }
                        .into_any(),
                    _ => view! {
                        <section class="panel">
                            <h1>"Not found in actions pack"</h1>
                        </section>
                    }
                        .into_any(),
                }}
            </main>
        </UiRoot>
    }
}

pub fn mount() {
    leptos::mount::mount_to_body(|| view! { <App /> });
}
