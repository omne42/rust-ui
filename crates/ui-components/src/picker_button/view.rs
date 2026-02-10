use crate::FieldButton;
use crate::picker_button::{PickerButtonStateInput, logic};
use leptos::{html, prelude::*};
use ui_headless::OnPress;

#[component]
pub fn PickerButton(
    #[prop(optional)] quiet: bool,
    #[prop(optional)] invalid: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] is_active: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, default = "button")] button_type: &'static str,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let class_name_for_inner = class_name.clone().unwrap_or_default();

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let has_custom_press_handler = on_press.is_some();

    let state = logic::resolve_state(PickerButtonStateInput {
        quiet,
        invalid,
        disabled,
        forced_active: is_active,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        has_custom_press_handler,
    });

    let class = logic::compose_class_name(class_name, state);

    if let Some(on_press) = on_press {
        view! {
            <div
                class=class
                data-slot="picker-button"
                data-state=state.data_state_attr
                data-quiet=state.is_quiet.then_some("true")
                data-invalid=state.is_invalid.then_some("true")
                data-disabled=state.is_disabled.then_some("true")
                data-active=state.is_forced_active.then_some("true")
                data-has-handler=state.has_custom_press_handler.then_some("true")
                data-active-mode=state.active_mode_attr
                data-quiet-mode=state.quiet_attr
                data-invalid-mode=state.invalid_attr
                data-disabled-mode=state.disabled_attr
                data-aria-source=state.aria_source_attr
                data-class-source=state.class_source_attr
                data-handler-source=state.handler_source_attr
                data-custom-class=state.has_custom_class_name.then_some("true")
            >
                <FieldButton
                    quiet=quiet
                    invalid=invalid
                    disabled=disabled
                    is_active=is_active
                    aria_label=aria_label
                    class_name=class_name_for_inner
                    button_type=button_type
                    node_ref=node_ref
                    on_press=on_press
                >
                    {children()}
                </FieldButton>
            </div>
        }
        .into_any()
    } else {
        view! {
            <div
                class=class
                data-slot="picker-button"
                data-state=state.data_state_attr
                data-quiet=state.is_quiet.then_some("true")
                data-invalid=state.is_invalid.then_some("true")
                data-disabled=state.is_disabled.then_some("true")
                data-active=state.is_forced_active.then_some("true")
                data-has-handler=state.has_custom_press_handler.then_some("true")
                data-active-mode=state.active_mode_attr
                data-quiet-mode=state.quiet_attr
                data-invalid-mode=state.invalid_attr
                data-disabled-mode=state.disabled_attr
                data-aria-source=state.aria_source_attr
                data-class-source=state.class_source_attr
                data-handler-source=state.handler_source_attr
                data-custom-class=state.has_custom_class_name.then_some("true")
            >
                <FieldButton
                    quiet=quiet
                    invalid=invalid
                    disabled=disabled
                    is_active=is_active
                    aria_label=aria_label
                    class_name=class_name_for_inner
                    button_type=button_type
                    node_ref=node_ref
                >
                    {children()}
                </FieldButton>
            </div>
        }
        .into_any()
    }
}
