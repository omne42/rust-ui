use crate::aspect_ratio::{
    AspectRatioStateInput,
    logic::{self, AspectRatioPreset, AspectRatioRadius},
};
use leptos::prelude::*;

#[component]
pub fn AspectRatio(
    #[prop(optional)] ratio: AspectRatioPreset,
    #[prop(optional)] radius: AspectRatioRadius,
    #[prop(optional)] bordered: bool,
    #[prop(optional)] fill: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(AspectRatioStateInput {
            ratio,
            radius,
            bordered,
            fill,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            class=move || class.get()
            data-slot="aspect-ratio"
            data-ratio=move || state.get().ratio_attr
            data-radius=move || state.get().radius_attr
            data-bordered=move || state.get().is_bordered.then_some("true")
            data-fill=move || state.get().is_fill.then_some("true")
            data-state=move || state.get().data_state_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            aria-label=aria_label
        >
            <div class="ui-aspect-ratio__inner" data-slot="aspect-ratio-inner">
                {children()}
            </div>
        </div>
    }
}
