use crate::spacer::{
    SpacerAxis, SpacerSize,
    logic::{self, SpacerStateInput},
};
use leptos::prelude::*;

#[component]
pub fn Spacer(
    #[prop(optional)] axis: SpacerAxis,
    #[prop(optional)] size: SpacerSize,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(SpacerStateInput {
        axis,
        size,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    view! {
        <div
            class=class
            data-slot="spacer"
            data-axis=state.axis_attr
            data-size=state.size_attr
            data-state=state.axis_attr
            data-vertical=state.is_vertical.then_some("true")
            data-horizontal=state.is_horizontal.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            aria-hidden="true"
        ></div>
    }
}
