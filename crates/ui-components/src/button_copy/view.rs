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
    let label = logic::normalize_optional_text(label);
    let copied_label = logic::normalize_optional_text(copied_label);
    let aria_label = logic::normalize_optional_text(aria_label);
    let class_name = logic::normalize_optional_text(class_name);

    let view_state = logic::resolve_view_state(
        &text,
        disabled,
        label.is_some(),
        copied_label.is_some(),
        aria_label.is_some(),
        class_name.is_some(),
    );

    let logic = crate::snippet::logic::use_snippet_logic(text.clone());

    let label = label.unwrap_or_else(|| "Copy".to_string());
    let copied_label = copied_label.unwrap_or_else(|| "Copied".to_string());
    let aria_label = aria_label.unwrap_or_else(|| label.clone());

    let label = StoredValue::new(label);
    let copied_label = StoredValue::new(copied_label);

    let class = logic::compose_class_name(class_name, view_state);

    view! {
        <span
            class=class
            data-slot="button-copy"
            data-state=if view_state.is_copyable {
                "copyable"
            } else if view_state.is_disabled {
                "disabled"
            } else {
                "empty"
            }
            data-copyable=view_state.is_copyable.then_some("true")
            data-disabled=view_state.is_disabled.then_some("true")
            data-empty=(!view_state.has_text).then_some("true")
            data-label=if view_state.has_custom_label {
                "custom"
            } else {
                "default"
            }
            data-copied-label=if view_state.has_custom_copied_label {
                "custom"
            } else {
                "default"
            }
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
