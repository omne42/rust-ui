use leptos::prelude::*;
use ui::{Theme, UiRoot};
use ui_layout::{Card, CardVariant, Flex, FlexDirection, FlexGap, Heading, HeadingLevel};

const DEFAULT_ROUTE: &str = "card";

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

    let layout_css = {
        let mut out = String::new();
        ui_layout::push_layout_css(&mut out);
        out
    };

    view! {
        <UiRoot theme=Signal::derive(Theme::light) safe_area=true inject_components_css=true>
            <style>{layout_css}</style>
            <main class="shell">
                {move || match route.get().as_str() {
                    "card" => view! {
                        <section class="panel">
                            <h1>"Card"</h1>
                            <p class="muted">"layout pack"</p>
                            <Flex direction=FlexDirection::Column gap=FlexGap::Sm>
                                <Card variant=CardVariant::Default class_name="docs-prose".to_string()>
                                    <Heading level=HeadingLevel::H3>"Default"</Heading>
                                    <p>"Default card variant."</p>
                                </Card>
                                <Card variant=CardVariant::Muted class_name="docs-prose".to_string()>
                                    <Heading level=HeadingLevel::H3>"Muted"</Heading>
                                    <p>"Muted card variant."</p>
                                </Card>
                                <Card variant=CardVariant::Outline class_name="docs-prose".to_string()>
                                    <Heading level=HeadingLevel::H3>"Outline"</Heading>
                                    <p>"Outline card variant."</p>
                                </Card>
                            </Flex>
                        </section>
                    }
                        .into_any(),
                    _ => view! {
                        <section class="panel">
                            <h1>"Not found in layout pack"</h1>
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
