use crate::code_block::{CodeBlockMotion, logic};
use crate::{Button, ButtonSize, ButtonVariant};
use leptos::prelude::*;

fn copy_icon(copied: bool) -> impl IntoView {
    if copied {
        view! {
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <path
                    d="M5 10.5l3 3 7-7"
                    stroke="currentColor"
                    stroke_width="1.8"
                    stroke_linecap="round"
                    stroke_linejoin="round"
                />
            </svg>
        }
        .into_any()
    } else {
        view! {
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                <rect
                    x="7"
                    y="7"
                    width="10"
                    height="10"
                    rx="2"
                    stroke="currentColor"
                    stroke_width="1.5"
                />
                <path
                    d="M5 13V5a2 2 0 0 1 2-2h8"
                    stroke="currentColor"
                    stroke_width="1.5"
                    stroke_linecap="round"
                />
            </svg>
        }
        .into_any()
    }
}

#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] language: Option<String>,
    #[prop(optional, default = true)] copyable: bool,
    #[prop(optional)] motion: CodeBlockMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let _ = motion;

    let label = label.filter(|value| !value.trim().is_empty());
    let language = language.filter(|value| !value.trim().is_empty());

    let view_state =
        logic::resolve_view_state(&code, label.as_deref(), language.as_deref(), copyable);

    let base_class = "ui-code-block".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let code_value = StoredValue::new(code);
    let label = StoredValue::new(label);
    let language = StoredValue::new(language);

    let copy_logic = crate::snippet::logic::use_snippet_logic(code_value.get_value());
    let copied_label = StoredValue::new("Copied".to_string());

    view! {
        <div class=class data-slot="code-block" data-multiline=view_state.is_multiline.then_some("true")>
            <Show when=move || view_state.show_header>
                <div class="ui-code-block__header" data-slot="code-block-header">
                    <div class="ui-code-block__meta" data-slot="code-block-meta">
                        {move || label.get_value().map(|label| view! {
                            <span class="ui-code-block__label" data-slot="code-block-label">{label}</span>
                        })}
                        {move || language.get_value().map(|language| view! {
                            <span class="ui-code-block__language" data-slot="code-block-language">{language}</span>
                        })}
                    </div>

                    <Show when=move || copyable>
                        <Button
                            class_name="ui-code-block__copy-button".to_string()
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::IconSm
                            aria_label="Copy to clipboard".to_string()
                            on_press=copy_logic.copy
                        >
                            {move || copy_icon(copy_logic.copied.get())}
                        </Button>
                    </Show>
                </div>
            </Show>

            <pre class="ui-code-block__pre" data-slot="code-block-pre">
                <code class="ui-code-block__code" data-slot="code-block-code">
                    {move || code_value.get_value()}
                </code>
            </pre>

            <span
                class="ui-code-block__a11y-status"
                data-slot="code-block-status"
                aria-live="polite"
                aria-atomic="true"
            >
                {move || if copy_logic.copied.get() {
                    copied_label.get_value()
                } else {
                    String::new()
                }}
            </span>
        </div>
    }
}
