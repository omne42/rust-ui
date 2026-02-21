use crate::roving_tabindex::{RovingOrientation, RovingTabIndexOptions, use_roving_tabindex};
use leptos::prelude::*;
use std::time::{Duration, Instant};

fn is_space_key(key: &str) -> bool {
    key == " " || key == "Space" || key == "Spacebar"
}

fn typeahead_char(key: &str) -> Option<char> {
    let mut chars = key.chars();
    let ch = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    if ch.is_ascii_alphanumeric() {
        Some(ch.to_ascii_lowercase())
    } else {
        None
    }
}

fn find_typeahead_match(
    query: &str,
    start_index: usize,
    item_count: usize,
    item_text: Callback<usize, String>,
    is_item_disabled: Option<&Callback<usize, bool>>,
) -> Option<usize> {
    let query = query.to_ascii_lowercase();
    for offset in 0..item_count {
        let index = (start_index + offset) % item_count;
        if is_item_disabled.is_some_and(|cb| cb.run(index)) {
            continue;
        }
        if item_text
            .run(index)
            .trim()
            .to_ascii_lowercase()
            .starts_with(&query)
        {
            return Some(index);
        }
    }
    None
}

#[derive(Clone)]
pub struct ListBoxAttrs {
    pub role: &'static str,
    pub tabindex: i32,
    pub aria_activedescendant: Memo<Option<String>>,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone)]
pub struct ListBoxHandlers {
    pub on_key_down: Callback<String, bool>,
    pub on_option_pointer_move: Callback<usize>,
    pub on_option_click: Callback<usize>,
}

#[derive(Clone)]
pub struct ListBoxAria {
    pub active_index: ReadSignal<usize>,
    pub selected_index: ReadSignal<Option<usize>>,
    pub option_id: Callback<usize, String>,
    pub attrs: ListBoxAttrs,
    pub handlers: ListBoxHandlers,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListBoxOptionA11yInput {
    pub is_disabled: bool,
    pub is_selected: bool,
    pub is_focused: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListBoxOptionA11yAttrs {
    pub role: &'static str,
    pub data_state: &'static str,
    pub aria_selected: Option<&'static str>,
    pub aria_disabled: Option<&'static str>,
    pub data_selected: Option<&'static str>,
    pub data_focused: Option<&'static str>,
    pub data_disabled: Option<&'static str>,
}

pub fn listbox_option_a11y_attrs(input: ListBoxOptionA11yInput) -> ListBoxOptionA11yAttrs {
    let data_state = if input.is_disabled {
        "disabled"
    } else if input.is_selected {
        "selected"
    } else if input.is_focused {
        "focused"
    } else {
        "idle"
    };

    ListBoxOptionA11yAttrs {
        role: "option",
        data_state,
        aria_selected: input.is_selected.then_some("true"),
        aria_disabled: input.is_disabled.then_some("true"),
        data_selected: input.is_selected.then_some("true"),
        data_focused: input.is_focused.then_some("true"),
        data_disabled: input.is_disabled.then_some("true"),
    }
}

#[derive(Clone)]
pub struct ListBoxOptions {
    pub is_disabled: bool,
    pub should_loop: bool,
    pub id_base: String,
    pub default_index: usize,
    /// When `true` (default), the active option will be aligned with the selected option whenever
    /// selection changes.
    ///
    /// Set to `false` to support focus strategies like "open and focus first/last" while keeping
    /// selection unchanged.
    pub sync_active_index_to_selected: bool,
    pub item_count: ReadSignal<usize>,
    pub selected_index: ReadSignal<Option<usize>>,
    pub set_selected_index: WriteSignal<Option<usize>>,
    pub on_action: Option<Callback<usize>>,
    /// Optional: disables specific options.
    pub is_item_disabled: Option<Callback<usize, bool>>,
    /// Optional: used for typeahead. When provided, typing alphanumeric keys will move the active
    /// option to the next match (prefix match, loops).
    pub item_text: Option<Callback<usize, String>>,
}

pub fn use_listbox(options: ListBoxOptions) -> ListBoxAria {
    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: options.is_disabled,
        default_index: options.default_index,
        should_loop: options.should_loop,
        orientation: RovingOrientation::Vertical,
        item_count: options.item_count,
        is_item_disabled: options.is_item_disabled,
    });

