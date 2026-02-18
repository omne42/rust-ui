use super::{
    Tag,
    logic::{
        TagGroupItemStateInput, merge_describedby_ids, normalize_group_input, resolve_item_state,
        resolve_state,
    },
};
use crate::ai_space::use_ai_space_state;
use crate::tag::{Tag as TagPrimitive, TagSize, TagVariant};
use leptos::prelude::*;
use ui_headless::{A11yDirection, OnPress, locale_attrs};

const TAG_GROUP_LABEL_CLASS: &str = "ui-tag-group__label";
const TAG_GROUP_LABEL_SLOT: &str = "tag-group-label";
const TAG_GROUP_LIST_CLASS: &str = "ui-tag-group__list";
const TAG_GROUP_LIST_SLOT: &str = "tag-group-list";
const TAG_GROUP_ITEM_CLASS: &str = "ui-tag-group__item";
const TAG_GROUP_ITEM_SLOT: &str = "tag-group-item";
const TAG_GROUP_DESCRIPTION_CLASS: &str = "ui-tag-group__description";
const TAG_GROUP_DESCRIPTION_SLOT: &str = "tag-group-description";
const TAG_GROUP_ERROR_CLASS: &str = "ui-tag-group__error";
const TAG_GROUP_ERROR_SLOT: &str = "tag-group-error";

fn render_group_label(label: Option<String>, label_id: String) -> AnyView {
    match label {
        Some(label) => view! {
            <div class=TAG_GROUP_LABEL_CLASS id=label_id data-slot=TAG_GROUP_LABEL_SLOT>
                {label}
            </div>
        }
        .into_any(),
        None => ().into_any(),
    }
}

fn render_group_description(description: Option<String>, description_id: String) -> AnyView {
    match description {
        Some(description) => view! {
            <div
                class=TAG_GROUP_DESCRIPTION_CLASS
                id=description_id
                data-slot=TAG_GROUP_DESCRIPTION_SLOT
            >
                {description}
            </div>
        }
        .into_any(),
        None => ().into_any(),
    }
}

fn render_group_error(error: Option<String>, error_id: String, invalid: Signal<bool>) -> AnyView {
    match error {
        Some(error) => {
            let error = StoredValue::new(error);
            let error_id = StoredValue::new(error_id);
            view! {
                <Show when=move || invalid.get()>
                    <div
                        class=TAG_GROUP_ERROR_CLASS
                        id=move || error_id.get_value()
                        data-slot=TAG_GROUP_ERROR_SLOT
                    >
                        {move || error.get_value()}
                    </div>
                </Show>
            }
            .into_any()
        }
        None => ().into_any(),
    }
}

#[derive(Clone, Copy)]
struct TagGroupItemRenderCtx {
    disabled: bool,
    has_remove_callback: bool,
    agent_source: RwSignal<super::logic::TagGroupAgentSource>,
    on_remove: StoredValue<Option<Callback<Tag>>>,
    variant: TagVariant,
    size: TagSize,
}

fn render_tag_node(
    label: String,
    variant: TagVariant,
    size: TagSize,
    is_disabled: bool,
    dismiss: Option<OnPress>,
) -> AnyView {
    match dismiss {
        Some(on_remove) => view! {
            <TagPrimitive
                disabled=is_disabled
                variant=variant
                size=size
                removable=true
                on_remove=on_remove
            >
                {label}
            </TagPrimitive>
        }
        .into_any(),
        None => view! {
            <TagPrimitive disabled=is_disabled variant=variant size=size>
                {label}
            </TagPrimitive>
        }
        .into_any(),
    }
}

