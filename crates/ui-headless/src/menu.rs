use crate::roving_tabindex::{use_roving_tabindex, RovingOrientation, RovingTabIndexOptions};
use leptos::prelude::*;

fn is_space_key(key: &str) -> bool {
    key == " " || key == "Space" || key == "Spacebar"
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
}

pub fn use_menu(options: MenuOptions) -> MenuAria {
    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: options.is_disabled,
        default_index: 0,
        should_loop: options.should_loop,
        orientation: RovingOrientation::Vertical,
        item_count: options.item_count,
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
        let on_item_focus = roving.handlers.on_item_focus;
        Callback::new(move |index: usize| {
            if is_disabled {
                return;
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
                if let Some(on_action) = on_action {
                    on_action.run(roving.active_index.get_untracked());
                }
                return true;
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
