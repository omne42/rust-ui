use crate::tag::{
    TagSize, TagStateInput, TagVariant,
    logic::{self},
};
use leptos::prelude::*;
use ui_headless::OnPress;

#[component]
pub fn Tag(
    #[prop(optional)] variant: TagVariant,
    #[prop(optional)] size: TagSize,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] removable: bool,
    #[prop(optional)] on_remove: Option<OnPress>,
    #[prop(optional, into)] remove_aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let (remove_aria_label, has_custom_remove_aria_label) =
        logic::normalize_remove_aria_label(remove_aria_label);

    let state = logic::resolve_state(TagStateInput {
        variant,
        size,
        disabled,
        removable,
        has_remove_handler: on_remove.is_some(),
        has_custom_remove_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);
    let remove_aria_label = StoredValue::new(remove_aria_label);
    let on_remove = StoredValue::new(on_remove);

    view! {
        <span
            class=class
            data-slot="tag"
            data-variant=state.variant_attr
            data-size=state.size_attr
            data-state=state.state_attr
            data-enabled=state.is_enabled.then_some("true")
            data-disabled=state.is_disabled.then_some("true")
            data-removable=state.is_removable.then_some("true")
            data-static=state.is_static.then_some("true")
            data-has-remove-handler=state.has_remove_handler.then_some("true")
            data-remove-label-source=state.remove_label_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
        >
            <span class="ui-tag__content" data-slot="tag-content">
                {children()}
            </span>

            <Show when=move || state.is_removable>
                <button
                    type="button"
                    class="ui-tag__remove"
                    aria-label=move || remove_aria_label.get_value()
                    data-slot="tag-remove-button"
                    data-disabled=state.is_disabled.then_some("true")
                    data-label-source=state.remove_label_source_attr
                    disabled=state.is_disabled
                    on:click=move |_| {
                        if state.is_enabled
                            && let Some(on_remove) = on_remove.get_value()
                        {
                            on_remove.run(());
                        }
                    }
                >
                    "×"
                </button>
            </Show>
        </span>
    }
}
