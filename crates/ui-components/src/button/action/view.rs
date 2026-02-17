use super::super::{
    ButtonColor, ButtonRadius, ButtonVariant, logic as button_logic, motion as button_motion,
};
use super::logic as action_logic;
#[cfg(feature = "component-action_button_group")]
use super::motion as action_motion;
#[cfg(feature = "component-action_button_group")]
use super::{ActionButtonGroupDensity, ActionButtonGroupMotion, ActionButtonGroupOrientation};
use super::{ActionButtonLoadingPlacement, ActionButtonMotion, ActionButtonSize, ActionButtonType};
#[cfg(feature = "component-action_group")]
use super::{ActionGroupItem, ActionGroupSelectionMode, ActionGroupStateInput, ActionGroupTone};
use leptos::children::ViewFn;
use leptos::{html, prelude::*};
#[cfg(feature = "component-action_group")]
use std::collections::BTreeSet;
#[cfg(feature = "component-action_group")]
use ui_headless::use_controllable_state;
use ui_headless::{
    ButtonOptions, CommonStrings, FocusRingOptions, HoverOptions, OnPress, popup_trigger_attrs,
    use_button, use_focus_ring, use_hover, use_ui_i18n,
};

fn render_action_start_slot(
    start_content: Option<StoredValue<ViewFn>>,
    is_loading: bool,
    loading_placement: ActionButtonLoadingPlacement,
) -> AnyView {
    let Some(start_content) = start_content else {
        return ().into_any();
    };

    view! {
        <span
            class="ui-button__start"
            data-slot="button-start"
            data-loading-start=(is_loading
                && matches!(loading_placement, ActionButtonLoadingPlacement::Start))
                .then_some("true")
        >
            <span class="ui-button__start-content" data-slot="button-start-content">
                {start_content.get_value().run()}
            </span>
            <Show when=move || {
                is_loading && matches!(loading_placement, ActionButtonLoadingPlacement::Start)
            }>
                <span class="ui-button__spinner" data-slot="button-spinner" aria-hidden="true"></span>
            </Show>
        </span>
    }
    .into_any()
}

fn render_action_end_slot(end_content: Option<StoredValue<ViewFn>>) -> AnyView {
    let Some(end_content) = end_content else {
        return ().into_any();
    };

    view! {
        <span class="ui-button__end" data-slot="button-end">
            {end_content.get_value().run()}
        </span>
    }
    .into_any()
}

