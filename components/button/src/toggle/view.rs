use super::super::toggle_button::motion;
#[cfg(feature = "component-toggle_group")]
use super::super::toggle_button::{ToggleButton, ToggleButtonSize, ToggleButtonVariant};
use super::{ToggleMotion, ToggleSize, ToggleStateInput, ToggleVariant, logic};
use leptos::{html, prelude::*};
#[cfg(feature = "component-toggle_group")]
use std::collections::BTreeSet;
use ui_headless as overlay_open;
#[cfg(feature = "component-toggle_group")]
use ui_headless::{A11yDirection, labeled_group_attrs};
use ui_headless::{
    ButtonOptions, FocusRingOptions, HoverOptions, use_button, use_focus_ring, use_hover,
};

#[component]
pub fn Toggle(
    #[prop(optional)] is_pressed: Option<Signal<bool>>,
    #[prop(optional)] default_pressed: Option<bool>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] variant: ToggleVariant,
    #[prop(optional)] size: ToggleSize,
    #[prop(optional)] motion: ToggleMotion,
    #[prop(optional)] on_pressed_change: Option<Callback<bool>>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let aria_label = logic::normalize_optional_text(aria_label);
    let has_on_pressed_change = on_pressed_change.is_some();

    let pressed_state =
        overlay_open::use_controllable_state(is_pressed, default_pressed, on_pressed_change);
    let pressed = pressed_state.value;
    let request_pressed_change = pressed_state.request_change;

    let has_custom_class_name = class_name.is_some();
    let has_custom_motion = motion != ToggleMotion::default();
    let has_custom_aria_label = aria_label.is_some();

    let on_press = Callback::new(move |_| {
        let next = !pressed.get_untracked();
        request_pressed_change.run(next);
    });

    let aria = use_button(ButtonOptions {
        is_disabled,
        on_press: Some(on_press),
        ..Default::default()
    });

    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
    let hover = use_hover(HoverOptions { is_disabled });

    motion::attach_motion(
        node_ref,
        hover.is_hovered,
        aria.is_pressed,
        is_disabled,
        motion,
    );

    let state = Memo::new(move |_| {
        logic::resolve_state(ToggleStateInput {
            selected: pressed.get(),
            disabled: is_disabled,
            hovered: !is_disabled && hover.is_hovered.get(),
            pressed_interaction: !is_disabled && aria.is_pressed.get(),
            focused: !is_disabled && focus_ring.is_focused.get(),
            focus_visible: !is_disabled && focus_ring.is_focus_visible.get(),
            variant,
            size,
            has_custom_class_name,
            has_custom_motion,
            has_custom_aria_label,
            has_on_pressed_change,
        })
    });

    let class = logic::compose_class_name(class_name, state.get_untracked());

    view! {
        <button
            type="button"
            node_ref=node_ref
            class=class
            class:ui-toggle-button--focus-visible=move || !is_disabled && focus_ring.is_focus_visible.get()
            disabled=is_disabled
            data-slot="toggle"
            data-state=move || state.get().state_attr
            data-interaction=move || state.get().interaction_attr
            data-variant=move || state.get().variant_attr
            data-size=move || state.get().size_attr
            data-selected=move || state.get().is_selected.then_some("true")
            data-unselected=move || (!state.get().is_selected).then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-enabled=move || (!state.get().is_disabled).then_some("true")
            data-hovered=move || state.get().is_hovered.then_some("true")
            data-pressed=move || state.get().is_pressed.then_some("true")
            data-focused=move || state.get().is_focused.then_some("true")
            data-focus-visible=move || state.get().is_focus_visible.then_some("true")
            data-variant-source=move || state.get().variant_source_attr
            data-size-source=move || state.get().size_source_attr
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-handler-source=move || state.get().handler_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            data-custom-aria-label=move || state.get().has_custom_aria_label.then_some("true")
            data-custom-aria=move || state.get().has_custom_aria_label.then_some("true")
            data-custom-handler=move || state.get().has_on_pressed_change.then_some("true")
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-pressed=move || if state.get().is_selected { "true" } else { "false" }
            aria-label=aria_label
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
            on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
            on:click=move |_| aria.handlers.press.on_click.run(())
            on:keydown=move |ev| {
                let key = ev.key();
                if aria.handlers.press.on_key_down.run(key) {
                    ev.prevent_default();
                }
            }
            on:keyup=move |ev| {
                let key = ev.key();
                if aria.handlers.press.on_key_up.run(key) {
                    ev.prevent_default();
                }
            }
            on:focus=move |_| focus_ring.handlers.on_focus.run(())
            on:blur=move |_| {
                aria.handlers.press.on_blur.run(());
                focus_ring.handlers.on_blur.run(());
            }
        >
            <span class="ui-toggle-button__label" data-slot="toggle-label">
                {children()}
            </span>
        </button>
    }
}

