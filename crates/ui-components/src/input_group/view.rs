use crate::input_group::logic::{self, InputGroupStateInput};
use leptos::{children::ViewFn, prelude::*};

#[component]
pub fn InputGroup(
    #[prop(default = true)] attached: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] invalid: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] start_content: Option<ViewFn>,
    #[prop(optional, into)] end_content: Option<ViewFn>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let (aria_label, has_custom_label) = logic::normalize_aria_label(aria_label);

    let start_content = start_content.map(StoredValue::new);
    let end_content = end_content.map(StoredValue::new);

    let state = Memo::new(move |_| {
        logic::resolve_state(InputGroupStateInput {
            disabled,
            invalid,
            attached,
            has_start_content: start_content.is_some(),
            has_end_content: end_content.is_some(),
            has_custom_label,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            class=move || class.get()
            data-slot="input-group"
            data-state=move || state.get().phase_attr
            data-attachment=move || state.get().attachment_attr
            data-enabled=move || state.get().is_enabled.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-attached=move || state.get().is_attached.then_some("true")
            data-detached=move || state.get().is_detached.then_some("true")
            data-has-start=move || state.get().has_start_content.then_some("true")
            data-has-end=move || state.get().has_end_content.then_some("true")
            data-label-source=move || state.get().label_source_attr
            data-custom-label=move || state.get().has_custom_label.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="group"
            aria-label=aria_label
        >
            <div class="ui-input-group__control" data-slot="input-group-control">
                {start_content.map(|content| {
                    view! {
                        <span class="ui-input-group__addon ui-input-group__addon--start" data-slot="input-group-start">
                            {content.get_value().run()}
                        </span>
                    }
                })}

                <div class="ui-input-group__field" data-slot="input-group-field">
                    {children()}
                </div>

                {end_content.map(|content| {
                    view! {
                        <span class="ui-input-group__addon ui-input-group__addon--end" data-slot="input-group-end">
                            {content.get_value().run()}
                        </span>
                    }
                })}
            </div>
        </div>
    }
}
