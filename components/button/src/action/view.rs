use super::super::{
    ButtonColor, ButtonRadius, logic as button_logic, motion as button_motion, view as button_view,
};
use super::logic as action_logic;
#[cfg(feature = "component-action_button_group")]
use super::motion as action_motion;
#[cfg(feature = "component-action_button_group")]
use super::{ActionButtonGroupDensity, ActionButtonGroupMotion, ActionButtonGroupOrientation};
use super::{ActionButtonLoadingPlacement, ActionButtonMotion, ActionButtonSize, ActionButtonType};
#[cfg(feature = "component-action_group")]
use super::{ActionGroupItem, ActionGroupSelectionMode, ActionGroupStateInput, ActionGroupTone};
use leptos::{html, prelude::*};
#[cfg(feature = "component-action_group")]
use std::collections::BTreeSet;
#[cfg(any(
    feature = "component-action_button_group",
    feature = "component-action_group"
))]
use ui_headless::labeled_toolbar_attrs;
#[cfg(feature = "component-action_group")]
use ui_headless::use_controllable_state;
use ui_headless::{
    A11yDirection, ButtonOptions, FocusRingOptions, HoverOptions, OnPress, popup_trigger_attrs,
    use_button, use_focus_ring, use_hover,
};

#[cfg(feature = "component-action_group")]
#[derive(Clone, Copy)]
struct ActionGroupRenderContext {
    id_base: StoredValue<String>,
    selection_mode: ActionGroupSelectionMode,
    selected_ids: Signal<BTreeSet<String>>,
    item_ids: StoredValue<BTreeSet<String>>,
    request_selected_change: Callback<BTreeSet<String>>,
    on_action: StoredValue<Option<Callback<String>>>,
}

