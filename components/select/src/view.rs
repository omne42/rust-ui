use crate::select::{SelectMotion, logic};
use crate::{OnPress, button::Button, list::List, popover::Popover};
use leptos::{ev, html, prelude::*};
use std::{collections::HashSet, sync::Arc, time::Duration};
use ui_headless as overlay_open;
use ui_headless::{A11yDirection, PopoverPlacement, Presence, locale_attrs, use_presence};

const SLOT_SELECT: &str = "select";
const SLOT_SELECT_PANEL: &str = "select-panel";
const CLASS_SELECT_PANEL: &str = "ui-select__panel";
const CLASS_SELECT_LISTBOX: &str = "ui-select__listbox";
const BOOL_TRUE: &str = "true";
const KEY_ARROW_DOWN: &str = "ArrowDown";
const KEY_ARROW_UP: &str = "ArrowUp";
const KEY_ARROW_LEFT: &str = "ArrowLeft";
const KEY_ARROW_RIGHT: &str = "ArrowRight";
const KEY_ENTER: &str = "Enter";
const KEY_SPACE: &str = " ";
const KEY_SPACEBAR: &str = "Spacebar";

struct SelectListRenderInput {
    id_base: String,
    listbox_id: String,
    trigger_id: String,
    items: Arc<[String]>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    is_disabled: bool,
    disabled_indices: Vec<usize>,
    on_action: Callback<usize>,
    default_active_index: usize,
    is_active_index_synced_to_selected: bool,
}

fn resolve_list_focus_plan(
    open_focus: logic::SelectOpenFocusStrategy,
    item_count: usize,
) -> (usize, bool) {
    let default_active_index = match open_focus {
        logic::SelectOpenFocusStrategy::Last => item_count.saturating_sub(1),
        logic::SelectOpenFocusStrategy::Selected | logic::SelectOpenFocusStrategy::First => 0,
    };
    let is_active_index_synced_to_selected =
        matches!(open_focus, logic::SelectOpenFocusStrategy::Selected);

    (default_active_index, is_active_index_synced_to_selected)
}

fn render_select_trigger(
    trigger_id: String,
    trigger_disabled: bool,
    anchor_ref: NodeRef<html::Button>,
    on_trigger_press: OnPress,
    open: Signal<bool>,
    aria_controls: Signal<Option<String>>,
    trigger_label: Memo<String>,
) -> impl IntoView {
    view! {
        <Button
            id=trigger_id
            is_disabled=trigger_disabled
            node_ref=anchor_ref
            on_press=on_trigger_press
            aria_haspopup="listbox"
            aria_expanded=open
            aria_controls_signal=aria_controls
        >
            {move || trigger_label.get()}
        </Button>
    }
}

fn render_select_list(input: SelectListRenderInput) -> impl IntoView {
    let set_selected_index = input.set_selected_index;
    view! {
        <List
            id_base=input.id_base
            id=input.listbox_id
            aria_labelledby=input.trigger_id
            class_name=CLASS_SELECT_LISTBOX
            items=input.items
            selected_index=input.selected_index.into()
            on_selected_index_change=Callback::new(move |next| set_selected_index.set(next))
            is_disabled=input.is_disabled
            disabled_indices=input.disabled_indices
            on_action=input.on_action
            default_active_index=input.default_active_index
            is_active_index_synced_to_selected=input.is_active_index_synced_to_selected
        />
    }
}

struct SelectPanelRenderInput {
    presence: Presence,
    open: Signal<bool>,
    anchor_ref: NodeRef<html::Button>,
    on_close: OnPress,
    placement: PopoverPlacement,
    motion: SelectMotion,
    open_focus: ReadSignal<logic::SelectOpenFocusStrategy>,
    id_base: StoredValue<String>,
    listbox_id: StoredValue<String>,
    trigger_id: StoredValue<String>,
    items: StoredValue<Arc<[String]>>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    is_disabled: bool,
    disabled_indices: StoredValue<Vec<usize>>,
    on_action: Callback<usize>,
}

fn render_select_panel(input: SelectPanelRenderInput) -> impl IntoView {
    view! {
        <Show when=move || input.presence.is_present.get()>
            <Popover
                open=input.open
                anchor_ref=input.anchor_ref
                on_close=input.on_close
                placement=input.placement
                motion=input.motion.popover
                on_exit_complete=input.presence.finish_exit
            >
                <div class=CLASS_SELECT_PANEL data-slot=SLOT_SELECT_PANEL>
                    {move || {
                        let items = input.items.get_value();
                        let (default_active_index, is_active_index_synced_to_selected) =
                            resolve_list_focus_plan(input.open_focus.get_untracked(), items.len());
                        render_select_list(SelectListRenderInput {
                            id_base: input.id_base.get_value(),
                            listbox_id: input.listbox_id.get_value(),
                            trigger_id: input.trigger_id.get_value(),
                            items,
                            selected_index: input.selected_index,
                            set_selected_index: input.set_selected_index,
                            is_disabled: input.is_disabled,
                            disabled_indices: input.disabled_indices.get_value(),
                            on_action: input.on_action,
                            default_active_index,
                            is_active_index_synced_to_selected,
                        })
                    }}
                </div>
            </Popover>
        </Show>
    }
}

