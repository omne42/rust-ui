use crate::roving_tabindex::{RovingOrientation, RovingTabIndexOptions, use_roving_tabindex};
use leptos::prelude::*;

#[derive(Clone)]
pub struct ComboBoxOptions {
    pub is_disabled: bool,
    pub id_base: String,
    pub is_open: ReadSignal<bool>,
    pub set_open: WriteSignal<bool>,
    pub item_count: ReadSignal<usize>,
    /// Selected index in the same coordinate space as the rendered options (e.g. filtered list).
    pub selected_index: Signal<Option<usize>>,
    /// Called when the user commits a selection (click, Enter, or Tab while open).
    pub on_action: Option<Callback<usize>>,
    /// Optional: disables specific options.
    pub is_item_disabled: Option<Callback<usize, bool>>,
}

#[derive(Clone)]
pub struct ComboBoxInputAttrs {
    pub id: String,
    pub role: &'static str,
    pub aria_controls: String,
    pub aria_expanded: Memo<Option<&'static str>>,
    pub aria_activedescendant: Memo<Option<String>>,
    pub aria_autocomplete: &'static str,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone)]
pub struct ComboBoxListBoxAttrs {
    pub id: String,
    pub role: &'static str,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone)]
pub struct ComboBoxHandlers {
    pub on_input_key_down: Callback<String, bool>,
    pub on_option_pointer_move: Callback<usize>,
    pub on_option_click: Callback<usize>,
    pub open: Callback<()>,
    pub close: Callback<()>,
    pub toggle: Callback<()>,
}

#[derive(Clone)]
pub struct ComboBoxAria {
    pub active_index: ReadSignal<usize>,
    pub option_id: Callback<usize, String>,
    pub input: ComboBoxInputAttrs,
    pub listbox: ComboBoxListBoxAttrs,
    pub handlers: ComboBoxHandlers,
}

pub fn use_combo_box(options: ComboBoxOptions) -> ComboBoxAria {
    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: options.is_disabled,
        default_index: 0,
        should_loop: true,
        orientation: RovingOrientation::Vertical,
        item_count: options.item_count,
        is_item_disabled: options.is_item_disabled,
    });

    // Keep active option aligned with selection when selection changes.
    {
        let on_item_focus = roving.handlers.on_item_focus;
        let selected_index = options.selected_index;
        let is_open = options.is_open;
        Effect::new(move |_| {
            if is_open.get() {
                return;
            }
            if let Some(selected) = selected_index.get() {
                on_item_focus.run(selected);
            }
        });
    }

    let id_base = options.id_base;
    let input_id = format!("{id_base}-input");
    let listbox_id = format!("{id_base}-listbox");

    let option_id = Callback::new({
        let id_base = id_base.clone();
        move |index: usize| format!("{id_base}-option-{index}")
    });

    let aria_expanded = Memo::new({
        let is_open = options.is_open;
        move |_| Some(if is_open.get() { "true" } else { "false" })
    });

    let aria_activedescendant = Memo::new({
        let is_open = options.is_open;
        let item_count = options.item_count;
        let id_base = id_base.clone();
        move |_| {
            if !is_open.get() {
                return None;
            }
            let count = item_count.get();
            if count == 0 {
                return None;
            }
            let index = roving.active_index.get();
            Some(format!("{id_base}-option-{index}"))
        }
    });

    let open = Callback::new({
        let is_disabled = options.is_disabled;
        let set_open = options.set_open;
        move |_| {
            if is_disabled {
                return;
            }
            set_open.set(true);
        }
    });

    let close = Callback::new({
        let set_open = options.set_open;
        move |_| set_open.set(false)
    });

    let toggle = Callback::new({
        let is_disabled = options.is_disabled;
        let is_open = options.is_open;
        let set_open = options.set_open;
        move |_| {
            if is_disabled {
                return;
            }
            set_open.set(!is_open.get_untracked());
        }
    });

    let on_option_pointer_move = roving.handlers.on_item_focus;

