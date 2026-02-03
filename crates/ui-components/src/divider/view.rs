use crate::divider::DividerOrientation;
use leptos::prelude::*;

#[component]
pub fn Divider(
    #[prop(optional)] orientation: DividerOrientation,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let base_class = format!("ui-divider {}", orientation.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let aria_orientation = match orientation {
        DividerOrientation::Vertical => Some("vertical"),
        DividerOrientation::Horizontal => None,
    };

    view! {
        <div
            class=class
            data-slot="divider"
            role="separator"
            aria-orientation=aria_orientation
        ></div>
    }
}
