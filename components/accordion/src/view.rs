use crate::{
    AccordionMotion, AccordionPanelLifecycleEvent, AccordionSelectionMode, AccordionSlotProjection,
    AccordionVariant, logic, motion,
};
use leptos::{children::Children, ev, html, prelude::*};
use std::{collections::BTreeSet, sync::Arc};
use ui_ai_runtime::use_ai_space_state;
use ui_headless::a11y::{A11yDirection, disclosure_trigger_attrs};
use ui_headless::{
    FocusRingOptions, HoverOptions, PressOptions, RovingOrientation, RovingTabIndexOptions,
    use_focus_ring, use_hover, use_press, use_roving_tabindex, use_ui_id_provider,
};

#[cfg(all(
    feature = "accordion-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
))]
mod wasm_debug {
    use super::logic;
    use leptos::prelude::*;
    use std::collections::BTreeSet;

    #[derive(Clone)]
    pub struct DebugEvent {
        pub sequence: usize,
        pub timestamp_ms: f64,
        pub source: logic::AccordionOpenChangeSource,
        pub before: BTreeSet<usize>,
        pub after: BTreeSet<usize>,
    }

    #[derive(Clone, Copy)]
    pub struct DebugStore {
        sequence: RwSignal<usize>,
        pub events: RwSignal<Vec<DebugEvent>>,
    }

    impl DebugStore {
        pub fn new() -> Self {
            Self {
                sequence: RwSignal::new(0),
                events: RwSignal::new(Vec::new()),
            }
        }

        pub fn record(
            self,
            source: logic::AccordionOpenChangeSource,
            before: &BTreeSet<usize>,
            after: &BTreeSet<usize>,
        ) {
            let sequence = self.sequence.get_untracked().saturating_add(1);
            self.sequence.set(sequence);
            let timestamp_ms = js_sys::Date::now();
            self.events.update(|events| {
                events.push(DebugEvent {
                    sequence,
                    timestamp_ms,
                    source,
                    before: before.clone(),
                    after: after.clone(),
                });
            });
            tracing::event!(
                target: "ui::accordion::state_change",
                tracing::Level::DEBUG,
                sequence,
                timestamp_ms,
                source = source.as_str(),
                before = %format_indices(before),
                after = %format_indices(after),
                "accordion state transition"
            );
        }
    }

    pub fn format_indices(values: &BTreeSet<usize>) -> String {
        format!("{values:?}")
    }
}

const ACCORDION_BASE_CLASS: &str = "ui-accordion";
const ACCORDION_INDICATOR_GLYPH: &str = "›";

mod item_collection {
    use super::{AccordionPanelLifecycleEvent, Arc, Children, logic};
    use leptos::prelude::*;
    use std::collections::{BTreeMap, BTreeSet};

    pub(super) struct AccordionItemConfig {
        pub(super) registration_id: usize,
        pub(super) label: String,
        pub(super) key: Option<usize>,
        pub(super) is_disabled: bool,
        pub(super) open: Option<Signal<bool>>,
        pub(super) default_open: bool,
        pub(super) on_open_change: Option<Callback<bool>>,
        pub(super) on_panel_lifecycle: Option<Callback<AccordionPanelLifecycleEvent>>,
        pub(super) panel: AnyView,
    }

    pub(super) struct TryCollectInput {
        pub(super) label: String,
        pub(super) key: Option<usize>,
        pub(super) is_disabled: bool,
        pub(super) open: Option<Signal<bool>>,
        pub(super) default_open: bool,
        pub(super) on_open_change: Option<Callback<bool>>,
        pub(super) on_panel_lifecycle: Option<Callback<AccordionPanelLifecycleEvent>>,
        pub(super) panel: AnyView,
    }

    pub(super) struct ResolvedAccordionItemConfig {
        pub(super) key: usize,
        pub(super) label: String,
        pub(super) is_disabled: bool,
        pub(super) open: Option<Signal<bool>>,
        pub(super) default_open: bool,
        pub(super) on_open_change: Option<Callback<bool>>,
        pub(super) on_panel_lifecycle: Option<Callback<AccordionPanelLifecycleEvent>>,
        pub(super) panel: AnyView,
    }

    pub(super) struct CollectedAccordionItems {
        pub(super) item_configs: Vec<ResolvedAccordionItemConfig>,
        pub(super) items_order: Vec<usize>,
    }

