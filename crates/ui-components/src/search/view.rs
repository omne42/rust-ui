use crate::search::{
    SearchStateInput,
    logic::{self},
};
use crate::{SearchField, SearchFieldMotion};
use leptos::{html, prelude::*};

#[component]
pub fn Search(
    id: String,
    label: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] read_only: bool,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] on_submit: Option<Callback<String>>,
    #[prop(optional)] on_clear: Option<Callback<()>>,
    #[prop(optional)] motion: SearchFieldMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let (label, has_custom_label) = logic::resolve_label(label);

    let description = logic::normalize_optional_text(description);
    let has_custom_description = description.is_some();
    let description_for_inner = description.clone().unwrap_or_default();

    let error = logic::normalize_optional_text(error);
    let has_custom_error = error.is_some();
    let error_for_inner = error.clone().unwrap_or_default();

    let placeholder = logic::normalize_optional_text(placeholder);
    let has_custom_placeholder = placeholder.is_some();
    let placeholder_for_inner = placeholder.clone().unwrap_or_default();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name_for_inner = class_name.clone().unwrap_or_default();

    let has_custom_submit_handler = on_submit.is_some();
    let has_custom_clear_handler = on_clear.is_some();
    let on_submit = on_submit.unwrap_or_else(|| Callback::new(|_: String| {}));
    let on_clear = on_clear.unwrap_or_else(|| Callback::new(|()| {}));

    let state = Signal::derive(move || {
        logic::resolve_state(SearchStateInput {
            disabled,
            read_only,
            required: required.get(),
            invalid: invalid.get(),
            has_value: !value.get().is_empty(),
            has_custom_label,
            has_custom_description,
            has_custom_error,
            has_custom_placeholder,
            has_custom_class_name,
            has_custom_motion: motion != SearchFieldMotion::default(),
            has_custom_submit_handler,
            has_custom_clear_handler,
        })
    });

    let class = Signal::derive(move || logic::compose_class_name(class_name.clone(), state.get()));

    view! {
        <div
            class=move || class.get()
            data-slot="search"
            data-state=move || state.get().state_attr
            data-value=move || state.get().value_attr
            data-requirement=move || state.get().requirement_attr
            data-label-source=move || state.get().label_source_attr
            data-description-source=move || state.get().description_source_attr
            data-error-source=move || state.get().error_source_attr
            data-placeholder-source=move || state.get().placeholder_source_attr
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-submit-handler-source=move || state.get().submit_handler_source_attr
            data-clear-handler-source=move || state.get().clear_handler_source_attr
            data-custom-label=move || state.get().has_custom_label.then_some("true")
            data-custom-description=move || state.get().has_custom_description.then_some("true")
            data-custom-error=move || state.get().has_custom_error.then_some("true")
            data-custom-placeholder=move || state.get().has_custom_placeholder.then_some("true")
            data-custom-submit-handler=move || state.get().has_custom_submit_handler.then_some("true")
            data-custom-clear-handler=move || state.get().has_custom_clear_handler.then_some("true")
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
        >
            <SearchField
                id=id
                label=label
                value=value
                set_value=set_value
                disabled=disabled
                read_only=read_only
                required=required
                invalid=invalid
                aria_describedby=aria_describedby
                description=description_for_inner
                error=error_for_inner
                placeholder=placeholder_for_inner
                on_submit=on_submit
                on_clear=on_clear
                motion=motion
                class_name=class_name_for_inner
                node_ref=node_ref
            />
        </div>
    }
}
