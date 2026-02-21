use crate::a11y::{A11yDirection, aria_controls_when_open, locale_attrs};
use crate::roving_tabindex::{RovingOrientation, RovingTabIndexOptions, use_roving_tabindex};
use leptos::prelude::*;

#[derive(Clone)]
pub struct ComboBoxOptions {
    pub is_disabled: bool,
    pub id_base: String,
    pub is_open: Signal<bool>,
    pub set_open: Callback<bool>,
    pub item_count: ReadSignal<usize>,
    /// Selected index in the same coordinate space as the rendered options (e.g. filtered list).
    pub selected_index: Signal<Option<usize>>,
    /// Called when the user commits a selection (click, Enter, or Tab while open).
    pub on_action: Option<Callback<usize>>,
    /// Optional: disables specific options.
    pub is_item_disabled: Option<Callback<usize, bool>>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct ComboBoxInputAttrs {
    pub id: String,
    pub role: &'static str,
    pub aria_controls: Signal<Option<String>>,
    pub aria_expanded: Memo<Option<&'static str>>,
    pub aria_activedescendant: Memo<Option<String>>,
    pub aria_autocomplete: &'static str,
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct ComboBoxListBoxAttrs {
    pub id: String,
    pub role: &'static str,
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComboBoxOptionAttrs {
    pub role: &'static str,
    pub aria_selected: Option<&'static str>,
    pub aria_disabled: Option<&'static str>,
    pub data_selected: Option<&'static str>,
    pub data_focused: Option<&'static str>,
    pub data_disabled: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComboBoxKeyDownResult {
    pub handled: bool,
    pub stop_propagation: bool,
}

impl ComboBoxKeyDownResult {
    pub const fn ignored() -> Self {
        Self {
            handled: false,
            stop_propagation: false,
        }
    }

    pub const fn handled(stop_propagation: bool) -> Self {
        Self {
            handled: true,
            stop_propagation,
        }
    }
}

#[derive(Clone)]
pub struct ComboBoxHandlers {
    pub on_input_key_down: Callback<String, ComboBoxKeyDownResult>,
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
    pub option_attrs: Callback<usize, ComboBoxOptionAttrs>,
    pub input: ComboBoxInputAttrs,
    pub listbox: ComboBoxListBoxAttrs,
    pub handlers: ComboBoxHandlers,
}

pub fn use_combo_box(options: ComboBoxOptions) -> ComboBoxAria {
    let ComboBoxOptions {
        is_disabled,
        id_base,
        is_open,
        set_open,
        item_count,
        selected_index,
        on_action,
        is_item_disabled,
        lang,
        dir,
    } = options;
    let locale = locale_attrs(lang, dir);

    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled,
        default_index: 0,
        should_loop: true,
        orientation: RovingOrientation::Vertical,
        item_count,
        is_item_disabled,
    });

    // Keep active option aligned with selection when selection changes.
    {
        let on_item_focus = roving.handlers.on_item_focus;
        Effect::new(move |_| {
            if is_open.get() {
                return;
            }
            if let Some(selected) = selected_index.get() {
                on_item_focus.run(selected);
            }
        });
    }

    let input_id = format!("{id_base}-input");
    let listbox_id = format!("{id_base}-listbox");
    let aria_controls = aria_controls_when_open(is_open, listbox_id.clone());

    let option_id = Callback::new({
        let id_base = id_base.clone();
        move |index: usize| format!("{id_base}-option-{index}")
    });
    let option_attrs = {
        let active_index = roving.active_index;
        Callback::new(move |index: usize| {
            let is_selected = selected_index.get() == Some(index);
            let is_disabled = is_item_disabled.as_ref().is_some_and(|cb| cb.run(index));
            ComboBoxOptionAttrs {
                role: "option",
                aria_selected: is_selected.then_some("true"),
                aria_disabled: is_disabled.then_some("true"),
                data_selected: is_selected.then_some("true"),
                data_focused: (active_index.get() == index).then_some("true"),
                data_disabled: is_disabled.then_some("true"),
            }
        })
    };

    let aria_expanded = Memo::new(move |_| Some(if is_open.get() { "true" } else { "false" }));

    let aria_activedescendant = Memo::new({
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

    let open = Callback::new(move |_| {
        if is_disabled {
            return;
        }
        set_open.run(true);
    });

    let close = Callback::new(move |_| set_open.run(false));

    let toggle = Callback::new(move |_| {
        if is_disabled {
            return;
        }
        set_open.run(!is_open.get_untracked());
    });

    let on_option_pointer_move = roving.handlers.on_item_focus;

    let on_option_click = {
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
            set_open.run(false);
        })
    };

    let on_input_key_down = {
        let roving_key_down = roving.handlers.on_key_down;
        let on_item_focus = roving.handlers.on_item_focus;
        Callback::new(move |key: String| -> ComboBoxKeyDownResult {
            if is_disabled {
                return ComboBoxKeyDownResult::ignored();
            }

            // When closed, ArrowDown/ArrowUp open the list without interfering with text editing
            // keys like Home/End.
            if !is_open.get_untracked() {
                match key.as_str() {
                    "ArrowDown" => {
                        set_open.run(true);
                        let count = item_count.get_untracked();
                        if count == 0 {
                            return ComboBoxKeyDownResult::handled(false);
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
                        return ComboBoxKeyDownResult::handled(false);
                    }
                    "ArrowUp" => {
                        set_open.run(true);
                        let count = item_count.get_untracked();
                        if count == 0 {
                            return ComboBoxKeyDownResult::handled(false);
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
                        return ComboBoxKeyDownResult::handled(false);
                    }
                    _ => return ComboBoxKeyDownResult::ignored(),
                }
            }

            match key.as_str() {
                "Escape" => {
                    if !is_open.get_untracked() {
                        return ComboBoxKeyDownResult::ignored();
                    }
                    set_open.run(false);
                    return ComboBoxKeyDownResult::handled(true);
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
                    set_open.run(false);
                    return ComboBoxKeyDownResult::ignored();
                }
                "Enter" => {
                    let count = item_count.get_untracked();
                    if count != 0 {
                        let index = roving.active_index.get_untracked();
                        if is_item_disabled.as_ref().is_some_and(|cb| cb.run(index)) {
                            return ComboBoxKeyDownResult::handled(false);
                        }
                        if let Some(on_action) = on_action {
                            on_action.run(index);
                        }
                    }
                    set_open.run(false);
                    return ComboBoxKeyDownResult::handled(false);
                }
                _ => {}
            }

            // Navigate the active option while open.
            if roving_key_down.run(key.clone()) {
                return ComboBoxKeyDownResult::handled(false);
            }

            ComboBoxKeyDownResult::ignored()
        })
    };

    ComboBoxAria {
        active_index: roving.active_index,
        option_id,
        option_attrs,
        input: ComboBoxInputAttrs {
            id: input_id,
            role: "combobox",
            aria_controls,
            aria_expanded,
            aria_activedescendant,
            aria_autocomplete: "list",
            aria_disabled: is_disabled.then_some("true"),
            lang: locale.lang.clone(),
            dir: locale.dir,
        },
        listbox: ComboBoxListBoxAttrs {
            id: listbox_id,
            role: "listbox",
            aria_disabled: is_disabled.then_some("true"),
            lang: locale.lang,
            dir: locale.dir,
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

#[cfg(test)]
#[path = "test/combo_box.rs"]
mod tests;
