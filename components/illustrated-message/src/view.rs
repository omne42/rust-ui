use crate::{IllustratedMessageMotion, IllustratedMessageOrientation, motion};
use leptos::children::ViewFn;
use leptos::prelude::*;
use ui_headless::a11y::{A11yDirection, locale_attrs};

#[component]
pub fn IllustratedMessage(
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] illustration: Option<ViewFn>,
    #[prop(optional, into)] actions: Option<ViewFn>,
    #[prop(optional)] orientation: IllustratedMessageOrientation,
    #[prop(optional)] motion: IllustratedMessageMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let motion = crate::motion::sanitize_motion(motion);
    let locale = locale_attrs(lang, dir);
    let resolved_view = crate::logic::resolve_view_model(
        title,
        description,
        illustration.as_ref(),
        actions.as_ref(),
    );
    let agent_contract = crate::logic::resolve_agent_contract_attrs(&resolved_view);
    let state = resolved_view.state;
    let view_state = resolved_view.view_state.as_data_attr();
    let title_state = resolved_view.title_state.as_data_attr();
    let description_state = resolved_view.description_state.as_data_attr();
    let illustration_state = resolved_view.illustration_state.as_data_attr();
    let actions_state = resolved_view.actions_state.as_data_attr();
    let content_state = resolved_view.content_state.as_data_attr();
    let title_source = resolved_view.title_source.as_data_attr();
    let description_source = resolved_view.description_source.as_data_attr();
    let illustration_source = resolved_view.illustration_source.as_data_attr();
    let actions_source = resolved_view.actions_source.as_data_attr();
    let orientation_attr = orientation.as_data_attr();
    let ui_schema = agent_contract.schema_attr;
    let ui_schema_version = agent_contract.schema_version_attr;
    let ui_intent = agent_contract.intent_attr;
    let ui_action = agent_contract.action_attr;
    let ui_state = agent_contract.state_attr;
    let ui_source = agent_contract.source_attr;
    let ui_config_policy = agent_contract.config_policy_attr;
    let ui_streaming_policy = agent_contract.streaming_policy_attr;
    let ui_streaming_fallback = agent_contract.streaming_fallback_attr;
    let ui_output_status = agent_contract.output_status_attr;

    let class = crate::logic::resolve_root_class(orientation, class_name);

    let root_ref = NodeRef::new();
    motion::attach_motion(root_ref, motion);

    let title = StoredValue::new(resolved_view.title);
    let description = StoredValue::new(resolved_view.description);
    let illustration = state.show_illustration.then_some(illustration).flatten();
    let actions = state.show_actions.then_some(actions).flatten();
    let illustration = illustration.map(StoredValue::new);
    let actions = actions.map(StoredValue::new);

    view! {
        <div
            class=class
            data-slot="illustrated-message"
            data-view-state=view_state
            data-content-state=content_state
            data-title-state=title_state
            data-description-state=description_state
            data-illustration-state=illustration_state
            data-actions-state=actions_state
            data-title-source=title_source
            data-description-source=description_source
            data-illustration-source=illustration_source
            data-actions-source=actions_source
            data-orientation=orientation_attr
            data-ui-schema=ui_schema
            data-ui-schema-version=ui_schema_version
            data-ui-intent=ui_intent
            data-ui-action=ui_action
            data-ui-state=ui_state
            data-ui-source=ui_source
            data-ui-config-policy=ui_config_policy
            data-ui-streaming-policy=ui_streaming_policy
            data-ui-streaming-fallback=ui_streaming_fallback
            data-ui-output-status=ui_output_status
            aria-live="off"
            lang=locale.lang
            dir=locale.dir
            node_ref=root_ref
        >
            {illustration.map(|illustration| {
                view! {
                    <div class="ui-illustrated-message__illustration" data-slot="illustrated-message-illustration">
                        {illustration.get_value().run()}
                    </div>
                }
                .into_any()
            })}

            <div class="ui-illustrated-message__content" data-slot="illustrated-message-content">
                {state.show_title.then(|| {
                    view! {
                        <h3 class="ui-illustrated-message__title" data-slot="illustrated-message-title">
                            {title.get_value().clone()}
                        </h3>
                    }
                })}

                {state.show_description.then(|| {
                    view! {
                        <p class="ui-illustrated-message__description" data-slot="illustrated-message-description">
                            {description.get_value().clone()}
                        </p>
                    }
                })}

                {actions.map(|actions| {
                    view! {
                        <div class="ui-illustrated-message__actions" data-slot="illustrated-message-actions">
                            {actions.get_value().run()}
                        </div>
                    }
                    .into_any()
                })}
            </div>
        </div>
    }
}
