use crate::snippet::{
    SnippetMotion,
    logic::{self, SnippetLogicOptions, SnippetStateInput},
};
use leptos::html;
use leptos::prelude::*;
use ui_headless::{A11yDirection, CommonStrings, use_ui_i18n};

#[component]
pub fn Snippet(
    #[prop(into)] text: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] is_copyable: Option<bool>,
    #[prop(optional)] copyable: Option<bool>,
    #[prop(optional, into)] copy_label: Option<String>,
    #[prop(optional, into)] copied_label: Option<String>,
    #[prop(optional, into)] copy_aria_label: Option<String>,
    #[prop(optional, into)] copy_error_label: Option<String>,
    #[prop(optional)] is_copied: Option<Signal<bool>>,
    #[prop(optional)] copied: Option<Signal<bool>>,
    #[prop(optional)] default_copied: Option<bool>,
    #[prop(optional)] on_copied_change: Option<Callback<bool>>,
    #[prop(optional)] motion: SnippetMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let label = logic::normalize_optional_text(label);
    let class_name = logic::normalize_optional_text(class_name);
    let lang = logic::normalize_optional_text(lang);

    let copied_label = logic::normalize_optional_text(copied_label);
    let has_custom_copied_label = copied_label.is_some();
    let common_strings = use_ui_i18n().strings::<CommonStrings>();
    let text_contract = logic::resolve_text_contract(
        copy_label,
        copied_label,
        copy_aria_label,
        copy_error_label,
        logic::SnippetTextFallbacks {
            copy_label: Some(common_strings.snippet_copy_label.as_ref().into()),
            copied_label: Some(common_strings.snippet_copied_label.as_ref().into()),
            copy_aria_label: Some(common_strings.snippet_copy_aria_label.as_ref().into()),
            copy_error_label: Some(common_strings.snippet_copy_retry_label.as_ref().into()),
        },
    );

    let copyable_contract = logic::resolve_copyable_contract(is_copyable, copyable);
    let is_copyable = copyable_contract.is_copyable;
    let controlled_copied = logic::resolve_controlled_copied(is_copied, copied);

    let state = logic::resolve_state(SnippetStateInput {
        is_multiline: text.contains('\n'),
        has_text: !text.trim().is_empty(),
        has_label: label.is_some(),
        is_copyable,
        has_custom_copied_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);
    let root_ref = NodeRef::<html::Div>::new();
    let motion = crate::snippet::motion::sanitize_motion(motion);

    let copy_label = StoredValue::new(text_contract.copy_label);
    let copied_label = StoredValue::new(text_contract.copied_label);
    let copy_aria_label = StoredValue::new(text_contract.copy_aria_label);
    let copy_error_label = StoredValue::new(text_contract.copy_error_label);

    let logic = logic::use_snippet_logic_with_options(SnippetLogicOptions {
        text: text.clone(),
        is_copyable,
        is_copied: controlled_copied.value,
        default_copied,
        on_copied_change,
        on_copy_error: None,
        lang,
        dir,
    });
    crate::snippet::motion::attach_motion(root_ref, logic.copied, motion);

    view! {
        <div
            node_ref=root_ref
            class=class
            lang=logic.lang.clone()
            dir=logic.dir
            aria-busy=logic.aria_busy
            data-slot="snippet"
            data-state=state.state_attr
            data-copy=state.copy_state_attr
            data-copyable-source=copyable_contract.source.as_attr()
            data-copied-source=controlled_copied.source.as_attr()
            data-copy-status=move || {
                if logic.is_loading.get() {
                    "loading"
                } else if logic.has_error.get() {
                    "error"
                } else if logic.copied.get() {
                    "copied"
                } else {
                    "idle"
                }
            }
            data-multiline=state.is_multiline.then_some("true")
            data-empty=state.is_empty.then_some("true")
            data-label=state.has_label.then_some("true")
            data-copyable=state.is_copyable.then_some("true")
            data-copy-actionable=state.copy_is_actionable.then_some("true")
            data-copied-label=state.copied_label_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-loading=move || logic.is_loading.get().then_some("true")
            data-copy-error=move || logic.has_error.get().then_some("true")
            data-retry-available=move || {
                (logic.has_error.get() && state.copy_is_actionable).then_some("true")
            }
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

            {state.is_copyable.then(|| {
                view! {
                    <button
                        type="button"
                        class="ui-snippet__copy-button"
                        data-slot="snippet-copy-button"
                        aria-label=copy_aria_label.get_value()
                        aria-busy=logic.aria_busy
                        disabled=move || !state.copy_is_actionable || logic.is_loading.get()
                        data-copied=move || logic.copied.get().then_some("true")
                        data-copying=move || logic.is_copying.get().then_some("true")
                        data-copy-error=move || logic.has_copy_error.get().then_some("true")
                        data-retry-available=move || {
                            (logic.has_copy_error.get() && state.copy_is_actionable).then_some("true")
                        }
                        on:click=move |_| {
                            if logic.has_copy_error.get() {
                                logic.retry_copy.run(())
                            } else {
                                logic.copy.run(())
                            }
                        }
                    >
                        {move || if logic.copied.get() {
                            copied_label.get_value()
                        } else {
                            copy_label.get_value()
                        }}
                    </button>

                    <span
                        class="ui-snippet__a11y-status"
                        data-slot="snippet-copied-status"
                        aria-live="polite"
                        aria-atomic="true"
                    >
                        {move || if logic.has_copy_error.get() {
                            copy_error_label.get_value()
                        } else if logic.copied.get() {
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
