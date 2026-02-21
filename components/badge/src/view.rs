use crate::{BadgeVariant, logic};
use leptos::prelude::*;
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn Badge(
    #[prop(optional, into)] variant: Option<BadgeVariant>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let render_state = logic::resolve_render_state(variant, class_name);
    let locale = locale_attrs(lang, dir);

    view! {
        <span
            class=render_state.class_name
            lang=locale.lang
            dir=locale.dir
            data-slot="badge"
            data-variant=render_state.state.variant_attr
            data-fill=render_state.state.fill_attr
            data-state=render_state.state.fill_attr
            data-solid=render_state.state.is_solid.then_some("true")
            data-outline=render_state.state.is_outline.then_some("true")
            data-custom-class=render_state.state.has_custom_class_name.then_some("true")
            data-class-source=render_state.agent_contract.source.as_attr()
            data-ui-schema=render_state.agent_contract.schema_name
            data-ui-schema-version=render_state.agent_contract.schema_version.as_attr()
            data-ui-intent=render_state.agent_contract.intent.as_attr()
            data-ui-action=render_state.agent_contract.action.as_attr()
            data-ui-state=render_state.agent_contract.state.as_attr()
            data-ui-source=render_state.agent_contract.source.as_attr()
            data-ui-stream-support=render_state.agent_contract.stream_support.as_attr()
            data-ui-stream-fallback=render_state.agent_contract.stream_fallback.as_attr()
            data-ui-stream-mode=render_state.agent_contract.stream_mode.as_attr()
            data-ui-output-status=render_state.agent_contract.output_status.as_attr()
        >
            {children()}
        </span>
    }
}

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
