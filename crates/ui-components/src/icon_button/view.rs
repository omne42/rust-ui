use crate::button::{Button, ButtonMotion, ButtonSize, ButtonVariant};
use crate::icon_button::{IconButtonStateInput, logic};
use leptos::{html, prelude::*};
use ui_headless::OnPress;
use ui_headless::i18n;
use ui_headless::i18n::CommonStrings;

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
    let i18n = i18n::use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let class_name = logic::normalize_optional_text(class_name);
    let class_name_for_inner = class_name.clone().unwrap_or_default();

    let (aria_label, has_explicit_aria_label) =
        logic::normalize_aria_label(aria_label, common.icon_button_aria_label.as_ref());
    let has_custom_press_handler = on_press.is_some();

    let state = logic::resolve_state(IconButtonStateInput {
        disabled,
        size,
        has_custom_press_handler,
        has_explicit_aria_label,
        has_custom_class_name: class_name.is_some(),
        has_custom_motion: motion != ButtonMotion::default(),
    });

    let class = logic::compose_class_name(class_name, state);

    let maybe_on_press = on_press;
    let on_press = Callback::new(move |_| {
        if let Some(handler) = maybe_on_press.as_ref() {
            handler.run(());
        }
    });

    view! {
        <div
            class=class
            data-slot="icon-button"
            data-state=state.state_attr
            data-size-mode=state.size_mode_attr
            data-handler-source=state.handler_source_attr
            data-label-source=state.label_source_attr
            data-class-source=state.class_source_attr
            data-motion-source=state.motion_source_attr
            data-disabled=state.is_disabled.then_some("true")
            data-enabled=state.is_enabled.then_some("true")
            data-fallback-label=state.has_fallback_aria_label.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-custom-motion=state.has_custom_motion.then_some("true")
        >
            <Button
                is_disabled=disabled
                variant=variant
                size=size
                motion=motion
                class_name=class_name_for_inner
                button_type=button_type
                aria_label=aria_label
                node_ref=node_ref
                on_press=on_press
            >
                {children()}
            </Button>
        </div>
    }
}
