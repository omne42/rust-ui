use crate::context_menu::{
    ContextMenuMotion, ContextMenuPartStateInput, ContextMenuSlot, MenuOpenFocusStrategy, logic,
};
use crate::menu::Menu;
use crate::popover::Popover;
use crate::{MenuItemKind, OnPress};
use leptos::{ev, html, prelude::*};
use ui_headless as overlay_open;
use ui_headless::use_presence;
use ui_headless::{A11yDirection, PopoverPlacement, locale_attrs};

#[component]
pub fn ContextMenu(
    id_base: String,
    items: Vec<String>,
    on_action: Callback<usize>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] item_kinds: Vec<MenuItemKind>,
    #[prop(optional)] is_close_on_action: Option<bool>,
    #[prop(default = true)] close_on_action: bool,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] is_open: Option<Signal<bool>>,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] motion: ContextMenuMotion,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let discrete_props = logic::normalize_discrete_props(logic::ContextMenuDiscreteInput {
        is_disabled,
        disabled,
        is_close_on_action,
        close_on_action,
    });
    let open_state_input = logic::normalize_open_state(logic::ContextMenuOpenStateInput {
        is_open,
        open,
        default_open,
        on_open_change,
    });
    let motion = crate::context_menu::motion::sanitize_motion(motion);
    let id_base = logic::normalize_id_base(id_base);
    let has_custom_id_base = id_base != logic::DEFAULT_ID_BASE;
    let id_base = StoredValue::new(id_base);

    let items: StoredValue<std::sync::Arc<[String]>> = StoredValue::new(items.into());
    let item_count = items.get_value().len();
    let item_count = StoredValue::new(item_count);

    let disabled_indices =
        logic::normalize_disabled_indices(disabled_indices, item_count.get_value());
    let has_custom_disabled_indices = !disabled_indices.is_empty();
    let disabled_indices: StoredValue<Vec<usize>> = StoredValue::new(disabled_indices);

    let has_custom_item_kinds = !item_kinds.is_empty();
    let item_kinds: StoredValue<Vec<MenuItemKind>> = StoredValue::new(item_kinds);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);

    let (aria_label, has_custom_aria_label) = logic::resolve_trigger_aria_label(aria_label);
    let aria_label = StoredValue::new(aria_label);

    let has_custom_disabled = is_disabled.is_some()
        || discrete_props.disabled_state.is_disabled() != logic::DEFAULT_DISABLED;
    let has_custom_close_on_action =
        discrete_props.action_mode.is_close_on_action() != logic::DEFAULT_CLOSE_ON_ACTION;
    let has_custom_placement = placement != logic::DEFAULT_PLACEMENT;

    let has_custom_open = open_state_input.is_controlled;
    let has_custom_default_open = open_state_input.default_open.is_some();
    let has_custom_on_open_change = open_state_input.on_open_change.is_some();
    let has_custom_motion = motion != ContextMenuMotion::default();

    let trigger_disabled = logic::resolve_trigger_disabled(
        discrete_props.disabled_state.is_disabled(),
        item_count.get_value(),
    );

    let is_controlled = open_state_input.is_controlled;
    let open_state = overlay_open::use_controllable_open_state_traced(
        "context-menu",
        open_state_input.open,
        open_state_input.default_open,
        open_state_input.on_open_change,
    );
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let root_state = Memo::new(move |_| {
        logic::resolve_state(ContextMenuPartStateInput {
            slot: ContextMenuSlot::Root,
            is_open: open.get(),
            item_count: item_count.get_value(),
            trigger_disabled,
            close_on_action: discrete_props.action_mode.is_close_on_action(),
            placement,
            is_controlled,
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

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();

    let on_close: OnPress = Callback::new(move |_| request_open_change.run(false));

    let on_action_wrapped = Callback::new(move |index: usize| {
        on_action.run(index);
        if discrete_props.action_mode.is_close_on_action() {
            request_open_change.run(false);
        }
    });

    let ids = logic::resolve_ids(&id_base.get_value());
    let trigger_id = StoredValue::new(ids.trigger_id);
    let menu_id = StoredValue::new(ids.menu_id);
    let aria_controls = ui_headless::aria_controls_when_open(open, menu_id.get_value());

    let presence = use_presence(open);

    let (open_focus, set_open_focus) = signal(MenuOpenFocusStrategy::First);

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if let Some(strategy) = logic::resolve_open_focus_strategy(
            &ev.key(),
            ev.shift_key(),
            trigger_disabled,
            open.get_untracked(),
        ) {
            set_open_focus.set(strategy);
            request_open_change.run(true);
            ev.prevent_default();
        }
    };

    let on_context_menu = move |ev: ev::MouseEvent| {
        if !logic::should_open_from_context_menu(trigger_disabled) {
            return;
        }
        ev.prevent_default();
        set_open_focus.set(MenuOpenFocusStrategy::First);
        request_open_change.run(true);
    };

    let trigger_slot = ContextMenuSlot::Trigger;

    view! {
        <div
            class=move || root_class.get()
            lang=locale.lang.clone()
            dir=locale.dir
            data-slot=move || root_state.get().slot_attr
            data-state=move || root_state.get().state_attr
            data-items=move || root_state.get().item_attr
            data-open=move || root_state.get().open_attr
            data-closed=move || root_state.get().closed_attr
            data-disabled=move || root_state.get().is_trigger_disabled.then_some("true")
            data-enabled=move || root_state.get().is_enabled.then_some("true")
            data-empty=move || root_state.get().is_empty.then_some("true")
            data-has-items=move || root_state.get().has_items.then_some("true")
            data-placement=move || root_state.get().placement_attr
            data-action-mode=move || root_state.get().action_attr
            data-open-mode=move || root_state.get().open_mode_attr
            data-controlled=move || root_state.get().is_controlled.then_some("true")
            data-uncontrolled=move || root_state.get().is_uncontrolled.then_some("true")
            data-close-on-action=move || root_state.get().close_on_action.then_some("true")
            data-keep-open-on-action=move || root_state.get().keep_open_on_action.then_some("true")
            data-has-disabled-items=move || root_state.get().has_custom_disabled_indices.then_some("true")
            data-has-item-kinds=move || root_state.get().has_custom_item_kinds.then_some("true")
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
            data-custom-default-open=move || {
                root_state.get().has_custom_default_open.then_some("true")
            }
            data-custom-open-change=move || {
                root_state.get().has_custom_on_open_change.then_some("true")
            }
            data-custom-motion=move || root_state.get().has_custom_motion.then_some("true")
            data-ui-schema="ui.context_menu.agent-contract.v1"
            data-ui-schema-version="1"
            data-ui-intent="open-context-actions"
            data-ui-action=move || logic::resolve_ui_action(root_state.get().is_open)
            data-ui-state=move || root_state.get().state_attr
            data-ui-source=move || root_state.get().open_source_attr
            data-ui-stream-support="unsupported"
            data-ui-stream-fallback="snapshot"
            data-ui-stream-mode="snapshot"
            data-ui-output-status=move || logic::resolve_ui_output_status(root_state.get().is_open)
        >
            <button
                type="button"
                class=trigger_slot.base_class()
                node_ref=anchor_ref
                id=trigger_id.get_value()
                disabled=move || root_state.get().is_trigger_disabled
                aria-label=aria_label.get_value()
                aria-haspopup="menu"
                aria-expanded=move || logic::resolve_aria_expanded(open.get())
                aria-controls=aria_controls
                data-slot=trigger_slot.as_attr()
                data-state=move || root_state.get().state_attr
                data-disabled=move || root_state.get().is_trigger_disabled.then_some("true")
                data-enabled=move || root_state.get().is_enabled.then_some("true")
                data-aria-label-source=move || root_state.get().aria_label_source_attr
                on:keydown=on_key_down
                on:contextmenu=on_context_menu
            >
                {children()}
            </button>

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
