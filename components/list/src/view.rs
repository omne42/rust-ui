use crate::logic;
use crate::motion::{ListMotion, ListSectionMotion};
use leptos::{children::Children, html, prelude::*};
use std::{collections::HashSet, sync::Arc};
use ui_headless::a11y::locale_attrs;
use ui_headless::listbox::{ListBoxOptionA11yInput, listbox_option_a11y_attrs};
use ui_headless::{
    A11yDirection, FocusRingOptions, ListBoxOptions, use_controllable_state, use_focus_ring,
    use_listbox, use_ui_id_provider,
};

const LISTBOX_HIGHLIGHT_CLASS: &str = "ui-active-highlight";
const LISTBOX_HIGHLIGHT_SLOT: &str = "listbox-highlight";
const LIST_ITEM_DIVIDER_CLASS: &str = "ui-listbox-item__divider";
const LIST_ITEM_DIVIDER_SLOT: &str = "listbox-item-divider";
const LIST_SECTION_DIVIDER_CLASS: &str = "ui-listbox-section__divider";
const LIST_SECTION_DIVIDER_SLOT: &str = "listbox-section-divider";

struct ListOptionRenderInput {
    index: usize,
    label: String,
    id: String,
    active_index: ReadSignal<usize>,
    selected_index: Signal<Option<usize>>,
    is_disabled_root: bool,
    disabled_indices: Arc<HashSet<usize>>,
    on_option_pointer_move: Callback<usize>,
    on_option_click: Callback<usize>,
}

fn render_list_option(input: ListOptionRenderInput) -> impl IntoView {
    let ListOptionRenderInput {
        index,
        label,
        id,
        active_index,
        selected_index,
        is_disabled_root,
        disabled_indices,
        on_option_pointer_move,
        on_option_click,
    } = input;

    let on_option_pointer_move_for_move = on_option_pointer_move;
    let on_option_pointer_move_for_click = on_option_pointer_move;
    let on_option_click_for_click = on_option_click;

    let option_a11y = Signal::derive(move || {
        let option_state = logic::resolve_option_state(logic::ListOptionStateInput {
            index,
            active_index: active_index.get(),
            selected_index: selected_index.get(),
            is_disabled_root,
            is_disabled_item: logic::is_disabled_index(&disabled_indices, index),
        });

        listbox_option_a11y_attrs(ListBoxOptionA11yInput {
            is_disabled: option_state.is_disabled,
            is_selected: option_state.is_selected,
            is_focused: option_state.is_focused,
        })
    });

    view! {
        <div
            id=id
            role=move || option_a11y.get().role
            aria-selected=move || option_a11y.get().aria_selected
            aria-disabled=move || option_a11y.get().aria_disabled
            class="ui-listbox__option"
            data-slot="listbox-option"
            data-index=index
            data-state=move || option_a11y.get().data_state
            data-selected=move || option_a11y.get().data_selected
            data-focused=move || option_a11y.get().data_focused
            data-disabled=move || option_a11y.get().data_disabled
            on:pointermove=move |_| on_option_pointer_move_for_move.run(index)
            on:click=move |_| {
                on_option_pointer_move_for_click.run(index);
                on_option_click_for_click.run(index);
            }
        >
            {label}
        </div>
    }
}

