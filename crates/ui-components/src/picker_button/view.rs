use crate::FieldButton;
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
    let aria_label = aria_label.unwrap_or_default();
    let class_name = class_name.unwrap_or_default();

    if let Some(on_press) = on_press {
        view! {
            <FieldButton
                quiet=quiet
                invalid=invalid
                disabled=disabled
                is_active=is_active
                aria_label=aria_label
                class_name=class_name
                button_type=button_type
                node_ref=node_ref
                on_press=on_press
            >
                {children()}
            </FieldButton>
        }
        .into_any()
    } else {
        view! {
            <FieldButton
                quiet=quiet
                invalid=invalid
                disabled=disabled
                is_active=is_active
                aria_label=aria_label
                class_name=class_name
                button_type=button_type
                node_ref=node_ref
            >
                {children()}
            </FieldButton>
        }
        .into_any()
    }
}