    // Keep the roving active index aligned with the selected option (when selection is present).
    // This is a minimal heuristic for v0, and can be refined with separate "focused" vs
    // "selected" state later.
    if options.sync_active_index_to_selected {
        let on_item_focus = roving.handlers.on_item_focus;
        let selected_index = options.selected_index;
        Effect::new(move |_| {
            if let Some(selected) = selected_index.get() {
                on_item_focus.run(selected);
            }
        });
    }

    let id_base = options.id_base;
    let option_id = Callback::new({
        let id_base = id_base.clone();
        move |index: usize| format!("{id_base}-option-{index}")
    });

    let aria_activedescendant = Memo::new({
        let id_base = id_base.clone();
        let item_count = options.item_count;
        move |_| {
            let count = item_count.get();
            if count == 0 {
                return None;
            }
            let index = roving.active_index.get();
            Some(format!("{id_base}-option-{index}"))
        }
    });

    let attrs = ListBoxAttrs {
        role: "listbox",
        tabindex: if options.is_disabled { -1 } else { 0 },
        aria_activedescendant,
        aria_disabled: options.is_disabled.then_some("true"),
    };

    let on_option_pointer_move = roving.handlers.on_item_focus;

    let on_option_click = {
        let is_disabled = options.is_disabled;
        let is_item_disabled = options.is_item_disabled;
        let set_selected_index = options.set_selected_index;
        let on_action = options.on_action;
        Callback::new(move |index: usize| {
            if is_disabled {
                return;
            }
            if let Some(is_item_disabled) = is_item_disabled
                && is_item_disabled.run(index)
            {
                return;
            }
            set_selected_index.set(Some(index));
            if let Some(on_action) = on_action {
                on_action.run(index);
            }
        })
    };

    let on_key_down = {
        let is_disabled = options.is_disabled;
        let item_count = options.item_count;
        let is_item_disabled = options.is_item_disabled;
        let set_selected_index = options.set_selected_index;
        let on_action = options.on_action;
        let item_text = options.item_text;
        let roving_key_down = roving.handlers.on_key_down;
        let on_item_focus = roving.handlers.on_item_focus;
        let (typeahead, set_typeahead) = signal(String::new());
        let (last_typed_at, set_last_typed_at) = signal(None::<Instant>);
        let timeout = Duration::from_millis(500);
        Callback::new(move |key: String| -> bool {
            if is_disabled {
                return false;
            }

            // Arrow/Home/End navigation updates the active index.
            if roving_key_down.run(key.clone()) {
                return true;
            }

            if key == "Enter" || is_space_key(&key) {
                let count = item_count.get_untracked();
                if count == 0 {
                    return false;
                }
                let index = roving.active_index.get_untracked();
                if let Some(is_item_disabled) = is_item_disabled
                    && is_item_disabled.run(index)
                {
                    return true;
                }
                set_selected_index.set(Some(index));
                if let Some(on_action) = on_action {
                    on_action.run(index);
                }
                return true;
            }

            if let Some(item_text) = item_text
                && let Some(ch) = typeahead_char(&key)
            {
                let now = Instant::now();
                let mut query = typeahead.get_untracked();
                if last_typed_at
                    .get_untracked()
                    .map(|t| now.duration_since(t) > timeout)
                    .unwrap_or(true)
                {
                    query.clear();
                }
                query.push(ch);

                let count = item_count.get_untracked();
                if count == 0 {
                    return false;
                }

                let start = (roving.active_index.get_untracked() + 1) % count;

                let next = find_typeahead_match(
                    &query,
                    start,
                    count,
                    item_text,
                    is_item_disabled.as_ref(),
                )
                .or_else(|| {
                    if query.len() <= 1 {
                        return None;
                    }
                    let single = ch.to_string();
                    let idx = find_typeahead_match(
                        &single,
                        start,
                        count,
                        item_text,
                        is_item_disabled.as_ref(),
                    )?;
                    query = single;
                    Some(idx)
                });

                set_typeahead.set(query);
                set_last_typed_at.set(Some(now));

                if let Some(next) = next {
                    on_item_focus.run(next);
                }
                return true;
            }

            false
        })
    };

    ListBoxAria {
        active_index: roving.active_index,
        selected_index: options.selected_index,
        option_id,
        attrs,
        handlers: ListBoxHandlers {
            on_key_down,
            on_option_pointer_move,
            on_option_click,
        },
    }
}

#[cfg(test)]
#[path = "test/listbox.rs"]
mod tests;
