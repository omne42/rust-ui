use crate::{
    CommandFilterState, CommandGroup, CommandMotion, CommandSlot, CommandSourceAttr,
    FilteredCommandGroup, FilteredCommandItem, logic,
};
use leptos::{ev, html, prelude::*};
use std::sync::Arc;
use ui_headless::{
    A11yDirection, CommandInputKeyDownResult, CommandOptionA11yInput, CommonStrings,
    ListBoxOptions, command_input_attrs, command_option_a11y_attrs, resolve_command_input_key_down,
    use_controllable_state, use_listbox, use_ui_i18n,
};

#[derive(Clone, Copy)]
struct CommandViewSlots {
    input_wrap: CommandSlot,
    input: CommandSlot,
    list: CommandSlot,
    options: CommandSlot,
    group: CommandSlot,
    group_heading: CommandSlot,
    group_items: CommandSlot,
    item: CommandSlot,
    item_label: CommandSlot,
    shortcut: CommandSlot,
    empty: CommandSlot,
    highlight: CommandSlot,
}

const COMMAND_VIEW_SLOTS: CommandViewSlots = CommandViewSlots {
    input_wrap: CommandSlot::InputWrap,
    input: CommandSlot::Input,
    list: CommandSlot::List,
    options: CommandSlot::Options,
    group: CommandSlot::Group,
    group_heading: CommandSlot::GroupHeading,
    group_items: CommandSlot::GroupItems,
    item: CommandSlot::Item,
    item_label: CommandSlot::ItemLabel,
    shortcut: CommandSlot::Shortcut,
    empty: CommandSlot::Empty,
    highlight: CommandSlot::Highlight,
};

fn render_empty_state(empty_slot: CommandSlot, empty_label: StoredValue<String>) -> AnyView {
    view! {
        <div class=empty_slot.base_class() data-slot=empty_slot.as_attr()>
            {empty_label.get_value()}
        </div>
    }
    .into_any()
}

#[derive(Clone)]
struct CommandOptionRenderCtx {
    option_id: Callback<usize, String>,
    active_index: ReadSignal<usize>,
    selected_index: ReadSignal<Option<usize>>,
    on_option_pointer_move: Callback<usize>,
    on_option_click: Callback<usize>,
    slots: CommandViewSlots,
}

#[derive(Clone)]
struct CommandOptionsRenderCtx {
    empty_label: StoredValue<String>,
    options_ref: NodeRef<html::Div>,
    highlight_ref: NodeRef<html::Div>,
    option: CommandOptionRenderCtx,
}

fn render_option_item(
    index: usize,
    item: FilteredCommandItem,
    ctx: &CommandOptionRenderCtx,
) -> AnyView {
    let option_id = ctx.option_id;
    let active_index = ctx.active_index;
    let selected_index = ctx.selected_index;
    let on_option_pointer_move = ctx.on_option_pointer_move;
    let on_option_click = ctx.on_option_click;
    let slots = ctx.slots;
    let id = option_id.run(index);
    let has_shortcut = item.shortcut.is_some();
    let shortcut = StoredValue::new(item.shortcut.unwrap_or_default());
    let item_label = StoredValue::new(item.label);
    let item_disabled = item.disabled;

    let option_attrs = move || {
        command_option_a11y_attrs(CommandOptionA11yInput {
            is_disabled: item_disabled,
            is_selected: selected_index.get() == Some(index),
            is_focused: active_index.get() == index,
        })
    };

    view! {
        <div
            id=id
            role=move || option_attrs().role
            class=slots.item.base_class()
            data-slot=slots.item.as_attr()
            data-index=index
            data-state=move || option_attrs().data_state
            aria-selected=move || option_attrs().aria_selected
            aria-disabled=move || option_attrs().aria_disabled
            data-disabled=move || option_attrs().data_disabled
            data-focused=move || option_attrs().data_focused
            data-selected=move || option_attrs().data_selected
            on:pointermove=move |_| on_option_pointer_move.run(index)
            on:click=move |_| {
                on_option_pointer_move.run(index);
                on_option_click.run(index);
            }
        >
            <span class=slots.item_label.base_class() data-slot=slots.item_label.as_attr()>
                {item_label.get_value()}
            </span>
            <Show when=move || has_shortcut>
                <kbd class=slots.shortcut.base_class() data-slot=slots.shortcut.as_attr()>
                    {shortcut.get_value()}
                </kbd>
            </Show>
        </div>
    }
    .into_any()
}

