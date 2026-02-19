use super::logic::{self, normalize_index_skipping_disabled, resolve_tabs_state};
use crate::tabs::{TabsKeyboardActivation, TabsMotion, motion};
use leptos::{children::ChildrenFragment as Children, ev, html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::{
    A11yDirection, FocusRingOptions, HoverOptions, PressOptions, RovingOrientation,
    RovingTabIndexOptions, TabsInteractionKind, resolve_tabs_selection_intent,
    tabs_list_a11y_attrs, tabs_tab_a11y_attrs, use_controllable_state, use_focus_ring, use_hover,
    use_press, use_roving_tabindex,
};

#[cfg(target_arch = "wasm32")]
fn focus_tab(tab_refs: &Arc<Vec<NodeRef<html::Button>>>, index: usize) {
    let Some(node_ref) = tab_refs.get(index) else {
        return;
    };
    let Some(el) = node_ref.get_untracked() else {
        return;
    };
    drop(el.focus());
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_tab(_tab_refs: &Arc<Vec<NodeRef<html::Button>>>, _index: usize) {}

const CLASS_TABS_TAB: &str = "ui-tabs__tab";
const CLASS_TABS_PANEL: &str = "ui-tabs__panel";
const CLASS_TABS_LIST: &str = "ui-tabs__list";
const CLASS_TABS_INDICATOR: &str = "ui-tabs__indicator";

const SLOT_TABS: &str = "tabs";
const SLOT_TABS_LIST: &str = "tabs-list";
const SLOT_TABS_INDICATOR: &str = "tabs-indicator";
const SLOT_TABS_TAB: &str = "tabs-tab";
const SLOT_TABS_PANEL: &str = "tabs-panel";

const ROLE_TABPANEL: &str = "tabpanel";
const ARIA_TRUE: &str = "true";
const KEYBOARD_ACTIVATION_AUTOMATIC: &str = "automatic";
const KEYBOARD_ACTIVATION_MANUAL: &str = "manual";

#[derive(Clone)]
struct TabButtonRenderContext {
    id_base: String,
    selected: Signal<usize>,
    disabled: bool,
    disabled_indices: Arc<HashSet<usize>>,
    item_count: usize,
    keyboard_activation: TabsKeyboardActivation,
    set_selected: Callback<usize>,
    roving: ui_headless::RovingTabIndexState,
    tab_refs: Arc<Vec<NodeRef<html::Button>>>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
}

fn render_tab_button(
    index: usize,
    label: &'static str,
    node_ref: NodeRef<html::Button>,
    context: TabButtonRenderContext,
) -> AnyView {
    let TabButtonRenderContext {
        id_base,
        selected,
        disabled,
        disabled_indices,
        item_count,
        keyboard_activation,
        set_selected,
        roving,
        tab_refs,
        lang,
        dir,
    } = context;
    let tab_id = format!("{id_base}-tab-{index}");
    let panel_id = format!("{id_base}-panel-{index}");

    let tab_is_selected = Signal::derive(move || selected.get() == index);
    let tab_is_disabled = disabled || disabled_indices.contains(&index);

    let tab_a11y = tabs_tab_a11y_attrs(
        tab_is_selected,
        panel_id.clone(),
        tab_is_disabled,
        lang,
        dir,
    );
    let tab_role = tab_a11y.role;
    let tab_aria_selected = tab_a11y.aria_selected;
    let tab_aria_controls = tab_a11y.aria_controls;
    let tab_aria_disabled = tab_a11y.aria_disabled;
    let tab_lang = tab_a11y.lang;
    let tab_dir = tab_a11y.dir;

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: tab_is_disabled,
    });
    let hover = use_hover(HoverOptions {
        is_disabled: tab_is_disabled,
    });
    let press = use_press(PressOptions {
        is_disabled: tab_is_disabled,
        on_press: Some(Callback::new({
            let disabled_indices = disabled_indices.clone();
            move |_| {
                let current = selected.get_untracked();
                if let Some(next) = resolve_tabs_selection_intent(
                    current,
                    index,
                    item_count,
                    |idx: usize| disabled || disabled_indices.contains(&idx),
                    keyboard_activation,
                    TabsInteractionKind::Press,
                ) {
                    set_selected.run(next);
                }
            }
        })),
        prevent_default_for_keyboard: true,
        ..Default::default()
    });

    let on_focus = {
        let disabled_indices = disabled_indices.clone();
        move |_| {
            roving.handlers.on_item_focus.run(index);
            focus_ring.handlers.on_focus.run(());
            let current = selected.get_untracked();
            if let Some(next) = resolve_tabs_selection_intent(
                current,
                index,
                item_count,
                |idx: usize| disabled || disabled_indices.contains(&idx),
                keyboard_activation,
                TabsInteractionKind::Focus,
            ) {
                set_selected.run(next);
            }
        }
    };

    let on_blur = move |_| {
        press.handlers.on_blur.run(());
        focus_ring.handlers.on_blur.run(());
    };

    let on_key_down = {
        let on_key_down = roving.handlers.on_key_down;
        let on_press_key_down = press.handlers.on_key_down;
        let active_index = roving.active_index;
        let tab_refs = tab_refs.clone();
        move |ev: ev::KeyboardEvent| {
            let key = ev.key();
            let handled_roving = on_key_down.run(key.clone());
            let handled_press = on_press_key_down.run(key);
            if handled_roving || handled_press {
                ev.prevent_default();
            }
            if handled_roving {
                focus_tab(&tab_refs, active_index.get_untracked());
            }
        }
    };

    let on_key_up = move |ev: ev::KeyboardEvent| {
        let key = ev.key();
        if press.handlers.on_key_up.run(key) {
            ev.prevent_default();
        }
    };

    view! {
        <button
            type="button"
            class=CLASS_TABS_TAB
            id=tab_id
            node_ref=node_ref
            role=tab_role
            class:ui-tabs__tab--focus-visible=move || focus_ring.is_focus_visible.get()
            disabled=tab_is_disabled
            tabindex=move || if roving.active_index.get() == index { 0 } else { -1 }
            aria-selected=move || tab_aria_selected.get()
            aria-controls=tab_aria_controls
            aria-disabled=tab_aria_disabled
            lang=tab_lang
            dir=tab_dir
            data-slot=SLOT_TABS_TAB
            data-index=index
            data-selected=move || tab_is_selected.get().then_some("true")
            data-disabled=tab_is_disabled.then_some("true")
            data-hovered=move || hover.is_hovered.get().then_some("true")
            data-pressed=move || press.is_pressed.get().then_some("true")
            data-focused=move || focus_ring.is_focused.get().then_some("true")
            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
            on:pointerdown=move |_| press.handlers.on_pointer_down.run(())
            on:pointerup=move |_| press.handlers.on_pointer_up.run(())
            on:pointercancel=move |_| press.handlers.on_pointer_cancel.run(())
            on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
            on:click=move |_| press.handlers.on_click.run(())
            on:keydown=on_key_down
            on:keyup=on_key_up
            on:focus=on_focus
            on:blur=on_blur
        >
            {label}
        </button>
    }
    .into_any()
}

