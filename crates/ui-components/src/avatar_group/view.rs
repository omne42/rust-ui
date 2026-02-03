use crate::avatar::{Avatar, AvatarSize};
use leptos::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AvatarGroupItem {
    pub name: Option<String>,
    pub src: Option<String>,
    pub alt: Option<String>,
}

#[component]
pub fn AvatarGroup(
    items: Vec<AvatarGroupItem>,
    #[prop(optional)] max: Option<usize>,
    #[prop(optional)] size: AvatarSize,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let max = max.unwrap_or(4);
    let total = items.len();
    let visible = total.min(max);
    let overflow = total.saturating_sub(visible);

    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Avatar group".to_string());

    let base_class = "ui-avatar-group".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let overflow_label = format!("+{overflow}");

    view! {
        <div class=class data-slot="avatar-group" role="group" aria-label=aria_label>
            {items
                .into_iter()
                .take(visible)
                .map(|item| {
                    view! {
                        <Avatar
                            class_name="ui-avatar-group__item"
                            size=size
                            name=item.name.unwrap_or_default()
                            src=item.src.unwrap_or_default()
                            alt=item.alt.unwrap_or_default()
                        />
                    }
                })
                .collect_view()}

            <Show when=move || overflow != 0>
                <span class="ui-avatar-group__overflow" data-slot="avatar-group-overflow">
                    {overflow_label.clone()}
                </span>
            </Show>
        </div>
    }
}
