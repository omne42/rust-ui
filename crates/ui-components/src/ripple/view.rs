use crate::ripple::RippleMotion;
use leptos::{html, prelude::*};

#[component]
pub fn MotionRipple(
    #[prop(optional)] motion: RippleMotion,
    #[prop(optional)] node_ref: NodeRef<html::Span>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let _ = motion;

    let base_class = "ui-ripple".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <span
            node_ref=node_ref
            class=class
            data-slot="ripple"
            aria-hidden="true"
        ></span>
    }
}