fn render_group_section(
    group: &FilteredCommandGroup,
    items: &[FilteredCommandItem],
    ctx: &CommandOptionRenderCtx,
) -> AnyView {
    let slots = ctx.slots;
    let heading = group.heading.clone();
    let rendered_items = group
        .item_indices
        .iter()
        .filter_map(|index| {
            items
                .get(*index)
                .cloned()
                .map(|item| render_option_item(*index, item, ctx))
        })
        .collect_view();

    view! {
        <section class=slots.group.base_class() data-slot=slots.group.as_attr()>
            <h3 class=slots.group_heading.base_class() data-slot=slots.group_heading.as_attr()>
                {heading}
            </h3>
            <div class=slots.group_items.base_class() data-slot=slots.group_items.as_attr()>
                {rendered_items}
            </div>
        </section>
    }
    .into_any()
}

fn render_options_content(state: &CommandFilterState, ctx: &CommandOptionsRenderCtx) -> AnyView {
    let slots = ctx.option.slots;
    if state.items.is_empty() {
        return render_empty_state(slots.empty, ctx.empty_label);
    }

    let rendered_groups = state
        .groups
        .iter()
        .map(|group| render_group_section(group, &state.items, &ctx.option))
        .collect_view();

    view! {
        <div
            class=slots.options.base_class()
            node_ref=ctx.options_ref
            data-slot=slots.options.as_attr()
        >
            <div
                class=slots.highlight.base_class()
                node_ref=ctx.highlight_ref
                data-slot=slots.highlight.as_attr()
            ></div>
            {rendered_groups}
        </div>
    }
    .into_any()
}

