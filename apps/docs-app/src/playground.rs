use crate::perf_probe::UiPerfProbe;
use leptos::children::ViewFn;
use leptos::prelude::*;
use std::collections::HashSet;
use ui_components::{Button, ButtonSize, ButtonVariant, CodeBlock, OnPress, push_components_css};
use ui_layout::{
    Card, Flex, FlexAlign, FlexDirection, FlexGap, FlexJustify, Heading, HeadingLevel,
};

const DEFAULT_PLAYGROUND_IMPORTS: &str = "use leptos::prelude::*;\nuse ui_components::*;";

fn normalize_code_snippet(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    (!trimmed.is_empty()).then(|| trimmed.into())
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
        trimmed.into()
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
            .update(|titles| titles.push(title.trim().into()));
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
        <section class="playground" id=anchor_id attr:data-slot="playground">
            <style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>

            <Flex
                justify=FlexJustify::SpaceBetween
                align=FlexAlign::Start
                gap=FlexGap::Sm
                class_name="playground__header".to_string()
            >
                <Flex direction=FlexDirection::Column gap=FlexGap::Xs class_name="playground__title".to_string()>
                    <Heading level=HeadingLevel::H2>{title}</Heading>
                    {description.map(|description| view! { <div class="docs-subtitle">{description}</div> })}
                </Flex>

                <Flex align=FlexAlign::Center gap=FlexGap::Xs class_name="playground__actions".to_string()>
                    {on_link.map(|on_link| {
                        view! {
                            <Button
                                aria_label="Link to section".to_string()
                                variant=ButtonVariant::Ghost
                                size=ButtonSize::IconSm
                                is_icon_only=true
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
                            </Button>
                        }
                    })}

                    {move || {
                        has_controls.then(|| {
                            view! {
                                <Button
                                    variant=ButtonVariant::Secondary
                                    size=ButtonSize::Sm
                                    attr:data-slot="playground-toggle-settings"
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
                                    attr:data-slot="playground-toggle-code"
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
                        attr:data-slot="playground-toggle-test"
                        on_press=on_toggle_test
                    >
                        {move || if show_test_panel.get() { "Hide test" } else { "Show test" }}
                    </Button>
                </Flex>
            </Flex>

            <Flex direction=FlexDirection::Column gap=FlexGap::Sm class_name="playground__body".to_string()>
                <UiPerfProbe name=format!("Playground::{title}")>
                    <div data-playground-scope=scope_id.clone()>
                        <Card class_name="playground__preview".to_string()>
                            <div class="playground__preview-stage">{children()}</div>
                        </Card>
                    </div>
                </UiPerfProbe>
            </Flex>

            {move || {
                has_controls.then(|| {
                    view! {
                        <Show when=move || show_settings_panel.get()>
                            <div attr:data-slot="playground-controls">
                                <Card class_name="playground__panel playground__controls".to_string()>
                                    {controls
                                        .get_value()
                                        .map(|panel| panel.run())
                                        .unwrap_or_else(|| ().into_any())}
                                </Card>
                            </div>
                        </Show>
                    }
                })
            }}

            {move || {
                has_code.get().then(|| {
                    view! {
                        <Show when=move || show_code_panel.get()>
                            <div attr:data-slot="playground-code">
                                <Card class_name="playground__panel playground__code".to_string()>
                                    <CodeBlock code=resolved_code.get() />
                                </Card>
                            </div>
                        </Show>
                    }
                })
            }}

            <Show when=move || show_test_panel.get()>
                <div attr:data-slot="playground-test">
                    <Card class_name="playground__panel playground__test".to_string()>
                        <Flex direction=FlexDirection::Column gap=FlexGap::Xs class_name="docs-stack docs-stack--tight".to_string()>
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
                            <Flex
                                justify=FlexJustify::End
                                align=FlexAlign::Center
                                gap=FlexGap::Sm
                                class_name="docs-row docs-row--end".to_string()
                            >
                                <Button
                                    variant=ButtonVariant::Secondary
                                    size=ButtonSize::Sm
                                    on_press=on_reset_test_css
                                >
                                    "Restore original CSS"
                                </Button>
                            </Flex>
                        </Flex>
                    </Card>
                </div>
            </Show>
        </section>
    }
}

#[cfg(test)]
#[path = "test/playground.rs"]
mod tests;
