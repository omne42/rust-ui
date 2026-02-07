use crate::button::{Button, ButtonMotion, ButtonSize, ButtonVariant};
use crate::icon_button::logic;
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
    let has_custom_press_handler = on_press.is_some();
    let class_name = logic::normalize_class_name(class_name);
    let (aria_label, has_explicit_aria_label) = logic::normalize_aria_label(aria_label);

    let state = logic::resolve_state(
        disabled,
        size,
        has_custom_press_handler,
        has_explicit_aria_label,
        class_name.is_some(),
    );

    let class_name = logic::compose_class_name(class_name, state);

    let maybe_on_press = on_press;
    let on_press = Callback::new(move |_| {
        if let Some(handler) = maybe_on_press.as_ref() {
            handler.run(());
        }
    });

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
