use crate::snippet::logic::use_snippet_logic;
use leptos::prelude::*;

#[component]
pub fn Snippet(
    #[prop(into)] text: String,
    #[prop(optional, into)] label: Option<String>,
    copyable: bool,
    #[prop(optional, into)] copied_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let base_class = "ui-snippet".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let label = label.filter(|value| !value.trim().is_empty());
    let copied_label = copied_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Copied".to_string());
    let copied_label = StoredValue::new(copied_label);

    let is_multiline = text.contains('\n');

    let logic = use_snippet_logic(text.clone());

    view! {
        <div
            class=class
            data-slot="snippet"
            data-multiline=is_multiline.then_some("true")
        >
            {label.map(|label| {
                view! {
                    <span class="ui-snippet__label" data-slot="snippet-label">
                        {label}
                    </span>
                }
            })}

            <pre class="ui-snippet__pre" data-slot="snippet-pre">
                {text}
            </pre>

            {copyable.then(|| {
                view! {
                    <button
                        type="button"
                        class="ui-snippet__copy-button"
                        data-slot="snippet-copy-button"
                        aria-label="Copy to clipboard"
                        data-copied=move || logic.copied.get().then_some("true")
                        on:click=move |_| logic.copy.run(())
                    >
                        {move || if logic.copied.get() {
                            copied_label.get_value()
                        } else {
                            "Copy".to_string()
                        }}
                    </button>

                    <span
                        class="ui-snippet__a11y-status"
                        data-slot="snippet-copied-status"
                        aria-live="polite"
                        aria-atomic="true"
                    >
                        {move || if logic.copied.get() {
                            copied_label.get_value()
                        } else {
                            String::new()
                        }}
                    </span>
                }
            })}
        </div>
    }
}
