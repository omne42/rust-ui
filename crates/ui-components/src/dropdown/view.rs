use crate::dropdown::logic;
use crate::overlay_open;
use crate::{Button, Menu, MenuItemKind, OnPress, Popover, presence::use_presence};
use leptos::{ev, html, prelude::*};
use ui_headless::PopoverPlacement;

#[component]
pub fn Dropdown(
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
    #[prop(optional, into)] aria_label: Option<String>,
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
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let is_controlled = open.is_some();
    let open_state = overlay_open::use_controllable_open_state(open, default_open, on_open_change);
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let state = logic::resolve_state(crate::dropdown::DropdownStateInput {
        item_count,
        disabled: logic::resolve_trigger_disabled(disabled, item_count),
        close_on_action,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        is_controlled,
        has_disabled_items: !disabled_indices.get_value().is_empty(),
        has_item_kinds: !item_kinds.get_value().is_empty(),
    });

    let class = logic::compose_class_name(class_name, state);

    let (open_focus, set_open_focus) = signal(logic::DropdownOpenFocusStrategy::First);

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let item_count = StoredValue::new(state.item_count);
    let trigger_disabled = StoredValue::new(state.is_disabled);

    let on_trigger_press: OnPress = Callback::new(move |_| {
        if trigger_disabled.get_value() {
            return;
        }

        let next_open = !open.get_untracked();
        if next_open {
            set_open_focus.set(logic::DropdownOpenFocusStrategy::First);
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

    let trigger_id = StoredValue::new(format!("{}-trigger", id_base.get_value()));
    let menu_id = StoredValue::new(format!("{}-menu", id_base.get_value()));
    let aria_controls = crate::a11y::aria_controls_when_open(open, menu_id.get_value());

    let presence = use_presence(open);

    let aria_label = StoredValue::new(aria_label);

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
            data-slot="dropdown"
            data-state=move || {
                if open.get() {
                    "open"
                } else {
                    state.data_state_attr
                }
            }
            data-open=move || open.get().then_some("true")
            data-closed=move || (!open.get()).then_some("true")
            data-empty=state.is_empty.then_some("true")
            data-has-items=state.has_items.then_some("true")
            data-disabled=state.is_disabled.then_some("true")
            data-close-on-action=state.close_on_action.then_some("true")
            data-keep-open-on-action=state.keep_open_on_action.then_some("true")
            data-controlled=state.is_controlled.then_some("true")
            data-uncontrolled=state.is_uncontrolled.then_some("true")
            data-custom-label=state.has_custom_aria_label.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-has-disabled-items=state.has_disabled_items.then_some("true")
            data-has-item-kinds=state.has_item_kinds.then_some("true")
            data-aria-source=state.aria_source_attr
            data-class-source=state.class_source_attr
            data-item-count=state.item_count.to_string()
            on:keydown=on_key_down
        >
            <Button
                node_ref=anchor_ref
                on_press=on_trigger_press
                id=trigger_id.get_value()
                variant=crate::ButtonVariant::Secondary
                size=crate::ButtonSize::Sm
                disabled=state.is_disabled
                aria_haspopup="menu"
                aria_expanded=open
                aria_controls_signal=aria_controls
                aria_label=aria_label.get_value()
            >
                {children()}
            </Button>

            <Show when=move || presence.is_present.get()>
                <Popover
                    open=open
                    anchor_ref=anchor_ref
                    on_close=on_close
                    placement=placement
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
