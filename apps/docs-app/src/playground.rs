use leptos::children::ViewFn;
use leptos::prelude::*;
use std::collections::HashSet;
use ui_components::{Button, ButtonSize, ButtonVariant, CodeBlock, IconButton, OnPress};

const DEFAULT_PLAYGROUND_IMPORTS: &str = "use leptos::prelude::*;\nuse ui_components::*;";

fn normalize_code_snippet(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn import_root(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    let path = trimmed.strip_prefix("use ")?.strip_suffix(';')?.trim();
    path.split("::").next().map(str::trim)
}

fn collect_import_roots(raw: &str) -> HashSet<String> {
    raw.lines()
        .filter_map(import_root)
        .map(ToString::to_string)
        .collect()
}

fn missing_import_lines(raw: &str, imports: &str) -> Vec<String> {
    let existing_roots = collect_import_roots(raw);
    let mut seen_missing = HashSet::new();

    imports
        .lines()
        .filter_map(normalize_code_snippet)
        .filter_map(|line| {
            let root = import_root(&line)?;
            if existing_roots.contains(root) || !seen_missing.insert(root.to_string()) {
                return None;
            }

            Some(line)
        })
        .collect()
}

fn compose_copy_ready_code(raw: &str, imports: &str) -> String {
    let Some(raw) = normalize_code_snippet(raw) else {
        return String::new();
    };

    let Some(imports) = normalize_code_snippet(imports) else {
        return raw;
    };

    let missing_imports = missing_import_lines(&raw, &imports);

    if missing_imports.is_empty() {
        raw
    } else {
        format!("{}\n\n{raw}", missing_imports.join("\n"))
    }
}

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
    #[prop(optional, into)] code_signal: Option<Signal<String>>,
    #[prop(optional, into)] code_imports: Option<String>,
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
    let static_code = StoredValue::new(normalize_code_snippet(code));
    let code_imports = StoredValue::new(
        code_imports
            .and_then(|imports| normalize_code_snippet(&imports))
            .unwrap_or_else(|| DEFAULT_PLAYGROUND_IMPORTS.to_string()),
    );
    let resolved_code = Signal::derive(move || {
        if let Some(dynamic_code) = code_signal {
            return compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value());
        }

        static_code
            .get_value()
            .map(|snippet| compose_copy_ready_code(&snippet, &code_imports.get_value()))
            .unwrap_or_default()
    });

    let has_code = Signal::derive(move || !resolved_code.get().trim().is_empty());
    let controls = StoredValue::new(controls);
    let has_controls = controls.get_value().is_some();
    let section_class = if has_controls {
        "docs-card playground playground--with-controls"
    } else {
        "docs-card playground"
    };
    let (show_code, set_show_code) = signal(has_code.get_untracked());

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

                    {move || {
                        has_code.get().then(|| {
                            view! {
                                <Button
                                    variant=ButtonVariant::Secondary
                                    size=ButtonSize::Sm
                                    on_press=on_toggle_code
                                >
                                    {move || if show_code.get() { "Hide code" } else { "Show code" }}
                                </Button>
                            }
                        })
                    }}
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
            {move || {
                has_code.get().then(|| {
                    view! {
                        <Show when=move || show_code.get()>
                            <CodeBlock code=resolved_code.get() />
                        </Show>
                    }
                })
            }}
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_copy_ready_code_prepends_imports_when_missing() {
        let code = compose_copy_ready_code(
            "<Button variant=ButtonVariant::Default>\"Button\"</Button>",
            "use leptos::prelude::*;\nuse ui_components::{Button, ButtonVariant};",
        );

        assert!(code.contains("use ui_components::{Button, ButtonVariant};"));
        assert!(code.contains("<Button variant=ButtonVariant::Default>\"Button\"</Button>"));
    }

    #[test]
    fn compose_copy_ready_code_keeps_existing_imports() {
        let code = compose_copy_ready_code(
            "use ui_components::{Button, ButtonVariant};\n\n<Button variant=ButtonVariant::Default>\"Button\"</Button>",
            "use leptos::prelude::*;\nuse ui_components::*;",
        );

        assert_eq!(
            code,
            "use leptos::prelude::*;\n\nuse ui_components::{Button, ButtonVariant};\n\n<Button variant=ButtonVariant::Default>\"Button\"</Button>"
        );
    }

    #[test]
    fn compose_copy_ready_code_does_not_duplicate_when_roots_exist() {
        let code = compose_copy_ready_code(
            "use leptos::prelude::*;\nuse ui_components::{Button, ButtonVariant};\n\n<Button variant=ButtonVariant::Default>\"Button\"</Button>",
            "use leptos::prelude::*;\nuse ui_components::*;",
        );

        assert_eq!(
            code,
            "use leptos::prelude::*;\nuse ui_components::{Button, ButtonVariant};\n\n<Button variant=ButtonVariant::Default>\"Button\"</Button>"
        );
    }
}
