use super::logic::{
    self, has_disabled_tabs, is_tab_disabled, normalize_selected_with_disabled,
    resolve_selection_request, resolve_tabs_state,
};
use crate::{TabsKeyboardActivation, TabsMotion, motion};
use leptos::{children::Children, ev, html, prelude::*};
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    sync::Arc,
};
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
    ui_observability::observe_js_result!(el.focus());
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

mod item_collection {
    use super::*;

    pub(super) struct TabsItemConfig {
        pub(super) registration_id: usize,
        pub(super) label: String,
        pub(super) panel: AnyView,
    }

    pub(super) struct ResolvedTabsItemConfig {
        pub(super) label: String,
        pub(super) panel: AnyView,
    }

    pub(super) struct CollectedTabsItems {
        pub(super) item_configs: Vec<ResolvedTabsItemConfig>,
        pub(super) items_order: Vec<usize>,
    }

    #[derive(Clone, Copy)]
    struct RegistrationContext {
        items: StoredValue<Vec<TabsItemConfig>, LocalStorage>,
        is_collecting: StoredValue<bool, LocalStorage>,
        next_registration_id: StoredValue<usize, LocalStorage>,
        registered_ids: StoredValue<BTreeSet<usize>, LocalStorage>,
        registration_actions: StoredValue<Vec<logic::TabsRegistrationAction>, LocalStorage>,
    }

    impl RegistrationContext {
        fn register(self) -> usize {
            let registration_id = self.next_registration_id.get_value();
            self.next_registration_id
                .set_value(registration_id.saturating_add(1));
            self.registered_ids.update_value(|registered| {
                registered.insert(registration_id);
            });
            self.registration_actions.update_value(|actions| {
                actions.push(logic::TabsRegistrationAction::Register { registration_id });
            });
            registration_id
        }

        fn unregister(self, registration_id: usize) {
            self.registration_actions.update_value(|actions| {
                actions.push(logic::TabsRegistrationAction::Unregister { registration_id });
            });
        }
    }

    pub(super) fn collect(children: Children) -> CollectedTabsItems {
        let items = StoredValue::new_local(Vec::new());
        let is_collecting = StoredValue::new_local(true);
        let next_registration_id = StoredValue::new_local(0_usize);
        let registered_ids = StoredValue::new_local(BTreeSet::<usize>::new());
        let registration_actions = StoredValue::new_local(Vec::new());
        let registration = RegistrationContext {
            items,
            is_collecting,
            next_registration_id,
            registered_ids,
            registration_actions,
        };
        provide_context(registration);
        drop(children());
        is_collecting.set_value(false);
        let active_registration_ids = items.with_value(|configs| {
            configs
                .iter()
                .map(|item| item.registration_id)
                .collect::<BTreeSet<_>>()
        });
        let stale_registration_ids = registration.registered_ids.with_value(|registered| {
            registered
                .iter()
                .copied()
                .filter(|registration_id| !active_registration_ids.contains(registration_id))
                .collect::<Vec<_>>()
        });
        for registration_id in stale_registration_ids {
            registration.unregister(registration_id);
        }
        resolve(
            items.into_inner().unwrap_or_default(),
            registration_actions.into_inner().unwrap_or_default(),
        )
    }

    fn resolve(
        item_configs: Vec<TabsItemConfig>,
        registration_actions: Vec<logic::TabsRegistrationAction>,
    ) -> CollectedTabsItems {
        let active_registration_ids = item_configs
            .iter()
            .map(|item| item.registration_id)
            .collect::<Vec<_>>();
        let items_order =
            logic::resolve_registered_items_order(&registration_actions, &active_registration_ids);
        let order_by_registration_id = items_order
            .iter()
            .copied()
            .enumerate()
            .map(|(index, registration_id)| (registration_id, index))
            .collect::<BTreeMap<_, _>>();
        let mut ordered = item_configs;
        ordered.sort_by_key(|item| {
            order_by_registration_id
                .get(&item.registration_id)
                .copied()
                .unwrap_or(usize::MAX)
        });
        CollectedTabsItems {
            item_configs: ordered
                .into_iter()
                .map(|item| ResolvedTabsItemConfig {
                    label: item.label,
                    panel: item.panel,
                })
                .collect(),
            items_order,
        }
    }