    let on_option_click = {
        let is_disabled = options.is_disabled;
        let is_item_disabled = options.is_item_disabled;
        let on_action = options.on_action;
        let set_open = options.set_open;
        let on_item_focus = roving.handlers.on_item_focus;
        Callback::new(move |index: usize| {
            if is_disabled {
                return;
            }
            if let Some(is_item_disabled) = is_item_disabled
                && is_item_disabled.run(index)
            {
                return;
            }
            on_item_focus.run(index);
            if let Some(on_action) = on_action {
                on_action.run(index);
            }
            set_open.set(false);
        })
    };

    let on_input_key_down = {
        let is_disabled = options.is_disabled;
        let item_count = options.item_count;
        let is_item_disabled = options.is_item_disabled;
        let on_action = options.on_action;
        let is_open = options.is_open;
        let set_open = options.set_open;
        let roving_key_down = roving.handlers.on_key_down;
        let on_item_focus = roving.handlers.on_item_focus;
        Callback::new(move |key: String| -> bool {
            if is_disabled {
                return false;
            }

            // When closed, ArrowDown/ArrowUp open the list without interfering with text editing
            // keys like Home/End.
            if !is_open.get_untracked() {
                match key.as_str() {
                    "ArrowDown" => {
                        set_open.set(true);
                        let count = item_count.get_untracked();
                        if count == 0 {
                            return true;
                        }
                        if let Some(is_item_disabled) = is_item_disabled {
                            if let Some(first_enabled) =
                                (0..count).find(|&idx| !is_item_disabled.run(idx))
                            {
                                on_item_focus.run(first_enabled);
                            }
                        } else {
                            on_item_focus.run(0);
                        }
                        return true;
                    }
                    "ArrowUp" => {
                        set_open.set(true);
                        let count = item_count.get_untracked();
                        if count == 0 {
                            return true;
                        }
                        if let Some(is_item_disabled) = is_item_disabled {
                            if let Some(last_enabled) =
                                (0..count).rev().find(|&idx| !is_item_disabled.run(idx))
                            {
                                on_item_focus.run(last_enabled);
                            }
                        } else {
                            on_item_focus.run(count.saturating_sub(1));
                        }
                        return true;
                    }
                    _ => return false,
                }
            }

            match key.as_str() {
                "Escape" => {
                    if !is_open.get_untracked() {
                        return false;
                    }
                    set_open.set(false);
                    return true;
                }
                "Tab" => {
                    // Commit the active option and allow focus to move.
                    let count = item_count.get_untracked();
                    if count != 0 {
                        let index = roving.active_index.get_untracked();
                        if !is_item_disabled.as_ref().is_some_and(|cb| cb.run(index))
                            && let Some(on_action) = on_action
                        {
                            on_action.run(index);
                        }
                    }
                    set_open.set(false);
                    return false;
                }
                "Enter" => {
                    let count = item_count.get_untracked();
                    if count != 0 {
                        let index = roving.active_index.get_untracked();
                        if is_item_disabled.as_ref().is_some_and(|cb| cb.run(index)) {
                            return true;
                        }
                        if let Some(on_action) = on_action {
                            on_action.run(index);
                        }
                    }
                    set_open.set(false);
                    return true;
                }
                _ => {}
            }

            // Navigate the active option while open.
            if roving_key_down.run(key.clone()) {
                return true;
            }

            false
        })
    };

    ComboBoxAria {
        active_index: roving.active_index,
        option_id,
        input: ComboBoxInputAttrs {
            id: input_id,
            role: "combobox",
            aria_controls: listbox_id.clone(),
            aria_expanded,
            aria_activedescendant,
            aria_autocomplete: "list",
            aria_disabled: options.is_disabled.then_some("true"),
        },
        listbox: ComboBoxListBoxAttrs {
            id: listbox_id,
            role: "listbox",
            aria_disabled: options.is_disabled.then_some("true"),
        },
        handlers: ComboBoxHandlers {
            on_input_key_down,
            on_option_pointer_move,
            on_option_click,
            open,
            close,
            toggle,
        },
    }
}
