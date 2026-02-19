use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::code_block::{
    CodeBlockMotion, CodeBlockStrings,
    logic::{self, CodeBlockStateInput},
    motion,
};
use leptos::{html, prelude::*};
use ui_headless::i18n;

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
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<CodeBlockStrings>();
    let copy_to_clipboard_aria_label: String = strings.copy_to_clipboard_aria_label.as_ref().into();
    let copy_to_clipboard_aria_label = StoredValue::new(copy_to_clipboard_aria_label);
    let copied_status_text = strings.copied_status_text.as_ref().into();
    let motion = crate::code_block::motion::sanitize_motion(motion);
    let label = logic::normalize_optional_text(label);
    let language = logic::normalize_optional_text(language);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(CodeBlockStateInput {
        is_multiline: code.contains('\n'),
        is_empty: code.trim().is_empty(),
        has_label: label.is_some(),
        has_language: language.is_some(),
        copyable,
        has_custom_class_name: class_name.is_some(),
        has_custom_motion: motion != CodeBlockMotion::default(),
    });

    let class = logic::compose_class_name(class_name, state);

    let code_value = StoredValue::new(code);
    let label = StoredValue::new(label);
    let language = StoredValue::new(language);

    let copy_logic = crate::snippet::logic::use_snippet_logic(code_value.get_value());
    let copied_label = StoredValue::new(copied_status_text);

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, copy_logic.copied, motion);

    view! {
        <div
            class=class
            data-slot="code-block"
            data-state=state.state_attr
            data-header=state.header_attr
            data-multiline=state.is_multiline.then_some("true")
            data-empty=state.is_empty.then_some("true")
            data-label=state.has_label.then_some("true")
            data-language=state.has_language.then_some("true")
            data-copyable=state.copyable.then_some("true")
            data-motion-source=state.motion_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            node_ref=root_ref
        >
            <Show when=move || state.show_header>
                <div class="ui-code-block__header" data-slot="code-block-header">
                    <div class="ui-code-block__meta" data-slot="code-block-meta">
                        {move || label.get_value().map(|label| view! {
                            <span class="ui-code-block__label" data-slot="code-block-label">{label}</span>
                        })}
                        {move || language.get_value().map(|language| view! {
                            <span class="ui-code-block__language" data-slot="code-block-language">{language}</span>
                        })}
                    </div>

                    <Show when=move || state.copyable>
                        <Button
                            class_name="ui-code-block__copy-button".to_string()
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::IconSm
                            aria_label=copy_to_clipboard_aria_label.get_value()
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
