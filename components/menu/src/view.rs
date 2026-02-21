use crate::menu::logic;
use crate::menu::{MenuItemSpec, MenuMotion};
use leptos::{html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{MenuAria, MenuItemKind, MenuItemOptions, MenuOptions, use_menu, use_menu_item};

const CHECKBOX_INDICATOR_MARK: &str = "✓";
const RADIO_INDICATOR_MARK: &str = "●";

fn render_menu_item(
    menu: MenuAria,
    active_index: ReadSignal<usize>,
    index: usize,
    label: String,
    kind: MenuItemKind,
    is_disabled: bool,
) -> impl IntoView {
    let item = use_menu_item(
        &menu,
        MenuItemOptions {
            index,
            kind,
            is_disabled,
        },
    );

    let indicator = move || match kind {
        MenuItemKind::Action => None,
        MenuItemKind::Checkbox { is_checked } => {
            is_checked.get().then_some(CHECKBOX_INDICATOR_MARK)
        }
        MenuItemKind::Radio { is_checked } => is_checked.get().then_some(RADIO_INDICATOR_MARK),
    };

    view! {
        <div
            id=item.attrs.id
            role=item.attrs.role
            aria-checked=move || item.attrs.aria_checked.get()
            aria-disabled=item.attrs.aria_disabled
            class="ui-menu__item"
            data-slot="menu-item"
            data-index=index
            data-kind=item.attrs.role
            data-checked=move || item.attrs.aria_checked.get().filter(|state| *state == "true")
            data-focused=move || (active_index.get() == index).then_some("true")
            data-disabled=if is_disabled { Some("true") } else { None }
            on:pointermove=move |_| item.handlers.on_pointer_move.run(())
            on:click=move |_| item.handlers.on_click.run(())
        >
            <span class="ui-menu__indicator">{indicator}</span>
            {label}
        </div>
    }
}

#[component]
pub fn Menu(
    id_base: String,
    #[prop(optional, into)] items: Arc<[String]>,
    on_action: Callback<usize>,
    #[prop(optional)] item_specs: Vec<MenuItemSpec>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] item_kinds: Vec<MenuItemKind>,
    #[prop(optional, default = 0)] default_index: usize,
    #[prop(optional)] motion: MenuMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let normalized_props = logic::normalize_props(logic::MenuNormalizeInput {
        is_disabled,
        disabled,
        class_name,
    });
    let normalized_items = logic::normalize_menu_items(logic::MenuItemsInput {
        item_specs,
        items,
        item_kinds,
        disabled_indices,
    });
    let has_item_specs = normalized_items.has_item_specs;
    let disabled = normalized_props.disabled;
    let item_count_value = normalized_items.item_count;
    let (item_count, _set_item_count) = signal(item_count_value);

    let disabled_set: HashSet<usize> = normalized_items.disabled_indices.into_iter().collect();
    let has_disabled = !disabled_set.is_empty();
    let disabled_indices: Arc<HashSet<usize>> = Arc::new(disabled_set);

    let item_kinds: Arc<Vec<MenuItemKind>> = Arc::new(normalized_items.item_kinds);
    let items = normalized_items.items;

    let item_text = {
        let items = items.clone();
        Callback::new(move |index: usize| logic::resolve_item_text(items.as_ref(), index))
    };

    let is_item_disabled = has_disabled.then_some({
        let disabled_indices = disabled_indices.clone();
        Callback::new(move |index: usize| disabled_indices.contains(&index))
    });

    let aria = use_menu(MenuOptions {
        is_disabled: disabled,
        should_loop: true,
        id_base,
        item_count,
        default_index,
        on_action: Some(on_action),
        is_item_disabled,
        item_text: Some(item_text),
    });
    let menu = aria.clone();

    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if aria.handlers.on_key_down.run(ev.key()) {
            ev.prevent_default();
        }
    };

    let items_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    crate::menu::motion::attach_motion(
        items_ref,
        highlight_ref,
        aria.active_index,
        aria.option_id,
        motion,
    );

    let class = normalized_props.class_name;

    let motion_source = crate::menu::motion::source_attr(motion);
    let custom_motion = (motion_source == "custom").then_some("true");

    let accessible_name = logic::resolve_accessible_name(aria_label, aria_labelledby);
    let aria_label = StoredValue::new(accessible_name.aria_label);
    let aria_labelledby = StoredValue::new(accessible_name.aria_labelledby);

    let has_checked_items = Signal::derive({
        let item_kinds = item_kinds.clone();
        move || {
            item_kinds.iter().any(|kind| match kind {
                MenuItemKind::Action => false,
                MenuItemKind::Checkbox { is_checked } | MenuItemKind::Radio { is_checked } => {
                    is_checked.get()
                }
            })
        }
    });

    let state = Signal::derive(move || {
        logic::resolve_state(
            item_count_value,
            has_checked_items.get(),
            has_disabled || disabled,
        )
    });
    let items_source = if has_item_specs {
        "item-spec"
    } else {
        "legacy-arrays"
    };
    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(logic::MenuAgentContractInput {
            render_state: state.get(),
            is_disabled: disabled,
            motion_source,
            items_source,
        })
    });

    view! {
        <div
            class=class
            id=id
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-label=aria_label.get_value()
            aria-labelledby=aria_labelledby.get_value()
            aria-disabled=aria.attrs.aria_disabled
            aria-activedescendant=move || aria.attrs.aria_activedescendant.get()
            data-slot="menu"
            data-disabled=disabled.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-has-checked-items=move || state.get().has_checked_items.then_some("true")
            data-checked-empty=move || (!state.get().has_checked_items).then_some("true")
            data-has-disabled-items=move || state.get().has_disabled_items.then_some("true")
            data-items-source=items_source
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()
            data-ui-state-source=move || agent_contract.get().state_source
            data-ui-motion-source=move || agent_contract.get().motion_source
            data-ui-items-source=move || agent_contract.get().items_source
            data-ui-config-policy=move || agent_contract.get().config_policy
            on:keydown=on_key_down
        >
            <div class="ui-menu__items" node_ref=items_ref data-slot="menu-items">
                <div class="ui-active-highlight" node_ref=highlight_ref data-slot="menu-highlight"></div>
                {items
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(move |(index, label)| {
                        let kind = logic::resolve_item_kind(item_kinds.as_ref(), index);
                        let is_disabled = disabled || disabled_indices.contains(&index);
                        render_menu_item(
                            menu.clone(),
                            aria.active_index,
                            index,
                            label,
                            kind,
                            is_disabled,
                        )
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
