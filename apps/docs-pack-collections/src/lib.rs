use leptos::prelude::*;
use ui::select::Select;
use ui::tabs::{Tabs, TabsItem};
use ui::{Theme, UiRoot};

const DEFAULT_ROUTE: &str = "select";

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
    let items = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Durian".to_string(),
    ];
    let (selected, set_selected) = signal(Some(1_usize));

    view! {
        <UiRoot theme=Signal::derive(Theme::light) safe_area=true inject_components_css=true>
            <main class="shell">
                {move || match route.get().as_str() {
                    "select" => view! {
                        <section class="panel">
                            <h1>"Select"</h1>
                            <p class="muted">"collections pack"</p>
                            <Select
                                id_base="pack-collections-select".to_string()
                                items=items.clone()
                                selected_index=selected
                                set_selected_index=set_selected
                            />
                            <p class="muted">
                                "selected_index: "
                                {move || selected.get().map_or_else(|| "None".to_string(), |v| v.to_string())}
                            </p>
                        </section>
                    }
                        .into_any(),
                    "tabs" => view! {
                        <section class="panel">
                            <h1>"Tabs"</h1>
                            <p class="muted">"collections pack"</p>
                            <Tabs id_base="pack-collections-tabs".to_string()>
                                <TabsItem label="Overview">
                                    <p>"Overview panel"</p>
                                </TabsItem>
                                <TabsItem label="Details">
                                    <p>"Details panel"</p>
                                </TabsItem>
                                <TabsItem label="Settings">
                                    <p>"Settings panel"</p>
                                </TabsItem>
                            </Tabs>
                        </section>
                    }
                        .into_any(),
                    _ => view! {
                        <section class="panel">
                            <h1>"Not found in collections pack"</h1>
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
