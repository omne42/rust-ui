use super::{
    Tag,
    logic::{merge_describedby_ids, normalize_optional_text, resolve_state},
};
use crate::tag::{Tag as TagPrimitive, TagSize, TagVariant};
use leptos::prelude::*;
use ui_headless::OnPress;

#[component]
pub fn TagGroup(
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
    let id_base = normalize_optional_text(id_base).unwrap_or_else(|| "tag-group".to_string());
    let label = normalize_optional_text(label);
    let description = normalize_optional_text(description);
    let error = normalize_optional_text(error);
    let aria_label = normalize_optional_text(aria_label).unwrap_or_else(|| "Tags".to_string());

    let label_id = format!("{id_base}-label");
    let description_id = format!("{id_base}-description");
    let error_id = format!("{id_base}-error");

    let aria_labelledby = label.as_ref().map(|_| label_id.clone());
    let description_id_for_aria = description.as_ref().map(|_| description_id.clone());
    let error_id_for_aria = error.as_ref().map(|_| error_id.clone());

    let description_id_for_aria = StoredValue::new(description_id_for_aria);
    let error_id_for_aria = StoredValue::new(error_id_for_aria);

    let group_aria_describedby = Memo::new(move |_| {
        let description_id = description_id_for_aria.get_value();
        let error_id = if invalid.get() {
            error_id_for_aria.get_value()
        } else {
            None
        };

        merge_describedby_ids(
            aria_describedby.get(),
            description_id.as_deref(),
            error_id.as_deref(),
        )
    });

    let base_class = "ui-tag-group".to_string();
    let class = class_name
        .and_then(|value| {
            let trimmed = value.trim();
            (!trimmed.is_empty()).then(|| format!("{base_class} {trimmed}"))
        })
        .unwrap_or(base_class);

    let has_remove_callback = on_remove.is_some();
    let state = Memo::new(move |_| {
        let tags = tags.get();
        resolve_state(
            &tags,
            disabled,
            has_remove_callback,
            invalid.get(),
            required.get(),
        )
    });

    let label = StoredValue::new(label);
    let description = StoredValue::new(description);
    let error = StoredValue::new(error);
    let aria_labelledby = StoredValue::new(aria_labelledby);
    let aria_label = StoredValue::new(aria_label);
    let on_remove = StoredValue::new(on_remove);

    view! {
        <div
            id=id_base
            class=class
            role="group"
            aria-label=move || {
                if aria_labelledby.get_value().is_some() {
                    None
                } else {
                    Some(aria_label.get_value())
                }
            }
            aria-labelledby=move || aria_labelledby.get_value()
            aria-describedby=move || group_aria_describedby.get()
            aria-invalid=move || state.get().is_invalid.then_some("true")
            aria-required=move || state.get().is_required.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-count=move || state.get().item_count.to_string()
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-disabled-tags=move || state.get().has_disabled_tags.then_some("true")
            data-has-removable-tags=move || state.get().has_removable_tags.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-required=move || state.get().is_required.then_some("true")
            data-slot="tag-group"
        >
            {label.get_value().map(|label| {
                view! {
                    <div class="ui-tag-group__label" id=label_id.clone() data-slot="tag-group-label">
                        {label}
                    </div>
                }
            })}

            <ul class="ui-tag-group__list" data-slot="tag-group-list">
                {move || {
                    tags.get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, tag)| {
                            let tag_id_for_attr = tag.id.clone();
                            let tag_for_remove = tag.clone();
                            let tag_label = tag.label.clone();
                            let is_disabled = disabled || tag.disabled;
                            let is_removable = has_remove_callback && !is_disabled;
                            let dismiss = if is_removable {
                                on_remove.get_value().map(|on_remove| {
                                    let on_press: OnPress =
                                        Callback::new(move |_| on_remove.run(tag_for_remove.clone()));
                                    on_press
                                })
                            } else {
                                None
                            };

                            let tag_view: AnyView = match dismiss {
                                Some(on_remove) => view! {
                                    <TagPrimitive
                                        disabled=is_disabled
                                        variant=variant
                                        size=size
                                        removable=true
                                        on_remove=on_remove
                                    >
                                        {tag_label.clone()}
                                    </TagPrimitive>
                                }
                                .into_any(),
                                None => view! {
                                    <TagPrimitive
                                        disabled=is_disabled
                                        variant=variant
                                        size=size
                                    >
                                        {tag_label}
                                    </TagPrimitive>
                                }
                                .into_any(),
                            };

                            view! {
                                <li
                                    class="ui-tag-group__item"
                                    data-slot="tag-group-item"
                                    data-index=index
                                    data-tag-id=tag_id_for_attr
                                    data-disabled=is_disabled.then_some("true")
                                    data-removable=is_removable.then_some("true")
                                >
                                    {tag_view}
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>

            {description.get_value().map(|description| {
                view! {
                    <div
                        class="ui-tag-group__description"
                        id=description_id.clone()
                        data-slot="tag-group-description"
                    >
                        {description}
                    </div>
                }
            })}

            {error.get_value().map(|error| {
                let error = StoredValue::new(error);
                view! {
                    <Show when=move || invalid.get()>
                        <div
                            class="ui-tag-group__error"
                            id=error_id.clone()
                            data-slot="tag-group-error"
                        >
                            {move || error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </div>
    }
}
