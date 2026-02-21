use crate::logic::{self, A11yDirection, DescriptionElement, DescriptionTone};
use leptos::prelude::*;

fn render_span(
    class: Memo<String>,
    state: Memo<logic::DescriptionState>,
    agent_contract: Memo<logic::DescriptionAgentContractAttrs>,
    aria_label: String,
    lang: StoredValue<Option<String>>,
    dir: StoredValue<Option<&'static str>>,
    text: StoredValue<String>,
) -> AnyView {
    view! {
        <span
            class=move || class.get()
            data-slot="description"
            slot="description"
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-truncate=move || state.get().is_truncated.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-ui-schema=move || agent_contract.get().schema_attr
            data-ui-schema-version=move || agent_contract.get().schema_version_attr
            data-ui-intent=move || agent_contract.get().intent_attr
            data-ui-action=move || agent_contract.get().action_attr
            data-ui-state=move || agent_contract.get().state_attr
            data-ui-source=move || agent_contract.get().source_attr
            data-ui-stream-support=move || agent_contract.get().stream_support_attr
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr
            data-ui-output-status=move || agent_contract.get().output_status_attr
            aria-label=aria_label
            lang=move || lang.get_value()
            dir=move || dir.get_value()
        >
            {text.get_value()}
        </span>
    }
    .into_any()
}

fn render_paragraph(
    class: Memo<String>,
    state: Memo<logic::DescriptionState>,
    agent_contract: Memo<logic::DescriptionAgentContractAttrs>,
    aria_label: String,
    lang: StoredValue<Option<String>>,
    dir: StoredValue<Option<&'static str>>,
    text: StoredValue<String>,
) -> AnyView {
    view! {
        <p
            class=move || class.get()
            data-slot="description"
            slot="description"
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-truncate=move || state.get().is_truncated.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-ui-schema=move || agent_contract.get().schema_attr
            data-ui-schema-version=move || agent_contract.get().schema_version_attr
            data-ui-intent=move || agent_contract.get().intent_attr
            data-ui-action=move || agent_contract.get().action_attr
            data-ui-state=move || agent_contract.get().state_attr
            data-ui-source=move || agent_contract.get().source_attr
            data-ui-stream-support=move || agent_contract.get().stream_support_attr
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr
            data-ui-output-status=move || agent_contract.get().output_status_attr
            aria-label=aria_label
            lang=move || lang.get_value()
            dir=move || dir.get_value()
        >
            {text.get_value()}
        </p>
    }
    .into_any()
}

fn render_div(
    class: Memo<String>,
    state: Memo<logic::DescriptionState>,
    agent_contract: Memo<logic::DescriptionAgentContractAttrs>,
    aria_label: String,
    lang: StoredValue<Option<String>>,
    dir: StoredValue<Option<&'static str>>,
    text: StoredValue<String>,
) -> AnyView {
    view! {
        <div
            class=move || class.get()
            data-slot="description"
            slot="description"
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-truncate=move || state.get().is_truncated.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-ui-schema=move || agent_contract.get().schema_attr
            data-ui-schema-version=move || agent_contract.get().schema_version_attr
            data-ui-intent=move || agent_contract.get().intent_attr
            data-ui-action=move || agent_contract.get().action_attr
            data-ui-state=move || agent_contract.get().state_attr
            data-ui-source=move || agent_contract.get().source_attr
            data-ui-stream-support=move || agent_contract.get().stream_support_attr
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr
            data-ui-output-status=move || agent_contract.get().output_status_attr
            aria-label=aria_label
            lang=move || lang.get_value()
            dir=move || dir.get_value()
        >
            {text.get_value()}
        </div>
    }
    .into_any()
}

#[component]
pub fn Description(
    text: String,
    #[prop(optional)] tone: DescriptionTone,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_truncated: bool,
    #[prop(optional)] element: DescriptionElement,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let logic::DescriptionViewModel {
        text,
        aria_label,
        class_name,
        state: resolved_state,
        agent_contract: resolved_agent_contract,
        lang,
        dir,
    } = logic::resolve_view_model(logic::DescriptionViewModelInput {
        text,
        tone,
        is_disabled,
        is_truncated,
        aria_label,
        class_name,
        lang,
        dir,
    });
    let text = StoredValue::new(text);
    let class_name = StoredValue::new(class_name);
    let lang = StoredValue::new(lang);
    let dir = StoredValue::new(dir);

    let state = Memo::new(move |_| resolved_state);
    let agent_contract = Memo::new(move |_| resolved_agent_contract);
    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    match element {
        DescriptionElement::Span => {
            render_span(class, state, agent_contract, aria_label, lang, dir, text)
        }
        DescriptionElement::Paragraph => {
            render_paragraph(class, state, agent_contract, aria_label, lang, dir, text)
        }
        DescriptionElement::Div => {
            render_div(class, state, agent_contract, aria_label, lang, dir, text)
        }
    }
}
