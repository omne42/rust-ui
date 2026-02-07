use crate::avatar::{Avatar, AvatarSize};
use crate::avatar_group::logic::{self, AvatarGroupStateInput};
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
    let max_visible = logic::normalize_max_visible(max);
    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(AvatarGroupStateInput {
        total_count: items.len(),
        max_visible,
        size,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);
    let visible_items = items
        .into_iter()
        .take(state.visible_count)
        .collect::<Vec<_>>();
    let overflow_label = format!("+{}", state.overflow_count);
    let overflow_aria_label = format!("{} more collaborators", state.overflow_count);

    view! {
        <div
            class=class
            data-slot="avatar-group"
            data-size=state.size_attr
            data-state=if state.has_overflow {
                "overflow"
            } else if state.is_empty {
                "empty"
            } else {
                "stable"
            }
            data-empty=state.is_empty.then_some("true")
            data-has-items=state.has_items.then_some("true")
            data-has-overflow=state.has_overflow.then_some("true")
            data-count=state.total_count.to_string()
            data-visible-count=state.visible_count.to_string()
            data-overflow-count=state.overflow_count.to_string()
            data-max-visible=state.max_visible.to_string()
            data-custom-aria-label=state.has_custom_aria_label.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            role="group"
            aria-label=aria_label
        >
            {visible_items
                .into_iter()
                .enumerate()
                .map(|(index, item)| {
                    let has_name = item.name.is_some();
                    let has_src = item.src.is_some();
                    let has_alt = item.alt.is_some();
                    let name = item.name.unwrap_or_default();
                    let src = item.src.unwrap_or_default();
                    let alt = item.alt.unwrap_or_default();

                    view! {
                        <span
                            class="ui-avatar-group__item"
                            data-slot="avatar-group-item"
                            data-index=index
                            data-has-name=has_name.then_some("true")
                            data-has-src=has_src.then_some("true")
                            data-has-alt=has_alt.then_some("true")
                        >
                            <Avatar
                                class_name="ui-avatar-group__avatar"
                                size=state.size
                                name=name
                                src=src
                                alt=alt
                            />
                        </span>
                    }
                })
                .collect_view()}

            <Show when=move || state.has_overflow>
                <span
                    class="ui-avatar-group__overflow"
                    data-slot="avatar-group-overflow"
                    data-count=state.overflow_count.to_string()
                    aria-label=overflow_aria_label.clone()
                >
                    {overflow_label.clone()}
                </span>
            </Show>
        </div>
    }
}