fn render_tab_panel(
    index: usize,
    panel: AnyView,
    id_base: &str,
    selected: Signal<usize>,
) -> AnyView {
    let tab_id = format!("{id_base}-tab-{index}");
    let panel_id = format!("{id_base}-panel-{index}");
    let is_selected = move || selected.get() == index;

    view! {
        <div
            class=CLASS_TABS_PANEL
            id=panel_id
            role=ROLE_TABPANEL
            aria-labelledby=tab_id
            hidden=move || !is_selected()
            data-slot=SLOT_TABS_PANEL
            data-index=index
            data-selected=move || is_selected().then_some("true")
        >
            {panel}
        </div>
    }
    .into_any()
}

#[component]
pub fn Tabs(
    labels: Vec<&'static str>,
    id_base: String,
    #[prop(optional)] keyboard_activation: TabsKeyboardActivation,
    #[prop(optional)] default_selected_index: usize,
    #[prop(optional)] selected_index: Option<ReadSignal<usize>>,
    #[prop(optional)] on_selection_change: Option<Callback<usize>>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] motion: TabsMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let panels = children().nodes;

    debug_assert_eq!(
        labels.len(),
        panels.iter().len(),
        "Tabs: expected `labels.len() == children.len()`; got labels={}, children={}",
        labels.len(),
        panels.iter().len()
    );

    let disabled_axis = logic::normalize_disabled_axis(is_disabled, disabled);
    let disabled = disabled_axis.is_disabled;
    let disabled_source = disabled_axis.source.as_attr();
    let item_count = labels.len().min(panels.iter().len());
    let (item_count_signal, _set_item_count) = signal(item_count);

    let disabled_indices: Arc<HashSet<usize>> = Arc::new(disabled_indices.into_iter().collect());
    let has_disabled_tabs = disabled || !disabled_indices.is_empty();

    let selection_axis = logic::normalize_selection_axis(logic::TabsSelectionAxisInput {
        selected_index,
        default_selected_index,
        on_selection_change,
    });
    let control_mode = selection_axis.control_mode;
    let controlled_selected_index = selection_axis.selected_index;
    let default_selected_index = selection_axis.default_selected_index;
    let on_selection_change = selection_axis.on_selection_change;

    let requested_controlled_selected_index =
        controlled_selected_index.map(|signal| signal.get_untracked());

    let initial_selected = normalize_index_skipping_disabled(
        logic::resolve_requested_selected_index(
            requested_controlled_selected_index,
            default_selected_index,
        ),
        item_count,
        {
            let disabled_indices = disabled_indices.clone();
            move |index: usize| disabled || disabled_indices.contains(&index)
        },
    );

    let selected_state = use_controllable_state(
        controlled_selected_index,
        Some(initial_selected),
        on_selection_change,
    );
    let selected_raw = selected_state.value;
    let request_selected_change = selected_state.request_change;

    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: disabled,
        default_index: initial_selected,
        should_loop: true,
        orientation: RovingOrientation::Horizontal,
        item_count: item_count_signal,
        is_item_disabled: (!disabled_indices.is_empty()).then_some({
            let disabled_indices = disabled_indices.clone();
            Callback::new(move |index: usize| disabled_indices.contains(&index))
        }),
    });

    let selected = Signal::derive({
        let disabled_indices = disabled_indices.clone();
        move || {
            normalize_index_skipping_disabled(selected_raw.get(), item_count, |index: usize| {
                disabled || disabled_indices.contains(&index)
            })
        }
    });

    let state =
        Signal::derive(move || resolve_tabs_state(item_count, selected.get(), has_disabled_tabs));

    let set_selected = Callback::new({
        let disabled_indices = disabled_indices.clone();
        move |index: usize| {
            if item_count == 0 {
                return;
            }
            let next = normalize_index_skipping_disabled(index, item_count, |idx: usize| {
                disabled || disabled_indices.contains(&idx)
            });
            if disabled || disabled_indices.contains(&next) {
                return;
            }
            if selected.get_untracked() == next {
                return;
            }
            request_selected_change.run(next);
        }
    });

    let tab_refs: Arc<Vec<NodeRef<html::Button>>> =
        Arc::new((0..item_count).map(|_| NodeRef::new()).collect());

    let list_ref: NodeRef<html::Div> = NodeRef::new();
    let indicator_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(list_ref, indicator_ref, tab_refs.clone(), selected, motion);

    let class = logic::compose_class_name(class_name);

    let has_custom_motion = motion != TabsMotion::default();
    let motion_source = logic::resolve_motion_source(has_custom_motion);
    let custom_motion = has_custom_motion.then_some("true");
    let list_a11y = tabs_list_a11y_attrs(
        logic::normalize_optional_text(aria_label),
        lang.clone(),
        dir,
    );
    let is_controlled = control_mode.is_controlled();
    let control_mode = control_mode.as_attr();

    let tabs_view = labels
        .into_iter()
        .take(item_count)
        .enumerate()
        .map({
            let id_base = id_base.clone();
            let roving = roving.clone();
            let tab_refs = tab_refs.clone();
            let disabled_indices = disabled_indices.clone();
            let lang = lang.clone();
            move |(index, label)| {
                let node_ref = tab_refs[index];
                render_tab_button(
                    index,
                    label,
                    node_ref,
                    TabButtonRenderContext {
                        id_base: id_base.clone(),
                        selected,
                        disabled,
                        disabled_indices: disabled_indices.clone(),
                        item_count,
                        keyboard_activation,
                        set_selected,
                        roving: roving.clone(),
                        tab_refs: tab_refs.clone(),
                        lang: lang.clone(),
                        dir,
                    },
                )
            }
        })
        .collect_view();

    let panels_view = panels
        .into_iter()
        .take(item_count)
        .enumerate()
        .map({
            let id_base = id_base.clone();
            move |(index, panel)| render_tab_panel(index, panel, &id_base, selected)
        })
        .collect_view();

    view! {
        <div
            class=class
            data-slot=SLOT_TABS
            data-disabled-source=disabled_source
            data-control-mode=control_mode
            data-controlled=is_controlled.then_some("true")
            data-uncontrolled=(!is_controlled).then_some("true")
            data-disabled=disabled.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-selected-index=move || state.get().selected_index.map(|index| index.to_string())
            data-selection-empty=move || state.get().selected_index.is_none().then_some("true")
            data-has-disabled-tabs=move || state.get().has_disabled_tabs.then_some("true")
            data-keyboard-activation=match keyboard_activation {
                TabsKeyboardActivation::Automatic => KEYBOARD_ACTIVATION_AUTOMATIC,
                TabsKeyboardActivation::Manual => KEYBOARD_ACTIVATION_MANUAL,
            }
            data-motion-source=motion_source
            data-custom-motion=custom_motion
        >
            <div
                class=CLASS_TABS_LIST
                node_ref=list_ref
                role=list_a11y.role
                aria-label=list_a11y.aria_label
                lang=list_a11y.lang
                dir=list_a11y.dir
                data-slot=SLOT_TABS_LIST
            >
                <div
                    class=CLASS_TABS_INDICATOR
                    node_ref=indicator_ref
                    aria-hidden=ARIA_TRUE
                    data-slot=SLOT_TABS_INDICATOR
                ></div>
                {tabs_view}
            </div>
            {panels_view}
        </div>
    }
}