#[component]
pub fn ActionButton(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] is_loading: bool,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] size: Option<ActionButtonSize>,
    #[prop(optional)] is_quiet: Option<bool>,
    #[prop(optional)] is_icon_only: bool,
    #[prop(optional, into)] start_content: Option<ViewFn>,
    #[prop(optional, into)] end_content: Option<ViewFn>,
    #[prop(optional)] motion: ActionButtonMotion,
    #[prop(optional)] loading_placement: ActionButtonLoadingPlacement,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] button_type: Option<ActionButtonType>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] aria_haspopup: Option<&'static str>,
    #[prop(optional)] aria_expanded: Option<Signal<bool>>,
    #[prop(optional, into)] aria_controls: Option<String>,
    #[prop(optional)] aria_controls_signal: Option<Signal<Option<String>>>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
    let i18n = use_ui_i18n();
    let common_strings = i18n.strings::<CommonStrings>();
    #[cfg(feature = "component-action_button_group")]
    let group = action_logic::action_button_group_logic::use_action_button_group_context();

    #[cfg(feature = "component-action_button_group")]
    let inherited_disabled = group.map(|ctx| ctx.is_disabled);
    #[cfg(not(feature = "component-action_button_group"))]
    let inherited_disabled: Option<bool> = None;

    #[cfg(feature = "component-action_button_group")]
    let inherited_size = group.map(|ctx| ctx.size);
    #[cfg(not(feature = "component-action_button_group"))]
    let inherited_size: Option<ActionButtonSize> = None;

    #[cfg(feature = "component-action_button_group")]
    let inherited_quiet = group.map(|ctx| ctx.is_quiet);
    #[cfg(not(feature = "component-action_button_group"))]
    let inherited_quiet: Option<bool> = None;

    let is_disabled = is_disabled.or(inherited_disabled).unwrap_or(false);
    let size = size.or(inherited_size).unwrap_or_default();
    let is_quiet = is_quiet.or(inherited_quiet).unwrap_or(false);
    let variant = if is_quiet {
        ButtonVariant::Ghost
    } else {
        ButtonVariant::Default
    };

    let class_name = button_logic::normalize_optional_text(class_name);
    let (aria_label, aria_label_source) = button_logic::resolve_aria_label(
        aria_label,
        is_icon_only,
        Some(common_strings.icon_button_aria_label.to_string()),
    );
    let has_start_content = start_content.is_some();
    let has_end_content = end_content.is_some();
    let has_custom_motion = motion != ActionButtonMotion::default();

    let state = button_logic::resolve_state(button_logic::ButtonStateInput {
        is_disabled,
        is_loading,
        variant,
        color: ButtonColor::default(),
        radius: ButtonRadius::default(),
        size,
        loading_placement,
        is_icon_only,
        is_full_width: false,
        has_start_content,
        has_end_content,
        has_custom_class_name: class_name.is_some(),
        has_custom_motion,
    });

    let aria = use_button(ButtonOptions {
        is_disabled: state.is_disabled,
        on_press,
        ..Default::default()
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: state.is_disabled,
    });

    let hover = use_hover(HoverOptions {
        is_disabled: state.is_disabled,
    });

    button_motion::attach_motion(
        node_ref,
        hover.is_hovered,
        aria.is_pressed,
        state.is_disabled,
        motion,
    );

    let class = button_logic::compose_class_name(class_name, state);
    let button_type = button_type.unwrap_or_default();
    let start_content = start_content.map(StoredValue::new);
    let end_content = end_content.map(StoredValue::new);
    let popup_a11y = popup_trigger_attrs(
        aria_haspopup,
        aria_controls,
        aria_controls_signal,
        aria_expanded,
        None::<String>,
        None,
    );

    view! {
        <button
            id=id
            type=button_type.as_attr()
            node_ref=node_ref
            class=class
            class:ui-button--focus-visible=move || focus_ring.is_focus_visible.get()
            disabled=state.is_disabled
            data-slot="action-button"
            data-state=state.state_attr
            data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
            data-pressed=move || if aria.is_pressed.get() { Some("true") } else { None }
            data-loading=state.is_loading.then_some("true")
            data-loading-placement=state.loading_placement_attr
            data-icon-only=state.is_icon_only.then_some("true")
            data-full-width=state.is_full_width.then_some("true")
            data-has-start=state.has_start_content.then_some("true")
            data-has-end=state.has_end_content.then_some("true")
            data-quiet=is_quiet.then_some("true")
            data-label-source=aria_label_source.as_attr()
            data-color=state.color_attr
            data-radius=state.radius_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-label=aria_label
            aria-haspopup=popup_a11y.aria_haspopup
            aria-controls=move || popup_a11y.aria_controls.get()
            aria-busy=state.is_loading.then_some("true")
            aria-expanded=move || popup_a11y.aria_expanded.get()
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
            <Show when=move || {
                state.is_loading
                    && matches!(loading_placement, ActionButtonLoadingPlacement::Start)
                    && !has_start_content
            }>
                <span class="ui-button__spinner" data-slot="button-spinner" aria-hidden="true"></span>
            </Show>

            {render_action_start_slot(start_content, state.is_loading, loading_placement)}

            <span class="ui-button__label" data-slot="button-label">
                {children()}
            </span>

            {render_action_end_slot(end_content)}

            <Show when=move || {
                state.is_loading && matches!(loading_placement, ActionButtonLoadingPlacement::End)
            }>
                <span class="ui-button__spinner" data-slot="button-spinner" aria-hidden="true"></span>
            </Show>

            <Show when=move || {
                state.is_loading
                    && matches!(loading_placement, ActionButtonLoadingPlacement::Center)
            }>
                <span class="ui-button__spinner" data-slot="button-spinner" aria-hidden="true"></span>
            </Show>
        </button>
    }
}

#[cfg(feature = "component-action_button_group")]
#[component]
pub fn ActionButtonGroup(
    children: Children,
    #[prop(optional)] size: ActionButtonSize,
    #[prop(optional)] density: ActionButtonGroupDensity,
    #[prop(optional)] orientation: ActionButtonGroupOrientation,
    #[prop(optional)] is_justified: bool,
    #[prop(optional)] is_quiet: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: ActionButtonGroupMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = action_logic::action_button_group_logic::normalize_optional_text(class_name);
    let (aria_label, has_explicit_label) =
        action_logic::action_button_group_logic::normalize_aria_label(aria_label);

    let state = action_logic::action_button_group_logic::resolve_state(
        orientation,
        density,
        is_justified,
        is_quiet,
        disabled,
        has_explicit_label,
        class_name.is_some(),
    );

    provide_context(
        action_logic::action_button_group_logic::ActionButtonGroupContextValue {
            size,
            density: state.density,
            orientation: state.orientation,
            is_justified: state.is_justified,
            is_quiet: state.is_quiet,
            is_disabled: state.is_disabled,
        },
    );

    let class = action_logic::action_button_group_logic::compose_class_name(class_name, state);
    let motion = action_motion::sanitize_motion(motion);
    let has_custom_motion = motion != ActionButtonGroupMotion::default();
    let panel_vars = action_motion::attach_motion(motion);

    view! {
        <div
            class=class
            style=panel_vars
            data-slot="action-button-group"
            data-state=if state.is_disabled { "disabled" } else { "ready" }
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-orientation=state.orientation_attr
            data-density=state.density_attr
            data-horizontal=state.is_horizontal.then_some("true")
            data-vertical=state.is_vertical.then_some("true")
            data-regular=state.is_regular.then_some("true")
            data-compact=state.is_compact.then_some("true")
            data-justified=state.is_justified.then_some("true")
            data-not-justified=state.is_not_justified.then_some("true")
            data-quiet=state.is_quiet.then_some("true")
            data-filled=state.is_filled.then_some("true")
            data-disabled=state.is_disabled.then_some("true")
            data-enabled=state.is_enabled.then_some("true")
            data-has-explicit-label=state.has_explicit_label.then_some("true")
            data-has-fallback-label=state.has_fallback_label.then_some("true")
            role="toolbar"
            aria-label=aria_label
            aria-orientation=state.orientation.aria_orientation()
            aria-disabled=state.is_disabled.then_some("true")
        >
            {children()}
        </div>
    }
}

