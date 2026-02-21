use crate::{
    ErrorViewMotion,
    logic::{self, ErrorViewTone},
    motion,
};
use leptos::children::{Children, ViewFn};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, error_view_attrs};

fn render_content(children: Option<Children>, message: StoredValue<String>) -> AnyView {
    if let Some(children) = children {
        view! {
            <div class="ui-error-view__content" data-slot="error-view-content">
                {children()}
            </div>
        }
        .into_any()
    } else {
        view! {
            <div class="ui-error-view__content" data-slot="error-view-content">
                <p class="ui-error-view__text" data-slot="error-view-text">
                    {message.get_value()}
                </p>
            </div>
        }
        .into_any()
    }
}

fn render_icon(icon: Option<StoredValue<ViewFn>>) -> Option<AnyView> {
    icon.map(|icon| {
        view! {
            <span class="ui-error-view__icon" data-slot="error-view-icon" aria-hidden="true">
                {icon.get_value().run()}
            </span>
        }
        .into_any()
    })
}

fn render_actions(actions: Option<StoredValue<ViewFn>>) -> Option<AnyView> {
    actions.map(|actions| {
        view! {
            <div class="ui-error-view__actions" data-slot="error-view-actions">
                {actions.get_value().run()}
            </div>
        }
        .into_any()
    })
}

#[component]
pub fn ErrorView(
    #[prop(optional)] is_invalid: bool,
    #[prop(optional)] tone: Option<ErrorViewTone>,
    #[prop(optional)] is_compact: Option<bool>,
    #[prop(optional)] is_bordered: Option<bool>,
    #[prop(optional)] motion: ErrorViewMotion,
    #[prop(optional, into)] message: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional, into)] actions: Option<ViewFn>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let has_icon = icon.is_some();
    let has_actions = actions.is_some();
    let has_children = children.is_some();
    let has_custom_motion = motion != ErrorViewMotion::default();

    let normalized = logic::normalize_props(logic::ErrorViewNormalizeInput {
        tone,
        is_invalid,
        is_compact,
        is_bordered,
        message,
        aria_label,
        class_name,
        has_icon,
        has_actions,
        has_children,
        has_custom_motion,
    });
    let message = StoredValue::new(normalized.message);
    let class_name = StoredValue::new(normalized.class_name);
    let state_input = StoredValue::new(normalized.state_input);

    let icon = icon.map(StoredValue::new);
    let actions = actions.map(StoredValue::new);

    let state = Signal::derive(move || logic::resolve_state(state_input.get_value()));
    let agent_contract = Memo::new(move |_| {
        let resolved_state = state.get();
        logic::resolve_agent_contract(logic::ErrorViewAgentContractInput {
            is_visible: resolved_state.is_visible,
            message_source_attr: resolved_state.message_source_attr,
            aria_source_attr: resolved_state.aria_source_attr,
            class_source_attr: resolved_state.class_source_attr,
            motion_source_attr: resolved_state.motion_source_attr,
            has_actions: resolved_state.has_actions,
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));
    let visible = Signal::derive(move || state.get().is_visible);
    let a11y = error_view_attrs(visible, normalized.aria_label, lang, dir);
    let role = a11y.role;
    let aria_live = a11y.aria_live;
    let aria_hidden = a11y.aria_hidden;
    let aria_label = a11y.aria_label;
    let lang = a11y.lang;
    let dir = a11y.dir;

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, visible, motion);

    let content = render_content(children, message);
    let icon = render_icon(icon);
    let actions = render_actions(actions);

    view! {
        <div
            node_ref=root_ref
            class=move || class.get()
            data-slot="error-view"
            data-tone=move || state.get().tone_attr
            data-tone-source=normalized.tone_source_attr
            data-state=move || state.get().state_attr
            data-invalid=move || state.get().is_visible.then_some("true")
            data-hidden=move || state.get().is_hidden.then_some("true")
            data-compact=move || state.get().is_compact.then_some("true")
            data-compact-source=normalized.compact_source_attr
            data-bordered=move || state.get().is_bordered.then_some("true")
            data-bordered-source=normalized.bordered_source_attr
            data-icon=move || state.get().has_icon.then_some("true")
            data-actions=move || state.get().has_actions.then_some("true")
            data-content=move || state.get().content_attr
            data-message-source=move || state.get().message_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-state-source=move || agent_contract.get().state_source.as_str()
            data-ui-action-source=move || agent_contract.get().action_source.as_str()
            data-ui-motion-source=move || agent_contract.get().motion_source.as_str()
            data-ui-config-policy=move || agent_contract.get().config_policy.as_str()
            role=role
            aria-live=move || aria_live.get()
            aria-hidden=move || aria_hidden.get()
            aria-label=aria_label
            lang=lang
            dir=dir
        >
            {icon}

            {content}

            {actions}
        </div>
    }
}
