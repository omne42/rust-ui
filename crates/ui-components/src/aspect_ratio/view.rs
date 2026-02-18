use crate::aspect_ratio::logic::{
    self, AspectRatioPreset, AspectRatioRadius, AspectRatioStateInput,
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, AspectRatioOptions, use_aspect_ratio};

#[component]
pub fn AspectRatio(
    #[prop(optional)] ratio: AspectRatioPreset,
    #[prop(optional)] radius: AspectRatioRadius,
    #[prop(optional)] bordered: bool,
    #[prop(optional)] fill: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
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
    let semantics = Memo::new(move |_| {
        use_aspect_ratio(AspectRatioOptions {
            state: state.get(),
            aria_label: aria_label.clone(),
            lang: lang.clone(),
            dir,
        })
    });

    view! {
        <div
            class=move || class.get()
            data-slot="aspect-ratio"
            data-ratio=move || semantics.get().attrs.data_ratio
            data-radius=move || semantics.get().attrs.data_radius
            data-bordered=move || semantics.get().attrs.data_bordered
            data-fill=move || semantics.get().attrs.data_fill
            data-state=move || semantics.get().attrs.data_state
            data-aria-source=move || semantics.get().attrs.data_aria_source
            data-custom-class=move || semantics.get().attrs.data_custom_class
            data-class-source=move || semantics.get().attrs.data_class_source
            role=move || semantics.get().attrs.role
            aria-label=move || semantics.get().attrs.aria_label
            lang=move || semantics.get().attrs.lang
            dir=move || semantics.get().attrs.dir
        >
            <div class="ui-aspect-ratio__inner" data-slot="aspect-ratio-inner">
                {children()}
            </div>
        </div>
    }
}
