use crate::logic::{self, AvatarSize};
use leptos::prelude::*;
use ui_avatar::Avatar;
use ui_headless::A11yDirection;
use ui_headless::i18n;
use ui_headless::i18n::CommonStrings;
use ui_headless::labeled_group_attrs;

const OVERFLOW_VISIBLE_LABEL_PREFIX: &str = "+";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AvatarGroupItem {
    pub name: Option<String>,
    pub src: Option<String>,
    pub alt: Option<String>,
}

fn render_avatar_group_overflow_label(overflow_count: usize) -> String {
    format!("{OVERFLOW_VISIBLE_LABEL_PREFIX}{overflow_count}")
}

fn render_avatar_group_item(
    index: usize,
    item: AvatarGroupItem,
    size: AvatarSize,
) -> impl IntoView {
    let fields = logic::normalize_avatar_group_item_fields(item.name, item.src, item.alt);

    view! {
        <span
            class="ui-avatar-group__item"
            data-slot="avatar-group-item"
            data-index=index
            data-has-name=fields.has_name.then_some("true")
            data-has-src=fields.has_src.then_some("true")
            data-has-alt=fields.has_alt.then_some("true")
        >
            <Avatar
                class_name="ui-avatar-group__avatar"
                size=size
                name=fields.name
                src=fields.src
                alt=fields.alt
            />
        </span>
    }
}

#[component]
pub fn AvatarGroup(
    items: Vec<AvatarGroupItem>,
    #[prop(optional)] max: Option<usize>,
    #[prop(optional)] size: AvatarSize,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let normalized = logic::normalize_avatar_group_input(
        max,
        aria_label,
        class_name,
        lang,
        common.avatar_group_aria_label.as_ref(),
    );
    let group_a11y = labeled_group_attrs(normalized.aria_label, normalized.lang.clone(), dir);

    let state = logic::resolve_avatar_group_render_state(logic::AvatarGroupStateInput {
        total_count: items.len(),
        max_visible: normalized.max_visible,
        size,
        has_custom_aria_label: normalized.has_custom_aria_label,
        has_custom_class_name: normalized.has_custom_class_name,
    });
    let agent_contract = logic::resolve_avatar_group_agent_contract(state);

    let class = logic::compose_avatar_group_class_name(normalized.class_name, state);
    let visible_items = items
        .into_iter()
        .take(state.visible_count)
        .collect::<Vec<_>>();
    let overflow_label = render_avatar_group_overflow_label(state.overflow_count);
    let overflow_aria_label = format!(
        "{} {}",
        state.overflow_count,
        common.avatar_group_overflow_aria_label_suffix.as_ref()
    );
    view! {
        <div
            class=class
            data-slot="avatar-group"
            data-size=state.size_attr
            data-state=state.visual_state.as_str()
            data-empty=state.visual_state.is_empty().then_some("true")
            data-has-items=state.has_items().then_some("true")
            data-has-overflow=state.visual_state.has_overflow().then_some("true")
            data-count=state.total_count.to_string()
            data-visible-count=state.visible_count.to_string()
            data-overflow-count=state.overflow_count.to_string()
            data-max-visible=state.max_visible.to_string()
            data-custom-aria-label=state.aria_label_source.is_custom().then_some("true")
            data-aria-label-source=state.aria_label_source.as_str()
            data-custom-class=state.class_source.is_custom().then_some("true")
            data-class-source=state.class_source.as_str()
            data-ui-schema=agent_contract.schema
            data-ui-intent=agent_contract.intent.as_str()
            data-ui-action=agent_contract.action.as_str()
            data-ui-state=agent_contract.state.as_str()
            data-ui-source=agent_contract.source.as_str()
            data-ui-stream-support=agent_contract.stream_support.as_str()
            data-ui-stream-fallback=agent_contract.stream_fallback.as_str()
            data-ui-output-status=agent_contract.output_status.as_str()
            role=group_a11y.role
            aria-label=group_a11y.aria_label
            lang=group_a11y.lang.clone()
            dir=group_a11y.dir
        >
            {visible_items
                .into_iter()
                .enumerate()
                .map(|(index, item)| render_avatar_group_item(index, item, state.size))
                .collect_view()}

            <Show when=move || state.visual_state.has_overflow()>
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
