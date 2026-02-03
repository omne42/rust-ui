use crate::roving_tabindex::{use_roving_tabindex, RovingOrientation, RovingTabIndexOptions};
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

pub type MenuOnAction = Callback<usize>;

#[derive(Clone)]
pub struct MenuAttrs {
    pub role: &'static str,
    pub tabindex: i32,
    pub aria_activedescendant: Memo<Option<String>>,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone)]
pub struct MenuHandlers {
    pub on_key_down: Callback<String, bool>,
    pub on_item_pointer_move: Callback<usize>,
    pub on_item_click: Callback<usize>,
}

#[derive(Clone)]
pub struct MenuAria {
    pub active_index: ReadSignal<usize>,
    pub option_id: Callback<usize, String>,
    pub attrs: MenuAttrs,
    pub handlers: MenuHandlers,
}

#[derive(Clone)]
pub struct MenuOptions {
    pub is_disabled: bool,
    pub should_loop: bool,
    pub id_base: String,
    pub item_count: ReadSignal<usize>,
    pub on_action: Option<MenuOnAction>,
    /// Optional: disables specific items.
    pub is_item_disabled: Option<Callback<usize, bool>>,
    /// Optional: used for typeahead. When provided, typing alphanumeric keys will move the active
    /// item to the next match (prefix match, loops).
    pub item_text: Option<Callback<usize, String>>,
}

pub fn use_menu(options: MenuOptions) -> MenuAria {
    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: options.is_disabled,
        default_index: 0,
        should_loop: options.should_loop,
        orientation: RovingOrientation::Vertical,
        item_count: options.item_count,
        is_item_disabled: options.is_item_disabled,
    });

    let id_base = options.id_base;
    let option_id = Callback::new({
        let id_base = id_base.clone();
        move |index: usize| format!("{id_base}-item-{index}")
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
            Some(format!("{id_base}-item-{index}"))
        }
    });

    let attrs = MenuAttrs {
        role: "menu",
        tabindex: if options.is_disabled { -1 } else { 0 },
        aria_activedescendant,
        aria_disabled: options.is_disabled.then_some("true"),
    };

    let on_item_pointer_move = roving.handlers.on_item_focus;

    let on_item_click = {
        let is_disabled = options.is_disabled;
        let on_action = options.on_action;
        let is_item_disabled = options.is_item_disabled;
        let on_item_focus = roving.handlers.on_item_focus;
        Callback::new(move |index: usize| {
            if is_disabled {
                return;
            }
            if let Some(is_item_disabled) = is_item_disabled {
                if is_item_disabled.run(index) {
                    return;
                }
            }
            on_item_focus.run(index);
            if let Some(on_action) = on_action {
                on_action.run(index);
            }
        })
    };

    let on_key_down = {
        let is_disabled = options.is_disabled;
        let item_count = options.item_count;
        let on_action = options.on_action;
        let roving_key_down = roving.handlers.on_key_down;
        let on_item_focus = roving.handlers.on_item_focus;
        let item_text = options.item_text;
        let is_item_disabled = options.is_item_disabled;
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
                if let Some(is_item_disabled) = is_item_disabled {
                    if is_item_disabled.run(index) {
                        return true;
                    }
                }
                if let Some(on_action) = on_action {
                    on_action.run(index);
                }
                return true;
            }

            if let Some(item_text) = item_text {
                if let Some(ch) = typeahead_char(&key) {
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
            }

            false
        })
    };

    MenuAria {
        active_index: roving.active_index,
        option_id,
        attrs,
        handlers: MenuHandlers {
            on_key_down,
            on_item_pointer_move,
            on_item_click,
        },
    }
}
