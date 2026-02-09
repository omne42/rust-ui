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
    let id_base = id_base.unwrap_or_default();
    let label = label.unwrap_or_default();
    let description = description.unwrap_or_default();
    let error = error.unwrap_or_default();
    let aria_label = aria_label.unwrap_or_default();
    let class_name = class_name.unwrap_or_default();
    let on_remove = StoredValue::new(on_remove);

    view! {
        {move || {
            if let Some(on_remove) = on_remove.get_value() {
                view! {
                    <TagGroup
                        tags=tags
                        disabled=disabled
                        on_remove=on_remove
                        variant=variant
                        size=size
                        id_base=id_base.clone()
                        label=label.clone()
                        description=description.clone()
                        error=error.clone()
                        invalid=invalid
                        required=required
                        aria_describedby=aria_describedby
                        aria_label=aria_label.clone()
                        class_name=class_name.clone()
                    />
                }
                    .into_any()
            } else {
                view! {
                    <TagGroup
                        tags=tags
                        disabled=disabled
                        variant=variant
                        size=size
                        id_base=id_base.clone()
                        label=label.clone()
                        description=description.clone()
                        error=error.clone()
                        invalid=invalid
                        required=required
                        aria_describedby=aria_describedby
                        aria_label=aria_label.clone()
                        class_name=class_name.clone()
                    />
                }
                    .into_any()
            }
        }}
    }
}
