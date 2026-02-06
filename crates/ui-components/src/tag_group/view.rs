use crate::{
    chip::{Chip, ChipSize, ChipVariant},
    tag_group::{
        Tag,
        logic::{merge_describedby_ids, normalize_optional_text},
    },
};
use leptos::prelude::*;
use ui_headless::OnPress;

#[component]
pub fn TagGroup(
    tags: ReadSignal<Vec<Tag>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_remove: Option<Callback<Tag>>,
    #[prop(optional)] variant: ChipVariant,
    #[prop(optional)] size: ChipSize,
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
            aria-invalid=move || invalid.get().then_some("true")
            aria-required=move || required.get().then_some("true")
            data-invalid=move || invalid.get().then_some("true")
            data-required=move || required.get().then_some("true")
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
                        .map(|tag| {
                            let is_disabled = disabled || tag.disabled;
                            let dismiss = on_remove.get_value().map(|on_remove| {
                                let tag = tag.clone();
                                let on_press: OnPress =
                                    Callback::new(move |_| on_remove.run(tag.clone()));
                                on_press
                            });

                            let chip: AnyView = match dismiss {
                                Some(on_dismiss) => view! {
                                    <Chip
                                        disabled=is_disabled
                                        variant=variant
                                        size=size
                                        on_dismiss=on_dismiss
                                    >
                                        {tag.label}
                                    </Chip>
                                }
                                .into_any(),
                                None => view! {
                                    <Chip
                                        disabled=is_disabled
                                        variant=variant
                                        size=size
                                    >
                                        {tag.label}
                                    </Chip>
                                }
                                .into_any(),
                            };

                            view! { <li class="ui-tag-group__item" data-slot="tag-group-item">{chip}</li> }
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
