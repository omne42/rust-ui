use crate::{
    chip::{Chip, ChipSize, ChipVariant},
    tag_group::Tag,
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
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Tags".to_string());

    let base_class = "ui-tag-group".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let label = label.filter(|value| !value.trim().is_empty());
    let label = StoredValue::new(label);
    let on_remove = StoredValue::new(on_remove);

    view! {
        <div class=class role="group" aria-label=aria_label data-slot="tag-group">
            {label.get_value().map(|label| {
                view! { <div class="ui-tag-group__label" data-slot="tag-group-label">{label}</div> }
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
        </div>
    }
}
