use crate::auto_height::{AutoHeightMotion, logic};
use leptos::prelude::*;

#[component]
pub fn AutoHeight(
    children: Children,
    #[prop(optional)] motion: AutoHeightMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let _ = motion;
    let state = logic::resolve_state();

    let base_class = "ui-auto-height".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <div
            class=class
            data-slot="auto-height"
            data-overflow-hidden=state.overflow_hidden.then_some("true")
        >
            {children()}
        </div>
    }
}
