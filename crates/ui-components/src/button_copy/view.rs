use crate::button_copy::{ButtonCopyMotion, logic};
use crate::{Button, ButtonSize, ButtonVariant};
use leptos::prelude::*;

#[component]
pub fn ButtonCopy(
    #[prop(into)] text: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] copied_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, default = ButtonVariant::Secondary)] variant: ButtonVariant,
    #[prop(optional, default = ButtonSize::Sm)] size: ButtonSize,
    #[prop(optional)] motion: ButtonCopyMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let view_state = logic::resolve_view_state(&text, disabled);
    let logic = crate::snippet::logic::use_snippet_logic(text.clone());

    let label = label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Copy".to_string());
    let copied_label = copied_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Copied".to_string());
    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| label.clone());

    let label = StoredValue::new(label);
    let copied_label = StoredValue::new(copied_label);

    let base_class = "ui-button-copy".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <span
            class=class
            data-slot="button-copy"
            data-copied=move || logic.copied.get().then_some("true")
        >
            <Button
                class_name="ui-button-copy__button".to_string()
                variant=variant
                size=size
                motion=motion.button
                aria_label=aria_label
                disabled=!view_state.is_copyable
                on_press=logic.copy
            >
                {move || {
                    if logic.copied.get() {
                        copied_label.get_value()
                    } else {
                        label.get_value()
                    }
                }}
            </Button>

            <span
                class="ui-button-copy__a11y-status"
                data-slot="button-copy-status"
                aria-live="polite"
                aria-atomic="true"
            >
                {move || {
                    if logic.copied.get() {
                        copied_label.get_value()
                    } else {
                        String::new()
                    }
                }}
            </span>
        </span>
    }
}