fn render_tag_group_item(index: usize, tag: Tag, ctx: TagGroupItemRenderCtx) -> impl IntoView {
    let tag_id_for_attr = tag.id.clone();
    let tag_for_remove = tag.clone();
    let item_state = resolve_item_state(TagGroupItemStateInput {
        group_disabled: ctx.disabled,
        supports_removal: ctx.has_remove_callback,
        tag_disabled: tag.disabled,
    });
    let dismiss = if item_state.is_removable {
        ctx.on_remove.get_value().map(|on_remove| {
            let on_press: OnPress = Callback::new(move |_| {
                ctx.agent_source
                    .set(super::logic::TagGroupAgentSource::RemovePointer);
                on_remove.run(tag_for_remove.clone());
            });
            on_press
        })
    } else {
        None
    };
    let tag_view = render_tag_node(
        tag.label,
        ctx.variant,
        ctx.size,
        item_state.is_disabled,
        dismiss,
    );

    view! {
        <li
            class=TAG_GROUP_ITEM_CLASS
            data-slot=TAG_GROUP_ITEM_SLOT
            data-index=index
            data-tag-id=tag_id_for_attr
            data-disabled=item_state.is_disabled.then_some("true")
            data-removable=item_state.is_removable.then_some("true")
            data-disabled-source=item_state.disabled_source_attr
            data-removable-source=item_state.removable_source_attr
        >
            {tag_view}
        </li>
    }
}

fn render_tag_group_items(tags: Vec<Tag>, ctx: TagGroupItemRenderCtx) -> impl IntoView {
    tags.into_iter()
        .enumerate()
        .map(|(index, tag)| render_tag_group_item(index, tag, ctx))
        .collect_view()
}

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
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let normalized = normalize_group_input(
        id_base,
        label,
        description,
        error,
        aria_label,
        class_name,
        lang,
    );
    let id_base = normalized.id_base;
    let label = normalized.label;
    let description = normalized.description;
    let error = normalized.error;
    let aria_label = normalized.aria_label;
    let id_base_source = normalized.id_base_source.as_attr();
    let aria_label_source = normalized.aria_label_source.as_attr();
    let class_name_source = normalized.class_name_source.as_attr();
    let lang_source = normalized.lang_source.as_attr();
    let locale = locale_attrs(normalized.lang, dir);

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

    let class = normalized.class_name;

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
    let agent_source = RwSignal::new(super::logic::TagGroupAgentSource::Init);
    let agent_contract = Signal::derive(move || {
        super::logic::resolve_agent_contract(state.get(), agent_source.get(), has_remove_callback)
    });
    let ai_space_state = StoredValue::new(use_ai_space_state());

    let label = StoredValue::new(label);
    let description = StoredValue::new(description);
    let error = StoredValue::new(error);
    let aria_labelledby = StoredValue::new(aria_labelledby);
    let aria_label = StoredValue::new(aria_label);
    let on_remove = StoredValue::new(on_remove);
    let label_view = render_group_label(label.get_value(), label_id.clone());
    let description_view =
        render_group_description(description.get_value(), description_id.clone());
    let error_view = render_group_error(error.get_value(), error_id.clone(), invalid);

    view! {
        <div
            id=id_base
            class=class
            role="group"
            lang=locale.lang.clone()
            dir=locale.dir
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
            data-id-base-source=id_base_source
            data-aria-label-source=aria_label_source
            data-class-source=class_name_source
            data-lang-source=lang_source
            data-slot="tag-group"
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode=move || {
                ai_space_state
                    .get_value()
                    .map(|state| state.get().mode.as_str())
                    .unwrap_or("snapshot")
            }
            data-ui-output-status=move || {
                ai_space_state
                    .get_value()
                    .map(|state| state.get().output_status.as_str())
                    .unwrap_or(agent_contract.get().output_status.as_str())
            }
            data-ui-capability-remove=move || {
                agent_contract.get().capabilities.can_remove.then_some("true")
            }
            data-ui-capability-validate=move || {
                agent_contract.get().capabilities.can_validate.then_some("true")
            }
            data-ui-capability-disable=move || {
                agent_contract.get().capabilities.can_disable.then_some("true")
            }
        >
            {label_view}

            <ul class=TAG_GROUP_LIST_CLASS data-slot=TAG_GROUP_LIST_SLOT>
                {move || {
                    let ctx = TagGroupItemRenderCtx {
                        disabled,
                        has_remove_callback,
                        agent_source,
                        on_remove,
                        variant,
                        size,
                    };
                    render_tag_group_items(
                        tags.get(),
                        ctx,
                    )
                }}
            </ul>

            {description_view}
            {error_view}
        </div>
    }
}
