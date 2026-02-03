use crate::menu::MenuAria;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MenuItemKind {
    #[default]
    Action,
    Checkbox {
        is_checked: Signal<bool>,
    },
    Radio {
        is_checked: Signal<bool>,
    },
}

impl MenuItemKind {
    pub fn role(self) -> &'static str {
        match self {
            Self::Action => "menuitem",
            Self::Checkbox { .. } => "menuitemcheckbox",
            Self::Radio { .. } => "menuitemradio",
        }
    }
}

#[derive(Clone)]
pub struct MenuItemOptions {
    pub index: usize,
    pub kind: MenuItemKind,
    pub is_disabled: bool,
}

#[derive(Clone)]
pub struct MenuItemAttrs {
    pub id: String,
    pub role: &'static str,
    pub aria_checked: Memo<Option<&'static str>>,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone)]
pub struct MenuItemHandlers {
    pub on_pointer_move: Callback<()>,
    pub on_click: Callback<()>,
}

#[derive(Clone)]
pub struct MenuItemAria {
    pub attrs: MenuItemAttrs,
    pub handlers: MenuItemHandlers,
}

pub fn use_menu_item(menu: &MenuAria, options: MenuItemOptions) -> MenuItemAria {
    let role = options.kind.role();
    let id = menu.option_id.run(options.index);

    let aria_checked = match options.kind {
        MenuItemKind::Action => Memo::new(move |_| None),
        MenuItemKind::Checkbox { is_checked } | MenuItemKind::Radio { is_checked } => {
            Memo::new(move |_| Some(if is_checked.get() { "true" } else { "false" }))
        }
    };

    let on_pointer_move = {
        let is_disabled = options.is_disabled;
        let index = options.index;
        let on_item_pointer_move = menu.handlers.on_item_pointer_move;
        Callback::new(move |_| {
            if is_disabled {
                return;
            }
            on_item_pointer_move.run(index);
        })
    };

    let on_click = {
        let is_disabled = options.is_disabled;
        let index = options.index;
        let on_item_click = menu.handlers.on_item_click;
        Callback::new(move |_| {
            if is_disabled {
                return;
            }
            on_item_click.run(index);
        })
    };

    MenuItemAria {
        attrs: MenuItemAttrs {
            id,
            role,
            aria_checked,
            aria_disabled: options.is_disabled.then_some("true"),
        },
        handlers: MenuItemHandlers {
            on_pointer_move,
            on_click,
        },
    }
}
