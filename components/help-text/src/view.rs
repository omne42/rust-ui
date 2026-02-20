use crate::{
    HelpTextMotion, HelpTextStateInput,
    logic::{self, HelpTextTone},
    motion,
};
use leptos::{html, prelude::*};

#[component]
pub fn HelpText(
    #[prop(optional)] tone: HelpTextTone,
    #[prop(optional)] invalid: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] show_error_icon: bool,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error_message: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] motion: HelpTextMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let description = logic::normalize_optional_text(description);
    let (error_message, has_custom_error_message) =
        logic::normalize_error_message(error_message, invalid);

    let has_description = description.is_some();
    let has_error_message = error_message.is_some();

    let description = StoredValue::new(description);
    let error_message = StoredValue::new(error_message);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != HelpTextMotion::default();

    let state = Memo::new(move |_| {
        logic::resolve_state(HelpTextStateInput {
            tone,
            invalid,
            disabled,
            show_error_icon,
            has_description,
            has_error_message,
            has_custom_aria_label,
            has_custom_error_message,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let is_error = Signal::derive(move || state.get().message_kind_attr == "error");
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, is_error, motion);

    view! {
        <div
            node_ref=root_ref
            class=move || class.get()
            data-slot="help-text"
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-message-kind=move || state.get().message_kind_attr
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-show-error-icon=move || state.get().show_error_icon.then_some("true")
            data-has-description=move || state.get().has_description.then_some("true")
            data-has-error=move || state.get().has_error_message.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-error-source=move || state.get().error_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            aria-label=aria_label
            aria-disabled=move || state.get().is_disabled.then_some("true")
            aria-invalid=move || state.get().is_invalid.then_some("true")
        >
            <Show when=move || state.get().message_kind_attr == "error">
                <Show when=move || state.get().show_error_icon>
                    <span class="ui-help-text__icon" data-slot="help-text-icon" aria-hidden="true">
                        "⚠"
                    </span>
                </Show>
                <p class="ui-help-text__text" data-slot="help-text-error" role="alert">
                    {move || error_message.get_value().unwrap_or_default()}
                </p>
            </Show>

            <Show when=move || state.get().message_kind_attr == "description">
                <p class="ui-help-text__text" data-slot="help-text-description">
                    {move || description.get_value().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}
