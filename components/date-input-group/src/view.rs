use crate::{
    DateInputGroupStateInput,
    logic::{self, DateInputGroupVariant},
};
use leptos::{children::ViewFn, prelude::*};

#[component]
pub fn DateInputGroup(
    #[prop(optional)] full_width: bool,
    #[prop(optional)] variant: DateInputGroupVariant,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] invalid: bool,
    #[prop(optional)] segmented: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] prefix: Option<ViewFn>,
    #[prop(optional, into)] suffix: Option<ViewFn>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let prefix = prefix.map(StoredValue::new);
    let suffix = suffix.map(StoredValue::new);

    let state = Memo::new(move |_| {
        logic::resolve_state(DateInputGroupStateInput {
            variant,
            full_width,
            disabled,
            invalid,
            segmented,
            has_prefix: prefix.is_some(),
            has_suffix: suffix.is_some(),
            has_custom_aria_label,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            class=move || class.get()
            data-slot="date-input-group"
            data-variant=move || state.get().variant_attr
            data-width=move || state.get().width_attr
            data-state=move || state.get().data_state_attr
            data-full-width=move || state.get().is_full_width.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-segmented=move || state.get().is_segmented.then_some("true")
            data-has-prefix=move || state.get().has_prefix.then_some("true")
            data-has-suffix=move || state.get().has_suffix.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="group"
            aria-label=aria_label
            aria-disabled=disabled.then_some("true")
        >
            {prefix.map(|prefix| {
                view! {
                    <div class="ui-date-input-group__prefix" data-slot="date-input-group-prefix">
                        {prefix.get_value().run()}
                    </div>
                }
            })}

            <div class="ui-date-input-group__input" data-slot="date-input-group-input">
                <div class="ui-date-input-group__segment" data-slot="date-input-group-segment">
                    {children()}
                </div>
            </div>

            {suffix.map(|suffix| {
                view! {
                    <div class="ui-date-input-group__suffix" data-slot="date-input-group-suffix">
                        {suffix.get_value().run()}
                    </div>
                }
            })}
        </div>
    }
}
