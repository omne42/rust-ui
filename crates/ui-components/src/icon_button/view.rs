use crate::button::{Button, ButtonMotion, ButtonSize, ButtonVariant};
use leptos::{html, prelude::*};
use ui_headless::OnPress;

#[component]
pub fn IconButton(
    #[prop(optional)] disabled: bool,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional, default = ButtonSize::Icon)] size: ButtonSize,
    #[prop(optional)] motion: ButtonMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(into)] aria_label: String,
    #[prop(optional, default = "button")] button_type: &'static str,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
    let class_name = class_name.unwrap_or_default();
    let on_press = on_press.unwrap_or_else(|| Callback::new(|_| {}));

    view! {
        <Button
            disabled=disabled
            variant=variant
            size=size
            motion=motion
            class_name=class_name
            button_type=button_type
            aria_label=aria_label
            node_ref=node_ref
            on_press=on_press
        >
            {children()}
        </Button>
    }
}
