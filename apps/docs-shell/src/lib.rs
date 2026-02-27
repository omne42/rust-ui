use leptos::prelude::*;

const DEFAULT_ROUTE: &str = "docs/welcome";

#[derive(Clone, Copy)]
struct DocPage {
    title: &'static str,
    route: &'static str,
    summary: &'static str,
}

#[derive(Clone, Copy)]
struct ComponentDoc {
    name: &'static str,
    slug: &'static str,
    group: &'static str,
    pack: &'static str,
}

const DOCS: &[DocPage] = &[
    DocPage {
        title: "Welcome",
        route: "docs/welcome",
        summary: "docs-shell base package. Components are loaded from extension packs.",
    },
    DocPage {
        title: "Start",
        route: "docs/start",
        summary: "This shell keeps first-load small and only loads component packs on demand.",
    },
    DocPage {
        title: "Rules",
        route: "docs/rules",
        summary: "dev path stays full in docs-app; non-dev can use shell + pack lazy loading.",
    },
];

const COMMON_COMPONENTS: &[ComponentDoc] = &[
    ComponentDoc {
        name: "Button",
        slug: "button",
        group: "Actions",
        pack: "actions",
    },
    ComponentDoc {
        name: "Input",
        slug: "input",
        group: "Forms",
        pack: "forms",
    },
    ComponentDoc {
        name: "Checkbox",
        slug: "checkbox",
        group: "Forms",
        pack: "forms",
    },
    ComponentDoc {
        name: "Select",
        slug: "select",
        group: "Collections",
        pack: "collections",
    },
    ComponentDoc {
        name: "Tabs",
        slug: "tabs",
        group: "Collections",
        pack: "collections",
    },
    ComponentDoc {
        name: "Card",
        slug: "card",
        group: "Layout",
        pack: "layout",
    },
];

fn route_path(route: &str) -> &str {
    route.split_once('?').map(|(path, _)| path).unwrap_or(route)
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
    drop(window.location().set_hash(&format!("#/{route}")));
}

#[cfg(not(target_arch = "wasm32"))]
fn set_hash_route(_route: &str) {}

fn use_hash_route() -> (ReadSignal<String>, Callback<String>) {
    let (route, set_route) = signal(read_hash_route());

    #[cfg(target_arch = "wasm32")]
    {
        use gloo_events::EventListener;

        let Some(window) = web_sys::window() else {
            return (
                route,
                Callback::new(move |next: String| {
                    let next = normalize_route(&next);
                    set_route.set(next.clone());
                    set_hash_route(&next);
                }),
            );
        };

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

fn doc_page(route: &str) -> Option<DocPage> {
    DOCS.iter().copied().find(|doc| doc.route == route)
}

fn component_doc(slug: &str) -> Option<ComponentDoc> {
    COMMON_COMPONENTS
        .iter()
        .copied()
        .find(|doc| doc.slug == slug)
}

#[component]
fn DocsIndex(route: ReadSignal<String>, navigate: Callback<String>) -> impl IntoView {
    view! {
        <section class="panel">
            <h2>"Docs"</h2>
            <ul>
                <For
                    each=move || DOCS.to_vec()
                    key=|doc| doc.route
                    children=move |doc| {
                        let href = format!("#/{}", doc.route);
                        view! {
                            <li>
                                <a
                                    href=href
                                    data-active=move || (route_path(&route.get()) == doc.route).then_some("true")
                                    on:click=move |ev| {
                                        ev.prevent_default();
                                        navigate.run(doc.route.into());
                                    }
                                >
                                    {doc.title}
                                </a>
                            </li>
                        }
                    }
                />
            </ul>
        </section>
    }
}

#[component]
fn ComponentsIndex(route: ReadSignal<String>, navigate: Callback<String>) -> impl IntoView {
    view! {
        <section class="panel">
            <h2>"Common Components"</h2>
            <p class="muted">"These are served from extension packs (N packs)."</p>
            <ul>
                <For
                    each=move || COMMON_COMPONENTS.to_vec()
                    key=|doc| doc.slug
                    children=move |doc| {
                        let href = format!("#/components/{}", doc.slug);
                        view! {
                            <li>
                                <a
                                    href=href
                                    data-active=move || {
                                        (route_path(&route.get()) == format!("components/{}", doc.slug)).then_some("true")
                                    }
                                    on:click=move |ev| {
                                        ev.prevent_default();
                                        navigate.run(format!("components/{}", doc.slug));
                                    }
                                >
                                    {doc.name} " (" {doc.group} ")"
                                </a>
                            </li>
                        }
                    }
                />
            </ul>
        </section>
    }
}

#[component]
fn MainView(route: ReadSignal<String>) -> impl IntoView {
    move || {
        let path = route_path(&route.get()).to_string();

        if let Some(doc) = doc_page(&path) {
            return view! {
                <article class="panel">
                    <h1>{doc.title}</h1>
                    <p>{doc.summary}</p>
                    <p class="muted">
                        "Route: " <code>{doc.route}</code>
                    </p>
                </article>
            }
            .into_any();
        }

        if path == "components" {
            return view! {
                <article class="panel">
                    <h1>"Components"</h1>
                    <p>
                        "Open a component route, shell will lazy load the mapped pack automatically."
                    </p>
                </article>
            }
            .into_any();
        }

        if let Some(slug) = path.strip_prefix("components/") {
            if let Some(doc) = component_doc(slug) {
                let src = format!("packs/{}/index.html#/{slug}", doc.pack);
                return view! {
                    <article class="panel panel-host">
                        <h1>{doc.name}</h1>
                        <p class="muted">
                            "Pack: " <code>{doc.pack}</code> " · slug: " <code>{doc.slug}</code>
                        </p>
                        <iframe
                            class="pack-frame"
                            title=doc.name
                            src=src
                        />
                    </article>
                }
                .into_any();
            }

            return view! {
                <article class="panel">
                    <h1>"Component Not In Current Packs"</h1>
                    <p>
                        "Slug " <code>{slug.to_string()}</code> " is not mapped in the current pack set yet."
                    </p>
                </article>
            }
            .into_any();
        }

        view! {
            <article class="panel">
                <h1>"Not found"</h1>
                <p>
                    "Unknown route: " <code>{path}</code>
                </p>
            </article>
        }
        .into_any()
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (route, navigate) = use_hash_route();

    view! {
        <div class="shell">
            <header class="topbar">
                <div class="title">"rust-ui docs shell"</div>
                <div class="muted">"base package + route-driven extension packs"</div>
            </header>

            <main class="layout">
                <aside class="sidebar">
                    <DocsIndex route=route navigate=navigate />
                    <ComponentsIndex route=route navigate=navigate />
                </aside>

                <section class="content">
                    <MainView route=route />
                </section>
            </main>
        </div>
    }
}

pub fn mount() {
    leptos::mount::mount_to_body(|| view! { <App /> });
}