#[cfg(feature = "component-toggle_group")]
#[component]
pub fn ToggleGroup(
    id_base: String,
    items: Vec<super::ToggleGroupItem>,
    #[prop(optional)] selection_mode: logic::ToggleGroupSelectionMode,
    #[prop(optional)] selected_ids: Option<Signal<BTreeSet<String>>>,
    #[prop(optional)] default_selected_ids: Option<BTreeSet<String>>,
    #[prop(optional)] on_selected_ids_change: Option<Callback<BTreeSet<String>>>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_attached: bool,
    #[prop(optional)] orientation: logic::ToggleGroupOrientation,
    #[prop(optional)] variant: ToggleButtonVariant,
    #[prop(optional)] size: ToggleButtonSize,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let items = logic::normalize_toggle_group_items(items);
    let item_ids = logic::collect_toggle_group_item_ids(&items);

    let default_selected_ids = logic::normalize_toggle_group_default_selected_ids(
        default_selected_ids,
        &item_ids,
        &items,
        selection_mode,
    );

    let selected_state = overlay_open::use_controllable_state(
        selected_ids,
        Some(default_selected_ids),
        on_selected_ids_change,
    );
    let selected_ids = selected_state.value;
    let request_selected_ids_change = selected_state.request_change;

    let (aria_label, has_custom_aria_label) = logic::normalize_toggle_group_aria_label(aria_label);
    let group_a11y = labeled_group_attrs(aria_label, lang, dir);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let items = StoredValue::new(items);
    let item_ids = StoredValue::new(item_ids);
    let on_action = StoredValue::new(on_action);

    let resolved_selected_ids = Signal::derive(move || {
        logic::sanitize_toggle_group_selected_ids(
            selected_ids.get(),
            &item_ids.get_value(),
            &items.get_value(),
            selection_mode,
        )
    });

    let state = Signal::derive(move || {
        let items_value = items.get_value();
        logic::resolve_toggle_group_state(super::ToggleGroupStateInput {
            orientation,
            selection_mode,
            disabled: is_disabled,
            attached: is_attached,
            item_count: items_value.len(),
            selected_count: resolved_selected_ids.get().len(),
            disabled_item_count: items_value.iter().filter(|item| item.disabled).count(),
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Signal::derive(move || {
        logic::compose_toggle_group_class_name(class_name.get_value(), state.get())
    });

    view! {
        <div
            id=id_base
            class=move || class.get()
            data-slot="toggle-group"
            data-orientation=move || state.get().orientation_attr
            data-selection-mode=move || state.get().selection_mode_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-attached=move || state.get().is_attached.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-item-count=move || state.get().item_count.to_string()
            data-selected-count=move || state.get().selected_count.to_string()
            data-has-disabled-items=move || state.get().has_disabled_items.then_some("true")
            data-disabled-item-count=move || state.get().disabled_item_count.to_string()
            data-aria-source=move || state.get().aria_source_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            role=group_a11y.role
            aria-label=group_a11y.aria_label.clone()
            lang=group_a11y.lang.clone()
            dir=group_a11y.dir
        >
            <div class="ui-toggle-group__items" data-slot="toggle-group-items">
                <For
                    each=move || items.get_value()
                    key=|item| item.id.clone()
                    children=move |item| {
                        let item_id = item.id.clone();
                        let item_label = item.label.clone();
                        let item_is_disabled = is_disabled || item.disabled;
                        let item_id_for_selected = item_id.clone();
                        let item_selected = Signal::derive(move || {
                            resolved_selected_ids.get().contains(&item_id_for_selected)
                        });

                        let on_item_change = {
                            let item_id = item_id.clone();
                            Callback::new(move |next_selected: bool| {
                                if item_is_disabled {
                                    return;
                                }

                                if let Some(on_action) = on_action.get_value() {
                                    on_action.run(item_id.clone());
                                }

                                let next_ids = logic::toggle_toggle_group_selected_id(
                                    resolved_selected_ids.get_untracked(),
                                    &item_id,
                                    &item_ids.get_value(),
                                    &items.get_value(),
                                    selection_mode,
                                    next_selected,
                                );
                                request_selected_ids_change.run(next_ids);
                            })
                        };

                        view! {
                            <ToggleButton
                                is_pressed=item_selected
                                is_disabled=item_is_disabled
                                variant=variant
                                size=size
                                on_pressed_change=on_item_change
                                class_name="ui-toggle-group__item".to_string()
                                aria_label=item_label.clone()
                            >
                                {item_label}
                            </ToggleButton>
                        }
                    }
                />
            </div>
        </div>
    }
}
