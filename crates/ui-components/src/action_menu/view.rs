use crate::action_menu::{
    ActionMenuActionMode, ActionMenuDisabledState, ActionMenuItemSpec, ActionMenuMotion,
    ActionMenuPartStateInput, ActionMenuSlot, MenuOpenFocusStrategy, logic,
};
use crate::button::action::{ActionButton, ActionButtonSize};
use crate::{Menu, MenuItemKind, OnPress, Popover};
use leptos::{ev, html, prelude::*};
use ui_headless as overlay_open;
use ui_headless::{
    A11yDirection, CommonStrings, PopoverPlacement, locale_attrs, use_presence, use_ui_i18n,
};

#[component]
pub fn ActionMenu(
    id_base: String,
    #[prop(optional)] items: Vec<String>,
    on_action: Callback<usize>,
    #[prop(optional)] item_specs: Vec<ActionMenuItemSpec>,
    #[prop(optional)] disabled_state: Option<ActionMenuDisabledState>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] item_kinds: Vec<MenuItemKind>,
    #[prop(optional)] action_mode: Option<ActionMenuActionMode>,
    #[prop(optional)] is_close_on_action: Option<bool>,
    #[prop(optional)] close_on_action: Option<bool>,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] size: ActionButtonSize,
    #[prop(optional)] is_quiet: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: ActionMenuMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let locale = locale_attrs(lang, dir);

    let normalized_items = logic::normalize_menu_items(logic::ActionMenuItemsInput {
        item_specs,
        items,
        item_kinds,
        disabled_indices,
    });
    let item_count = normalized_items.item_count;
    let item_kinds_len = normalized_items.item_kinds.len();
    let items: StoredValue<std::sync::Arc<[String]>> =
        StoredValue::new(normalized_items.items.into());

    let has_custom_open = open.is_some();
    let has_custom_default_open = default_open.is_some();
    let has_custom_on_open_change = on_open_change.is_some();
    let motion = crate::action_menu::motion::sanitize_motion(motion);
    let has_custom_motion = motion != ActionMenuMotion::default();

    let logic::ActionMenuNormalizedProps {
        id_base,
        has_custom_id_base,
        disabled_indices,
        has_disabled_items,
        has_custom_disabled_indices,
        has_item_kinds,
        has_custom_item_kinds,
        class_name,
        has_custom_class_name,
        aria_label,
        has_custom_aria_label,
        disabled_state: _,
        action_mode,
        has_custom_disabled,
        has_custom_close_on_action,
        has_custom_placement,
        trigger_disabled,
        is_controlled,
        has_custom_open: _,
        has_custom_default_open: _,
        has_custom_on_open_change: _,
        has_custom_motion: _,
    } = logic::normalize_props(logic::ActionMenuNormalizeInput {
        id_base,
        item_count,
        disabled_indices: normalized_items.disabled_indices,
        item_kinds_len,
        class_name,
        aria_label,
        fallback_aria_label: common.action_menu_trigger_aria_label.to_string(),
        disabled_state,
        is_disabled,
        disabled,
        action_mode,
        is_close_on_action,
        close_on_action,
        placement,
        has_custom_open,
        has_custom_default_open,
        has_custom_on_open_change,
        has_custom_motion,
    });

    let id_base = StoredValue::new(id_base);
    let disabled_indices: StoredValue<Vec<usize>> = StoredValue::new(disabled_indices);
    let item_kinds: StoredValue<Vec<MenuItemKind>> = StoredValue::new(normalized_items.item_kinds);
    let class_name = StoredValue::new(class_name);
    let aria_label = StoredValue::new(aria_label);

    let open_state = overlay_open::use_controllable_open_state_traced(
        "action-menu",
        open,
        default_open,
        on_open_change,
    );
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let root_state = Memo::new(move |_| {
        logic::resolve_state(ActionMenuPartStateInput {
            slot: ActionMenuSlot::Root,
            is_open: open.get(),
            item_count,
            trigger_disabled,
            close_on_action: action_mode.is_close_on_action(),
            has_disabled_items,
            has_item_kinds,
            is_controlled,
            placement,
            has_custom_id_base,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_disabled,
            has_custom_disabled_indices,
            has_custom_item_kinds,
            has_custom_close_on_action,
            has_custom_placement,
            has_custom_open,
            has_custom_default_open,
            has_custom_on_open_change,
            has_custom_motion,
        })
    });
    let root_state_for_class = root_state;

    let root_class = Memo::new(move |_| {
        logic::compose_class_name(class_name.get_value(), root_state_for_class.get())
    });

    let (open_focus, set_open_focus) = signal(MenuOpenFocusStrategy::First);

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let item_count = StoredValue::new(item_count);

    let on_trigger_press: OnPress = Callback::new(move |_| {
        if let Some(result) = logic::resolve_trigger_press(trigger_disabled, open.get_untracked()) {
            if let Some(strategy) = result.open_focus {
                set_open_focus.set(strategy);
            }
            request_open_change.run(result.next_open);
        }
    });
    let on_close: OnPress = Callback::new(move |_| request_open_change.run(false));

    let on_action_wrapped = Callback::new(move |index: usize| {
        on_action.run(index);
        if let Some(next_open) = logic::resolve_action_open_change(action_mode) {
            request_open_change.run(next_open);
        }
    });

    let ids = logic::resolve_ids(&id_base.get_value());
    let trigger_id = StoredValue::new(ids.trigger_id);
    let menu_id = StoredValue::new(ids.menu_id);
    let aria_controls = ui_headless::aria_controls_when_open(open, menu_id.get_value());
    let presence = use_presence(open);

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if let Some(strategy) = ui_headless::menu_trigger_open_focus_strategy(
            &ev.key(),
            trigger_disabled,
            open.get_untracked(),
        ) {
            set_open_focus.set(strategy);
            request_open_change.run(true);
            ev.prevent_default();
        }
    };

    view! {
        <div
            class=move || root_class.get()
            data-slot=move || root_state.get().slot_attr
            data-state=move || root_state.get().state_attr
            data-items=move || root_state.get().item_attr
            data-action-mode=move || root_state.get().action_attr
            data-open-mode=move || root_state.get().open_mode_attr
            data-open=move || root_state.get().open_attr
            data-closed=move || root_state.get().closed_attr
            data-disabled=move || root_state.get().is_trigger_disabled.then_some("true")
            data-enabled=move || root_state.get().is_enabled.then_some("true")
            data-empty=move || root_state.get().is_empty.then_some("true")
            data-has-items=move || root_state.get().has_items.then_some("true")
            data-placement=move || root_state.get().placement_attr
            data-controlled=move || root_state.get().is_controlled.then_some("true")
            data-uncontrolled=move || root_state.get().is_uncontrolled.then_some("true")
            data-close-on-action=move || root_state.get().close_on_action.then_some("true")
            data-keep-open-on-action=move || root_state.get().keep_open_on_action.then_some("true")
            data-has-disabled-items=move || root_state.get().has_disabled_items.then_some("true")
            data-has-item-kinds=move || root_state.get().has_item_kinds.then_some("true")
            data-id-source=move || root_state.get().id_source_attr
            data-aria-label-source=move || root_state.get().aria_label_source_attr
            data-class-source=move || root_state.get().class_source_attr
            data-disabled-source=move || root_state.get().disabled_source_attr
            data-disabled-indices-source=move || root_state.get().disabled_indices_source_attr
            data-item-kinds-source=move || root_state.get().item_kinds_source_attr
            data-close-on-action-source=move || root_state.get().close_on_action_source_attr
            data-placement-source=move || root_state.get().placement_source_attr
            data-open-source=move || root_state.get().open_source_attr
            data-default-open-source=move || root_state.get().default_open_source_attr
            data-open-change-source=move || root_state.get().open_change_source_attr
            data-motion-source=move || root_state.get().motion_source_attr
            data-custom-id=move || root_state.get().has_custom_id_base.then_some("true")
            data-custom-aria-label=move || root_state.get().has_custom_aria_label.then_some("true")
            data-custom-class=move || root_state.get().has_custom_class_name.then_some("true")
            data-custom-disabled=move || root_state.get().has_custom_disabled.then_some("true")
            data-custom-disabled-indices=move || {
                root_state.get().has_custom_disabled_indices.then_some("true")
            }
            data-custom-item-kinds=move || root_state.get().has_custom_item_kinds.then_some("true")
            data-custom-close-on-action=move || {
                root_state.get().has_custom_close_on_action.then_some("true")
            }
            data-custom-placement=move || root_state.get().has_custom_placement.then_some("true")
            data-custom-open=move || root_state.get().has_custom_open.then_some("true")
            data-custom-default-open=move || root_state.get().has_custom_default_open.then_some("true")
            data-custom-open-change=move || root_state.get().has_custom_on_open_change.then_some("true")
            data-custom-motion=move || root_state.get().has_custom_motion.then_some("true")
            lang=locale.lang.clone()
            dir=locale.dir
            on:keydown=on_key_down
        >
            <ActionButton
                node_ref=anchor_ref
                on_press=on_trigger_press
                id=trigger_id.get_value()
                size=size
                is_disabled=trigger_disabled
                is_quiet=is_quiet
                is_icon_only=true
                aria_label=aria_label.get_value()
                aria_haspopup="menu"
                aria_expanded=open
                aria_controls_signal=aria_controls
            >
                <svg viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                    <circle cx="10" cy="4" r="1.5" />
                    <circle cx="10" cy="10" r="1.5" />
                    <circle cx="10" cy="16" r="1.5" />
                </svg>
            </ActionButton>

            <Show when=move || presence.is_present.get()>
                <Popover
                    open=open
                    anchor_ref=anchor_ref
                    on_close=on_close
                    placement=placement
                    motion=motion.popover
                    on_exit_complete=presence.finish_exit
                >
                    {move || {
                        let default_index =
                            open_focus.get_untracked().default_index(item_count.get_value());

                        view! {
                            <Menu
                                id_base=id_base.get_value()
                                id=menu_id.get_value()
                                aria_labelledby=trigger_id.get_value()
                                items=items.get_value()
                                on_action=on_action_wrapped
                                disabled_indices=disabled_indices.get_value()
                                item_kinds=item_kinds.get_value()
                                default_index=default_index
                            />
                        }
                    }}
                </Popover>
            </Show>
        </div>
    }
}
