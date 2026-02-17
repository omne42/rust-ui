use super::{Button, ButtonSize, ButtonType};
use leptos::{html, prelude::*};
use ui_headless::OnPress;

mod logic;
pub mod styles;
pub use logic::DEFAULT_ARIA_LABEL;

#[component]
pub fn FieldButton(
    #[prop(optional)] is_quiet: bool,
    #[prop(optional)] is_invalid: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_active: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] button_type: ButtonType,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
    let resolved = logic::resolve_props(logic::FieldButtonResolveInput {
        is_quiet,
        is_invalid,
        is_disabled,
        is_active,
        aria_label,
        class_name,
        button_type,
        on_press,
    });

    view! {
        <Button
            variant=resolved.variant
            color=resolved.color
            size=ButtonSize::S
            is_disabled=resolved.is_disabled
            class_name=resolved.class_name
            button_type=resolved.button_type
            aria_label=resolved.aria_label
            node_ref=node_ref
            on_press=resolved.on_press
        >
            {children()}
        </Button>
    }
}
