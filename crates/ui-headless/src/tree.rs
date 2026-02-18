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
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[test]
    fn root_attrs_expose_role_label_and_locale() {
        let attrs = tree_root_attrs(
            "Files".to_string(),
            Some(" en-US ".to_string()),
            Some(A11yDirection::Rtl),
        );
        assert_eq!(attrs.role, "tree");
        assert_eq!(attrs.aria_label, "Files");
        assert_eq!(attrs.lang.as_deref(), Some("en-US"));
        assert_eq!(attrs.dir, Some("rtl"));
    }

    #[test]
    fn tree_item_contract_wires_attrs_and_handlers() {
        let selected_calls = Arc::new(AtomicUsize::new(0));
        let toggle_calls = Arc::new(AtomicUsize::new(0));
        let selected_calls_2 = Arc::clone(&selected_calls);
        let toggle_calls_2 = Arc::clone(&toggle_calls);

        let contract = use_tree_item(
            TreeItemA11yInput {
                depth: 2,
                has_children: true,
                is_expanded: true,
                is_selected: false,
                is_disabled: false,
                is_tree_disabled: false,
                is_first_visible: false,
            },
            TreeItemOptions {
                on_select: Callback::new(move |_| {
                    selected_calls_2.fetch_add(1, Ordering::SeqCst);
                }),
                on_toggle: Some(Callback::new(move |_| {
                    toggle_calls_2.fetch_add(1, Ordering::SeqCst);
                })),
            },
        );

        assert_eq!(contract.attrs.role, "treeitem");
        assert_eq!(contract.attrs.aria_level, 3);
        assert_eq!(contract.attrs.aria_expanded, Some("true"));
        assert_eq!(contract.attrs.aria_selected, "false");
        assert_eq!(contract.attrs.aria_disabled, None);
        assert_eq!(contract.attrs.tabindex, -1);
        assert!(contract.state.is_interactive);
        assert!(contract.state.toggles_expansion);

        contract.handlers.on_click.run(());
        assert_eq!(selected_calls.load(Ordering::SeqCst), 1);
        assert_eq!(toggle_calls.load(Ordering::SeqCst), 1);

        let consumed = contract.handlers.on_key_down.run("Enter".to_string());
        assert!(consumed);
        assert_eq!(selected_calls.load(Ordering::SeqCst), 2);
        assert_eq!(toggle_calls.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn disabled_item_does_not_emit_interactions() {
        let selected_calls = Arc::new(AtomicUsize::new(0));
        let selected_calls_2 = Arc::clone(&selected_calls);

        let contract = use_tree_item(
            TreeItemA11yInput {
                depth: 0,
                has_children: false,
                is_expanded: false,
                is_selected: true,
                is_disabled: true,
                is_tree_disabled: false,
                is_first_visible: false,
            },
            TreeItemOptions {
                on_select: Callback::new(move |_| {
                    selected_calls_2.fetch_add(1, Ordering::SeqCst);
                }),
                on_toggle: None,
            },
        );

        assert!(!contract.state.is_interactive);
        assert!(!contract.state.toggles_expansion);
        assert_eq!(contract.attrs.aria_expanded, None);
        assert_eq!(contract.attrs.aria_selected, "true");
        assert_eq!(contract.attrs.aria_disabled, Some("true"));
        assert_eq!(contract.attrs.tabindex, 0);

        contract.handlers.on_click.run(());
        assert_eq!(selected_calls.load(Ordering::SeqCst), 0);
        assert!(!contract.handlers.on_key_down.run(" ".to_string()));
        assert_eq!(selected_calls.load(Ordering::SeqCst), 0);
    }
}
