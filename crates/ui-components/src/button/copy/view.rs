use super::{ButtonCopyMotion, logic};
use crate::button::{Button, ButtonSize, ButtonVariant};
use leptos::html;
use leptos::prelude::*;

fn icon_view(copied: bool) -> impl IntoView {
    if copied {
        view! {
            <svg
                class="ui-button-copy__icon-svg"
                viewBox="0 0 16 16"
                aria-hidden="true"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    d="M3 8.5L6.25 11.5L13 4.5"
                    stroke="currentColor"
                    stroke-width="1.8"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                />
            </svg>
        }
        .into_any()
    } else {
        view! {
            <svg
                class="ui-button-copy__icon-svg"
                viewBox="0 0 16 16"
                aria-hidden="true"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <rect
                    x="5"
                    y="3"
                    width="8"
                    height="10"
                    rx="1.75"
                    stroke="currentColor"
                    stroke-width="1.5"
                />
                <path
                    d="M3 11V5.5C3 4.67 3.67 4 4.5 4H10"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                />
            </svg>
        }
        .into_any()
    }
}

#[component]
pub fn ButtonCopy(
    #[prop(into)] text: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] copied_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional, default = logic::ButtonCopyMode::default())] mode: logic::ButtonCopyMode,
    #[prop(optional, default = ButtonVariant::Secondary)] variant: ButtonVariant,
    #[prop(optional, default = ButtonSize::Sm)] size: ButtonSize,
    #[prop(optional)] motion: ButtonCopyMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let motion = super::motion::sanitize_motion(motion);
    let root_ref = NodeRef::<html::Span>::new();
    let label = logic::normalize_optional_text(label);
    let copied_label = logic::normalize_optional_text(copied_label);
    let aria_label = logic::normalize_optional_text(aria_label);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_label = label.is_some();
    let has_custom_copied_label = copied_label.is_some();
    let has_custom_aria_label = aria_label.is_some();
    let has_custom_class_name = class_name.is_some();

    let view_state = logic::resolve_view_state(
        &text,
        is_disabled,
        mode,
        has_custom_label,
        has_custom_copied_label,
        has_custom_aria_label,
        has_custom_class_name,
    );

    let logic = crate::snippet::logic::use_snippet_logic(text.clone());
    super::motion::attach_motion(root_ref, logic.copied, motion);
    let logic::ButtonCopyTextContract {
        label,
        copied_label,
        aria_label,
    } = logic::resolve_text_contract(label, copied_label, aria_label);

    let label = StoredValue::new(label);
    let copied_label = StoredValue::new(copied_label);

    let class = logic::compose_class_name(class_name, view_state);

    view! {
        <span
            node_ref=root_ref
            class=class
            data-slot="button-copy"
            data-state=if view_state.is_copyable {
                "copyable"
            } else if view_state.is_disabled {
                "disabled"
            } else {
                "empty"
            }
            data-mode=view_state.mode_attr
            data-icon-only=view_state.is_icon_only.then_some("true")
            data-with-icon=view_state.shows_icon.then_some("true")
            data-with-text=view_state.shows_text.then_some("true")
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
            data-motion-source=if motion == ButtonCopyMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != ButtonCopyMotion::default()).then_some("true")
        >
            <Button
                class_name="ui-button-copy__button".to_string()
                variant=variant
                size=size
                motion=motion.button
                is_icon_only=view_state.is_icon_only
                aria_label=aria_label
                is_disabled=!view_state.is_copyable
                on_press=logic.copy
            >
                {move || {
                    let copied = logic.copied.get();
                    let text = if copied {
                        copied_label.get_value()
                    } else {
                        label.get_value()
                    };
                    if view_state.shows_icon && view_state.shows_text {
                        view! {
                            <span class="ui-button-copy__content" data-slot="button-copy-content">
                                <span class="ui-button-copy__icon" data-slot="button-copy-icon">
                                    {icon_view(copied)}
                                </span>
                                <span class="ui-button-copy__text" data-slot="button-copy-text">
                                    {text}
                                </span>
                            </span>
                        }
                        .into_any()
                    } else if view_state.shows_icon {
                        view! {
                            <span class="ui-button-copy__icon" data-slot="button-copy-icon">
                                {icon_view(copied)}
                            </span>
                        }
                        .into_any()
                    } else {
                        view! {
                            <span class="ui-button-copy__text" data-slot="button-copy-text">
                                {text}
                            </span>
                        }
                        .into_any()
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
