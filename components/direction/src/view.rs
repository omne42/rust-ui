use crate::logic::{self, DirectionMode as DirectionModeImpl};
use leptos::prelude::*;
use ui_headless::{DirectionOptions as DirectionA11yOptions, use_direction};

pub use crate::logic::DirectionMode;

#[component]
pub fn DirectionProvider(
    #[prop(optional)] direction: Option<DirectionModeImpl>,
    // Compatibility alias for legacy callers; prefer `direction`.
    #[prop(optional)] dir: Option<DirectionModeImpl>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (direction, direction_source) = logic::resolve_direction(direction, dir);
    let class_name = logic::compose_class_name(class_name);
    let contract = use_direction(DirectionA11yOptions { direction, lang });
    let agent_contract = logic::resolve_agent_contract(direction, direction_source);

    view! {
        <div
            class=class_name
            lang=contract.attrs.lang
            dir=contract.attrs.dir
            data-slot="direction-provider"
            data-direction=contract.attrs.data_direction
            data-direction-source=direction_source.as_attr()
            data-ui-schema=agent_contract.schema_name
            data-ui-schema-version=agent_contract.schema_version
            data-ui-intent=agent_contract.intent.as_attr()
            data-ui-action=agent_contract.action.as_attr()
            data-ui-state=agent_contract.state.as_attr()
            data-ui-source=agent_contract.source.as_attr()
            data-ui-stream-support=agent_contract.stream_support.as_attr()
            data-ui-stream-fallback=agent_contract.stream_fallback.as_attr()
            data-ui-output-status=agent_contract.output_status.as_attr()
        >
            {children()}
        </div>
    }
}
