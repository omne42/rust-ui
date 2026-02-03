use crate::spacer::{SpacerAxis, SpacerSize};
use leptos::prelude::*;

#[component]
pub fn Spacer(
    #[prop(optional)] axis: SpacerAxis,
    #[prop(optional)] size: SpacerSize,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let base_class = format!("ui-spacer {} {}", axis.class_name(), size.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! { <div class=class data-slot="spacer" aria-hidden="true"></div> }
}
