use crate::roving_tabindex::{RovingOrientation, RovingTabIndexOptions, use_roving_tabindex};
use leptos::prelude::*;

fn is_space_key(key: &str) -> bool {
    key == " " || key == "Space" || key == "Spacebar"
}

#[derive(Clone)]
pub struct RadioGroupOptions {
    pub is_disabled: bool,
    pub id_base: String,
    pub orientation: RovingOrientation,
    pub item_count: ReadSignal<usize>,
    pub selected_index: ReadSignal<Option<usize>>,
    pub set_selected_index: WriteSignal<Option<usize>>,
    pub on_change: Option<Callback<usize>>,
    pub is_item_disabled: Option<Callback<usize, bool>>,
}

#[derive(Clone)]
pub struct RadioGroupAttrs {
    pub role: &'static str,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone)]
pub struct RadioGroupHandlers {
    pub on_key_down: Callback<String, bool>,
    pub on_radio_focus: Callback<usize>,
    pub on_radio_click: Callback<usize>,
}

#[derive(Clone)]
pub struct RadioGroupAria {
    pub active_index: ReadSignal<usize>,
    pub selected_index: ReadSignal<Option<usize>>,
    pub radio_id: Callback<usize, String>,
    pub attrs: RadioGroupAttrs,
    pub handlers: RadioGroupHandlers,
}

pub fn use_radio_group(options: RadioGroupOptions) -> RadioGroupAria {
    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: options.is_disabled,
        default_index: 0,
        should_loop: true,
        orientation: options.orientation,
        item_count: options.item_count,
        is_item_disabled: options.is_item_disabled,
    });

    // Keep active radio aligned with selection when selection changes.
    {
        let on_item_focus = roving.handlers.on_item_focus;
        let selected_index = options.selected_index;
        Effect::new(move |_| {
            if let Some(selected) = selected_index.get() {
                on_item_focus.run(selected);
            }
        });
    }

    let id_base = options.id_base;
    let radio_id = Callback::new({
        let id_base = id_base.clone();
        move |index: usize| format!("{id_base}-radio-{index}")
    });

    let on_radio_click = {
        let is_disabled = options.is_disabled;
        let is_item_disabled = options.is_item_disabled;
        let set_selected_index = options.set_selected_index;
        let on_change = options.on_change;
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
            set_selected_index.set(Some(index));
            if let Some(on_change) = on_change {
                on_change.run(index);
            }
        })
    };

    let on_key_down = {
        let is_disabled = options.is_disabled;
        let item_count = options.item_count;
        let is_item_disabled = options.is_item_disabled;
        let set_selected_index = options.set_selected_index;
        let on_change = options.on_change;
        let roving_key_down = roving.handlers.on_key_down;
        Callback::new(move |key: String| -> bool {
            if is_disabled {
                return false;
            }

            if key == "Enter" || is_space_key(&key) {
                let count = item_count.get_untracked();
                if count == 0 {
                    return true;
                }
                let index = roving.active_index.get_untracked();
                if let Some(is_item_disabled) = is_item_disabled
                    && is_item_disabled.run(index)
                {
                    return true;
                }
                on_radio_click.run(index);
                return true;
            }

            if !roving_key_down.run(key) {
                return false;
            }

            let count = item_count.get_untracked();
            if count == 0 {
                return true;
            }
            let index = roving.active_index.get_untracked();
            if let Some(is_item_disabled) = is_item_disabled
                && is_item_disabled.run(index)
            {
                return true;
            }
            set_selected_index.set(Some(index));
            if let Some(on_change) = on_change {
                on_change.run(index);
            }
            true
        })
    };

    RadioGroupAria {
        active_index: roving.active_index,
        selected_index: options.selected_index,
        radio_id,
        attrs: RadioGroupAttrs {
            role: "radiogroup",
            aria_disabled: options.is_disabled.then_some("true"),
        },
        handlers: RadioGroupHandlers {
            on_key_down,
            on_radio_focus: roving.handlers.on_item_focus,
            on_radio_click,
        },
    }
}
