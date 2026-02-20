use super::{FormLabelAlign, FormLabelPosition, logic};
use leptos::prelude::*;

#[component]
pub fn Form(
    children: Children,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_read_only: bool,
    #[prop(optional)] is_required: bool,
    #[prop(optional)] label_position: FormLabelPosition,
    #[prop(optional)] label_align: FormLabelAlign,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    provide_context(logic::FormContextValue {
        disabled: is_disabled,
        read_only: is_read_only,
        required: is_required,
        label_position,
        label_align,
    });

    let view_state = logic::resolve_view_state(label_position, label_align);

    let base_class = "ui-form".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <form
            class=class
            data-slot="form"
            data-disabled=is_disabled.then_some("true")
            data-readonly=is_read_only.then_some("true")
            data-required=is_required.then_some("true")
            data-label-position=view_state.label_position
            data-label-align=view_state.label_align
            aria-disabled=is_disabled.then_some("true")
        >
            {children()}
        </form>
    }
}
