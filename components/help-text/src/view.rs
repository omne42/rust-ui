use crate::{
    A11yDirection, HelpTextMotion,
    logic::{self, HelpTextTone},
    motion,
};
use leptos::{html, prelude::*};

#[component]
pub fn HelpText(
    #[prop(optional)] tone: HelpTextTone,
    #[prop(optional)] is_invalid: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_error_icon_visible: bool,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error_message: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] motion: HelpTextMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let logic::HelpTextRenderModel {
        aria_label,
        description_text,
        error_message_text,
        class_name,
        state: resolved_state,
    } = logic::resolve_render_model(logic::HelpTextLogicInput {
        tone,
        is_invalid,
        is_disabled,
        is_error_icon_visible,
        description,
        error_message,
        aria_label,
        class_name,
    });
    let description_text = StoredValue::new(description_text);
    let error_message_text = StoredValue::new(error_message_text);
    let state = StoredValue::new(resolved_state);
    let class = StoredValue::new(logic::compose_class_name(class_name, resolved_state));
    let agent_contract = StoredValue::new(logic::resolve_agent_contract_attrs(resolved_state));

    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != HelpTextMotion::default();

    let locale = logic::resolve_locale_attrs(lang, dir);
    let error_live_region = logic::resolve_error_live_region_attrs();
    let is_error =
        Signal::derive(move || state.get_value().message_kind == logic::HelpTextMessageKind::Error);
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, is_error, motion);

    view! {
        <div
            node_ref=root_ref
            class=move || class.get_value()
            data-slot="help-text"
            data-tone=move || state.get_value().tone_attr
            data-state=move || state.get_value().data_state.as_attr()
            data-message-kind=move || state.get_value().message_kind.as_attr()
            data-invalid=move || state.get_value().is_invalid.then_some("true")
            data-disabled=move || state.get_value().is_disabled.then_some("true")
            data-show-error-icon=move || state.get_value().show_error_icon.then_some("true")
            data-has-description=move || state.get_value().has_description.then_some("true")
            data-has-error=move || state.get_value().has_error_message.then_some("true")
            data-aria-source=move || state.get_value().aria_source.as_attr()
            data-error-source=move || state.get_value().error_source.as_attr()
            data-custom-class=move || state.get_value().has_custom_class_name.then_some("true")
            data-class-source=move || state.get_value().class_source.as_attr()
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-ui-schema=move || agent_contract.get_value().data_ui_schema
            data-ui-schema-version=move || agent_contract.get_value().data_ui_schema_version
            data-ui-intent=move || agent_contract.get_value().data_ui_intent
            data-ui-action=move || agent_contract.get_value().data_ui_action
            data-ui-state=move || agent_contract.get_value().data_ui_state
            data-ui-source=move || agent_contract.get_value().data_ui_source
            data-ui-stream-support=move || agent_contract.get_value().data_ui_stream_support
            data-ui-stream-mode=move || agent_contract.get_value().data_ui_stream_mode
            data-ui-stream-fallback=move || agent_contract.get_value().data_ui_stream_fallback
            data-ui-output-status=move || agent_contract.get_value().data_ui_output_status
            aria-label=aria_label
            aria-disabled=move || state.get_value().is_disabled.then_some("true")
            aria-invalid=move || state.get_value().is_invalid.then_some("true")
            lang=locale.lang
            dir=locale.dir
        >
            <Show when=move || state.get_value().message_kind == logic::HelpTextMessageKind::Error>
                <Show when=move || state.get_value().show_error_icon>
                    <span class="ui-help-text__icon" data-slot="help-text-icon" aria-hidden="true">
                        "⚠"
                    </span>
                </Show>
                <p
                    class="ui-help-text__text"
                    data-slot="help-text-error"
                    role=error_live_region.role
                    aria-live=error_live_region.aria_live
                >
                    {move || error_message_text.get_value()}
                </p>
            </Show>

            <Show when=move || state.get_value().message_kind == logic::HelpTextMessageKind::Description>
                <p class="ui-help-text__text" data-slot="help-text-description">
                    {move || description_text.get_value()}
                </p>
            </Show>
        </div>
    }
}
