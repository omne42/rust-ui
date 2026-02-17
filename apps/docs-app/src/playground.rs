use leptos::children::ViewFn;
use leptos::prelude::*;
use std::collections::HashSet;
use ui_components::{
    Button, ButtonSize, ButtonVariant, CodeBlock, IconButton, OnPress, push_components_css,
};
use ui_headless::UiPerfProbe;

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

fn sanitize_scope_key(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut prev_dash = false;

    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
            continue;
        }

        if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }

    let trimmed = out.trim_matches('-');
    if trimmed.is_empty() {
        "playground-scope".to_string()
    } else {
        trimmed.to_string()
    }
}

fn compose_scoped_css(scope_selector: &str, raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if trimmed.contains('{') {
        return trimmed.replace(":scope", scope_selector);
    }

    format!("{scope_selector} {{\n{trimmed}\n}}")
}

fn compose_original_css_source() -> String {
    let mut out = String::new();
    out.push_str("/* apps/docs-app/app.css */\n");
    out.push_str(include_str!("../app.css"));
    out.push_str("\n\n/* ui-components aggregated css */\n");
    push_components_css(&mut out);
    out
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
    #[prop(optional, default = true)] show_code: bool,
    #[prop(optional, into)] controls: Option<ViewFn>,
    #[prop(optional, into)] test_css_source: Option<Signal<String>>,
    #[prop(optional, into)] test_source_path: Option<String>,
    #[prop(optional, into)] test_config_signal: Option<Signal<String>>,
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
    let code_imports = StoredValue::new(match code_imports {
        Some(imports) => imports,
        None => DEFAULT_PLAYGROUND_IMPORTS.to_string(),
    });
    let resolved_code = Signal::derive(move || {
        if let Some(dynamic_code) = code_signal {
            return compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value());
        }

        static_code
            .get_value()
            .map(|snippet| compose_copy_ready_code(&snippet, &code_imports.get_value()))
            .unwrap_or_default()
    });

    let has_code = Signal::derive(move || show_code);
    let controls = StoredValue::new(controls);
    let has_controls = controls.get_value().is_some();
    let section_class = "docs-card playground";
    let scope_id = anchor_id
        .clone()
        .map(|id| format!("playground-scope-{id}"))
        .unwrap_or_else(|| format!("playground-scope-{}", sanitize_scope_key(title)));
    let scope_selector = format!("[data-playground-scope=\"{scope_id}\"]");
    let scope_selector = StoredValue::new(scope_selector);
    let fallback_test_css = StoredValue::new(compose_original_css_source());
    let test_source_path = StoredValue::new(test_source_path);
    let default_test_css = Signal::derive(move || {
        test_css_source
            .map(|signal| signal.get())
            .unwrap_or_else(|| fallback_test_css.get_value())
    });
    let (show_settings_panel, set_show_settings_panel) = signal(false);
    let (show_code_panel, set_show_code_panel) = signal(false);
    let (show_test_panel, set_show_test_panel) = signal(false);
    let (test_css, set_test_css) = signal(default_test_css.get_untracked());

    let on_toggle_settings: OnPress =
        Callback::new(move |_| set_show_settings_panel.update(|v| *v = !*v));
    let on_toggle_code: OnPress = Callback::new(move |_| set_show_code_panel.update(|v| *v = !*v));
    let on_toggle_test: OnPress = Callback::new(move |_| set_show_test_panel.update(|v| *v = !*v));
    let on_reset_test_css: OnPress =
        Callback::new(move |_| set_test_css.set(default_test_css.get_untracked()));

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
            <style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>

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
                        has_controls.then(|| {
                            view! {
                                <Button
                                    variant=ButtonVariant::Secondary
                                    size=ButtonSize::Sm
                                    on_press=on_toggle_settings
                                >
                                    {move || if show_settings_panel.get() { "Hide settings" } else { "Show settings" }}
                                </Button>
                            }
                        })
                    }}

                    {move || {
                        has_code.get().then(|| {
                            view! {
                                <Button
                                    variant=ButtonVariant::Secondary
                                    size=ButtonSize::Sm
                                    on_press=on_toggle_code
                                >
                                    {move || if show_code_panel.get() { "Hide code" } else { "Show code" }}
                                </Button>
                            }
                        })
                    }}

                    <Button
                        variant=ButtonVariant::Secondary
                        size=ButtonSize::Sm
                        on_press=on_toggle_test
                    >
                        {move || if show_test_panel.get() { "Hide test" } else { "Show test" }}
                    </Button>
                </div>
            </div>

            <div class="playground__body">
                <UiPerfProbe name=format!("Playground::{title}")>
                    <div class="playground__preview" data-playground-scope=scope_id.clone()>
                        <div class="playground__preview-stage">{children()}</div>
                    </div>
                </UiPerfProbe>
            </div>

            {move || {
                has_controls.then(|| {
                    view! {
                        <Show when=move || show_settings_panel.get()>
                            <aside class="playground__panel playground__controls" data-slot="playground-controls">
                                {controls
                                    .get_value()
                                    .expect("checked controls")
                                    .run()}
                            </aside>
                        </Show>
                    }
                })
            }}

            {move || {
                has_code.get().then(|| {
                    view! {
                        <Show when=move || show_code_panel.get()>
                            <div class="playground__panel playground__code" data-slot="playground-code">
                                <CodeBlock code=resolved_code.get() />
                            </div>
                        </Show>
                    }
                })
            }}

            <Show when=move || show_test_panel.get()>
                <section class="playground__panel playground__test" data-slot="playground-test">
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Scoped CSS"</div>
                        {test_source_path
                            .get_value()
                            .map(|path| {
                                view! { <div class="ui-muted">{format!("Source: {path}")}</div> }
                            })}
                        <textarea
                            class="playground__test-editor"
                            prop:value=move || test_css.get()
                            on:input=move |ev| set_test_css.set(event_target_value(&ev))
                            placeholder="/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */"
                        ></textarea>
                        <div class="ui-muted">
                            "Original CSS is loaded. Use :scope to target this playground only."
                        </div>
                        {test_config_signal.map(|signal| {
                            view! {
                                <>
                                    <div class="docs-search__label">"Actual config"</div>
                                    <CodeBlock code=signal.get() />
                                </>
                            }
                        })}
                        <div class="docs-row docs-row--end">
                            <Button
                                variant=ButtonVariant::Secondary
                                size=ButtonSize::Sm
                                on_press=on_reset_test_css
                            >
                                "Restore original CSS"
                            </Button>
                        </div>
                    </div>
                </section>
            </Show>
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

    #[test]
    fn compose_copy_ready_code_skips_imports_when_none_requested() {
        let code = compose_copy_ready_code("<Accordion />", "");
        assert_eq!(code, "<Accordion />");
    }

    #[test]
    fn compose_scoped_css_wraps_plain_declarations() {
        let css = compose_scoped_css("[data-playground-scope=\"x\"]", "--ui-radius-md: 12px;");
        assert_eq!(
            css,
            "[data-playground-scope=\"x\"] {\n--ui-radius-md: 12px;\n}"
        );
    }

    #[test]
    fn compose_scoped_css_replaces_scope_token_for_rule_blocks() {
        let css = compose_scoped_css(
            "[data-playground-scope=\"x\"]",
            ":scope .ui-button { border-radius: 18px; }",
        );
        assert_eq!(
            css,
            "[data-playground-scope=\"x\"] .ui-button { border-radius: 18px; }"
        );
    }

    #[test]
    fn compose_original_css_source_contains_base_and_components_sections() {
        let css = compose_original_css_source();
        assert!(css.contains("apps/docs-app/app.css"));
        assert!(css.contains("ui-components aggregated css"));
        assert!(css.contains(".playground__preview"));
    }
}
