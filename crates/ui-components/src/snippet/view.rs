use crate::snippet::logic::{self, SnippetStateInput, use_snippet_logic};
use leptos::prelude::*;

#[component]
pub fn Snippet(
    #[prop(into)] text: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, default = true)] copyable: bool,
    #[prop(optional, into)] copied_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let label = logic::normalize_optional_text(label);
    let copied_label = logic::normalize_optional_text(copied_label);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(SnippetStateInput {
        is_multiline: text.contains('\n'),
        has_text: !text.trim().is_empty(),
        has_label: label.is_some(),
        copyable,
        has_custom_copied_label: copied_label.is_some(),
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);

    let copied_label = copied_label.unwrap_or_else(|| "Copied".to_string());
    let copied_label = StoredValue::new(copied_label);

    let logic = use_snippet_logic(text.clone());

    view! {
        <div
            class=class
            data-slot="snippet"
            data-state=state.state_attr
            data-copy=state.copy_state_attr
            data-multiline=state.is_multiline.then_some("true")
            data-empty=state.is_empty.then_some("true")
            data-label=state.has_label.then_some("true")
            data-copyable=state.copyable.then_some("true")
            data-copy-actionable=state.copy_is_actionable.then_some("true")
            data-copied-label=state.copied_label_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
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

            {state.copyable.then(|| {
                view! {
                    <button
                        type="button"
                        class="ui-snippet__copy-button"
                        data-slot="snippet-copy-button"
                        aria-label="Copy to clipboard"
                        disabled=!state.copy_is_actionable
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
