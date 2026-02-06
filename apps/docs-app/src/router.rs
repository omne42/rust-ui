use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct DocsRouter {
    pub route: ReadSignal<String>,
    pub navigate: Callback<String>,
}

pub fn provide_docs_router(route: ReadSignal<String>, navigate: Callback<String>) -> DocsRouter {
    let router = DocsRouter { route, navigate };
    provide_context(router);
    router
}

pub fn use_docs_router() -> Option<DocsRouter> {
    use_context::<DocsRouter>()
}
