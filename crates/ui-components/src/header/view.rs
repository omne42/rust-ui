use crate::header::{
    HeaderStateInput,
    logic::{self, HeaderTone},
    motion::HeaderMotion,
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn Header(
    #[prop(optional)] tone: HeaderTone,
    #[prop(optional)] bordered: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: HeaderMotion,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(HeaderStateInput {
            tone,
            bordered,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let motion = crate::header::motion::sanitize_motion(motion);
    let motion_source = crate::header::motion::source_attr(motion);
    let style_vars = crate::header::motion::attach_motion(None, motion);
    let agent_contract = Memo::new(move |_| logic::resolve_agent_contract(state.get()));
    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <header
            class=move || class.get()
            style=style_vars
            data-slot="header"
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-bordered=move || state.get().is_bordered.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=motion_source
            data-custom-motion=(motion_source == "custom").then_some("true")
            data-ui-schema=move || agent_contract.get().schema_attr
            data-ui-intent=move || agent_contract.get().intent_attr
            data-ui-action=move || agent_contract.get().action.as_attr()
            data-ui-state=move || agent_contract.get().state.as_attr()
            data-ui-source=move || agent_contract.get().source.as_attr()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_attr()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_attr()
            data-ui-stream-mode="snapshot"
            data-ui-output-status=move || agent_contract.get().output_status.as_attr()
            aria-label=aria_label
            lang=locale.lang.clone()
            dir=locale.dir
        >
            {children()}
        </header>
    }
}
