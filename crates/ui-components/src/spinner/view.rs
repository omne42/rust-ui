use crate::{
    CircularProgress,
    spinner::{
        SpinnerSize,
        logic::{self, SpinnerStateInput},
    },
};
use leptos::prelude::*;

#[component]
pub fn Spinner(
    #[prop(optional)] size: SpinnerSize,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label);

    let state = logic::resolve_state(SpinnerStateInput {
        size,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);

    view! {
        <span
            class=class
            data-slot="spinner"
            data-size=state.size_attr
            data-state="indeterminate"
            data-custom-aria-label=state.has_custom_aria_label.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            <CircularProgress aria_label=aria_label class_name="ui-spinner__progress" />
        </span>
    }
}