    #[derive(Clone, Copy)]
    struct RegistrationContext {
        items: StoredValue<Vec<AccordionItemConfig>, LocalStorage>,
        is_collecting: StoredValue<bool, LocalStorage>,
        next_registration_id: StoredValue<usize, LocalStorage>,
        registered_ids: StoredValue<BTreeSet<usize>, LocalStorage>,
        registration_actions: StoredValue<Vec<logic::AccordionRegistrationAction>, LocalStorage>,
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
                actions.push(logic::AccordionRegistrationAction::Register { registration_id });
            });
            registration_id
        }

        fn unregister(self, registration_id: usize) {
            self.registration_actions.update_value(|actions| {
                actions.push(logic::AccordionRegistrationAction::Unregister { registration_id });
            });
        }
    }

    pub(super) fn collect(children: Children) -> CollectedAccordionItems {
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

    pub(super) fn try_collect(input: TryCollectInput) -> Result<(), AnyView> {
        let TryCollectInput {
            label,
            key,
            is_disabled,
            open,
            default_open,
            on_open_change,
            on_panel_lifecycle,
            panel,
        } = input;
        if let Some(registration) = use_context::<RegistrationContext>()
            && registration.is_collecting.get_value()
        {
            let registration_id = registration.register();
            registration.items.update_value(|items| {
                items.push(AccordionItemConfig {
                    registration_id,
                    label,
                    key,
                    is_disabled,
                    open,
                    default_open,
                    on_open_change,
                    on_panel_lifecycle,
                    panel,
                });
            });
            Ok(())
        } else {
            Err(panel)
        }
    }

    pub(super) fn runtime_init(
        item_configs: &[ResolvedAccordionItemConfig],
    ) -> logic::AccordionRuntimeInit {
        let inputs = item_configs
            .iter()
            .map(|item| logic::AccordionItemStateInput {
                key: item.key,
                open: item.open.map(|signal| signal.get_untracked()),
                default_open: item.default_open,
                is_disabled: item.is_disabled,
            })
            .collect::<Vec<_>>();
        logic::derive_runtime_init(&inputs)
    }

    pub(super) fn open_callbacks(
        item_configs: &[ResolvedAccordionItemConfig],
    ) -> Arc<Vec<(usize, Option<Callback<bool>>)>> {
        Arc::new(
            item_configs
                .iter()
                .map(|item| (item.key, item.on_open_change))
                .collect::<Vec<_>>(),
        )
    }

    pub(super) fn disabled_flags(item_configs: &[ResolvedAccordionItemConfig]) -> Arc<Vec<bool>> {
        Arc::new(item_configs.iter().map(|item| item.is_disabled).collect())
    }

    fn resolve(
        item_configs: Vec<AccordionItemConfig>,
        registration_actions: Vec<logic::AccordionRegistrationAction>,
    ) -> CollectedAccordionItems {
        let configured_keys = item_configs.iter().map(|item| item.key).collect::<Vec<_>>();
        let resolved_keys = logic::assign_item_keys(&configured_keys);
        let registration_key_pairs = item_configs
            .iter()
            .zip(resolved_keys.iter())
            .map(|(item, key)| (item.registration_id, *key))
            .collect::<Vec<_>>();
        let items_order =
            logic::resolve_registered_item_keys(&registration_actions, &registration_key_pairs);
        let order_by_key = items_order
            .iter()
            .copied()
            .enumerate()
            .map(|(index, key)| (key, index))
            .collect::<BTreeMap<_, _>>();
        let mut resolved = item_configs
            .into_iter()
            .zip(resolved_keys)
            .map(|(item, key)| ResolvedAccordionItemConfig {
                key,
                label: item.label,
                is_disabled: item.is_disabled,
                open: item.open,
                default_open: item.default_open,
                on_open_change: item.on_open_change,
                on_panel_lifecycle: item.on_panel_lifecycle,
                panel: item.panel,
            })
            .collect::<Vec<_>>();
        resolved.sort_by_key(|item| order_by_key.get(&item.key).copied().unwrap_or(usize::MAX));
        CollectedAccordionItems {
            item_configs: resolved,
            items_order,
        }
    }
}

mod state_bindings {
    use super::{BTreeSet, item_collection::ResolvedAccordionItemConfig, logic};
    use leptos::prelude::*;

