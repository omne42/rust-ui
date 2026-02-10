use crate::tags::{TagsStateInput, logic};
use crate::{Tag, TagGroup, TagSize, TagVariant};
use leptos::prelude::*;

#[component]
pub fn Tags(
    tags: ReadSignal<Vec<Tag>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_remove: Option<Callback<Tag>>,
    #[prop(optional)] variant: TagVariant,
    #[prop(optional)] size: TagSize,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_optional_text(id_base);
    let has_custom_id_base = id_base.is_some();
    let id_base_for_inner = id_base.clone().unwrap_or_default();

    let label = logic::normalize_optional_text(label);
    let has_custom_label = label.is_some();
    let label_for_inner = label.clone().unwrap_or_default();

    let description = logic::normalize_optional_text(description);
    let has_custom_description = description.is_some();
    let description_for_inner = description.clone().unwrap_or_default();

    let error = logic::normalize_optional_text(error);
    let has_custom_error = error.is_some();
    let error_for_inner = error.clone().unwrap_or_default();

    let aria_label = logic::normalize_optional_text(aria_label);
    let has_custom_aria_label = aria_label.is_some();
    let aria_label_for_inner = aria_label.clone().unwrap_or_default();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name_for_wrapper = class_name.clone();
    let class_name_for_inner = class_name.clone().unwrap_or_default();

    let has_remove_handler = on_remove.is_some();
    let has_custom_variant = variant != TagVariant::default();
    let has_custom_size = size != TagSize::default();

    let normalized_aria_describedby =
        Signal::derive(move || logic::normalize_optional_text(aria_describedby.get()));

    let state = Signal::derive(move || {
        let tags = tags.get();
        let (has_tags, has_disabled_tags, has_removable_tags) =
            logic::derive_tag_flags(&tags, disabled, has_remove_handler);

        logic::resolve_state(TagsStateInput {
            disabled,
            has_tags,
            has_disabled_tags,
            has_removable_tags,
            is_invalid: invalid.get(),
            is_required: required.get(),
            has_remove_handler,
            has_custom_id_base,
            has_custom_label,
            has_custom_description,
            has_custom_error,
            has_custom_aria_describedby: normalized_aria_describedby.get().is_some(),
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_variant,
            has_custom_size,
        })
    });

    let class = Signal::derive(move || {
        logic::compose_class_name(class_name_for_wrapper.clone(), state.get())
    });

    if let Some(on_remove) = on_remove {
        view! {
            <div
                class=move || class.get()
                data-slot="tags"
                data-state=move || state.get().state_attr
                data-content=move || state.get().content_attr
                data-removal=move || state.get().removal_attr
                data-constraint=move || state.get().constraint_attr
                data-id-source=move || state.get().id_source_attr
                data-label-source=move || state.get().label_source_attr
                data-description-source=move || state.get().description_source_attr
                data-error-source=move || state.get().error_source_attr
                data-describedby-source=move || state.get().describedby_source_attr
                data-aria-source=move || state.get().aria_source_attr
                data-class-source=move || state.get().class_source_attr
                data-variant-source=move || state.get().variant_source_attr
                data-size-source=move || state.get().size_source_attr
                data-handler-source=move || state.get().handler_source_attr
                data-count=move || tags.get().len().to_string()
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-invalid=move || state.get().is_invalid.then_some("true")
                data-required=move || state.get().is_required.then_some("true")
                data-has-disabled-tags=move || state.get().has_disabled_tags.then_some("true")
                data-has-removable-tags=move || state.get().has_removable_tags.then_some("true")
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-custom-handler=move || state.get().has_remove_handler.then_some("true")
                data-custom-variant=move || state.get().has_custom_variant.then_some("true")
                data-custom-size=move || state.get().has_custom_size.then_some("true")
            >
                <TagGroup
                    tags=tags
                    disabled=disabled
                    on_remove=on_remove
                    variant=variant
                    size=size
                    id_base=id_base_for_inner
                    label=label_for_inner
                    description=description_for_inner
                    error=error_for_inner
                    invalid=invalid
                    required=required
                    aria_describedby=normalized_aria_describedby
                    aria_label=aria_label_for_inner
                    class_name=class_name_for_inner
                />
            </div>
        }
        .into_any()
    } else {
        view! {
            <div
                class=move || class.get()
                data-slot="tags"
                data-state=move || state.get().state_attr
                data-content=move || state.get().content_attr
                data-removal=move || state.get().removal_attr
                data-constraint=move || state.get().constraint_attr
                data-id-source=move || state.get().id_source_attr
                data-label-source=move || state.get().label_source_attr
                data-description-source=move || state.get().description_source_attr
                data-error-source=move || state.get().error_source_attr
                data-describedby-source=move || state.get().describedby_source_attr
                data-aria-source=move || state.get().aria_source_attr
                data-class-source=move || state.get().class_source_attr
                data-variant-source=move || state.get().variant_source_attr
                data-size-source=move || state.get().size_source_attr
                data-handler-source=move || state.get().handler_source_attr
                data-count=move || tags.get().len().to_string()
                data-disabled=move || state.get().is_disabled.then_some("true")
                data-invalid=move || state.get().is_invalid.then_some("true")
                data-required=move || state.get().is_required.then_some("true")
                data-has-disabled-tags=move || state.get().has_disabled_tags.then_some("true")
                data-has-removable-tags=move || state.get().has_removable_tags.then_some("true")
                data-custom-class=move || state.get().has_custom_class_name.then_some("true")
                data-custom-handler=move || state.get().has_remove_handler.then_some("true")
                data-custom-variant=move || state.get().has_custom_variant.then_some("true")
                data-custom-size=move || state.get().has_custom_size.then_some("true")
            >
                <TagGroup
                    tags=tags
                    disabled=disabled
                    variant=variant
                    size=size
                    id_base=id_base_for_inner
                    label=label_for_inner
                    description=description_for_inner
                    error=error_for_inner
                    invalid=invalid
                    required=required
                    aria_describedby=normalized_aria_describedby
                    aria_label=aria_label_for_inner
                    class_name=class_name_for_inner
                />
            </div>
        }
        .into_any()
    }
}
