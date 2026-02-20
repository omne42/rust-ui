use crate::a11y::{A11yDirection, locale_attrs};
use leptos::prelude::*;

fn is_activation_key(key: &str) -> bool {
    key == "Enter" || key == " " || key == "Space" || key == "Spacebar"
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeRootAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

pub fn tree_root_attrs(
    aria_label: String,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> TreeRootAttrs {
    let locale = locale_attrs(lang, dir);
    TreeRootAttrs {
        role: "tree",
        aria_label,
        lang: locale.lang,
        dir: locale.dir,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeItemA11yInput {
    pub depth: usize,
    pub has_children: bool,
    pub is_expanded: bool,
    pub is_selected: bool,
    pub is_disabled: bool,
    pub is_tree_disabled: bool,
    pub is_first_visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeItemAttrs {
    pub role: &'static str,
    pub aria_level: usize,
    pub aria_expanded: Option<&'static str>,
    pub aria_selected: &'static str,
    pub aria_disabled: Option<&'static str>,
    pub tabindex: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeItemState {
    pub is_interactive: bool,
    pub toggles_expansion: bool,
}

#[derive(Clone)]
pub struct TreeItemHandlers {
    pub on_click: Callback<()>,
    pub on_key_down: Callback<String, bool>,
}

#[derive(Clone)]
pub struct TreeItemContract {
    pub attrs: TreeItemAttrs,
    pub handlers: TreeItemHandlers,
    pub state: TreeItemState,
}

#[derive(Clone)]
pub struct TreeItemOptions {
    pub on_select: Callback<()>,
    pub on_toggle: Option<Callback<()>>,
}

pub fn use_tree_item(input: TreeItemA11yInput, options: TreeItemOptions) -> TreeItemContract {
    let is_interactive = !input.is_tree_disabled && !input.is_disabled;
    let toggles_expansion = input.has_children && options.on_toggle.is_some();

    let attrs = TreeItemAttrs {
        role: "treeitem",
        aria_level: input.depth.saturating_add(1),
        aria_expanded: input.has_children.then_some(if input.is_expanded {
            "true"
        } else {
            "false"
        }),
        aria_selected: if input.is_selected { "true" } else { "false" },
        aria_disabled: input.is_disabled.then_some("true"),
        tabindex: if input.is_selected || input.is_first_visible {
            0
        } else {
            -1
        },
    };

    let on_click = {
        let on_select = options.on_select;
        let on_toggle = options.on_toggle;
        Callback::new(move |_| {
            if !is_interactive {
                return;
            }
            on_select.run(());
            if let Some(on_toggle) = on_toggle {
                on_toggle.run(());
            }
        })
    };

    let on_key_down = {
        let on_select = options.on_select;
        let on_toggle = options.on_toggle;
        Callback::new(move |key: String| -> bool {
            if !is_interactive || !is_activation_key(&key) {
                return false;
            }
            on_select.run(());
            if let Some(on_toggle) = on_toggle {
                on_toggle.run(());
            }
            true
        })
    };

    TreeItemContract {
        attrs,
        handlers: TreeItemHandlers {
            on_click,
            on_key_down,
        },
        state: TreeItemState {
            is_interactive,
            toggles_expansion,
        },
    }
}

#[cfg(test)]
#[path = "test/tree.rs"]
mod tests;