#[cfg(feature = "component-action_group")]
#[component]
pub fn ActionGroup(
    id_base: String,
    items: Vec<ActionGroupItem>,
    #[prop(optional)] tone: ActionGroupTone,
    #[prop(optional)] selection_mode: ActionGroupSelectionMode,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] selected_ids: Option<Signal<BTreeSet<String>>>,
    #[prop(optional)] default_selected_ids: Option<BTreeSet<String>>,
    #[prop(optional)] on_selected_ids_change: Option<Callback<BTreeSet<String>>>,
    #[prop(optional)] on_selected_change: Option<Callback<BTreeSet<String>>>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let items = action_logic::action_group_logic::normalize_items(items);
    let item_ids = action_logic::action_group_logic::collect_item_ids(&items);

    let default_selected_ids = action_logic::action_group_logic::sanitize_selected_ids(
        default_selected_ids.unwrap_or_default(),
        &item_ids,
        selection_mode,
    );

    let on_selected_change = on_selected_ids_change.or(on_selected_change);
    let selected_state =
        use_controllable_state(selected_ids, Some(default_selected_ids), on_selected_change);
    let selected_ids = selected_state.value;
    let request_selected_change = selected_state.request_change;

    let (aria_label, has_custom_aria_label) =
        action_logic::action_group_logic::normalize_aria_label(aria_label);

    let class_name = action_logic::action_group_logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let id_base = StoredValue::new(id_base);
    let items = StoredValue::new(items);
    let item_ids = StoredValue::new(item_ids);
    let on_action = StoredValue::new(on_action);

    let resolved_selected_ids = Memo::new(move |_| {
        action_logic::action_group_logic::sanitize_selected_ids(
            selected_ids.get(),
            &item_ids.get_value(),
            selection_mode,
        )
    });

    let state = Memo::new(move |_| {
        action_logic::action_group_logic::resolve_state(ActionGroupStateInput {
            tone,
            selection_mode,
            disabled,
            item_count: items.get_value().len(),
            selected_count: resolved_selected_ids.get().len(),
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| {
        action_logic::action_group_logic::compose_class_name(class_name.get_value(), state.get())
    });

    view! {
        <div
            id=move || id_base.get_value()
            class=move || class.get()
            data-slot="action-group"
            data-tone=move || state.get().tone_attr
            data-selection-mode=move || state.get().selection_mode_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-item-count=move || state.get().item_count.to_string()
            data-selected-count=move || state.get().selected_count.to_string()
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="toolbar"
            aria-label=aria_label
        >
            <ul class="ui-action-group__list" data-slot="action-group-list">
                {move || {
                    let resolved_selected_ids = resolved_selected_ids.get();
                    items
                        .get_value()
                        .into_iter()
                        .enumerate()
                        .map(|(index, item)| {
                            let item_id_for_action = item.id.clone();
                            let item_id_for_selection = item.id.clone();
                            let is_item_disabled = disabled || item.disabled;
                            let is_selected = resolved_selected_ids.contains(&item.id);
                            let item_node_id =
                                format!("{}-item-{}", id_base.get_value(), index + 1);

                            let on_click = move |_| {
                                if is_item_disabled {
                                    return;
                                }

                                if let Some(on_action) = on_action.get_value() {
                                    on_action.run(item_id_for_action.clone());
                                }

                                let selected_ids = action_logic::action_group_logic::sanitize_selected_ids(
                                    selected_ids.get_untracked(),
                                    &item_ids.get_value(),
                                    selection_mode,
                                );
                                let next = action_logic::action_group_logic::toggle_selected_id(
                                    selected_ids,
                                    &item_id_for_selection,
                                    &item_ids.get_value(),
                                    selection_mode,
                                );
                                request_selected_change.run(next);
                            };

                            let item_class = format!(
                                "ui-action-group__item{}{}",
                                if is_selected {
                                    " ui-action-group__item--selected"
                                } else {
                                    ""
                                },
                                if is_item_disabled {
                                    " ui-action-group__item--disabled"
                                } else {
                                    ""
                                }
                            );

                            view! {
                                <li class="ui-action-group__node" data-slot="action-group-node" data-index=index>
                                    <button
                                        id=item_node_id
                                        type="button"
                                        class=item_class
                                        data-slot="action-group-item"
                                        data-id=item.id.clone()
                                        data-selected=is_selected.then_some("true")
                                        data-disabled=is_item_disabled.then_some("true")
                                        disabled=is_item_disabled
                                        aria-pressed=if is_selected { Some("true") } else { Some("false") }
                                        on:click=on_click
                                    >
                                        {item.label}
                                    </button>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </div>
    }
}
