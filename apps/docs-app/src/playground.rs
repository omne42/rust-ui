use leptos::children::ViewFn;
use leptos::prelude::*;
use ui_components::{Button, ButtonSize, ButtonVariant, CodeBlock, IconButton, OnPress};

#[derive(Clone, Copy)]
pub struct PlaygroundRegistry {
    titles: RwSignal<Vec<String>>,
}

pub fn provide_playground_registry() -> PlaygroundRegistry {
    let registry = PlaygroundRegistry {
        titles: RwSignal::new(Vec::new()),
    };
    provide_context(registry);
    registry
}

pub fn use_playground_registry() -> Option<PlaygroundRegistry> {
    use_context::<PlaygroundRegistry>()
}

impl PlaygroundRegistry {
    pub fn titles(self) -> ReadSignal<Vec<String>> {
        self.titles.read_only()
    }

    pub fn clear(self) {
        self.titles.set(Vec::new());
    }

    pub fn register(self, title: &str) {
        self.titles
            .update(|titles| titles.push(title.trim().to_string()));
    }
}

#[component]
pub fn Playground(
    title: &'static str,
    #[prop(optional)] description: &'static str,
    #[prop(optional)] code: &'static str,
    #[prop(optional, into)] controls: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let anchor_id = crate::toc::use_docs_toc()
        .map(|toc| toc.register(title, 2))
        .unwrap_or_default();
    let anchor_id = (!anchor_id.trim().is_empty()).then_some(anchor_id);

    if let Some(registry) = use_playground_registry() {
        registry.register(title);
    }

    let description = (!description.trim().is_empty()).then_some(description);
    let code = (!code.trim().is_empty()).then_some(code);
    let controls = StoredValue::new(controls);
    let has_controls = controls.get_value().is_some();
    let section_class = if has_controls {
        "docs-card playground playground--with-controls"
    } else {
        "docs-card playground"
    };
    let (show_code, set_show_code) = signal(code.is_some());

    let on_toggle_code: OnPress = Callback::new(move |_| set_show_code.update(|v| *v = !*v));

    let router = crate::router::use_docs_router();
    let on_link = router.and_then(|router| {
        anchor_id.clone().map(|anchor| {
            Callback::new(move |_| {
                let next = crate::route::route_with_section(&router.route.get_untracked(), &anchor);
                router.navigate.run(next);
            })
        })
    });

    view! {
        <section class=section_class id=anchor_id>
            <div class="playground__header">
                <div class="playground__title">
                    <h2>{title}</h2>
                    {description.map(|description| view! { <div class="docs-subtitle">{description}</div> })}
                </div>

                <div class="playground__actions">
                    {on_link.map(|on_link| {
                        view! {
                            <IconButton
                                aria_label="Link to section".to_string()
                                variant=ButtonVariant::Ghost
                                size=ButtonSize::IconSm
                                on_press=on_link
                            >
                                <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                    <path
                                        d="M8.2 11.8a3 3 0 0 1 0-4.2l1.2-1.2a3 3 0 0 1 4.2 4.2l-.6.6"
                                        stroke="currentColor"
                                        stroke_width="1.5"
                                        stroke_linecap="round"
                                        stroke_linejoin="round"
                                    />
                                    <path
                                        d="M11.8 8.2a3 3 0 0 1 0 4.2l-1.2 1.2a3 3 0 0 1-4.2-4.2l.6-.6"
                                        stroke="currentColor"
                                        stroke_width="1.5"
                                        stroke_linecap="round"
                                        stroke_linejoin="round"
                                    />
                                </svg>
                            </IconButton>
                        }
                    })}

                    {code.map(|_| {
                        view! {
                            <Button
                                variant=ButtonVariant::Secondary
                                size=ButtonSize::Sm
                                on_press=on_toggle_code
                            >
                                {move || if show_code.get() { "Hide code" } else { "Show code" }}
                            </Button>
                        }
                    })}
                </div>
            </div>

            <div class="playground__body">
                <div class="playground__preview">{children()}</div>
                {controls.get_value().map(|controls| {
                    view! {
                        <aside class="playground__controls" data-slot="playground-controls">
                            {controls.run()}
                        </aside>
                    }
                })}
            </div>
            {code.map(|code| {
                view! {
                    <Show when=move || show_code.get()>
                        <CodeBlock code=code.to_string() />
                    </Show>
                }
            })}
        </section>
    }
}
