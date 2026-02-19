use crate::menu::item::{MenuItemState, MenuItemStateInput};
use leptos::prelude::Get;
use ui_headless::MenuItemKind;

pub const DEFAULT_ARIA_LABEL: &str = "Menu item";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuItemSelectionIndicator {
    Hidden,
    Checkbox,
    Radio,
}

impl MenuItemSelectionIndicator {
    pub fn marker(self, is_checked: bool) -> Option<&'static str> {
        match self {
            MenuItemSelectionIndicator::Hidden => None,
            MenuItemSelectionIndicator::Checkbox => is_checked.then_some("✓"),
            MenuItemSelectionIndicator::Radio => is_checked.then_some("●"),
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            MenuItemSelectionIndicator::Hidden => "hidden",
            MenuItemSelectionIndicator::Checkbox => "checkbox",
            MenuItemSelectionIndicator::Radio => "radio",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
}

pub fn resolve_selection_indicator(kind: MenuItemKind) -> MenuItemSelectionIndicator {
    match kind {
        MenuItemKind::Action => MenuItemSelectionIndicator::Hidden,
        MenuItemKind::Checkbox { .. } => MenuItemSelectionIndicator::Checkbox,
        MenuItemKind::Radio { .. } => MenuItemSelectionIndicator::Radio,
    }
}

pub fn resolve_kind_attr(kind: MenuItemKind) -> &'static str {
    match kind {
        MenuItemKind::Action => "action",
        MenuItemKind::Checkbox { .. } => "checkbox",
        MenuItemKind::Radio { .. } => "radio",
    }
}

pub fn resolve_kind_class(kind: MenuItemKind) -> &'static str {
    match kind {
        MenuItemKind::Action => "ui-menu-item--kind-action",
        MenuItemKind::Checkbox { .. } => "ui-menu-item--kind-checkbox",
        MenuItemKind::Radio { .. } => "ui-menu-item--kind-radio",
    }
}

pub fn resolve_state(input: MenuItemStateInput) -> MenuItemState {
    let indicator = resolve_selection_indicator(input.kind);

    let data_state_attr = if input.disabled {
        "disabled"
    } else if input.focused && input.is_checked {
        "focused-checked"
    } else if input.focused {
        "focused"
    } else if input.is_checked {
        "checked"
    } else {
        "idle"
    };

    MenuItemState {
        role_attr: input.kind.role(),
        kind_attr: resolve_kind_attr(input.kind),
        kind_class: resolve_kind_class(input.kind),
        is_checkable: indicator != MenuItemSelectionIndicator::Hidden,
        is_checked: input.is_checked,
        is_disabled: input.disabled,
        is_focused: input.focused,
        has_submenu: input.has_submenu,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        data_state_attr,
        aria_source_attr: if input.has_custom_aria_label {
            "custom"
        } else {
            "default"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: MenuItemState) -> String {
    let mut classes = vec!["ui-menu-item".to_string(), state.kind_class.into()];

    if state.is_checkable {
        classes.push("ui-menu-item--checkable".to_string());
    }

    if state.is_checked {
        classes.push("ui-menu-item--checked".to_string());
    }

    if state.is_disabled {
        classes.push("ui-menu-item--disabled".to_string());
    }

    if state.is_focused {
        classes.push("ui-menu-item--focused".to_string());
    }

    if state.has_submenu {
        classes.push("ui-menu-item--submenu".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-menu-item--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

pub fn resolve_checked(kind: MenuItemKind) -> bool {
    match kind {
        MenuItemKind::Action => false,
        MenuItemKind::Checkbox { is_checked } | MenuItemKind::Radio { is_checked } => {
            is_checked.get()
        }
    }
}

pub fn resolve_aria_checked(kind: MenuItemKind) -> Option<&'static str> {
    match kind {
        MenuItemKind::Action => None,
        MenuItemKind::Checkbox { is_checked } | MenuItemKind::Radio { is_checked } => {
            Some(if is_checked.get() { "true" } else { "false" })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  Rename profile  ".to_string())),
            Some("Rename profile".to_string())
        );

        assert_eq!(
            normalize_aria_label(Some("  Danger action  ".to_string())),
            ("Danger action".to_string(), true)
        );
        assert_eq!(
            normalize_aria_label(Some("\n\t".to_string())),
            (DEFAULT_ARIA_LABEL.into(), false)
        );
    }

    #[test]
    fn kind_contract_is_stable() {
        assert_eq!(
            resolve_selection_indicator(MenuItemKind::Action),
            MenuItemSelectionIndicator::Hidden
        );
        assert_eq!(
            resolve_selection_indicator(MenuItemKind::Checkbox {
                is_checked: Signal::derive(|| false),
            }),
            MenuItemSelectionIndicator::Checkbox
        );
        assert_eq!(
            resolve_selection_indicator(MenuItemKind::Radio {
                is_checked: Signal::derive(|| false),
            }),
            MenuItemSelectionIndicator::Radio
        );

        assert_eq!(resolve_kind_attr(MenuItemKind::Action), "action");
        assert_eq!(
            resolve_kind_class(MenuItemKind::Action),
            "ui-menu-item--kind-action"
        );
    }

    #[test]
    fn resolve_state_tracks_kind_and_state_sources() {
        let state = resolve_state(MenuItemStateInput {
            kind: MenuItemKind::Radio {
                is_checked: Signal::derive(|| true),
            },
            is_checked: true,
            disabled: false,
            focused: true,
            has_submenu: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        assert_eq!(state.role_attr, "menuitemradio");
        assert_eq!(state.kind_attr, "radio");
        assert!(state.is_checkable);
        assert!(state.is_checked);
        assert!(state.is_focused);
        assert!(state.has_submenu);
        assert_eq!(state.data_state_attr, "focused-checked");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let state = resolve_state(MenuItemStateInput {
            kind: MenuItemKind::Checkbox {
                is_checked: Signal::derive(|| true),
            },
            is_checked: true,
            disabled: false,
            focused: false,
            has_submenu: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-menu-item-custom".to_string()), state);

        for needle in [
            "ui-menu-item",
            "ui-menu-item--kind-checkbox",
            "ui-menu-item--checkable",
            "ui-menu-item--checked",
            "ui-menu-item--custom-class",
            "docs-menu-item-custom",
        ] {
            assert!(
                class_name.contains(needle),
                "MenuItem class list should include `{needle}`"
            );
        }
    }

    #[test]
    fn checked_and_aria_checked_reflect_kind_state() {
        let (checked, set_checked) = signal(true);
        let checkbox = MenuItemKind::Checkbox {
            is_checked: Signal::derive(move || checked.get()),
        };

        assert!(resolve_checked(checkbox));
        assert_eq!(resolve_aria_checked(checkbox), Some("true"));

        set_checked.set(false);
        assert!(!resolve_checked(checkbox));
        assert_eq!(resolve_aria_checked(checkbox), Some("false"));

        assert!(!resolve_checked(MenuItemKind::Action));
        assert_eq!(resolve_aria_checked(MenuItemKind::Action), None);
    }
}