#[component]
pub fn List(
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(into)] items: Arc<[String]>,
    #[prop(optional)] selected_index: Option<Signal<Option<usize>>>,
    #[prop(optional)] default_selected_index: Option<usize>,
    #[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] disabled_indices: Vec<usize>,
    #[prop(optional)] on_action: Option<Callback<usize>>,
    #[prop(optional, default = 0)] default_active_index: usize,
    #[prop(optional, default = true)] is_active_index_synced_to_selected: bool,
    #[prop(optional)] motion: ListMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (motion, has_custom_motion) = crate::motion::resolve_motion(motion);
    let id_base = id_base.or_else(|| {
        use_ui_id_provider().map(|provider| provider.next_prefixed_id(logic::DEFAULT_ID_BASE))
    });
    let id_base = logic::normalize_id_base(id_base);

    let item_count_value = items.len();
    let (item_count, _set_item_count) = signal(item_count_value);
    let selection_axis = logic::normalize_selection_axis(logic::ListSelectionAxisInput {
        selected_index,
        default_selected_index,
        on_selected_index_change,
        item_count: item_count_value,
    });
    let is_selected_index_controlled = selection_axis.selected_index.is_some();
    let selection_sources =
        logic::resolve_selection_source_state(logic::ListSelectionSourceStateInput {
            is_controlled: is_selected_index_controlled,
            has_default_selected_index: selection_axis.default_selected_index.is_some(),
            has_on_selected_index_change: selection_axis.on_selected_index_change.is_some(),
        });
    let selected_state = use_controllable_state(
        selection_axis.selected_index,
        Some(selection_axis.default_selected_index),
        selection_axis.on_selected_index_change,
    );
    let selected_state_value = selected_state.value;
    let request_selected_index_change = selected_state.request_change;
    let (listbox_selected_index, set_listbox_selected_index) =
        signal(selected_state_value.get_untracked());
    let selected_state_value_for_sync = selected_state_value;
    let selected_state_value_for_request = selected_state_value;
    let selected_state_value_for_root_state = selected_state_value;
    let selected_state_value_for_option_state = selected_state_value;
    let request_selected_index_change_for_listbox = request_selected_index_change;
    let listbox_selected_index_for_sync = listbox_selected_index;
    let listbox_selected_index_for_request = listbox_selected_index;
    let set_listbox_selected_index_for_sync = set_listbox_selected_index;
    let set_listbox_selected_index_for_request = set_listbox_selected_index;

    // Keep listbox selection adapter aligned to the canonical controllable axis.
    Effect::new(move |_| {
        let canonical = selected_state_value_for_sync.get();
        if listbox_selected_index_for_sync.get_untracked() != canonical {
            set_listbox_selected_index_for_sync.set(canonical);
        }
    });

    // Route listbox selection intents back to the controllable axis.
    Effect::new(move |_| {
        let requested = listbox_selected_index_for_request.get();
        let canonical = selected_state_value_for_request.get_untracked();
        if requested == canonical {
            return;
        }

        request_selected_index_change_for_listbox.run(requested);

        if is_selected_index_controlled {
            set_listbox_selected_index_for_request.set(canonical);
        }
    });

    let options_axis = logic::normalize_options_axis(logic::ListOptionsAxisInput {
        is_disabled,
        disabled_indices,
    });
    let has_disabled_options = options_axis.has_disabled_options;
    let disabled_indices_for_options = options_axis.disabled_indices.clone();

    let item_text = {
        let items = items.clone();
        Callback::new(move |index: usize| items.get(index).cloned().unwrap_or_default())
    };

    let is_item_disabled = has_disabled_options.then_some({
        let disabled_indices = options_axis.disabled_indices.clone();
        Callback::new(move |index: usize| logic::is_disabled_index(&disabled_indices, index))
    });

    let aria = use_listbox(ListBoxOptions {
        is_disabled,
        should_loop: true,
        id_base,
        default_index: default_active_index,
        sync_active_index_to_selected: is_active_index_synced_to_selected,
        item_count,
        selected_index: listbox_selected_index,
        set_selected_index: set_listbox_selected_index,
        on_action,
        is_item_disabled,
        item_text: Some(item_text),
    });

    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
    let (interaction_source, set_interaction_source) = signal(logic::ListInteractionSource::None);
    let set_interaction_source_for_keyboard = set_interaction_source;
    let set_interaction_source_for_pointer = set_interaction_source;

    let on_key_down = move |ev: leptos::ev::KeyboardEvent| {
        if aria.handlers.on_key_down.run(ev.key()) {
            set_interaction_source_for_keyboard.set(logic::ListInteractionSource::Keyboard);
            ev.prevent_default();
        }
    };

    let options_ref: NodeRef<html::Div> = NodeRef::new();
    let highlight_ref: NodeRef<html::Div> = NodeRef::new();
    crate::motion::attach_motion(
        options_ref,
        highlight_ref,
        aria.active_index,
        aria.option_id,
        motion,
    );

    let class = logic::normalize_list_class_name(class_name);

    let accessible_name = logic::resolve_accessible_name(aria_label, aria_labelledby);
    let aria_label = StoredValue::new(accessible_name.aria_label);
    let aria_labelledby = StoredValue::new(accessible_name.aria_labelledby);
    let locale = locale_attrs(lang, dir);
    let list_lang = locale.lang;
    let list_dir = locale.dir;

    let state = Signal::derive(move || {
        logic::resolve_state(
            item_count_value,
            selected_state_value_for_root_state.get(),
            has_disabled_options,
        )
    });
    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(logic::ListAgentContractInput {
            state: state.get(),
            is_disabled,
            is_controlled: is_selected_index_controlled,
        })
    });

    view! {
        <div
            class=class
            class:ui-listbox--focus-visible=move || focus_ring.is_focus_visible.get()
            id=id
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-label=aria_label.get_value()
            aria-labelledby=aria_labelledby.get_value()
            aria-disabled=aria.attrs.aria_disabled
            aria-activedescendant=move || aria.attrs.aria_activedescendant.get()
            lang=list_lang.clone()
            dir=list_dir
            data-slot="listbox"
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-selection-mode=selection_sources.selection_mode_attr
            data-selection-value-source=selection_sources.selection_value_source_attr
            data-default-selection-source=selection_sources.default_selection_source_attr
            data-selection-change-source=selection_sources.selection_change_source_attr
            data-interaction-source=move || interaction_source.get().as_attr()
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_attr()
            data-ui-intent=move || agent_contract.get().intent.as_attr()
            data-ui-action=move || agent_contract.get().action.as_attr()
            data-ui-state=move || agent_contract.get().state.as_attr()
            data-ui-source=move || agent_contract.get().source.as_attr()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_attr()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_attr()
            data-ui-output-status=move || agent_contract.get().output_status.as_attr()
            data-ui-config-policy=move || agent_contract.get().config_policy.as_attr()
            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
            data-disabled=is_disabled.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-selection-empty=move || (!state.get().has_selection).then_some("true")
            data-has-disabled-options=move || {
                state.get().has_disabled_options.then_some("true")
            }
            on:keydown=on_key_down
            on:pointerdown=move |_| {
                set_interaction_source_for_pointer.set(logic::ListInteractionSource::Pointer);
            }
            on:focus=move |_| focus_ring.handlers.on_focus.run(())
            on:blur=move |_| focus_ring.handlers.on_blur.run(())
        >
            <div class="ui-listbox__options" node_ref=options_ref data-slot="listbox-options">
                <div class=LISTBOX_HIGHLIGHT_CLASS node_ref=highlight_ref data-slot=LISTBOX_HIGHLIGHT_SLOT></div>
                {items
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(index, label)| {
                        render_list_option(ListOptionRenderInput {
                            index,
                            label,
                            id: aria.option_id.run(index),
                            active_index: aria.active_index,
                            selected_index: selected_state_value_for_option_state,
                            is_disabled_root: is_disabled,
                            disabled_indices: disabled_indices_for_options.clone(),
                            on_option_pointer_move: aria.handlers.on_option_pointer_move,
                            on_option_click: aria.handlers.on_option_click,
                        })
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn ListItem(
    children: Children,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] index: Option<usize>,
    #[prop(optional)] is_selected: bool,
    #[prop(optional)] is_focused: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_selection_indicator_visible: bool,
    #[prop(optional)] is_divider_visible: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] selected_text: Option<String>,
    #[prop(optional, into)] unselected_text: Option<String>,
    #[prop(optional)] on_press: Option<Callback<()>>,
    #[prop(optional)] on_pointer_move: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (on_press, on_pointer_move) = logic::item::normalize_callbacks(on_press, on_pointer_move);
    let is_interaction_blocked = logic::item::is_interaction_blocked(is_disabled);

    let (aria_label, has_custom_aria_label) = logic::item::normalize_aria_label(aria_label);

    let class_name = logic::item::normalize_class_name(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Signal::derive(move || {
        logic::item::resolve_state(logic::item::ListItemStateInput {
            selected: is_selected,
            focused: is_focused,
            disabled: is_disabled,
            show_selection_indicator: is_selection_indicator_visible,
            has_divider: is_divider_visible,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Signal::derive(move || {
        logic::item::compose_class_name(class_name.get_value(), state.get())
    });

    let selection_indicator =
        logic::item::resolve_selection_indicator(is_selection_indicator_visible);
    let indicator_text = move || selection_indicator.marker(state.get().is_selected);
    let index_text = index;
    let selection_status_text =
        logic::item::normalize_selection_status_text(selected_text, unselected_text);
    let selection_selected_text = StoredValue::new(selection_status_text.selected);
    let selection_unselected_text = StoredValue::new(selection_status_text.unselected);

    view! {
        <div
            class=move || class.get()
            id=id
            role="option"
            tabindex=if is_disabled { Some(-1) } else { Some(0) }
            aria-label=aria_label
            aria-selected=is_selected.then_some("true")
            aria-disabled=is_disabled.then_some("true")
            data-slot="listbox-item"
            data-index=index_text
            data-state=move || state.get().data_state_attr
            data-selected=move || state.get().is_selected.then_some("true")
            data-unselected=move || (!state.get().is_selected).then_some("true")
            data-focused=move || state.get().is_focused.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-divider=move || state.get().has_divider.then_some("true")
            data-show-selection-indicator=move || {
                state.get().show_selection_indicator.then_some("true")
            }
            data-selection-indicator=move || state.get().selection_indicator_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            on:pointermove=move |_| {
                if is_interaction_blocked {
                    return;
                }
                on_pointer_move.run(());
            }
            on:click=move |_| {
                if is_interaction_blocked {
                    return;
                }
                on_press.run(());
            }
        >
            <span
                class="ui-listbox-item__indicator"
                data-slot="listbox-item-indicator"
                data-visible=move || {
                    indicator_text().is_some().then_some("true")
                }
            >
                {indicator_text}
            </span>

            <span class="ui-listbox-item__label" data-slot="listbox-item-label">
                {children()}
            </span>

            <Show when=move || selection_indicator != logic::ListItemSelectionIndicator::Hidden>
                <span class="ui-listbox-item__selection-sr" data-slot="listbox-item-selection-sr">
                    {move || {
                        if state.get().is_selected {
                            selection_selected_text.get_value()
                        } else {
                            selection_unselected_text.get_value()
                        }
                    }}
                </span>
            </Show>

            <Show when=move || state.get().has_divider>
                <span class=LIST_ITEM_DIVIDER_CLASS data-slot=LIST_ITEM_DIVIDER_SLOT></span>
            </Show>
        </div>
    }
}

#[component]
pub fn ListSection(
    children: Children,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional)] item_count: Option<usize>,
    #[prop(optional)] heading_tone: logic::ListSectionHeadingTone,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_sticky_heading: bool,
    #[prop(optional)] is_divider_visible: bool,
    #[prop(optional)] motion: ListSectionMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let title = logic::section::normalize_title(title);
    let has_title = title.is_some();
    let title_text = logic::section::resolve_title_text(title);
    let title = StoredValue::new(title_text);
    let resolved_item_count = logic::section::normalize_item_count(item_count);

    let (aria_label, has_custom_aria_label) = logic::section::normalize_aria_label(aria_label);
    let motion = crate::motion::sanitize_section_motion(motion);
    let has_custom_motion = motion != ListSectionMotion::default();
    let items_ref: NodeRef<html::Div> = NodeRef::new();
    crate::motion::attach_section_motion(items_ref, motion);

    let class_name = logic::section::normalize_class_name(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Signal::derive(move || {
        logic::section::resolve_state(logic::section::ListSectionStateInput {
            heading_tone,
            item_count: resolved_item_count,
            disabled: is_disabled,
            sticky_heading: is_sticky_heading,
            show_divider: is_divider_visible,
            has_title,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Signal::derive(move || {
        logic::section::compose_class_name(class_name.get_value(), state.get())
    });

    view! {
        <section
            class=move || class.get()
            role="group"
            aria-label=aria_label
            aria-disabled=is_disabled.then_some("true")
            data-slot="listbox-section"
            data-tone=move || state.get().heading_tone_attr
            data-state=move || state.get().data_state_attr
            data-item-count=move || state.get().item_count
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-has-title=move || state.get().has_title.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-sticky-heading=move || state.get().is_sticky_heading.then_some("true")
            data-divided=move || state.get().has_divider.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-title-source=move || state.get().title_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
        >
            <Show when=move || state.get().has_title>
                <header
                    class="ui-listbox-section__header"
                    data-slot="listbox-section-header"
                    data-sticky=move || state.get().is_sticky_heading.then_some("true")
                >
                    {move || title.get_value()}
                </header>
            </Show>

            <div node_ref=items_ref class="ui-listbox-section__items" data-slot="listbox-section-items">
                {children()}
            </div>

            <Show when=move || state.get().has_divider>
                <div class=LIST_SECTION_DIVIDER_CLASS data-slot=LIST_SECTION_DIVIDER_SLOT></div>
            </Show>
        </section>
    }
}
