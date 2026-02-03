use crate::{CircularProgress, spinner::SpinnerSize};
use leptos::prelude::*;

#[component]
pub fn Spinner(
    #[prop(optional)] size: SpinnerSize,
    #[prop(optional, into, default = "Loading".to_string())] aria_label: String,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let base_class = format!("ui-spinner {}", size.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! { <CircularProgress aria_label=aria_label class_name=class /> }
}