    pub(super) fn try_collect(label: String, panel: AnyView) -> Result<(), AnyView> {
        if let Some(registration) = use_context::<RegistrationContext>()
            && registration.is_collecting.get_value()
        {
            let registration_id = registration.register();
            registration.items.update_value(|items| {
                items.push(TabsItemConfig {
                    registration_id,
                    label,
                    panel,
                });
            });
            Ok(())
        } else {
            Err(panel)
        }
    }
}

fn collect_tabs_items(children: Children) -> item_collection::CollectedTabsItems {
    item_collection::collect(children)
}

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
    label: String,
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
    let tab_is_disabled = is_tab_disabled(disabled, disabled_indices.as_ref(), index);

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
                    |idx: usize| is_tab_disabled(disabled, disabled_indices.as_ref(), idx),
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
                |idx: usize| is_tab_disabled(disabled, disabled_indices.as_ref(), idx),
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
    id_base: String,
    #[prop(optional)] keyboard_activation: TabsKeyboardActivation,
    #[prop(optional)] default_selected_index: usize,
    #[prop(optional)] selected_index: Option<ReadSignal<usize>>,
    #[prop(optional)] on_selection_change: Option<Callback<usize>>,
    // Canonical boolean API uses `is_disabled`; keep `disabled` as a legacy alias for
    // compatibility. Resolution priority is handled in logic.rs: `is_disabled > disabled`.
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
    let item_collection::CollectedTabsItems {
        item_configs,
        items_order,
    } = collect_tabs_items(children);
    let item_count = item_configs.len();
    debug_assert_eq!(
        items_order.len(),
        item_count,
        "tabs items_order should stay in sync with collected item count"
    );
    let labels = item_configs
        .iter()
        .map(|item| item.label.clone())
        .collect::<Vec<_>>();
    let panels = item_configs
        .into_iter()
        .map(|item| item.panel)
        .collect::<Vec<_>>();

    let disabled_axis = logic::normalize_disabled_axis(is_disabled, disabled);
    let disabled = disabled_axis.is_disabled;
    let disabled_source = disabled_axis.source.as_attr();
    let (item_count_signal, _set_item_count) = signal(item_count);

    let disabled_indices: Arc<HashSet<usize>> = Arc::new(disabled_indices.into_iter().collect());
    let has_disabled_tabs = has_disabled_tabs(disabled, disabled_indices.as_ref());

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

    let initial_selected = normalize_selected_with_disabled(
        logic::resolve_requested_selected_index(
            requested_controlled_selected_index,
            default_selected_index,
        ),
        item_count,
        {
            let disabled_indices = disabled_indices.clone();
            move |index: usize| is_tab_disabled(disabled, disabled_indices.as_ref(), index)
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
            normalize_selected_with_disabled(selected_raw.get(), item_count, |index: usize| {
                is_tab_disabled(disabled, disabled_indices.as_ref(), index)
            })
        }
    });

    let state =
        Signal::derive(move || resolve_tabs_state(item_count, selected.get(), has_disabled_tabs));

    let set_selected = Callback::new({
        let disabled_indices = disabled_indices.clone();
        move |index: usize| {
            if let Some(next) = resolve_selection_request(
                index,
                selected.get_untracked(),
                item_count,
                |idx: usize| is_tab_disabled(disabled, disabled_indices.as_ref(), idx),
            ) {
                request_selected_change.run(next);
            }
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

#[component]
pub fn TabsItem(#[prop(into)] label: String, children: Children) -> impl IntoView {
    let panel = children().into_any();
    match item_collection::try_collect(label, panel) {
        Ok(()) => ().into_any(),
        Err(panel) => panel,
    }
}
