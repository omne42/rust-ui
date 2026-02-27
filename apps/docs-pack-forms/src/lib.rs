use leptos::prelude::*;
use ui::checkbox::Checkbox;
use ui::text_input::input::Input;
use ui::{Theme, UiRoot};

const DEFAULT_ROUTE: &str = "input";

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
    let (value, set_value) = signal(String::new());
    let (checked, set_checked) = signal(false);

    view! {
        <UiRoot theme=Signal::derive(Theme::light) safe_area=true inject_components_css=true>
            <main class="shell">
                {move || match route.get().as_str() {
                    "input" => view! {
                        <section class="panel">
                            <h1>"Input"</h1>
                            <p class="muted">"forms pack"</p>
                            <Input
                                id="pack-forms-input".to_string()
                                value=value
                                set_value=set_value
                                label="Name".to_string()
                            />
                            <p class="muted">"value: " {move || value.get()}</p>
                        </section>
                    }
                        .into_any(),
                    "checkbox" => view! {
                        <section class="panel">
                            <h1>"Checkbox"</h1>
                            <p class="muted">"forms pack"</p>
                            <Checkbox is_checked=checked on_checked_change=set_checked>
                                "Enable notifications"
                            </Checkbox>
                            <p class="muted">"checked: " {move || checked.get()}</p>
                        </section>
                    }
                        .into_any(),
                    _ => view! {
                        <section class="panel">
                            <h1>"Not found in forms pack"</h1>
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
