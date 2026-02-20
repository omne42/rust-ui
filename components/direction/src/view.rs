use crate::logic::{self, DirectionMode as DirectionModeImpl};
use leptos::prelude::*;

pub use crate::logic::DirectionMode;

#[component]
pub fn DirectionProvider(
    #[prop(optional)] direction: Option<DirectionModeImpl>,
    #[prop(optional)] dir: Option<DirectionModeImpl>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let direction = direction.or(dir).unwrap_or_default();
    let class_name = logic::compose_class_name(class_name);

    view! {
        <div
            class=class_name
            dir=direction.as_attr()
            data-slot="direction-provider"
            data-direction=direction.as_attr()
        >
            {children()}
        </div>
    }
}
