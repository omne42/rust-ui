use crate::dropdown_menu::{DropdownMenuMotion, logic};
use crate::{Button, ButtonSize, ButtonVariant, Menu, MenuItemKind, OnPress, Popover};
use leptos::{ev, html, prelude::*};
use ui_headless as overlay_open;
use ui_headless::PopoverPlacement;
use ui_headless::use_presence;

#[component]
pub fn DropdownMenu(
    id_base: String,
    items: Vec<String>,
    on_action: Callback<usize>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] item_kinds: Vec<MenuItemKind>,
    #[prop(default = true)] close_on_action: bool,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional, default = ButtonVariant::Secondary)] trigger_variant: ButtonVariant,
    #[prop(optional, default = ButtonSize::Sm)] trigger_size: ButtonSize,
    #[prop(optional)] motion: DropdownMenuMotion,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let id_base = StoredValue::new(id_base);

    let items: StoredValue<std::sync::Arc<[String]>> = StoredValue::new(items.into());
    let item_count = items.get_value().len();

    let disabled_indices = logic::normalize_disabled_indices(disabled_indices, item_count);
    let disabled_indices: StoredValue<Vec<usize>> = StoredValue::new(disabled_indices);

    let item_kinds: StoredValue<Vec<MenuItemKind>> = StoredValue::new(item_kinds);

    let class_name = logic::normalize_optional_text(class_name);

    let motion = crate::dropdown_menu::motion::sanitize_motion(motion);

    let is_controlled = open.is_some();
    let open_state = overlay_open::use_controllable_open_state_traced(
        "dropdown-menu",
        open,
        default_open,
        on_open_change,
    );
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let state = logic::resolve_state(logic::DropdownMenuStateInput {
        item_count,
        trigger_disabled: logic::resolve_trigger_disabled(disabled, item_count),
        close_on_action,
        has_custom_class_name: class_name.is_some(),
        has_disabled_items: !disabled_indices.get_value().is_empty(),
        has_item_kinds: !item_kinds.get_value().is_empty(),
        is_controlled,
        placement,
    });

    let class = logic::compose_class_name(class_name, state);

    let (open_focus, set_open_focus) = signal(logic::MenuOpenFocusStrategy::First);

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let item_count = StoredValue::new(state.item_count);
    let trigger_disabled = StoredValue::new(state.is_trigger_disabled);

    let on_trigger_press: OnPress = Callback::new(move |_| {
        if trigger_disabled.get_value() {
            return;
        }

        let next_open = !open.get_untracked();
        if next_open {
            set_open_focus.set(logic::MenuOpenFocusStrategy::First);
        }
        request_open_change.run(next_open);
    });
    let on_close: OnPress = Callback::new(move |_| request_open_change.run(false));

    let on_action_wrapped = Callback::new(move |index: usize| {
        on_action.run(index);
        if state.close_on_action {
            request_open_change.run(false);
        }
    });

    let ids = logic::resolve_ids(&id_base.get_value());
    let trigger_id = StoredValue::new(ids.trigger_id);
    let menu_id = StoredValue::new(ids.menu_id);
    let aria_controls = ui_headless::aria_controls_when_open(open, menu_id.get_value());
    let presence = use_presence(open);

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if trigger_disabled.get_value() {
            return;
        }
        if open.get_untracked() {
            return;
        }

        let key = ev.key();
        if let Some(strategy) = logic::focus_strategy_for_open_key(&key) {
            set_open_focus.set(strategy);
            request_open_change.run(true);
            ev.prevent_default();
        }
    };

    view! {
        <div
            class=class
            data-slot="dropdown-menu"
            data-state=move || {
                if open.get() {
                    "open"
                } else if state.is_trigger_disabled {
                    "disabled"
                } else {
                    "closed"
                }
            }
            data-open=move || open.get().then_some("true")
            data-closed=move || (!open.get()).then_some("true")
            data-disabled=state.is_trigger_disabled.then_some("true")
            data-enabled=state.is_enabled.then_some("true")
            data-empty=state.is_empty.then_some("true")
            data-has-items=state.has_items.then_some("true")
            data-placement=state.placement_attr
            data-controlled=state.is_controlled.then_some("true")
            data-uncontrolled=state.is_uncontrolled.then_some("true")
            data-close-on-action=state.close_on_action.then_some("true")
            data-keep-open-on-action=state.keep_open_on_action.then_some("true")
            data-has-disabled-items=state.has_disabled_items.then_some("true")
            data-has-item-kinds=state.has_item_kinds.then_some("true")
            data-motion-source=if motion == DropdownMenuMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != DropdownMenuMotion::default()).then_some("true")
            on:keydown=on_key_down
        >
            <Button
                node_ref=anchor_ref
                on_press=on_trigger_press
                id=trigger_id.get_value()
                variant=trigger_variant
                size=trigger_size
                disabled=state.is_trigger_disabled
                aria_haspopup="menu"
                aria_expanded=open
                aria_controls_signal=aria_controls
            >
                {children()}
            </Button>

            <Show when=move || presence.is_present.get()>
                <Popover
                    open=open
                    anchor_ref=anchor_ref
                    on_close=on_close
                    placement=state.placement
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