    pub(super) fn mount_controlled_open_sync_effects(
        item_configs: &[ResolvedAccordionItemConfig],
        open_keys: RwSignal<BTreeSet<usize>>,
        commit_open_change: Callback<(BTreeSet<usize>, logic::AccordionOpenChangeSource, bool)>,
    ) {
        for item in item_configs {
            let Some(open) = item.open else {
                continue;
            };
            let key = item.key;
            Effect::new(move |_| {
                let should_open = open.get();
                let next =
                    logic::apply_external_item_sync(&open_keys.get_untracked(), key, should_open);
                commit_open_change.run((
                    next,
                    logic::AccordionOpenChangeSource::ExternalSync,
                    false,
                ));
            });
        }
    }
}

use item_collection::ResolvedAccordionItemConfig;

fn collect_accordion_items(children: Children) -> item_collection::CollectedAccordionItems {
    item_collection::collect(children)
}

#[component]
pub fn AccordionItem(
    #[prop(into)] label: String,
    #[prop(optional)] key: Option<usize>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: bool,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] on_panel_lifecycle: Option<Callback<AccordionPanelLifecycleEvent>>,
    children: Children,
) -> impl IntoView {
    let panel = children().into_any();
    match item_collection::try_collect(item_collection::TryCollectInput {
        label,
        key,
        is_disabled,
        open,
        default_open,
        on_open_change,
        on_panel_lifecycle,
        panel,
    }) {
        Ok(()) => ().into_any(),
        Err(panel) => panel,
    }
}

fn compose_root_class_name(class_name: Option<String>) -> String {
    class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{ACCORDION_BASE_CLASS} {value}"))
        .unwrap_or_else(|| ACCORDION_BASE_CLASS.to_string())
}

fn render_item_label(label: String) -> impl IntoView {
    view! {
        <span class="ui-accordion__label" data-slot="accordion-label">
            {label}
        </span>
    }
}

fn render_item_indicator(indicator_ref: NodeRef<html::Span>) -> impl IntoView {
    view! {
        <span
            class="ui-accordion__indicator"
            node_ref=indicator_ref
            aria-hidden="true"
            data-slot="accordion-indicator"
        >
            {ACCORDION_INDICATOR_GLYPH}
        </span>
    }
}

struct AccordionPanelRenderInput<V: IntoView + 'static> {
    panel_id: String,
    panel_ref: NodeRef<html::Div>,
    trigger_id: String,
    panel_hidden: RwSignal<bool>,
    panel_lifecycle: RwSignal<AccordionPanelLifecycleEvent>,
    open: Signal<bool>,
    index: usize,
    slot_projection: AccordionSlotProjection,
    render_surface: Signal<bool>,
    panel_surface_ref: NodeRef<html::Div>,
    panel: V,
}