#[component]
pub fn Command(
    id_base: String,
    #[prop(into)] groups: Arc<[CommandGroup]>,
    #[prop(optional)] query: Option<Signal<String>>,
    #[prop(optional, into)] default_query: Option<String>,
    #[prop(optional)] on_query_change: Option<Callback<String>>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] motion: CommandMotion,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] empty_label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let has_custom_id_base = id_base != logic::DEFAULT_ID_BASE;
    let id_base = StoredValue::new(id_base);

    let i18n = use_ui_i18n();
    let common_strings = i18n.strings::<CommonStrings>();

    let (placeholder, placeholder_source_attr) = logic::resolve_placeholder(
        placeholder,
        Some(common_strings.command_placeholder.as_ref()),
    );
    let has_custom_placeholder = matches!(placeholder_source_attr, CommandSourceAttr::Custom);
    let has_i18n_placeholder = matches!(placeholder_source_attr, CommandSourceAttr::I18n);
    let placeholder = StoredValue::new(placeholder);

    let (empty_label, empty_label_source_attr) = logic::resolve_empty_label(
        empty_label,
        Some(common_strings.command_empty_label.as_ref()),
    );
    let has_custom_empty_label = matches!(empty_label_source_attr, CommandSourceAttr::Custom);
    let has_i18n_empty_label = matches!(empty_label_source_attr, CommandSourceAttr::I18n);
    let empty_label = StoredValue::new(empty_label);

    let (aria_label, aria_label_source_attr) =
        logic::resolve_aria_label(aria_label, Some(common_strings.command_aria_label.as_ref()));
    let has_custom_aria_label = matches!(aria_label_source_attr, CommandSourceAttr::Custom);
    let has_i18n_aria_label = matches!(aria_label_source_attr, CommandSourceAttr::I18n);
    let aria_label = StoredValue::new(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let motion = crate::motion::sanitize_motion(motion);

    let has_custom_disabled = is_disabled != logic::DEFAULT_DISABLED;
    let has_custom_on_action = on_action.is_some();
    let has_custom_motion = crate::motion::is_custom_motion(motion);
    let is_query_controlled = query.is_some();
    let has_custom_default_query = default_query
        .as_ref()
        .is_some_and(|query| !query.trim().is_empty());
    let has_custom_query_change_handler = on_query_change.is_some();

    let groups = StoredValue::new(groups);
    let default_query = logic::resolve_default_query(default_query);
    let query_state = use_controllable_state(query, Some(default_query), on_query_change);
    let query = query_state.value;
    let request_query_change = query_state.request_change;

    let filtered = Memo::new(move |_| {
        let groups = groups.get_value();
        logic::filter_groups(&groups, &query.get())
    });

    let (item_count, set_item_count) = signal(filtered.get_untracked().items.len());
    Effect::new(move |_| {
        set_item_count.set(filtered.with(|state| state.items.len()));
    });

    let (selected_index, set_selected_index) = signal(None::<usize>);
    Effect::new(move |_| {
        let count = item_count.get();
        let current = selected_index.get();
        let next = logic::normalize_selected_index(current, count);

        if next != current {
            set_selected_index.set(next);
        }
    });

    let is_item_disabled = Callback::new(move |index: usize| {
        filtered.with(|state| {
            state
                .items
                .get(index)
                .map(|item| item.disabled)
                .unwrap_or(true)
        })
    });

    let item_text = Callback::new(move |index: usize| {
        filtered.with(|state| {
            state
                .items
                .get(index)
                .map(|item| item.label.clone())
                .unwrap_or_default()
        })
    });

    let on_action_by_index = on_action.map(|on_action| {
        Callback::new(move |index: usize| {
            if let Some(id) =
                filtered.with(|state| state.items.get(index).map(|item| item.id.clone()))
            {
                on_action.run(id);
            }
        })
    });

    let listbox = use_listbox(ListBoxOptions {
        is_disabled,
        should_loop: true,
        id_base: format!("{}-command", id_base.get_value()),
        default_index: 0,
        sync_active_index_to_selected: true,
        item_count,
        selected_index,
        set_selected_index,
        on_action: on_action_by_index,
        is_item_disabled: Some(is_item_disabled),
        item_text: Some(item_text),
    });

    let input_a11y = command_input_attrs(lang, dir);
    let input_lang = input_a11y.lang.clone();
    let input_dir = input_a11y.dir;

    let options_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    crate::motion::attach_motion(
        options_ref,
        highlight_ref,
        listbox.active_index,
        listbox.option_id,
        motion,
    );

    let on_input_key_down = {
        let on_key_down = listbox.handlers.on_key_down;
        move |ev: ev::KeyboardEvent| {
            let key = ev.key();
            let query_value = query.get_untracked();

            match resolve_command_input_key_down(&key, logic::has_query_text(query_value.as_str()))
            {
                CommandInputKeyDownResult::ClearedQuery => {
                    request_query_change.run(String::new());
                    ev.prevent_default();
                }
                CommandInputKeyDownResult::DelegatedToListBox => {
                    if on_key_down.run(key) {
                        ev.prevent_default();
                    }
                }
                CommandInputKeyDownResult::Ignored => {}
            }
        }
    };

    let root_state = Memo::new(move |_| {
        filtered.with(|filtered| {
            let query_value = query.get();

            logic::resolve_root_state(logic::CommandRootStateInput {
                item_count: filtered.items.len(),
                group_count: filtered.groups.len(),
                is_disabled,
                query: query_value.as_str(),
                has_custom_id_base,
                has_custom_placeholder,
                has_i18n_placeholder,
                has_custom_empty_label,
                has_i18n_empty_label,
                has_custom_aria_label,
                has_i18n_aria_label,
                has_custom_class_name,
                has_custom_disabled,
                has_custom_on_action,
                has_custom_motion,
                is_query_controlled,
                has_custom_default_query,
                has_custom_query_change_handler,
            })
        })
    });
    let root_state_for_class = root_state;
    let agent_contract = Memo::new(move |_| {
        let state = root_state.get();
        logic::resolve_agent_contract(logic::CommandAgentContractInput {
            state_attr: state.state_attr,
            query_control_attr: state.query_control_attr,
        })
    });

    let root_class = Memo::new(move |_| {
        logic::compose_class_name(class_name.get_value(), root_state_for_class.get())
    });

    let listbox_id = StoredValue::new(format!("{}-listbox", id_base.get_value()));

    let option_id = listbox.option_id;
    let active_index = listbox.active_index;
    let selected_index = listbox.selected_index;
    let on_option_pointer_move = listbox.handlers.on_option_pointer_move;
    let on_option_click = listbox.handlers.on_option_click;

    let slots = COMMAND_VIEW_SLOTS;
    let options_render_ctx = CommandOptionsRenderCtx {
        empty_label,
        options_ref,
        highlight_ref,
        option: CommandOptionRenderCtx {
            option_id,
            active_index,
            selected_index,
            on_option_pointer_move,
            on_option_click,
            slots,
        },
    };

    view! {
        <section
            class=move || root_class.get()
            data-slot=move || root_state.get().slot_attr
            data-state=move || root_state.get().state_attr.as_attr()
            data-items=move || root_state.get().item_attr.as_attr()
            data-groups=move || root_state.get().group_attr.as_attr()
            data-query=move || root_state.get().query_attr.as_attr()
            data-disabled=move || root_state.get().disabled_attr.as_attr()
            data-empty=move || root_state.get().is_empty.then_some("true")
            data-has-items=move || root_state.get().has_items.then_some("true")
            data-item-count=move || root_state.get().item_count
            data-group-count=move || root_state.get().group_count
            data-has-query=move || root_state.get().has_query.then_some("true")
            data-is-disabled=move || root_state.get().is_disabled.then_some("true")
            data-is-enabled=move || root_state.get().is_enabled.then_some("true")
            data-id-source=move || root_state.get().id_source_attr.as_attr()
            data-placeholder-source=move || root_state.get().placeholder_source_attr.as_attr()
            data-empty-label-source=move || root_state.get().empty_label_source_attr.as_attr()
            data-aria-label-source=move || root_state.get().aria_label_source_attr.as_attr()
            data-class-source=move || root_state.get().class_source_attr.as_attr()
            data-disabled-source=move || root_state.get().disabled_source_attr.as_attr()
            data-action-source=move || root_state.get().action_source_attr.as_attr()
            data-motion-source=move || root_state.get().motion_source_attr.as_attr()
            data-query-control=move || root_state.get().query_control_attr.as_attr()
            data-query-default-source=move || root_state.get().query_default_source_attr.as_attr()
            data-query-change-source=move || root_state.get().query_change_source_attr.as_attr()
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-state-source=move || root_state.get().query_control_attr.as_attr()
            data-ui-action-source=move || root_state.get().action_source_attr.as_attr()
            data-ui-motion-source=move || root_state.get().motion_source_attr.as_attr()
            data-ui-config-policy=move || agent_contract.get().config_policy.as_str()
            data-custom-id=move || root_state.get().has_custom_id_base.then_some("true")
            data-custom-placeholder=move || root_state.get().has_custom_placeholder.then_some("true")
            data-custom-empty-label=move || root_state.get().has_custom_empty_label.then_some("true")
            data-custom-aria-label=move || root_state.get().has_custom_aria_label.then_some("true")
            data-custom-class=move || root_state.get().has_custom_class_name.then_some("true")
            data-custom-disabled=move || root_state.get().has_custom_disabled.then_some("true")
            data-custom-action=move || root_state.get().has_custom_on_action.then_some("true")
            data-custom-motion=move || root_state.get().has_custom_motion.then_some("true")
        >
            <div class=slots.input_wrap.base_class() data-slot=slots.input_wrap.as_attr()>
                <input
                    type="text"
                    class=slots.input.base_class()
                    data-slot=slots.input.as_attr()
                    placeholder=placeholder.get_value()
                    value=move || query.get()
                    disabled=is_disabled
                    role=input_a11y.role
                    aria-autocomplete=input_a11y.aria_autocomplete
                    aria-expanded=input_a11y.aria_expanded
                    aria-label=aria_label.get_value()
                    aria-controls=listbox_id.get_value()
                    aria-activedescendant=move || listbox.attrs.aria_activedescendant.get()
                    lang=input_lang.clone()
                    dir=input_dir
                    on:input=move |ev| request_query_change.run(event_target_value(&ev))
                    on:keydown=on_input_key_down
                />
            </div>

            <div
                class=slots.list.base_class()
                id=listbox_id.get_value()
                role=listbox.attrs.role
                tabindex=listbox.attrs.tabindex
                aria-label=aria_label.get_value()
                aria-disabled=listbox.attrs.aria_disabled
                lang=input_lang.clone()
                dir=input_dir
                data-slot=slots.list.as_attr()
                data-empty=move || root_state.get().is_empty.then_some("true")
            >
                {move || {
                    filtered.with(|state| {
                        render_options_content(state, &options_render_ctx)
                    })
                }}
            </div>
        </section>
    }
}
