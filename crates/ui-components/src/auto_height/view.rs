use crate::auto_height::{AutoHeightMotion, logic, motion};
use leptos::{html, prelude::*};

#[component]
pub fn AutoHeight(
    children: Children,
    #[prop(optional)] motion: AutoHeightMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let state = logic::resolve_state();

    let container_ref: NodeRef<html::Div> = NodeRef::new();
    let content_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(container_ref, content_ref, motion);

    let base_class = "ui-auto-height".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <div
            class=class
            node_ref=container_ref
            data-slot="auto-height"
            data-overflow-hidden=state.overflow_hidden.then_some("true")
        >
            <div class="ui-auto-height__content" node_ref=content_ref data-slot="auto-height-content">
                {children()}
            </div>
        </div>
    }
}