fn render_item_panel<V: IntoView + 'static>(input: AccordionPanelRenderInput<V>) -> impl IntoView {
    let AccordionPanelRenderInput {
        panel_id,
        panel_ref,
        trigger_id,
        panel_hidden,
        panel_lifecycle,
        open,
        index,
        slot_projection,
        render_surface: _render_surface,
        panel_surface_ref,
        panel,
    } = input;
    let panel = panel.into_view().into_any();

    view! {
        <div
            id=panel_id
            class="ui-accordion__panel"
            node_ref=panel_ref
            role="region"
            aria-labelledby=trigger_id
            hidden=move || panel_hidden.get()
            data-open=move || if open.get() { Some("true") } else { None }
            data-index=index
            data-slot-projection=slot_projection.as_str()
            data-panel-lifecycle=move || panel_lifecycle.get().as_str()
            data-slot="accordion-panel"
            data-ui-fragment-kind="accordion-panel"
        >
            <div
                class="ui-accordion__panel-surface"
                node_ref=panel_surface_ref
                data-slot="accordion-panel-surface"
            >
                {panel}
            </div>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn focus_trigger(trigger_refs: &Arc<Vec<NodeRef<html::Button>>>, index: usize) {
    let Some(node_ref) = trigger_refs.get(index) else {
        return;
    };
    let Some(el) = node_ref.get_untracked() else {
        return;
    };
    if let Err(error) = el.focus() {
        ui_observability::warn_js_error("accordion.view.focus_trigger", &error);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_trigger(_trigger_refs: &Arc<Vec<NodeRef<html::Button>>>, _index: usize) {}

#[cfg(all(
    feature = "accordion-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
))]
fn render_debug_panel(
    debug_store: wasm_debug::DebugStore,
    request_open_change: Callback<(BTreeSet<usize>, logic::AccordionOpenChangeSource)>,
) -> impl IntoView {
    let events = debug_store.events;

    view! {
        <details class="ui-accordion__debug" data-slot="accordion-debug" open>
            <summary data-slot="accordion-debug-entry">
                "Accordion Debug (wasm dev)"
            </summary>
            <ul class="ui-accordion__debug-list" data-slot="accordion-debug-list">
                {move || {
                    events
                        .get()
                        .into_iter()
                        .map(|event| {
                            let replay_next = event.after.clone();
                            let sequence = event.sequence;
                            let source = event.source.as_str();
                            let before_text = wasm_debug::format_indices(&event.before);
                            let after_text = wasm_debug::format_indices(&event.after);
                            let timestamp_text = format!("{:.0}", event.timestamp_ms);
                            view! {
                                <li
                                    class="ui-accordion__debug-item"
                                    data-slot="accordion-debug-event"
                                    data-debug-sequence=sequence
                                    data-debug-source=source
                                    data-debug-timestamp-ms=timestamp_text.clone()
                                    data-debug-before=before_text.clone()
                                    data-debug-after=after_text.clone()
                                >
                                    <button
                                        type="button"
                                        class="ui-accordion__debug-replay"
                                        data-slot="accordion-debug-replay"
                                        on:click=move |_| {
                                            request_open_change.run((
                                                replay_next.clone(),
                                                logic::AccordionOpenChangeSource::Programmatic,
                                            ));
                                        }
                                    >
                                        "Replay"
                                    </button>
                                    <span class="ui-accordion__debug-meta">
                                        {format!(
                                            "#{sequence} t={timestamp_text}ms source={source} before={before_text} after={after_text}"
                                        )}
                                    </span>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </details>
    }
}

/// Accordion component with roving focus, disclosure semantics, and optional spring motion.
///
/// Public props:
/// - `id_base`: optional stable prefix used for `id`/`aria-*` wiring; defaults to `UiRoot` `IdProvider` sequence when omitted.
/// - `selection_mode`: single or multiple panel selection behavior.
/// - `variant`: visual variant (`light` / `shadow` / `bordered` / `splitted`).
/// - `disallow_empty_selection`: when `true`, keeps at least one item open.
/// - `is_disabled`: disable all triggers.
/// - `lang`: optional locale tag forwarded to disclosure trigger semantics.
/// - `dir`: optional text direction (`ltr`/`rtl`) for disclosure trigger semantics.
/// - `motion`: per-instance motion contract overrides.
/// - `slot_projection`: panel content projection strategy (`lazy` / `keep-alive` / `eager`).
/// - `class_name`: optional extra class names merged onto root element.
/// - `children`: explicit item composition: `<AccordionItem label=... key=...>...</AccordionItem>`.
#[component]
pub fn Accordion(
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional)] selection_mode: AccordionSelectionMode,
    #[prop(optional)] variant: AccordionVariant,
    #[prop(optional)] disallow_empty_selection: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: AccordionMotion,
    #[prop(optional)] slot_projection: AccordionSlotProjection,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let item_collection::CollectedAccordionItems {
        item_configs,
        items_order,
    } = collect_accordion_items(children);
    let logic::AccordionRuntimeInit {
        has_controlled_open,
        has_default_open,
        item_keys: _,
        requested_open,
        has_per_item_disabled,
    } = item_collection::runtime_init(&item_configs);
    let generated_id_base = use_ui_id_provider()
        .map(|id_provider| id_provider.next_prefixed_id(logic::DEFAULT_ID_BASE_PREFIX))
        .unwrap_or_else(|| logic::DEFAULT_ID_BASE_PREFIX.to_string());
    let id_base = logic::resolve_id_base(id_base, generated_id_base);
    let item_count = item_configs.len();
    let (item_count_signal, _set_item_count) = signal(item_count);
    let item_keys = Arc::new(items_order);
    let initial_open = logic::normalize_default_open_for_items(
        selection_mode,
        Some(&requested_open),
        item_keys.as_slice(),
        disallow_empty_selection,
    );
    let open_state_source = logic::resolve_open_state_source(has_controlled_open);
    let open_init_source = logic::resolve_open_init_source(has_controlled_open, has_default_open);
    let open_last_change_source = RwSignal::new(logic::AccordionOpenChangeSource::Init);
    let pending_open_change_source = RwSignal::new(logic::AccordionOpenChangeSource::Programmatic);
    #[cfg(all(
        feature = "accordion-wasm-debug",
        debug_assertions,
        target_arch = "wasm32"
    ))]
    let debug_store = wasm_debug::DebugStore::new();
    let item_open_callbacks = item_collection::open_callbacks(&item_configs);
    let callback_keys = Arc::new(
        item_open_callbacks
            .iter()
            .map(|(key, _)| *key)
            .collect::<Vec<_>>(),
    );
    let open_keys = RwSignal::new(initial_open);

    let commit_open_change = {
        let item_keys = item_keys.clone();
        let item_open_callbacks = item_open_callbacks.clone();
        let callback_keys = callback_keys.clone();
        #[cfg(all(
            feature = "accordion-wasm-debug",
            debug_assertions,
            target_arch = "wasm32"
        ))]
        let debug_store = debug_store;
        Callback::new(
            move |(next, source, emit_item_callbacks): (
                BTreeSet<usize>,
                logic::AccordionOpenChangeSource,
                bool,
            )| {
                let before = open_keys.get_untracked();
                let Some(plan) = logic::plan_open_commit(
                    selection_mode,
                    &before,
                    &next,
                    item_keys.as_slice(),
                    callback_keys.as_slice(),
                    disallow_empty_selection,
                ) else {
                    return;
                };
                #[cfg(all(
                    feature = "accordion-wasm-debug",
                    debug_assertions,
                    target_arch = "wasm32"
                ))]
                debug_store.record(source, &before, &plan.next);
                let logic::AccordionOpenCommitPlan {
                    next,
                    changed_by_key,
                } = plan;
                open_last_change_source.set(source);
                open_keys.set(next);
                if emit_item_callbacks {
                    for (key, callback) in item_open_callbacks.iter() {
                        if let Some(callback) = callback
                            && let Some(after_open) = changed_by_key.get(key)
                        {
                            callback.run(*after_open);
                        }
                    }
                }
            },
        )
    };
    let request_open_change = {
        Callback::new(
            move |(next, source): (BTreeSet<usize>, logic::AccordionOpenChangeSource)| {
                commit_open_change.run((next, source, true));
            },
        )
    };
    state_bindings::mount_controlled_open_sync_effects(
        &item_configs,
        open_keys,
        commit_open_change,
    );

    let item_disabled_flags = item_collection::disabled_flags(&item_configs);
    let has_disabled_items = is_disabled || has_per_item_disabled;
    let item_disabled_flags_for_cb = item_disabled_flags.clone();
    let is_item_disabled = has_per_item_disabled.then_some(Callback::new(move |index: usize| {
        item_disabled_flags_for_cb
            .get(index)
            .copied()
            .unwrap_or(false)
    }));

    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled,
        default_index: 0,
        should_loop: true,
        orientation: RovingOrientation::Vertical,
        item_count: item_count_signal,
        is_item_disabled,
    });

    let trigger_refs: Arc<Vec<NodeRef<html::Button>>> =
        Arc::new((0..item_count).map(|_| NodeRef::new()).collect());

    let class = compose_root_class_name(class_name);

    let open_keys_for_state = open_keys;
    let state = Signal::derive(move || {
        logic::resolve_state(
            selection_mode,
            item_count,
            open_keys_for_state.get().len(),
            has_disabled_items,
        )
    });
    let open_keys_for_agent = open_keys;
    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(
            open_last_change_source.get(),
            item_count,
            open_keys_for_agent.get().len(),
        )
    });
    let ai_space_state = StoredValue::new(use_ai_space_state());

    let items = item_configs
        .into_iter()
        .enumerate()
        .map({
            let id_base = id_base.clone();
            let roving = roving.clone();
            let trigger_refs = trigger_refs.clone();
            let lang = lang.clone();
            let item_keys = item_keys.clone();
            move |(index, item)| {
                let ResolvedAccordionItemConfig {
                    key,
                    label,
                    is_disabled: is_item_disabled_by_item,
                    on_panel_lifecycle,
                    panel,
                    ..
                } = item;
                let label = logic::resolve_item_label(label, index);
                let trigger_id = format!("{id_base}-trigger-{index}");
                let panel_id = format!("{id_base}-panel-{index}");

                let is_item_disabled = is_disabled || is_item_disabled_by_item;

                let focus_ring = use_focus_ring(FocusRingOptions {
                    is_disabled: is_item_disabled,
                });
                let hover = use_hover(HoverOptions {
                    is_disabled: is_item_disabled,
                });

                let is_open = move || open_keys.with(|set| set.contains(&key));
                let open: Signal<bool> = Signal::derive(is_open);
                let disclosure_a11y =
                    disclosure_trigger_attrs(open, panel_id.clone(), lang.clone(), dir);

                let indicator_ref: NodeRef<html::Span> = NodeRef::new();
                motion::attach_indicator_motion(indicator_ref, open, motion);

                let panel_ref: NodeRef<html::Div> = NodeRef::new();
                let panel_surface_ref: NodeRef<html::Div> = NodeRef::new();
                let panel_hidden = RwSignal::new(!open.get_untracked());
                let panel_lifecycle = RwSignal::new(if open.get_untracked() {
                    AccordionPanelLifecycleEvent::NotifyShown
                } else {
                    AccordionPanelLifecycleEvent::NotifyHidden
                });
                let notify_panel_lifecycle = Callback::new(move |event: AccordionPanelLifecycleEvent| {
                    panel_lifecycle.set(event);
                    if let Some(callback) = on_panel_lifecycle {
                        callback.run(event);
                    }
                });
                motion::attach_panel_motion(
                    panel_ref,
                    panel_surface_ref,
                    open,
                    panel_hidden,
                    motion,
                    slot_projection,
                    notify_panel_lifecycle,
                );
                let panel_has_opened_once = RwSignal::new(open.get_untracked());
                Effect::new(move |_| {
                    if open.get() {
                        panel_has_opened_once.set(true);
                    }
                });
                let render_surface = Signal::derive(move || {
                    logic::should_render_panel_surface(
                        slot_projection,
                        open.get(),
                        panel_has_opened_once.get(),
                    )
                });

                let item_keys_for_press = item_keys.clone();
                let on_press = Callback::new(move |_| {
                    let next = logic::toggle_open_for_items(
                        selection_mode,
                        &open_keys.get_untracked(),
                        key,
                        item_keys_for_press.as_slice(),
                        disallow_empty_selection,
                    );
                    request_open_change.run((next, pending_open_change_source.get_untracked()));
                });

                let press = use_press(PressOptions {
                    is_disabled: is_item_disabled,
                    on_press: Some(on_press),
                    prevent_default_for_keyboard: true,
                    ..Default::default()
                });

                let on_key_down = {
                    let on_key_down = roving.handlers.on_key_down;
                    let on_press_key_down = press.handlers.on_key_down;
                    let active_index = roving.active_index;
                    let trigger_refs = trigger_refs.clone();
                    move |ev: ev::KeyboardEvent| {
                        let key = ev.key();
                        let handled_roving = on_key_down.run(key.clone());
                        let handled_press = on_press_key_down.run(key);

                        if handled_roving || handled_press {
                            ev.prevent_default();
                        }
                        if handled_press {
                            pending_open_change_source
                                .set(logic::AccordionOpenChangeSource::Keyboard);
                        }

                        if handled_roving {
                            focus_trigger(&trigger_refs, active_index.get_untracked());
                        }
                    }
                };

                let on_key_up = move |ev: ev::KeyboardEvent| {
                    let key = ev.key();
                    if press.handlers.on_key_up.run(key) {
                        ev.prevent_default();
                    }
                };

                let node_ref = trigger_refs[index];
                let label_view = render_item_label(label);
                let indicator_view = render_item_indicator(indicator_ref);
                let panel_view = render_item_panel(AccordionPanelRenderInput {
                    panel_id,
                    panel_ref,
                    trigger_id: trigger_id.clone(),
                    panel_hidden,
                    panel_lifecycle,
                    open,
                    index,
                    slot_projection,
                    render_surface,
                    panel_surface_ref,
                    panel,
                });

                view! {
                    <div
                        class="ui-accordion__item"
                        data-slot="accordion-item"
                        data-index=index
                        data-key=key.to_string()
                        data-open=move || if open.get() { Some("true") } else { None }
                    >
                        <button
                            type="button"
                            class="ui-accordion__trigger"
                            class:ui-accordion__trigger--focus-visible=move || focus_ring.is_focus_visible.get()
                            node_ref=node_ref
                            id=trigger_id.clone()
                            disabled=is_item_disabled
                            data-slot="accordion-trigger"
                            data-index=index
                            data-open=move || open.get().then_some("true")
                            tabindex=move || {
                                if is_item_disabled {
                                    -1
                                } else if roving.active_index.get() == index {
                                    0
                                } else {
                                    -1
                                }
                            }
                            aria-expanded=disclosure_a11y.aria_expanded
                            aria-controls=disclosure_a11y.aria_controls.clone()
                            lang=disclosure_a11y.lang.clone()
                            dir=disclosure_a11y.dir
                            data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
                            data-pressed=move || if press.is_pressed.get() { Some("true") } else { None }
                            data-focused=move || focus_ring.is_focused.get().then_some("true")
                            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
                            data-disabled=is_item_disabled.then_some("true")
                            on:focus=move |_| {
                                focus_ring.handlers.on_focus.run(());
                                roving.handlers.on_item_focus.run(index);
                            }
                            on:blur=move |_| {
                                press.handlers.on_blur.run(());
                                focus_ring.handlers.on_blur.run(());
                            }
                            on:keydown=on_key_down
                            on:keyup=on_key_up
                            on:pointerdown=move |_| {
                                pending_open_change_source
                                    .set(logic::AccordionOpenChangeSource::Pointer);
                                press.handlers.on_pointer_down.run(())
                            }
                            on:pointerup=move |_| press.handlers.on_pointer_up.run(())
                            on:pointercancel=move |_| press.handlers.on_pointer_cancel.run(())
                            on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
                            on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
                            on:click=move |_| {
                                pending_open_change_source
                                    .set(logic::AccordionOpenChangeSource::Pointer);
                                press.handlers.on_click.run(())
                            }
                        >
                            {label_view}
                            {indicator_view}
                        </button>

                        {panel_view}
                    </div>
                }
            }
        })
        .collect_view();

    let debug_panel: Option<AnyView> = crate::wasm_debug_proxy!(
        "accordion-wasm-debug",
        { Some(render_debug_panel(debug_store, request_open_change).into_any()) },
        { None }
    );

    view! {
        <div
            class=class
            data-slot="accordion"
            data-disabled=is_disabled.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-open-count=move || state.get().open_count.to_string()
            data-all-closed=move || (!state.get().has_open_items).then_some("true")
            data-multiple-open=move || state.get().has_multiple_open.then_some("true")
            data-has-disabled-items=move || state.get().has_disabled_items.then_some("true")
            data-open-state-source=open_state_source.as_str()
            data-open-init-source=open_init_source.as_str()
            data-open-last-change-source=move || open_last_change_source.get().as_str()
            data-selection-mode=match selection_mode {
                AccordionSelectionMode::Single => "single",
                AccordionSelectionMode::Multiple => "multiple",
            }
            data-slot-projection=slot_projection.as_str()
            data-disallow-empty-selection=disallow_empty_selection.then_some("true")
            data-variant=variant.as_str()
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-stream-support=move || {
                ai_space_state
                    .get_value()
                    .map(|_| agent_contract.get().stream_support.as_str())
            }
            data-ui-stream-fallback=move || {
                ai_space_state
                    .get_value()
                    .map(|_| agent_contract.get().stream_fallback.as_str())
            }
            data-ui-stream-mode=move || {
                ai_space_state
                    .get_value()
                    .map(|state| state.get().mode.as_str())
            }
            data-ui-output-status=move || {
                ai_space_state
                    .get_value()
                    .map(|state| state.get().output_status.as_str())
            }
            data-ui-capability-toggle=move || {
                agent_contract.get().capabilities.can_toggle.then_some("true")
            }
            data-ui-capability-focus-move=move || {
                agent_contract.get().capabilities.can_focus_move.then_some("true")
            }
            data-ui-capability-external-sync=move || {
                agent_contract.get().capabilities.can_external_sync.then_some("true")
            }
            data-ui-capability-programmatic-replay=move || {
                agent_contract
                    .get()
                    .capabilities
                    .can_programmatic_replay
                    .then_some("true")
            }
            data-motion-source=if motion == AccordionMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != AccordionMotion::default()).then_some("true")
        >
            {items}
            {debug_panel}
        </div>
    }
}