#[component]
pub fn ActionButton(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] is_loading: bool,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] size: Option<ActionButtonSize>,
    #[prop(optional)] is_quiet: Option<bool>,
    #[prop(optional)] motion: ActionButtonMotion,
    #[prop(optional)] loading_placement: ActionButtonLoadingPlacement,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] button_type: Option<ActionButtonType>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] aria_haspopup: Option<&'static str>,
    #[prop(optional)] aria_expanded: Option<Signal<bool>>,
    #[prop(optional, into)] aria_controls: Option<String>,
    #[prop(optional)] aria_controls_signal: Option<Signal<Option<String>>>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
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

    let resolved = action_logic::action_button_logic::resolve_input(
        action_logic::action_button_logic::ActionButtonInputResolutionInput {
            is_disabled,
            inherited_disabled,
            size,
            inherited_size,
            is_quiet,
            inherited_quiet,
        },
    );
    let is_quiet = resolved.is_quiet;
    let variant = resolved.variant;

    let normalized = button_logic::normalize_input(button_logic::ButtonInputNormalizationInput {
        is_disabled: resolved.is_disabled,
        is_full_width: false,
        class_name,
        aria_label,
        button_type: action_logic::action_button_logic::resolve_button_type(button_type),
    });
    let aria_label = normalized.aria_label.clone();
    let aria_label_source = normalized.aria_label_source;
    let has_custom_motion = motion != ActionButtonMotion::default();
    let button_type = normalized.button_type;

    let view_state = button_logic::resolve_view_state(button_logic::ButtonLogicInput {
        normalized,
        is_loading,
        variant,
        color: ButtonColor::default(),
        radius: ButtonRadius::default(),
        size: resolved.size,
        loading_placement,
        has_custom_motion,
    });
    let state = view_state.state;
    let render = view_state.render;

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

    let class = view_state.class_name;
    let popup_a11y = popup_trigger_attrs(
        aria_haspopup,
        aria_controls,
        aria_controls_signal,
        aria_expanded,
        lang,
        dir,
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
            data-full-width=state.is_full_width.then_some("true")
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
            lang=popup_a11y.lang.clone()
            dir=popup_a11y.dir
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
            {button_view::render_button_content(render, children)}
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
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] motion: ActionButtonGroupMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let i18n = ui_headless::use_ui_i18n();
    let common_strings = i18n.strings::<ui_headless::CommonStrings>();
    let _aria_label_fallback = common_strings.action_button_group_aria_label.as_ref();
    let class_name = action_logic::action_button_group_logic::normalize_optional_text(class_name);
    let normalized_aria_label =
        action_logic::action_button_group_logic::normalize_aria_label(aria_label);
    let (aria_label, has_explicit_label) = normalized_aria_label;

    let state = action_logic::action_button_group_logic::resolve_state(
        orientation,
        density,
        is_justified,
        is_quiet,
        is_disabled,
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
    let toolbar_a11y = labeled_toolbar_attrs(
        aria_label,
        state.orientation.aria_orientation(),
        state.is_disabled,
        lang,
        dir,
    );

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
            role=toolbar_a11y.role
            aria-label=toolbar_a11y.aria_label.clone()
            aria-orientation=toolbar_a11y.aria_orientation
            aria-disabled=toolbar_a11y.aria_disabled
            lang=toolbar_a11y.lang.clone()
            dir=toolbar_a11y.dir
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
    #[prop(optional)] is_disabled: bool,
    #[prop(optional, into)] selected_ids: Option<Signal<BTreeSet<String>>>,
    #[prop(optional)] default_selected_ids: Option<BTreeSet<String>>,
    #[prop(optional)] on_selected_ids_change: Option<Callback<BTreeSet<String>>>,
    #[prop(optional)] on_action: Option<Callback<String>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let i18n = ui_headless::use_ui_i18n();
    let common_strings = i18n.strings::<ui_headless::CommonStrings>();
    let items = action_logic::action_group_logic::normalize_items(items);
    let item_ids = action_logic::action_group_logic::collect_item_ids(&items);

    let default_selected_ids = action_logic::action_group_logic::normalize_default_selected_ids(
        default_selected_ids,
        &item_ids,
        selection_mode,
    );
    let is_selection_controlled = selected_ids.is_some();

    let selected_state = use_controllable_state(
        selected_ids,
        Some(default_selected_ids),
        on_selected_ids_change,
    );
    let selected_ids = selected_state.value;
    let request_selected_change = selected_state.request_change;

    let (aria_label, has_custom_aria_label) =
        action_logic::action_group_logic::normalize_aria_label(
            aria_label,
            common_strings.action_group_aria_label.as_ref(),
        );

    let class_name = action_logic::action_group_logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let id_base = StoredValue::new(id_base);
    let items = StoredValue::new(items);
    let item_ids = StoredValue::new(item_ids);
    let on_action = StoredValue::new(on_action);
    let render_context = ActionGroupRenderContext {
        id_base,
        selection_mode,
        selected_ids,
        item_ids,
        request_selected_change,
        on_action,
    };

    let resolved_selected_ids = Memo::new(move |_| {
        action_logic::action_group_logic::resolve_selected_ids(
            selected_ids.get(),
            &item_ids.get_value(),
            selection_mode,
        )
    });

    let state = Memo::new(move |_| {
        action_logic::action_group_logic::resolve_state(ActionGroupStateInput {
            tone,
            selection_mode,
            is_disabled,
            is_selection_controlled,
            item_count: items.get_value().len(),
            selected_count: resolved_selected_ids.get().len(),
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| {
        action_logic::action_group_logic::compose_class_name(class_name.get_value(), state.get())
    });
    let toolbar_a11y = labeled_toolbar_attrs(aria_label, "horizontal", is_disabled, lang, dir);

    view! {
        <div
            id=move || id_base.get_value()
            class=move || class.get()
            data-slot="action-group"
            data-tone=move || state.get().tone_attr
            data-selection-mode=move || state.get().selection_mode_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-selection-source=move || state.get().selection_source_attr
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-item-count=move || state.get().item_count.to_string()
            data-selected-count=move || state.get().selected_count.to_string()
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role=toolbar_a11y.role
            aria-label=toolbar_a11y.aria_label.clone()
            aria-orientation=toolbar_a11y.aria_orientation
            aria-disabled=toolbar_a11y.aria_disabled
            lang=toolbar_a11y.lang.clone()
            dir=toolbar_a11y.dir
        >
            <ul class="ui-action-group__list" data-slot="action-group-list">
                {move || {
                    render_action_group_items(
                        render_context,
                        items.get_value(),
                        resolved_selected_ids.get(),
                        is_disabled,
                    )
                }}
            </ul>
        </div>
    }
}

#[cfg(feature = "component-action_group")]
fn render_action_group_items(
    render_context: ActionGroupRenderContext,
    items: Vec<ActionGroupItem>,
    resolved_selected_ids: BTreeSet<String>,
    is_disabled: bool,
) -> impl IntoView {
    items
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            let is_selected = resolved_selected_ids.contains(&item.id);
            let item_render_state = action_logic::action_group_logic::resolve_item_render_state(
                is_disabled,
                item.disabled,
                is_selected,
            );
            render_action_group_item(render_context, index, item, item_render_state)
        })
        .collect_view()
}

#[cfg(feature = "component-action_group")]
fn render_action_group_item(
    render_context: ActionGroupRenderContext,
    index: usize,
    item: ActionGroupItem,
    item_render_state: action_logic::action_group_logic::ActionGroupItemRenderState,
) -> impl IntoView {
    let item_id_for_action = item.id.clone();
    let item_id_for_selection = item.id.clone();
    let item_node_id = format!("{}-item-{}", render_context.id_base.get_value(), index + 1);
    let is_item_disabled = item_render_state.is_disabled;
    let is_selected = item_render_state.is_selected;
    let item_class = item_render_state.class_name;

    let on_click = move |_| {
        let next = action_logic::action_group_logic::resolve_next_selected_ids(
            render_context.selected_ids.get_untracked(),
            &item_id_for_selection,
            &render_context.item_ids.get_value(),
            render_context.selection_mode,
            is_item_disabled,
        );

        if let Some(next) = next {
            if let Some(on_action) = render_context.on_action.get_value() {
                on_action.run(item_id_for_action.clone());
            }
            render_context.request_selected_change.run(next);
        }
    };

    view! {
        <li class="ui-action-group__node" data-slot="action-group-node" data-index=index>
            <button
                id=item_node_id
                type="button"
                class=item_class
                data-slot="action-group-item"
                data-id=item.id
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
}