#[component]
pub fn Select(
    id_base: String,
    items: Vec<String>,
    selected_index: ReadSignal<Option<usize>>,
    set_selected_index: WriteSignal<Option<usize>>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: SelectMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let motion = crate::select::motion::sanitize_motion(motion);
    let has_custom_motion = motion != SelectMotion::default();

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let items: StoredValue<Arc<[String]>> = StoredValue::new(items.into());
    let item_count = items.get_value().len();
    let is_disabled = logic::normalize_is_disabled(is_disabled, disabled);
    let trigger_disabled = logic::resolve_trigger_disabled(is_disabled, item_count);

    let disabled_set: HashSet<usize> = disabled_indices.iter().copied().collect();
    let disabled_set: StoredValue<Arc<HashSet<usize>>> = StoredValue::new(Arc::new(disabled_set));
    let disabled_indices: StoredValue<Vec<usize>> = StoredValue::new(disabled_indices);
    let disabled_option_count =
        logic::resolve_disabled_option_count(disabled_set.get_value().as_ref(), item_count);

    let (open_focus, set_open_focus) = signal(logic::SelectOpenFocusStrategy::Selected);
    let (typeahead, set_typeahead) = signal(String::new());
    let (last_typed_at, set_last_typed_at) = signal(None::<std::time::Instant>);
    let typeahead_timeout = Duration::from_millis(500);

    let open_state = overlay_open::use_controllable_open_state_traced(
        "select",
        open,
        default_open,
        on_open_change,
    );
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let presence = use_presence(open);

    let state = Signal::derive(move || {
        logic::resolve_state(logic::SelectStateInput {
            disabled: is_disabled,
            item_count,
            selected_index: selected_index.get(),
            disabled_option_count,
            is_open: open.get(),
            has_custom_class_name,
            has_custom_motion,
        })
    });
    let class = Signal::derive(move || logic::compose_class_name(class_name.clone(), state.get()));

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();

    let on_trigger_press: OnPress = Callback::new(move |_| {
        if trigger_disabled {
            return;
        }

        let next_open = !open.get_untracked();
        if next_open {
            set_open_focus.set(logic::SelectOpenFocusStrategy::Selected);
        }
        request_open_change.run(next_open);
    });
    let on_close: OnPress = Callback::new(move |_| request_open_change.run(false));

    let placeholder = logic::resolve_placeholder(placeholder);
    let trigger_label = Memo::new({
        let placeholder = placeholder.clone();
        move |_| {
            let items = items.get_value();
            selected_index
                .get()
                .and_then(|i| items.get(i).cloned())
                .unwrap_or_else(|| placeholder.clone())
        }
    });

    let id_base = logic::normalize_id_base(id_base);
    let id_base = StoredValue::new(id_base);
    let ids = logic::resolve_ids(&id_base.get_value());
    let trigger_id = StoredValue::new(ids.trigger_id);
    let listbox_id = StoredValue::new(ids.listbox_id);
    let aria_controls = ui_headless::aria_controls_when_open(open, listbox_id.get_value());
    let locale = locale_attrs(lang, dir);

    let on_action: Callback<usize> = Callback::new(move |_| request_open_change.run(false));

    let on_key_down = move |ev: ev::KeyboardEvent| {
        if trigger_disabled {
            return;
        }
        let items = items.get_value();
        let key = ev.key();
        let is_open = open.get_untracked();

        match key.as_str() {
            KEY_ARROW_DOWN => {
                if is_open {
                    return;
                }
                set_open_focus.set(logic::SelectOpenFocusStrategy::First);
                request_open_change.run(true);
                ev.prevent_default();
            }
            KEY_ARROW_UP => {
                if is_open {
                    return;
                }
                set_open_focus.set(logic::SelectOpenFocusStrategy::Last);
                request_open_change.run(true);
                ev.prevent_default();
            }
            KEY_ARROW_LEFT | KEY_ARROW_RIGHT => {
                if is_open {
                    return;
                }
                let direction = if key == KEY_ARROW_LEFT {
                    logic::SelectHorizontalNav::Previous
                } else {
                    logic::SelectHorizontalNav::Next
                };

                let disabled = disabled_set.get_value();
                let target = logic::resolve_horizontal_nav_target(
                    selected_index.get_untracked(),
                    direction,
                    items.len(),
                    disabled.as_ref(),
                );
                if let Some(target) = target {
                    set_selected_index.set(Some(target));
                }
                ev.prevent_default();
            }
            KEY_ENTER => {
                set_open_focus.set(logic::SelectOpenFocusStrategy::First);
            }
            _ => {
                if is_open {
                    return;
                }
                let Some(ch) = logic::typeahead_char(&key) else {
                    return;
                };

                let now = std::time::Instant::now();
                let mut query = typeahead.get_untracked();
                if last_typed_at
                    .get_untracked()
                    .map(|t| now.duration_since(t) > typeahead_timeout)
                    .unwrap_or(true)
                {
                    query.clear();
                }
                query.push(ch);

                let disabled = disabled_set.get_value();
                let count = items.len();
                if count == 0 {
                    return;
                }

                let start = selected_index
                    .get_untracked()
                    .map(|idx| (idx + 1) % count)
                    .unwrap_or(0);

                let mut matched =
                    logic::find_typeahead_match(&query, start, items.as_ref(), disabled.as_ref());
                if matched.is_none() && query.len() > 1 {
                    let single = ch.to_string();
                    matched = logic::find_typeahead_match(
                        &single,
                        start,
                        items.as_ref(),
                        disabled.as_ref(),
                    );
                    if matched.is_some() {
                        query = single;
                    }
                }

                set_typeahead.set(query);
                set_last_typed_at.set(Some(now));
                if let Some(next) = matched {
                    set_selected_index.set(Some(next));
                }
            }
        }
    };

    let on_key_up = move |ev: ev::KeyboardEvent| {
        if trigger_disabled {
            return;
        }

        let key = ev.key();
        if matches!(key.as_str(), KEY_SPACE | "Space" | KEY_SPACEBAR) {
            set_open_focus.set(logic::SelectOpenFocusStrategy::First);
        }
    };

    let agent_contract = Signal::derive(move || logic::resolve_agent_contract(state.get()));
    let trigger_view = render_select_trigger(
        trigger_id.get_value(),
        trigger_disabled,
        anchor_ref,
        on_trigger_press,
        open,
        aria_controls,
        trigger_label,
    );
    let panel_view = render_select_panel(SelectPanelRenderInput {
        presence,
        open,
        anchor_ref,
        on_close,
        placement,
        motion,
        open_focus,
        id_base,
        listbox_id,
        trigger_id,
        items,
        selected_index,
        set_selected_index,
        is_disabled,
        disabled_indices,
        on_action,
    });

    view! {
        <div
            class=move || class.get()
            lang=locale.lang.clone()
            dir=locale.dir
            on:keydown=on_key_down
            on:keyup=on_key_up
            data-slot=SLOT_SELECT
            data-open=move || state.get().is_open.then_some(BOOL_TRUE)
            data-closed=move || state.get().is_closed.then_some(BOOL_TRUE)
            data-disabled=move || state.get().trigger_disabled.then_some(BOOL_TRUE)
            data-component-disabled=move || state.get().is_disabled.then_some(BOOL_TRUE)
            data-empty=move || state.get().is_empty.then_some(BOOL_TRUE)
            data-has-items=move || state.get().has_items.then_some(BOOL_TRUE)
            data-count=move || state.get().item_count.to_string()
            data-has-selection=move || state.get().has_selection.then_some(BOOL_TRUE)
            data-selection-empty=move || state.get().selection_empty.then_some(BOOL_TRUE)
            data-selected-index=move || state.get().selected_index.map(|index| index.to_string())
            data-has-disabled-options=move || state.get().has_disabled_options.then_some(BOOL_TRUE)
            data-disabled-option-count=move || state.get().disabled_option_count.to_string()
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some(BOOL_TRUE)
            data-custom-motion=move || state.get().has_custom_motion.then_some(BOOL_TRUE)
            data-ui-schema=move || agent_contract.get().schema_attr
            data-ui-schema-version=move || agent_contract.get().schema_version_attr
            data-ui-intent=move || agent_contract.get().intent_attr
            data-ui-action=move || agent_contract.get().action_attr
            data-ui-state=move || agent_contract.get().state_attr
            data-ui-source=move || agent_contract.get().source_attr
            data-ui-stream-support=move || agent_contract.get().stream_support_attr
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr
            data-ui-stream-mode=move || agent_contract.get().stream_mode_attr
            data-ui-output-status=move || agent_contract.get().output_status_attr
        >
            {trigger_view}
            {panel_view}
        </div>
    }
}
